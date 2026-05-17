use crate::backend::{
    panier::{panier_get, panier_save, panier_vider},
    user::{authenticate_user, get_credentials, register_user},
    utils::inject_auth,
};
use crate::formulaire::{LoginForm, RegisterForm};

use runique::context;
use runique::prelude::*;

pub async fn handle_login(request: &mut Request, form: &mut LoginForm) -> AppResult<Response> {
    inject_auth(request).await;
    if is_authenticated(&request.session).await {
        return Ok(Redirect::to("/compte").into_response());
    }
    let template = "auth/connexion.html";
    if request.is_get() {
        context_update!(request => {
            "title"         => "Login",
            "login_form"    => form,
        });
        return request.render(template);
    }
    if request.is_post() && form.is_valid().await {
        let credentials = get_credentials(form);
        if let Some((username_val, password_val)) = &credentials
            && let Some(user) =
                authenticate_user(&request.engine.db, username_val, password_val).await
        {
            auth_login(&request.session, &request.engine.db, user.id)
                .await
                .ok();
            // Vider le panier seulement s'il appartenait à un autre compte.
            // Un panier guest (user_id: None) est conservé et rattaché au compte.
            let mut panier = panier_get(&request.session).await;
            if let Some(existing_uid) = panier.user_id {
                if existing_uid != user.id {
                    panier_vider(&request.session).await;
                }
            } else if !panier.lignes.is_empty() {
                panier.user_id = Some(user.id);
                panier_save(&request.session, &panier).await;
            }
            return Ok(Redirect::to("/compte").into_response());
        }
        context_update!(request => {
            "title"         => "Login",
            "login_form"    => form,
            "auth_error"    => &true,
            "messages"      => flash_now!(error => "Invalid credentials"),
        });
        return request.render(template);
    }
    context_update!(request => {
        "title"         => "Login",
        "login_form"    => form,
    });
    request.render(template)
}

pub async fn handle_inscription(
    request: &mut Request,
    form: &mut RegisterForm,
    headers: &HeaderMap,
) -> AppResult<Response> {
    inject_auth(request).await;
    if is_authenticated(&request.session).await {
        return Ok(Redirect::to("/compte").into_response());
    }

    let template = "auth/inscription.html";

    if request.is_get() {
        context_update!(request => {
            "title"            => "Créer un compte",
            "inscription_form" => &*form,
        });
        return request.render(template);
    }

    if request.is_post() && form.is_valid().await {
        match register_user(form, &request.engine.db).await {
            Ok(user) => {
                let token = reset_token::generate(&user.email);
                let encrypted = reset_token::encrypt_email(&token, &user.email);
                let base_url = headers
                    .get("host")
                    .and_then(|v| v.to_str().ok())
                    .map(|h| format!("http://{h}"))
                    .unwrap_or_else(|| "http://localhost:3000".to_string());
                let activate_url = format!("{}/activer/{}/{}", base_url, token, encrypted);

                if mailer_configured() {
                    let ctx = context! {
                        "username"     => &user.username,
                        "activate_url" => &activate_url,
                    };
                    if let Ok(msg) = Email::new()
                        .to(user.email.clone())
                        .subject("Activez votre compte — U Campanile")
                        .template(&request.engine.tera, "emails/bienvenu.html", ctx.into())
                    {
                        msg.send().await.ok();
                    }
                }

                return Ok(Redirect::to("/connexion").into_response());
            }
            Err(err) => {
                let msg = err.to_string();
                let is_unique =
                    msg.contains("unique") || msg.contains("UNIQUE") || msg.contains("Duplicate");
                if is_unique && msg.contains("email") {
                    form.get_form_mut()
                        .errors
                        .push("Un compte avec cet email existe déjà.".to_string());
                } else if is_unique && msg.contains("username") {
                    form.get_form_mut()
                        .errors
                        .push("Ce nom d'utilisateur est déjà pris.".to_string());
                } else {
                    form.get_form_mut().database_error(&err);
                }
            }
        }
    }

    context_update!(request => {
        "title"            => "Créer un compte",
        "inscription_form" => &*form,
    });
    request.render(template)
}
