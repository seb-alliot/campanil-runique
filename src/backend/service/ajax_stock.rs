use crate::backend::service::charger_commandes::garde_acces;
use crate::entities::menu_traiteur;
use runique::prelude::*;
use sea_orm::{ActiveModelTrait, Set};

pub async fn ajax_stock_get(request: &Request) -> AppResult<Response> {
    if !garde_acces(request) {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "403"})),
        )
            .into_response());
    }

    let menus = search!(menu_traiteur::Entity => asc Titre,)
        .all(request.db())
        .await
        .unwrap_or_default();

    let data: Vec<_> = menus
        .iter()
        .map(|m| {
            serde_json::json!({
                "id":    m.id,
                "titre": m.titre,
                "stock": m.stock,
                "actif": m.actif,
            })
        })
        .collect();

    Ok(Json(serde_json::json!({ "menus": data })).into_response())
}

pub async fn ajax_stock_update(request: &mut Request) -> AppResult<Response> {
    if !garde_acces(request) {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "403"})),
        )
            .into_response());
    }

    let Some(id) = request.get_path_as::<i32>("id") else {
        return Ok(Json(serde_json::json!({ "ok": false })).into_response());
    };

    let Some(menu) = search!(menu_traiteur::Entity => Id eq id,)
        .first(request.db())
        .await
        .ok()
        .flatten()
    else {
        return Ok(Json(serde_json::json!({ "ok": false })).into_response());
    };

    let Some(data) = request.prisme.checked_data() else {
        return Ok(Json(serde_json::json!({ "ok": false })).into_response());
    };
    let new_stock = if let Some(delta_str) = data.get("delta")
        && let Ok(delta) = delta_str.parse::<i32>()
    {
        (menu.stock + delta).max(0)
    } else if let Some(stock_str) = data.get("stock")
        && let Ok(val) = stock_str.parse::<i32>()
    {
        val.max(0)
    } else {
        return Ok(Json(serde_json::json!({ "ok": false })).into_response());
    };

    let mut active: menu_traiteur::ActiveModel = menu.into();
    active.stock = Set(new_stock);
    if active.save(request.db()).await.is_err() {
        return Ok(Json(serde_json::json!({ "ok": false })).into_response());
    }

    Ok(Json(serde_json::json!({ "ok": true, "stock": new_stock })).into_response())
}
