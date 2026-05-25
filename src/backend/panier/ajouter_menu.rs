use crate::backend::menus::PlatDetail;
use crate::backend::panier::{LignePanier, MenuChoixPanier, panier_get, panier_save};
use crate::backend::stats::get_plat_views;
use crate::entities::{dessert, entree, menu, plat};
use runique::prelude::*;
use std::collections::HashMap;

pub async fn panier_ajouter_menu(
    request: &Request,
    session: &Session,
    db: &sea_orm::DatabaseConnection,
    menu_id: Pk,
    quantite: i32,
    choix: Vec<MenuChoixPanier>,
    user_id: Option<Pk>,
) -> Result<(), &'static str> {
    let Some(m) = menu::Entity::find_by_id(menu_id)
        .one(db)
        .await
        .ok()
        .flatten()
    else {
        return Err("Menu introuvable");
    };

    let plat_ids: Vec<Pk> = choix
        .iter()
        .filter(|c| c.cours == "plat")
        .map(|c| c.plat_id)
        .collect();
    let entree_ids: Vec<Pk> = choix
        .iter()
        .filter(|c| c.cours == "entree")
        .map(|c| c.plat_id)
        .collect();
    let dessert_ids: Vec<Pk> = choix
        .iter()
        .filter(|c| c.cours == "dessert")
        .map(|c| c.plat_id)
        .collect();

    let plats_map: HashMap<Pk, plat::Model> = if !plat_ids.is_empty() {
        search!(plat::Entity => Id in (plat_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|p| (p.id, p))
            .collect()
    } else {
        HashMap::new()
    };

    let entrees_map: HashMap<Pk, entree::Model> = if !entree_ids.is_empty() {
        search!(entree::Entity => Id in (entree_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|e| (e.id, e))
            .collect()
    } else {
        HashMap::new()
    };

    let desserts_map: HashMap<Pk, dessert::Model> = if !dessert_ids.is_empty() {
        search!(dessert::Entity => Id in (dessert_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|d| (d.id, d))
            .collect()
    } else {
        HashMap::new()
    };

    let menu_choix: Vec<MenuChoixPanier> = choix
        .into_iter()
        .map(|mut c| {
            c.plat_titre = match c.cours.as_str() {
                "entree" => entrees_map
                    .get(&c.plat_id)
                    .map(|e| e.label.clone().unwrap_or(e.titre.clone()))
                    .unwrap_or_default(),
                "dessert" => desserts_map
                    .get(&c.plat_id)
                    .map(|d| d.label.clone().unwrap_or(d.titre.clone()))
                    .unwrap_or_default(),
                _ => plats_map
                    .get(&c.plat_id)
                    .map(|p| p.label.clone().unwrap_or(p.titre.clone()))
                    .unwrap_or_default(),
            };
            c
        })
        .collect();

    let mut panier = panier_get(session).await;
    if panier.user_id.is_none() {
        panier.user_id = user_id;
    }
    panier.lignes.push(LignePanier {
        plat_id: 0,
        type_article: "menu".to_string(),
        boisson_id: None,
        menu_id: Some(menu_id),
        supplement_id: None,
        titre: m.nom,
        prix_unitaire: format!("{:.2}", m.prix),
        quantite,
        est_viande: false,
        cuisson: None,
        note: None,
        garniture_ids: vec![],
        sans_sel: false,
        menu_choix,
    });

    panier_save(session, &panier).await;

    let plats_detail: Vec<PlatDetail> = plats_map
        .values()
        .map(|p| PlatDetail {
            id: p.id,
            titre: p.titre.clone(),
            description: p.description.clone(),
            image: p.image.clone(),
            est_viande: p.est_viande,
            allergenes: vec![],
        })
        .collect();
    if !plats_detail.is_empty() {
        let _ = get_plat_views(request, &plats_detail).await;
    }

    Ok(())
}
