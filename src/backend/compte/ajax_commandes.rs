use crate::backend::compte::get_commandes_user;
use crate::backend::utils::inject_auth;
use runique::prelude::*;

pub async fn compte_commandes_ajax(request: &mut Request) -> AppResult<Response> {
    inject_auth(request).await;
    let Some(user) = request.user.clone() else {
        return Ok((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "non connecté"})),
        )
            .into_response());
    };
    let page: u64 = request
        .get_query("page")
        .and_then(|p| p.parse().ok())
        .unwrap_or(1);
    let filtre = request.get_query("statut").unwrap_or("").to_string();
    let service = request.get_query("service").unwrap_or("").to_string();
    let db = request.db();
    let (commandes, page_courante, total_pages) =
        get_commandes_user(db, user.id, page, &filtre, &service).await;
    Ok(Json(serde_json::json!({
        "commandes": commandes,
        "page": page_courante,
        "total_pages": total_pages,
    }))
    .into_response())
}
