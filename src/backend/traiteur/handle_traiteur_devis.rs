use crate::backend::utils::inject_auth;
use crate::entities::{devis_traiteur, info_resto, menu_traiteur};
use crate::formulaire::DevisTraiteurForm;
use runique::auth::session::UserEntity;
use runique::auth::user::BuiltinUserEntity;
use runique::context;
use runique::prelude::*;
use sea_orm::{ActiveModelTrait, Set};

pub async fn handle_devis_traiteur(
    request: &mut Request,
    form: &mut DevisTraiteurForm,
) -> AppResult<Response> {
    inject_auth(request).await;

    let Some(current_user) = request.user.clone() else {
        return Ok(Redirect::to("/connexion").into_response());
    };

    let db = request.db();
    let user_full = BuiltinUserEntity::find_by_id(db, current_user.id).await;

    let template = "traiteur/devis.html";

    let menu_id = request
        .get_query("menu_id")
        .and_then(|s| s.parse::<Pk>().ok());

    let menu_model = if let Some(id) = menu_id {
        search!(menu_traiteur::Entity => Id eq id,)
            .first(request.db())
            .await
            .ok()
            .flatten()
    } else {
        None
    };

    let user_email = user_full.as_ref().map(|u| u.email.as_str()).unwrap_or("");

    if request.is_get() {
        context_update!(request => {
            "title"      => "Demande de devis — U Campanile",
            "devis_form" => &*form,
            "menu"       => &menu_model,
        });
        return request.render(template);
    }

    if request.is_post() && form.is_valid().await {
        let nom = current_user.username.clone();
        let email = user_email.to_string();
        let telephone = form.cleaned_string("telephone").filter(|s| !s.is_empty());
        let date_evenement = form
            .cleaned_naive_date("date_evenement")
            .unwrap_or_default();
        let nb_personnes = form.cleaned_i32("nb_personnes").unwrap_or(0);

        if let Some(menu) = &menu_model
            && nb_personnes < menu.nb_personnes_min
        {
            if let Some(f) = form.get_form_mut().fields.get_mut("nb_personnes") {
                f.set_error(format!(
                    "Ce menu nécessite au moins {} personnes.",
                    menu.nb_personnes_min
                ));
            }
            context_update!(request => {
                "title"      => "Demande de devis — U Campanile",
                "devis_form" => &*form,
                "menu"       => &menu_model,
            });
            return request.render(template);
        }
        let message = form.cleaned_string("message").unwrap_or_default();

        let (prix_total, remise_appliquee) = menu_model
            .as_ref()
            .map(|menu| {
                let base = menu.prix_par_personne * Decimal::from(nb_personnes);
                if let Some(remise) = menu.remise_groupe
                    && remise > Decimal::ZERO
                    && nb_personnes >= menu.remise_groupe_min.unwrap_or(0)
                {
                    (
                        Some(base * (Decimal::ONE - remise / Decimal::from(100))),
                        Some(remise),
                    )
                } else {
                    (Some(base), None)
                }
            })
            .unwrap_or((None, None));

        let model = devis_traiteur::ActiveModel {
            user_id: Set(current_user.id),
            nom: Set(nom.clone()),
            email: Set(email.clone()),
            telephone: Set(telephone.clone()),
            date_evenement: Set(date_evenement),
            nb_personnes: Set(nb_personnes),
            message: Set(message.clone()),
            prix_total: Set(prix_total),
            remise_appliquee: Set(remise_appliquee),
            menu_id: Set(menu_id),
            statut: Set(devis_traiteur::StatutDevis::EnAttente),
            ..Default::default()
        };

        match model.insert(request.db()).await {
            Ok(_) => {
                if let Some(resto) = search!(info_resto::Entity)
                    .first(request.db())
                    .await
                    .ok()
                    .flatten()
                    && let Some(dest) = resto.email.filter(|e| !e.is_empty())
                {
                    let menu_titre = menu_model
                        .as_ref()
                        .map(|m| m.titre.as_str())
                        .unwrap_or("Non spécifié");
                    let ctx = context! {
                        "nom"            => &nom,
                        "email"          => &email,
                        "telephone"      => telephone.as_deref().unwrap_or("—"),
                        "date_evenement" => &date_evenement,
                        "nb_personnes"   => nb_personnes,
                        "message"        => &message,
                        "menu_titre"     => menu_titre,
                        "nom_resto"      => &resto.nom,
                    };
                    if let Ok(email_msg) = Email::new()
                        .to(&dest)
                        .subject(format!("Demande de devis traiteur — {nom}"))
                        .reply_to(&email)
                        .template(
                            &request.engine.tera,
                            "emails/devis_notification.html",
                            ctx.into(),
                        )
                    {
                        let _ = email_msg.send().await;
                    }
                }
                return Ok(Redirect::to("/traiteur/devis/confirmation").into_response());
            }
            Err(err) => form.get_form_mut().database_error(&err),
        }
    }

    context_update!(request => {
        "title"      => "Demande de devis — U Campanile",
        "devis_form" => &*form,
        "menu"       => &menu_model,
    });
    request.render(template)
}
