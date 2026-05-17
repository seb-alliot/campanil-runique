use crate::backend::menus::PlatDetail;
use crate::backend::panier::{LignePanier, MenuChoixPanier, panier_get, panier_save};
use crate::backend::stats::get_plat_views;
use crate::entities::{menu_resto, plat};
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
    let Some(menu) = menu_resto::Entity::find_by_id(menu_id)
        .filter(menu_resto::Column::Disponible.eq(true))
        .one(db)
        .await
        .ok()
        .flatten()
    else {
        return Err("Menu introuvable ou indisponible");
    };

    let plat_ids: Vec<Pk> = choix.iter().map(|c| c.plat_id).collect();
    let plats_data: HashMap<Pk, plat::Model> = if !plat_ids.is_empty() {
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

    let plats_labels: HashMap<Pk, String> = plats_data
        .iter()
        .map(|(id, p)| (*id, p.label.clone().unwrap_or(p.titre.clone())))
        .collect();

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
        menu_resto_id: Some(menu_id),
        supplement_id: None,
        titre: menu.nom,
        prix_unitaire: format!("{:.2}", menu.prix),
        quantite,
        est_viande: false,
        cuisson: None,
        note: None,
        garniture_ids: vec![],
        avec_legumes: false,
        sans_sel: false,
        menu_choix,
    });

    panier_save(session, &panier).await;

    let plats_detail: Vec<PlatDetail> = plats_data
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
