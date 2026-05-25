use crate::backend::carte::{
    CarteGarniture, CarteMenuResto, CarteMenuSection, CartePage, CartePlat, CoursMenu,
    build_boissons, build_supplements,
};
use crate::entities::{
    allergene, dessert, dessert_allergene, entree, entree_allergene, garniture, menu, menu_dessert,
    menu_entree, menu_plat, plat, plat_allergene,
};
use runique::prelude::*;
use std::collections::HashMap;

pub async fn get_carte(db: &sea_orm::DatabaseConnection) -> CartePage {
    let (entrees, desserts_carte, plats_all, menus) = tokio::join!(
        build_entrees(db),
        build_desserts(db),
        build_plats(db),
        build_menus(db),
    );

    let supplements = build_supplements(db).await;
    let boissons = build_boissons(db).await;

    let menus_groupes = grouper_menus(menus);

    CartePage {
        entrees,
        specialites: plats_all.0,
        viandes: plats_all.1,
        poissons: plats_all.2,
        desserts: desserts_carte,
        menus: menus_groupes,
        supplements,
        boissons,
    }
}

async fn build_entrees(db: &sea_orm::DatabaseConnection) -> Vec<CartePlat> {
    let entree_models: Vec<entree::Model> =
        search!(entree::Entity => Disponible eq true, Usage ne entree::UsageEntree::Menu, asc Ordre, asc Titre,)
            .all(db)
            .await
            .unwrap_or_default();

    if entree_models.is_empty() {
        return vec![];
    }

    let ids: Vec<Pk> = entree_models.iter().map(|e| e.id).collect();
    let links = search!(entree_allergene::Entity => EntreeId in (ids))
        .all(db)
        .await
        .unwrap_or_default();
    let allergene_labels = load_allergene_labels(db).await;

    let mut allergenes_by_id: HashMap<Pk, Vec<String>> = HashMap::new();
    for link in links {
        if let Some(label) = allergene_labels.get(&(link.allergene_id as Pk)) {
            allergenes_by_id
                .entry(link.entree_id as Pk)
                .or_default()
                .push(label.clone());
        }
    }

    entree_models
        .into_iter()
        .map(|e| CartePlat {
            id: e.id,
            titre: e.titre,
            label: e.label,
            description: e.description,
            prix: format!("{:.2}", e.prix),
            image: e.image,
            est_viande: false,
            disponible: e.disponible,

            allergenes: allergenes_by_id.remove(&e.id).unwrap_or_default(),
            garnitures: vec![],
        })
        .collect()
}

async fn build_desserts(db: &sea_orm::DatabaseConnection) -> Vec<CartePlat> {
    let dessert_models: Vec<dessert::Model> =
        search!(dessert::Entity => Disponible eq true, Usage ne dessert::UsageDessert::Menu, asc Ordre, asc Titre,)
            .all(db)
            .await
            .unwrap_or_default();

    if dessert_models.is_empty() {
        return vec![];
    }

    let ids: Vec<Pk> = dessert_models.iter().map(|d| d.id).collect();
    let links = search!(dessert_allergene::Entity => DessertId in (ids))
        .all(db)
        .await
        .unwrap_or_default();
    let allergene_labels = load_allergene_labels(db).await;

    let mut allergenes_by_id: HashMap<Pk, Vec<String>> = HashMap::new();
    for link in links {
        if let Some(label) = allergene_labels.get(&(link.allergene_id as Pk)) {
            allergenes_by_id
                .entry(link.dessert_id as Pk)
                .or_default()
                .push(label.clone());
        }
    }

    dessert_models
        .into_iter()
        .map(|d| CartePlat {
            id: d.id,
            titre: d.titre,
            label: d.label,
            description: d.description,
            prix: format!("{:.2}", d.prix),
            image: d.image,
            est_viande: false,
            disponible: d.disponible,

            allergenes: allergenes_by_id.remove(&d.id).unwrap_or_default(),
            garnitures: vec![],
        })
        .collect()
}

async fn build_plats(
    db: &sea_orm::DatabaseConnection,
) -> (Vec<CartePlat>, Vec<CartePlat>, Vec<CartePlat>) {
    use crate::entities::plat::TypePlat;

    let plat_models: Vec<plat::Model> =
        search!(plat::Entity => Disponible eq true, Usage ne plat::UsagePlat::Menu, asc TypePlat, asc Ordre, asc Titre,)
            .all(db)
            .await
            .unwrap_or_default();

    if plat_models.is_empty() {
        return (vec![], vec![], vec![]);
    }

    let ids: Vec<Pk> = plat_models.iter().map(|p| p.id).collect();
    let links = search!(plat_allergene::Entity => PlatId in (ids))
        .all(db)
        .await
        .unwrap_or_default();
    let allergene_labels = load_allergene_labels(db).await;

    let mut allergenes_by_id: HashMap<Pk, Vec<String>> = HashMap::new();
    for link in links {
        if let Some(label) = allergene_labels.get(&(link.allergene_id as Pk)) {
            allergenes_by_id
                .entry(link.plat_id as Pk)
                .or_default()
                .push(label.clone());
        }
    }

    let feculents = build_feculents(db).await;

    let mut specialites = Vec::new();
    let mut viandes = Vec::new();
    let mut poissons = Vec::new();

    for p in plat_models {
        let garnitures = if p.est_viande
            || matches!(
                p.type_plat,
                TypePlat::Specialite | TypePlat::Poisson | TypePlat::Plat | TypePlat::Viande
            ) {
            feculents.clone()
        } else {
            vec![]
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
            allergenes: allergenes_by_id.remove(&p.id).unwrap_or_default(),
            garnitures,
        };
        match p.type_plat {
            TypePlat::Viande => viandes.push(carte_plat),
            TypePlat::Poisson => poissons.push(carte_plat),
            _ => specialites.push(carte_plat),
        }
    }

    (specialites, viandes, poissons)
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
    let menus = search!(menu::Entity => asc Ordre, asc Nom,)
        .all(db)
        .await
        .unwrap_or_default();

    if menus.is_empty() {
        return vec![];
    }

    let menu_ids: Vec<Pk> = menus.iter().map(|m| m.id).collect();

    let entree_liens = search!(menu_entree::Entity => MenuId in (menu_ids),)
        .all(db)
        .await
        .unwrap_or_default();
    let plat_liens = search!(menu_plat::Entity => MenuId in (menu_ids),)
        .all(db)
        .await
        .unwrap_or_default();
    let dessert_liens = search!(menu_dessert::Entity => MenuId in (menu_ids),)
        .all(db)
        .await
        .unwrap_or_default();

    let entree_ids: Vec<i32> = entree_liens.iter().map(|l| l.entree_id).collect();
    let plat_ids_all: Vec<i32> = plat_liens.iter().map(|l| l.plat_id).collect();
    let dessert_ids: Vec<i32> = dessert_liens.iter().map(|l| l.dessert_id).collect();

    let allergene_labels = load_allergene_labels(db).await;
    let feculents = build_feculents(db).await;

    // Charger entrees du menu
    let entrees_map: HashMap<i32, entree::Model> = if entree_ids.is_empty() {
        HashMap::new()
    } else {
        search!(entree::Entity => Id in (entree_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|e| (e.id, e))
            .collect()
    };

    let entree_allergene_links = if entree_ids.is_empty() {
        vec![]
    } else {
        search!(entree_allergene::Entity => EntreeId in (entree_ids))
            .all(db)
            .await
            .unwrap_or_default()
    };
    let mut allergenes_by_entree: HashMap<Pk, Vec<String>> = HashMap::new();
    for link in entree_allergene_links {
        if let Some(label) = allergene_labels.get(&(link.allergene_id as Pk)) {
            allergenes_by_entree
                .entry(link.entree_id as Pk)
                .or_default()
                .push(label.clone());
        }
    }

    // Charger plats du menu
    let plats_map: HashMap<i32, plat::Model> = if plat_ids_all.is_empty() {
        HashMap::new()
    } else {
        search!(plat::Entity => Id in (plat_ids_all),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|p| (p.id, p))
            .collect()
    };

    let plat_allergene_links = if plat_ids_all.is_empty() {
        vec![]
    } else {
        search!(plat_allergene::Entity => PlatId in (plat_ids_all))
            .all(db)
            .await
            .unwrap_or_default()
    };
    let mut allergenes_by_plat: HashMap<Pk, Vec<String>> = HashMap::new();
    for link in plat_allergene_links {
        if let Some(label) = allergene_labels.get(&(link.allergene_id as Pk)) {
            allergenes_by_plat
                .entry(link.plat_id as Pk)
                .or_default()
                .push(label.clone());
        }
    }

    // Charger desserts du menu
    let desserts_map: HashMap<i32, dessert::Model> = if dessert_ids.is_empty() {
        HashMap::new()
    } else {
        search!(dessert::Entity => Id in (dessert_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|d| (d.id, d))
            .collect()
    };

    let dessert_allergene_links = if dessert_ids.is_empty() {
        vec![]
    } else {
        search!(dessert_allergene::Entity => DessertId in (dessert_ids))
            .all(db)
            .await
            .unwrap_or_default()
    };
    let mut allergenes_by_dessert: HashMap<Pk, Vec<String>> = HashMap::new();
    for link in dessert_allergene_links {
        if let Some(label) = allergene_labels.get(&(link.allergene_id as Pk)) {
            allergenes_by_dessert
                .entry(link.dessert_id as Pk)
                .or_default()
                .push(label.clone());
        }
    }

    menus
        .into_iter()
        .map(|m| {
            let m_entree_ids: Vec<i32> = entree_liens
                .iter()
                .filter(|l| l.menu_id as Pk == m.id)
                .map(|l| l.entree_id)
                .collect();
            let m_plat_ids: Vec<i32> = plat_liens
                .iter()
                .filter(|l| l.menu_id as Pk == m.id)
                .map(|l| l.plat_id)
                .collect();
            let m_dessert_ids: Vec<i32> = dessert_liens
                .iter()
                .filter(|l| l.menu_id as Pk == m.id)
                .map(|l| l.dessert_id)
                .collect();

            let entrees_cours: Vec<CartePlat> = m_entree_ids
                .iter()
                .filter_map(|id| entrees_map.get(id))
                .map(|e| CartePlat {
                    id: e.id,
                    titre: e.titre.clone(),
                    label: e.label.clone(),
                    description: e.description.clone(),
                    prix: format!("{:.2}", e.prix),
                    image: e.image.clone(),
                    est_viande: false,
                    disponible: e.disponible,

                    allergenes: allergenes_by_entree.get(&e.id).cloned().unwrap_or_default(),
                    garnitures: vec![],
                })
                .collect();

            let plats_cours: Vec<CartePlat> = m_plat_ids
                .iter()
                .filter_map(|id| plats_map.get(id))
                .map(|p| CartePlat {
                    id: p.id,
                    titre: p.titre.clone(),
                    label: p.label.clone(),
                    description: p.description.clone(),
                    prix: format!("{:.2}", p.prix),
                    image: p.image.clone(),
                    est_viande: p.est_viande,
                    disponible: p.disponible,
                    allergenes: allergenes_by_plat.get(&p.id).cloned().unwrap_or_default(),
                    garnitures: if p.est_viande {
                        feculents.clone()
                    } else {
                        vec![]
                    },
                })
                .collect();

            let desserts_cours: Vec<CartePlat> = m_dessert_ids
                .iter()
                .filter_map(|id| desserts_map.get(id))
                .map(|d| CartePlat {
                    id: d.id,
                    titre: d.titre.clone(),
                    label: d.label.clone(),
                    description: d.description.clone(),
                    prix: format!("{:.2}", d.prix),
                    image: d.image.clone(),
                    est_viande: false,
                    disponible: d.disponible,

                    allergenes: allergenes_by_dessert
                        .get(&d.id)
                        .cloned()
                        .unwrap_or_default(),
                    garnitures: vec![],
                })
                .collect();

            let cours = [
                CoursMenu {
                    label: "Entrée".to_string(),
                    key: "entree".to_string(),
                    plats: entrees_cours,
                },
                CoursMenu {
                    label: "Plat".to_string(),
                    key: "plat".to_string(),
                    plats: plats_cours,
                },
                CoursMenu {
                    label: "Dessert".to_string(),
                    key: "dessert".to_string(),
                    plats: desserts_cours,
                },
            ]
            .into_iter()
            .filter(|c| !c.plats.is_empty())
            .collect();

            CarteMenuResto {
                id: m.id,
                nom: m.nom.clone(),
                type_menu: m.type_menu.to_value(),
                prix: format!("{:.2}", m.prix),
                description: m.description.clone(),
                entree_libre: m.entree_libre.clone(),
                plat_libre: m.plat_libre.clone(),
                dessert_libre: m.dessert_libre.clone(),
                cours,
            }
        })
        .collect()
}

fn grouper_menus(menus: Vec<CarteMenuResto>) -> Vec<CarteMenuSection> {
    [
        ("menu_resto", "Menus restaurant"),
        ("menu_enfant", "Menus enfant"),
        ("formule_jour", "Formules du jour"),
    ]
    .into_iter()
    .filter_map(|(key, label)| {
        let items: Vec<CarteMenuResto> = menus
            .iter()
            .filter(|m| m.type_menu == key)
            .cloned()
            .collect();
        if items.is_empty() {
            None
        } else {
            Some(CarteMenuSection {
                label: label.to_string(),
                key: key.to_string(),
                menus: items,
            })
        }
    })
    .collect()
}

async fn load_allergene_labels(db: &sea_orm::DatabaseConnection) -> HashMap<Pk, String> {
    search!(allergene::Entity)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|a| (a.id, a.libelle))
        .collect()
}
