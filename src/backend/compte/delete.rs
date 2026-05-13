use crate::backend::utils::inject_auth;
use crate::entities::commande;
use runique::prelude::*;

pub async fn handle_supprimer_compte(request: &mut Request) -> AppResult<Response> {
    if !request.is_post() {
        return Ok(Redirect::to("/compte").into_response());
    }
    inject_auth(request).await;
    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to("/connexion").into_response());
    };

    let db = request.db();

    commande::Entity::delete_many()
        .filter(commande::Column::UserId.eq(user.id))
        .exec(db)
        .await
        .ok();

    runique_users::Entity::delete_by_id(user.id)
        .exec(db)
        .await
        .ok();

    logout(&request.session, None).await.ok();
    request
        .notices
        .info("Votre compte a été supprimé.".to_string())
        .await;
    Ok(Redirect::to("/").into_response())
}
