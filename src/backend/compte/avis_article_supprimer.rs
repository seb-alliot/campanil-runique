use crate::backend::utils::inject_auth;
use crate::entities::avis_plat;
use runique::axum;
use runique::prelude::*;
use serde_json::json;

pub async fn handle_avis_article_supprimer(request: Request) -> AppResult<Response> {
    let mut request = request;
    inject_auth(&mut request).await;

    let Some(user) = request.user.clone() else {
        return Ok(axum::Json(json!({"ok": false, "error": "non authentifié"})).into_response());
    };

    if !request.prisme.csrf_valid {
        return Ok(axum::Json(json!({"ok": false, "error": "csrf"})).into_response());
    }

    let type_article = request.get_path("type_article").unwrap_or("").to_string();
    let article_id: Pk = request.get_path_as::<Pk>("id").unwrap_or(0);
    if article_id == 0 {
        return Ok(axum::Json(json!({"ok": false, "error": "article invalide"})).into_response());
    }

    let existant = match type_article.as_str() {
        "plat" => search!(avis_plat::Entity => PlatId eq article_id, UserId eq user.id,)
            .first(request.db())
            .await
            .ok()
            .flatten(),
        "entree" => search!(avis_plat::Entity => EntreeId eq article_id, UserId eq user.id,)
            .first(request.db())
            .await
            .ok()
            .flatten(),
        "dessert" => search!(avis_plat::Entity => DessertId eq article_id, UserId eq user.id,)
            .first(request.db())
            .await
            .ok()
            .flatten(),
        _ => None,
    };

    let Some(avis) = existant else {
        return Ok(axum::Json(json!({"ok": false, "error": "avis introuvable"})).into_response());
    };

    avis_plat::Entity::delete_by_id(avis.id)
        .exec(request.db())
        .await
        .ok();

    Ok(axum::Json(json!({"ok": true})).into_response())
}
