use runique::prelude::*;

pub async fn handle_activate(request: &mut Request, token: String) -> AppResult<Response> {
    let Some(user_id) = reset_token::consume(&request.engine.db, &token).await else {
        warning!(request.notices => "Lien d'activation invalide ou expiré.");
        return Ok(Redirect::to("/login").into_response());
    };

    let db = request.engine.db.clone();

    // `is_active` and `activated_at` together: the database refuses one without the other.
    let activated = match BuiltinUserEntity::activate_account(&db, user_id).await {
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

    success!(request.notices => format!("Bienvenue {} ! Votre compte est activé.", activated.username));
    Ok(Redirect::to("/compte").into_response())
}
