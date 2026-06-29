use crate::backend::load_stats;
use crate::backend::service::charger_commandes::garde_acces;
use runique::axum::http::{HeaderValue, header::CACHE_CONTROL};
use runique::prelude::*;

pub async fn ajax_stats(request: &Request) -> AppResult<Response> {
    if !garde_acces(request) {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "403"})),
        )
            .into_response());
    }
    let periode: u32 = request
        .get_query("periode")
        .and_then(|v| v.parse().ok())
        .unwrap_or(7)
        .clamp(1, 365);
    let stats = load_stats(request, periode).await;
    let mut response = Json(stats).into_response();
    response
        .headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
    Ok(response)
}
