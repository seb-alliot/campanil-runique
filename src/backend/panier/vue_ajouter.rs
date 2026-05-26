use crate::backend::panier::{
    MenuChoixPanier, PanierAjouterParams, panier_ajouter, panier_ajouter_boisson,
    panier_ajouter_menu, panier_ajouter_supplement, panier_get,
};
use runique::prelude::*;

fn parse_choix(request: &Request, prefix: &str, cours: &str) -> Option<MenuChoixPanier> {
    let plat_id: Pk = request
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
    let garniture_ids: Vec<Pk> = request
        .get_query(&format!("{prefix}_garniture_ids"))
        .map(|v| v.split(',').filter_map(|s| s.parse().ok()).collect())
        .unwrap_or_default();
    let sans_sel = request.get_query(&format!("{prefix}_sans_sel")) == Some("1");
    let note = request
        .get_query(&format!("{prefix}_note"))
        .filter(|v| !v.is_empty())
        .map(str::to_string);

    Some(MenuChoixPanier {
        cours: cours.to_string(),
        plat_id,
        plat_titre: String::new(),
        cuisson,
        garniture_ids,
        sans_sel,
        note,
    })
}

pub async fn vue_ajouter_panier(request: Request) -> AppResult<Response> {
    let plat_id: Pk = request
        .get_query("plat_id")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let boisson_id: Pk = request
        .get_query("boisson_id")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let supplement_id: Pk = request
        .get_query("supplement_id")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let menu_id: Pk = request
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
    let is_json = request.get_query("format") == Some("json");
    let return_to = request
        .get_query("next")
        .filter(|v| v.starts_with('/') && !v.starts_with("//"))
        .map(str::to_string)
        .unwrap_or_else(|| "/carte".to_string());

    let user_id = request.user.as_ref().map(|u| u.id);
    let type_article = request
        .get_query("type_article")
        .filter(|v| matches!(*v, "entree" | "dessert" | "plat"))
        .map(str::to_string)
        .unwrap_or_else(|| "plat".to_string());

    let note = request
        .get_query("note")
        .filter(|v| !v.is_empty())
        .map(str::to_string);
    let garniture_ids: Vec<Pk> = request
        .get_query("garniture_ids")
        .map(|v| v.split(',').filter_map(|s| s.parse().ok()).collect())
        .unwrap_or_default();
    let sans_sel = request.get_query("sans_sel") == Some("1");
    let supplement_ids: Vec<Pk> = request
        .get_query("supplement_ids")
        .map(|v| v.split(',').filter_map(|s| s.parse().ok()).collect())
        .unwrap_or_default();

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
            &request,
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
            &request,
            &request.session,
            &request.engine.db,
            boisson_id,
            quantite,
            user_id,
        )
        .await;
    } else if plat_id > 0 {
        let _ = panier_ajouter(
            &request,
            PanierAjouterParams {
                plat_id,
                type_article,
                quantite,
                cuisson,
                note,
                garniture_ids,
                sans_sel,
                user_id,
            },
        )
        .await;
        for sid in supplement_ids {
            let _ = panier_ajouter_supplement(
                &request.session,
                &request.engine.db,
                sid,
                quantite,
                user_id,
            )
            .await;
        }
    }

    if is_json {
        let nb = panier_get(&request.session).await.nb_articles();
        return Ok(Json(serde_json::json!({ "ok": true, "nb": nb })).into_response());
    }
    Ok(Redirect::to(&return_to).into_response())
}
