use crate::backend::utils::inject_auth;
use crate::formulaire::{LoginForm, RegisterForm};
use runique::prelude::runique_users::ActiveModel as UserActiveModel;
use runique::prelude::runique_users::Entity as UserEntity;
use runique::prelude::*;

pub async fn register_user(
    form: &RegisterForm,
    db: &sea_orm::DatabaseConnection,
) -> Result<runique::prelude::runique_users::Model, sea_orm::DbErr> {
    form.save(db).await
}

pub async fn authenticate_user(
    db: &sea_orm::DatabaseConnection,
    username: &str,
    password: &str,
) -> Option<runique::prelude::runique_users::Model> {
    let user = find_user_by_username(db, username).await?;
    if user.is_active && verify(password, &user.password) {
        Some(user)
    } else {
        None
    }
}

pub async fn find_user_by_username(
    db: &sea_orm::DatabaseConnection,
    username: &str,
) -> Option<runique::prelude::runique_users::Model> {
    let query = search!(UserEntity => Username eq username.trim());
    query.first(db).await.unwrap_or(None)
}

pub async fn find_user_by_id(
    db: &sea_orm::DatabaseConnection,
    id: runique::utils::pk::Pk,
) -> Option<runique::prelude::runique_users::Model> {
    UserEntity::find_by_id(id).one(db).await.unwrap_or(None)
}

pub fn get_credentials(form: &LoginForm) -> Option<(String, String)> {
    let u = form.cleaned_string("username")?;
    let p = form.cleaned_string("password")?;
    Some((u, p))
}

pub async fn get_profile_user(
    user_id: Option<runique::utils::pk::Pk>,
    db: &sea_orm::DatabaseConnection,
) -> Option<runique::prelude::runique_users::Model> {
    find_user_by_id(db, user_id?).await
}

// --- Handlers ---

pub async fn handle_login(request: &mut Request, form: &mut LoginForm) -> AppResult<Response> {
    inject_auth(request).await;
    if is_authenticated(&request.session).await {
        return Ok(Redirect::to("/compte").into_response());
    }

    let template = "connexion.html";

    if request.is_get() {
        context_update!(request => {
            "title"      => "Connexion",
            "login_form" => &*form,
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
            success!(request.notices => format!("Bienvenue {} !", user.username));
            return Ok(Redirect::to("/compte").into_response());
        }

        context_update!(request => {
            "title"      => "Connexion",
            "login_form" => form,
            "auth_error" => &true,
        });
        return request.render(template);
    }

    context_update!(request => {
        "title"      => "Connexion",
        "login_form" => form,
    });
    request.render(template)
}

pub async fn handle_inscription(
    request: &mut Request,
    form: &mut RegisterForm,
) -> AppResult<Response> {
    inject_auth(request).await;
    if is_authenticated(&request.session).await {
        return Ok(Redirect::to("/compte").into_response());
    }

    let template = "inscription.html";

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
                let base_url = std::env::var("APP_URL")
                    .unwrap_or_else(|_| "http://localhost:3000".to_string());
                let activate_url = format!("{}/activer/{}/{}", base_url, token, encrypted);

                if mailer_configured() {
                    Email::new()
                        .to(user.email.clone())
                        .subject("Activez votre compte — U Campanile")
                        .html(format!(
                            "<p>Bonjour {},</p><p>Cliquez sur ce lien pour activer votre compte :</p><p><a href=\"{}\">Activer mon compte</a></p>",
                            user.username, activate_url
                        ))
                        .send()
                        .await
                        .ok();
                }

                success!(request.notices => "Compte créé ! Consultez vos emails pour l'activer.");
                return Ok(Redirect::to("/connexion").into_response());
            }
            Err(err) => form.get_form_mut().database_error(&err),
        }
    }

    context_update!(request => {
        "title"            => "Créer un compte",
        "inscription_form" => &*form,
    });
    request.render(template)
}

pub async fn handle_activate(request: &mut Request) -> AppResult<Response> {
    let token = request.path_param("token").unwrap_or_default().to_string();
    let encrypted_email = request
        .path_param("encrypted_email")
        .unwrap_or_default()
        .to_string();
    if !reset_token::peek(&token) {
        warning!(request.notices => "Lien d'activation invalide ou expiré.");
        return Ok(Redirect::to("/connexion").into_response());
    }

    let Some(email) = reset_token::decrypt_email(&token, &encrypted_email) else {
        warning!(request.notices => "Lien d'activation invalide.");
        return Ok(Redirect::to("/connexion").into_response());
    };

    reset_token::consume(&token);

    let db = request.engine.db.clone();
    let query = search!(UserEntity => Email eq email.trim());
    let Some(user) = query.first(&db).await.unwrap_or(None) else {
        warning!(request.notices => "Compte introuvable.");
        return Ok(Redirect::to("/connexion").into_response());
    };

    let active_model = UserActiveModel {
        id: Set(user.id),
        is_active: Set(true),
        ..Default::default()
    };
    if active_model.update(&*db).await.is_err() {
        warning!(request.notices => "Erreur lors de l'activation.");
        return Ok(Redirect::to("/connexion").into_response());
    }

    auth_login(&request.session, &db, user.id).await.ok();
    success!(request.notices => format!("Bienvenue {} ! Votre compte est activé.", user.username));
    Ok(Redirect::to("/compte").into_response())
}
