use crate::backend::panier::{panier_get, panier_retirer, panier_retirer_boisson, panier_retirer_menu};
use runique::prelude::*;

pub async fn vue_retirer_panier(request: Request) -> AppResult<Response> {
    let plat_id: i32 = request
        .get_query("plat_id")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let boisson_id: i32 = request
        .get_query("boisson_id")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let menu_id: i32 = request
        .get_query("menu_id")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let is_json = request.get_query("format").as_deref() == Some("json");

    if boisson_id > 0 {
        panier_retirer_boisson(&request.session, boisson_id).await;
    } else if menu_id > 0 {
        panier_retirer_menu(&request.session, menu_id).await;
    } else if plat_id > 0 {
        panier_retirer(&request.session, plat_id).await;
    }

    if is_json {
        let panier = panier_get(&request.session).await;
        let nb = panier.nb_articles();
        let total = format!("{:.2}", panier.total());
        return Ok(
            Json(serde_json::json!({ "ok": true, "nb": nb, "total": total })).into_response(),
        );
    }
    Ok(Redirect::to("/panier").into_response())
}
