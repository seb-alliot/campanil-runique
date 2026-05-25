use crate::backend::menus::{MenuDetail, PlatDetail};
use crate::entities::{allergene, menu_traiteur, menu_traiteur_plat, plat, plat_allergene};
use runique::prelude::*;

pub async fn get_menu_detail(db: &DatabaseConnection, id: Pk) -> Option<MenuDetail> {
    let menu_model = search!(menu_traiteur::Entity => Id eq id, Actif eq true,)
        .one(db)
        .await
        .ok()??;

    let plat_ids: Vec<i32> = search!(menu_traiteur_plat::Entity => MenuTraiteurId eq id,)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|mp| mp.plat_id)
        .collect();

    let (plat_models, allergene_links) = if plat_ids.is_empty() {
        (vec![], vec![])
    } else {
        let plats = search!(plat::Entity => Id in (plat_ids), asc Titre,)
            .all(db)
            .await
            .unwrap_or_default();
        let links = search!(plat_allergene::Entity => PlatId in (plat_ids),)
            .all(db)
            .await
            .unwrap_or_default();
        (plats, links)
    };

    let allergene_labels: HashMap<Pk, String> = search!(allergene::Entity)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|a| (a.id, a.libelle))
        .collect();

    let mut allergenes_by_plat: HashMap<Pk, Vec<String>> = HashMap::new();
    for link in allergene_links {
        if let Some(label) = allergene_labels.get(&(link.allergene_id as Pk)) {
            allergenes_by_plat
                .entry(link.plat_id as Pk)
                .or_default()
                .push(label.clone());
        }
    }

    let plats = plat_models
        .into_iter()
        .map(|p| PlatDetail {
            id: p.id,
            titre: p.titre,
            description: p.description,
            image: p.image,
            est_viande: p.est_viande,
            allergenes: allergenes_by_plat.remove(&p.id).unwrap_or_default(),
        })
        .collect();

    Some(MenuDetail {
        id: menu_model.id,
        titre: menu_model.titre,
        description: menu_model.description,
        conditions: menu_model.conditions,
        prix_par_personne: format!("{:.2}", menu_model.prix_par_personne),
        nb_personnes_min: menu_model.nb_personnes_min,
        theme: menu_model.theme.to_value(),
        regime: menu_model.regime.to_value(),
        stock: menu_model.stock,
        plats_principaux: plats,
        desserts: vec![],
        entrees: vec![],
    })
}
