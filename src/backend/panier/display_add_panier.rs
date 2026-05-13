use crate::backend::panier::{
    MenuChoixPanier, panier_ajouter, panier_ajouter_boisson, panier_ajouter_menu,
    panier_ajouter_supplement, panier_get,
};
use runique::prelude::*;

fn parse_choix(request: &Request, prefix: &str, cours: &str) -> Option<MenuChoixPanier> {
    let plat_id: i32 = request
        .get_query(&format!("{prefix}_plat_id"))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    if plat_id <= 0 {
        return None;
    }
    let cuisson = request
        .get_query(&format!("{prefix}_cuisson"))
        .filter(|v| !v.is_empty())
        .map(str::to_string);
    let garniture_ids: Vec<i32> = request
        .get_query(&format!("{prefix}_garniture_ids"))
        .map(|v| v.split(',').filter_map(|s| s.parse().ok()).collect())
        .unwrap_or_default();
    let avec_legumes =
        request.get_query(&format!("{prefix}_avec_legumes")).as_deref() == Some("1");
    let sans_sel = request.get_query(&format!("{prefix}_sans_sel")).as_deref() == Some("1");
    let note = request
        .get_query(&format!("{prefix}_note"))
        .filter(|v| !v.is_empty())
        .map(str::to_string);

    Some(MenuChoixPanier {
        cours: cours.to_string(),
        plat_id,
        plat_titre: String::new(), // rempli par add_menu via DB
        cuisson,
        garniture_ids,
        avec_legumes,
        sans_sel,
        note,
    })
}

pub async fn vue_ajouter_panier(request: Request) -> AppResult<Response> {
    let plat_id: i32 = request
        .get_query("plat_id")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let boisson_id: i32 = request
        .get_query("boisson_id")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let supplement_id: i32 = request
        .get_query("supplement_id")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let menu_id: i32 = request
        .get_query("menu_id")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let quantite: i32 = request
        .get_query("quantite")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
        .max(1);
    let cuisson = request
        .get_query("cuisson")
        .filter(|v| !v.is_empty())
        .map(str::to_string);
    let is_json = request.get_query("format").as_deref() == Some("json");
    let return_to = request
        .get_query("next")
        .map(str::to_string)
        .unwrap_or_else(|| "/carte".to_string());

    let user_id = request.user.as_ref().map(|u| u.id);

    let note = request
        .get_query("note")
        .filter(|v| !v.is_empty())
        .map(str::to_string);
    let garniture_ids: Vec<i32> = request
        .get_query("garniture_ids")
        .map(|v| v.split(',').filter_map(|s| s.parse().ok()).collect())
        .unwrap_or_default();
    let avec_legumes = request.get_query("avec_legumes").as_deref() == Some("1");
    let sans_sel = request.get_query("sans_sel").as_deref() == Some("1");

    if supplement_id > 0 {
        let _ = panier_ajouter_supplement(
            &request.session,
            &request.engine.db,
            supplement_id,
            quantite,
            user_id,
        )
        .await;
    } else if menu_id > 0 {
        let mut choix = Vec::new();
        if let Some(c) = parse_choix(&request, "entree", "entree") {
            choix.push(c);
        }
        if let Some(c) = parse_choix(&request, "plat", "plat") {
            choix.push(c);
        }
        if let Some(c) = parse_choix(&request, "dessert", "dessert") {
            choix.push(c);
        }
        let _ = panier_ajouter_menu(
            &request.session,
            &request.engine.db,
            menu_id,
            quantite,
            choix,
            user_id,
        )
        .await;
    } else if boisson_id > 0 {
        let _ = panier_ajouter_boisson(
            &request.session,
            &request.engine.db,
            boisson_id,
            quantite,
            user_id,
        )
        .await;
    } else if plat_id > 0 {
        let _ = panier_ajouter(
            &request.session,
            &request.engine.db,
            plat_id,
            quantite,
            cuisson,
            note,
            garniture_ids,
            avec_legumes,
            sans_sel,
            user_id,
        )
        .await;
    }

    if is_json {
        let nb = panier_get(&request.session).await.nb_articles();
        return Ok(Json(serde_json::json!({ "ok": true, "nb": nb })).into_response());
    }
    Ok(Redirect::to(&return_to).into_response())
}
