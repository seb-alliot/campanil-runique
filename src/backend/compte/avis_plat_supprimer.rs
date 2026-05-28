use crate::backend::utils::inject_auth;
use crate::entities::avis_plat;
use runique::axum;
use runique::prelude::*;
use serde_json::json;

pub async fn handle_avis_plat_supprimer(request: Request) -> AppResult<Response> {
    let mut request = request;
    inject_auth(&mut request).await;

    let Some(user) = request.user.clone() else {
        return Ok(axum::Json(json!({"ok": false, "error": "non authentifié"})).into_response());
    };

    if !request.prisme.csrf_valid {
        return Ok(axum::Json(json!({"ok": false, "error": "csrf"})).into_response());
    }

    let plat_id: Pk = request.get_path_as::<Pk>("plat_id").unwrap_or(0);
    if plat_id == 0 {
        return Ok(axum::Json(json!({"ok": false, "error": "plat invalide"})).into_response());
    }

    let existant = search!(avis_plat::Entity => PlatId eq plat_id, UserId eq user.id,)
        .first(request.db())
        .await
        .ok()
        .flatten();

    let Some(avis) = existant else {
        return Ok(axum::Json(json!({"ok": false, "error": "avis introuvable"})).into_response());
    };

    avis_plat::Entity::delete_by_id(avis.id)
        .exec(request.db())
        .await
        .ok();

    Ok(axum::Json(json!({"ok": true})).into_response())
}
