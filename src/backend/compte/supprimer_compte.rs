use crate::backend::utils::inject_auth;
use crate::entities::{
    avis, commande, commande_ligne, commande_ligne_garniture, commande_menu_choix, commande_statut,
};
use runique::prelude::*;

pub async fn handle_supprimer_compte(request: &mut Request) -> AppResult<Response> {
    if !request.is_post() {
        return Ok(Redirect::to("/compte").into_response());
    }
    inject_auth(request).await;
    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to("/connexion").into_response());
    };

    if !request.prisme.csrf_valid {
        return Ok(Redirect::to("/compte").into_response());
    }

    let db = request.db();

    let cmd_ids: Vec<Pk> = search!(commande::Entity => UserId eq user.id,)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|c| c.id)
        .collect();

    if !cmd_ids.is_empty() {
        let ligne_ids: Vec<Pk> = search!(commande_ligne::Entity => CommandeId in (cmd_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|l| l.id)
            .collect();

        if !ligne_ids.is_empty() {
            commande_ligne_garniture::Entity::delete_many()
                .filter(commande_ligne_garniture::Column::CommandeLigneId.is_in(ligne_ids.clone()))
                .exec(db)
                .await
                .ok();
            commande_menu_choix::Entity::delete_many()
                .filter(commande_menu_choix::Column::CommandeLigneId.is_in(ligne_ids))
                .exec(db)
                .await
                .ok();
        }

        commande_ligne::Entity::delete_many()
            .filter(commande_ligne::Column::CommandeId.is_in(cmd_ids.clone()))
            .exec(db)
            .await
            .ok();

        commande_statut::Entity::delete_many()
            .filter(commande_statut::Column::CommandeId.is_in(cmd_ids.clone()))
            .exec(db)
            .await
            .ok();

        avis::Entity::delete_many()
            .filter(avis::Column::CommandeId.is_in(cmd_ids.clone()))
            .exec(db)
            .await
            .ok();

        commande::Entity::delete_many()
            .filter(commande::Column::UserId.eq(user.id))
            .exec(db)
            .await
            .ok();
    }

    if runique_users::Entity::delete_by_id(user.id)
        .exec(db)
        .await
        .is_err()
    {
        request
            .notices
            .error("Impossible de supprimer le compte. Contactez le restaurant.".to_string())
            .await;
        return Ok(Redirect::to("/compte").into_response());
    }

    logout(&request.session, None).await.ok();
    request
        .notices
        .info("Votre compte a été supprimé.".to_string())
        .await;
    Ok(Redirect::to("/").into_response())
}
