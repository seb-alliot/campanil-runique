use crate::backend::utils::inject_auth;
use crate::entities::{contact, info_resto};
use crate::formulaire::ContactForm;
use runique::context;
use runique::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

pub async fn handle_contact(request: &mut Request, form: ContactForm) -> AppResult<Response> {
    inject_auth(request).await;
    let template = "contact.html";

    let mut validated = match ValidationForm::try_new(form, request).await {
        Ok(validated) => validated,
        Err(form) => {
            match crate::backend::form_error_flash(&form) {
                Some(messages) => context_update!(request => {
                    "title"        => "Contact — U Campanile",
                    "contact_form" => &form,
                    "messages"     => messages,
                }),
                None => context_update!(request => {
                    "title"        => "Contact — U Campanile",
                    "contact_form" => &form,
                }),
            }
            return request.render(template);
        }
    };

    let raison_mail = match validated.cleaned_string("raison").as_deref() {
        Some("reservation") => contact::RaisonContact::Reservation,
        Some("traiteur") => contact::RaisonContact::Traiteur,
        Some("commande") => contact::RaisonContact::Commande,
        _ => contact::RaisonContact::Autre,
    };
    let titre_mail = validated.cleaned_string("titre").unwrap_or_default();
    let email_mail = validated.cleaned_string("email").unwrap_or_default();
    let description_mail = validated.cleaned_string("description").unwrap_or_default();

    let model = contact::ActiveModel {
        raison: Set(raison_mail),
        titre: Set(titre_mail.clone()),
        email: Set(email_mail.clone()),
        description: Set(description_mail.clone()),
        ..Default::default()
    };

    match model.insert(request.db()).await {
        Ok(_) => {
            if let Some(resto) = info_resto::Entity::find()
                .one(request.db())
                .await
                .unwrap_or(None)
                && let Some(dest) = resto.email.filter(|e| !e.is_empty())
            {
                let ctx = context! {
                    "titre"      => &titre_mail,
                    "email"      => &email_mail,
                    "message"    => &description_mail,
                    "nom_resto"  => &resto.nom,
                };
                if let Ok(email_msg) = Email::new()
                    .to(&dest)
                    .subject(format!("Nouveau message : {titre_mail}"))
                    .reply_to(&email_mail)
                    .template(
                        &request.engine.tera,
                        "emails/contact_notification.html",
                        ctx.into(),
                    )
                {
                    let _ = email_msg.send().await;
                }
            }
            success!(request.notices => "Votre message a bien été envoyé. Nous vous répondrons dans les meilleurs délais.");
            return Ok(Redirect::to("/contact").into_response());
        }
        Err(err) => validated.database_error(&err),
    }

    context_update!(request => {
        "title"        => "Contact — U Campanile",
        "contact_form" => &*validated,
    });
    request.render(template)
}
