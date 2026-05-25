use crate::backend::utils::inject_auth;
use crate::entities::{commande, commande_ligne};
use runique::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

pub async fn handle_supprimer_ligne(mut request: Request) -> AppResult<Response> {
    inject_auth(&mut request).await;
    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to("/connexion").into_response());
    };
    if !request.prisme.csrf_valid {
        return Ok(Redirect::to("/compte").into_response());
    }

    let numero = request.get_path("numero").unwrap_or("").to_string();
    let ligne_id: Pk = request
        .get_path("ligne_id")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let db = request.db();

    let Some(cmd) = search!(commande::Entity => Numero eq &numero, UserId eq user.id,)
        .first(db)
        .await
        .unwrap_or(None)
    else {
        return Ok(Redirect::to("/compte").into_response());
    };

    if !cmd.modifiable {
        return Ok(Redirect::to("/compte").into_response());
    }

    let Some(ligne) = search!(commande_ligne::Entity => Id eq ligne_id, CommandeId eq cmd.id,)
        .first(db)
        .await
        .unwrap_or(None)
    else {
        return Ok(Redirect::to("/compte").into_response());
    };

    let prix_ligne = ligne.prix_unitaire * Decimal::from(ligne.quantite);
    commande_ligne::Entity::delete_by_id(ligne.id)
        .exec(db)
        .await
        .ok();

    let nouveau_total = cmd.prix_total - prix_ligne;
    commande::ActiveModel {
        id: Set(cmd.id),
        prix_total: Set(nouveau_total),
        ..Default::default()
    }
    .update(db)
    .await
    .ok();

    Ok(Json(serde_json::json!({
        "ok": true,
        "nouveau_total": format!("{:.2}", nouveau_total),
    }))
    .into_response())
}
