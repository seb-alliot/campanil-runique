use crate::backend::carte::db_boissons::{get_boissons_par_type, slug_to_type};
use crate::backend::utils::inject_auth;
use runique::prelude::*;

pub async fn vue_boissons_type(request: &mut Request) -> AppResult<Response> {
    inject_auth(request).await;

    let type_slug = match request.get_path("type") {
        Some(s) => s.to_string(),
        None => return Ok((StatusCode::NOT_FOUND, "Type introuvable").into_response()),
    };

    let (type_label, boissons) = if let Some(type_val) = slug_to_type(&type_slug) {
        let label = type_val.to_string();
        let items = get_boissons_par_type(&request.engine.db, type_val).await;
        (label, items)
    } else {
        (type_slug.replace('-', " "), vec![])
    };

    context_update!(request => {
        "title"      => format!("{} — U Campanile", type_label),
        "type_label" => type_label,
        "boissons"   => boissons,
    });
    request.render("menus/boisson_type.html")
}
