use crate::backend::carte::{
    CarteGarniture, CarteMenuResto, CartePage, CartePlat, CoursMenu, build_boissons,
    build_supplements,
};
use crate::entities::{
    allergene, garniture, menu_resto, menu_resto_plat, plat, plat::TypePlat, plat_allergene,
};
use runique::prelude::*;
use std::collections::HashMap;

pub async fn get_carte(db: &sea_orm::DatabaseConnection) -> CartePage {
    let plat_models: Vec<plat::Model> =
        search!(plat::Entity => asc TypePlat, asc Ordre, asc Titre,)
            .all(db)
            .await
            .unwrap_or_default();

    let menus = build_menus(db).await;

    if plat_models.is_empty() {
        return CartePage {
            entrees: vec![],
            specialites: vec![],
            viandes: vec![],
            poissons: vec![],
            desserts: vec![],
            menus,
            supplements: build_supplements(db).await,
            boissons: build_boissons(db).await,
        };
    }

    let plat_ids: Vec<Pk> = plat_models.iter().map(|p| p.id).collect();

    let allergene_links = search!(plat_allergene::Entity => PlatId in (plat_ids))
        .all(db)
        .await
        .unwrap_or_default();

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

    let feculents = build_feculents(db).await;

    let mut entrees = Vec::new();
    let mut specialites = Vec::new();
    let mut viandes = Vec::new();
    let mut poissons = Vec::new();
    let mut desserts = Vec::new();

    for p in plat_models {
        let garnitures = match p.type_plat {
            TypePlat::Entree | TypePlat::Dessert => vec![],
            _ => feculents.clone(),
        };
        let carte_plat = CartePlat {
            id: p.id,
            titre: p.titre,
            label: p.label,
            description: p.description,
            prix: format!("{:.2}", p.prix),
            image: p.image,
            est_viande: p.est_viande,
            disponible: p.disponible,
            avec_legumes: p.avec_legumes,
            allergenes: allergenes_by_plat.remove(&p.id).unwrap_or_default(),
            garnitures,
        };
        match p.type_plat {
            TypePlat::Entree => entrees.push(carte_plat),
            TypePlat::Specialite => specialites.push(carte_plat),
            TypePlat::Viande => viandes.push(carte_plat),
            TypePlat::Poisson => poissons.push(carte_plat),
            TypePlat::Plat => specialites.push(carte_plat),
            TypePlat::Dessert => desserts.push(carte_plat),
        }
    }

    let supplements = build_supplements(db).await;
    let boissons = build_boissons(db).await;
    CartePage {
        entrees,
        specialites,
        viandes,
        poissons,
        desserts,
        menus,
        supplements,
        boissons,
    }
}

pub async fn build_feculents(db: &sea_orm::DatabaseConnection) -> Vec<CarteGarniture> {
    search!(garniture::Entity => Disponible eq true, asc Libelle,)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|g| CarteGarniture {
            id: g.id,
            libelle: g.libelle,
            type_garniture: g.type_garniture.to_string(),
            est_defaut: false,
        })
        .collect()
}

async fn build_menus(db: &sea_orm::DatabaseConnection) -> Vec<CarteMenuResto> {
    let menus = search!(menu_resto::Entity => asc Ordre, asc Nom,)
        .filter(menu_resto::Column::Disponible.eq(true))
        .all(db)
        .await
        .unwrap_or_default();

    if menus.is_empty() {
        return vec![];
    }

    let menu_ids: Vec<Pk> = menus.iter().map(|m| m.id).collect();

    let liens = search!(menu_resto_plat::Entity => MenuId in (menu_ids),)
        .all(db)
        .await
        .unwrap_or_default();

    if liens.is_empty() {
        return menus
            .into_iter()
            .map(|m| CarteMenuResto {
                id: m.id,
                titre: m.nom,
                prix: format!("{:.2}", m.prix),
                description: m.description,
                dessert: m.dessert,
                cours: vec![],
            })
            .collect();
    }

    let plat_ids: Vec<i32> = liens.iter().map(|l| l.plat_id).collect();

    let plats_map: HashMap<Pk, plat::Model> = search!(plat::Entity => Id in (plat_ids),)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|p| (p.id, p))
        .collect();

    let allergene_links = search!(plat_allergene::Entity => PlatId in (plat_ids))
        .all(db)
        .await
        .unwrap_or_default();

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

    let feculents = build_feculents(db).await;

    menus
        .into_iter()
        .map(|m| {
            let mut entrees = Vec::new();
            let mut plats = Vec::new();
            let mut desserts = Vec::new();

            for lien in liens.iter().filter(|l| l.menu_id as Pk == m.id) {
                if let Some(p) = plats_map.get(&(lien.plat_id as Pk)) {
                    let garnitures = match p.type_plat {
                        TypePlat::Entree | TypePlat::Dessert => vec![],
                        _ => feculents.clone(),
                    };
                    let carte_plat = CartePlat {
                        id: p.id,
                        titre: p.titre.clone(),
                        label: p.label.clone(),
                        description: p.description.clone(),
                        prix: format!("{:.2}", p.prix),
                        image: p.image.clone(),
                        est_viande: p.est_viande,
                        disponible: p.disponible,
                        avec_legumes: p.avec_legumes,
                        allergenes: allergenes_by_plat.get(&p.id).cloned().unwrap_or_default(),
                        garnitures,
                    };
                    match lien.cours.as_str() {
                        "entree" => entrees.push(carte_plat),
                        "plat" => plats.push(carte_plat),
                        "dessert" => desserts.push(carte_plat),
                        _ => {}
                    }
                }
            }

            let cours = [
                CoursMenu {
                    label: "Entrée".to_string(),
                    key: "entree".to_string(),
                    plats: entrees,
                },
                CoursMenu {
                    label: "Plat".to_string(),
                    key: "plat".to_string(),
                    plats,
                },
                CoursMenu {
                    label: "Dessert".to_string(),
                    key: "dessert".to_string(),
                    plats: desserts,
                },
            ]
            .into_iter()
            .filter(|c| !c.plats.is_empty())
            .collect();

            CarteMenuResto {
                id: m.id,
                titre: m.nom.clone(),
                prix: format!("{:.2}", m.prix),
                description: m.description.clone(),
                dessert: m.dessert.clone(),
                cours,
            }
        })
        .collect()
}
