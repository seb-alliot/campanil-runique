use runique::prelude::runique_users::Entity as UserEntity;
use runique::prelude::*;

pub async fn handle_activate(
    request: &mut Request,
    token: String,
    encrypted_email: String,
) -> AppResult<Response> {
    if !reset_token::peek(&request.engine.db, &token).await {
        warning!(request.notices => "Lien d'activation invalide ou expiré.");
        return Ok(Redirect::to("/login").into_response());
    }

    let Some(email) = reset_token::decrypt_email(&token, &encrypted_email) else {
        warning!(request.notices => "Lien d'activation invalide.");
        return Ok(Redirect::to("/login").into_response());
    };

    let _ = reset_token::consume(&request.engine.db, &token).await;

    let db = request.engine.db.clone();

    // Find the user by email
    let query = search!(UserEntity => Email eq email.trim());
    let Some(user) = query.first(&db).await.unwrap_or(None) else {
        warning!(request.notices => "Compte introuvable.");
        return Ok(Redirect::to("/login").into_response());
    };

    // `is_active` and `activated_at` together: the database refuses one without the other.
    let activated = match BuiltinUserEntity::activate_account(&db, user.id).await {
        Ok(Some(activated)) => activated,
        Ok(None) => {
            info!(request.notices => "Ce compte est déjà activé, connectez-vous.");
            return Ok(Redirect::to("/login").into_response());
        }
        Err(_) => {
            warning!(request.notices => "Erreur lors de l'activation.");
            return Ok(Redirect::to("/login").into_response());
        }
    };

    // Directly log in
    login(&request.session, &activated, None, false).await.ok();

    success!(request.notices => format!("Bienvenue {} ! Votre compte est activé.", user.username));
    Ok(Redirect::to("/compte").into_response())
}
