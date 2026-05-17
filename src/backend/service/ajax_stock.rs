use crate::backend::service::charger_commandes::garde_acces;
use crate::entities::menu;
use runique::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

pub async fn ajax_stock_get(request: &Request) -> AppResult<Response> {
    if !garde_acces(request) {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "403"})),
        )
            .into_response());
    }
    let db = request.db();
    let menus = menu::Entity::find().all(db).await.unwrap_or_default();

    let list: Vec<_> = menus
        .into_iter()
        .map(|m| {
            serde_json::json!({
                "id":    m.id,
                "titre": m.titre,
                "stock": m.stock,
                "actif": m.actif,
            })
        })
        .collect();

    Ok(Json(serde_json::json!({ "menus": list })).into_response())
}

pub async fn ajax_stock_update(request: &mut Request) -> AppResult<Response> {
    if !garde_acces(request) {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "403"})),
        )
            .into_response());
    }
    if !request.prisme.csrf_valid {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "csrf"})),
        )
            .into_response());
    }

    let id: Pk = match request.get_path("id").and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => {
            return Ok((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "id invalide"})),
            )
                .into_response());
        }
    };

    let delta: i32 = request
        .prisme
        .data
        .get("delta")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let stock_direct: Option<i32> = request
        .prisme
        .data
        .get("stock")
        .and_then(|s| s.parse().ok());

    let db = request.db();
    let Some(m) = menu::Entity::find_by_id(id).one(db).await.ok().flatten() else {
        return Ok((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "introuvable"})),
        )
            .into_response());
    };

    let new_stock = stock_direct.unwrap_or_else(|| (m.stock + delta).max(0));

    menu::ActiveModel {
        id: Set(m.id),
        stock: Set(new_stock),
        ..Default::default()
    }
    .update(db)
    .await
    .ok();

    Ok(Json(serde_json::json!({ "ok": true, "stock": new_stock })).into_response())
}
