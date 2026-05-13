use crate::backend::panier::{LignePanier, MenuChoixPanier, panier_get, panier_save};
use crate::entities::{menu_resto, plat};
use runique::prelude::*;
use std::collections::HashMap;

pub async fn panier_ajouter_menu(
    session: &Session,
    db: &sea_orm::DatabaseConnection,
    menu_id: i32,
    quantite: i32,
    choix: Vec<MenuChoixPanier>,
    user_id: Option<i32>,
) -> Result<(), &'static str> {
    let Some(menu) = menu_resto::Entity::find_by_id(menu_id)
        .filter(menu_resto::Column::Disponible.eq(true))
        .one(db)
        .await
        .ok()
        .flatten()
    else {
        return Err("Menu introuvable ou indisponible");
    };

    let plat_ids: Vec<i32> = choix.iter().map(|c| c.plat_id).collect();
    let plats_labels: HashMap<i32, String> = if !plat_ids.is_empty() {
        search!(plat::Entity => Id in (plat_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|p| (p.id, p.label.unwrap_or(p.titre)))
            .collect()
    } else {
        HashMap::new()
    };

    let menu_choix = choix
        .into_iter()
        .map(|mut c| {
            c.plat_titre = plats_labels.get(&c.plat_id).cloned().unwrap_or_default();
            c
        })
        .collect();

    let mut panier = panier_get(session).await;
    if panier.user_id.is_none() {
        panier.user_id = user_id;
    }
    panier.lignes.push(LignePanier {
        plat_id: 0,
        boisson_id: None,
        menu_id: Some(menu_id),
        titre: menu.titre,
        prix_unitaire: format!("{:.2}", menu.prix),
        quantite,
        est_viande: false,
        cuisson: None,
        note: None,
        garniture_ids: vec![],
        avec_legumes: false,
        sans_sel: false,
        menu_choix,
        supplement_id: None,
    });

    panier_save(session, &panier).await;
    Ok(())
}
