use crate::backend::service::charger_commandes::garde_acces;
use runique::prelude::*;

pub async fn ajax_stock_get(request: &Request) -> AppResult<Response> {
    if !garde_acces(request) {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "403"})),
        )
            .into_response());
    }
    Ok(Json(serde_json::json!({ "menus": [] })).into_response())
}

pub async fn ajax_stock_update(request: &mut Request) -> AppResult<Response> {
    if !garde_acces(request) {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "403"})),
        )
            .into_response());
    }
    Ok(Json(serde_json::json!({ "ok": false, "error": "stock non applicable" })).into_response())
}
