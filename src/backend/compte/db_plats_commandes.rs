use crate::backend::compte::PlatCommande;
use crate::entities::commande::StatutCommande;
use crate::entities::{avis_plat, commande, commande_ligne, dessert, entree, plat};
use runique::prelude::*;
use std::collections::HashMap;

pub async fn get_plats_commandes_user(
    db: &sea_orm::DatabaseConnection,
    user_id: Pk,
) -> Vec<PlatCommande> {
    let commande_ids: Vec<Pk> = search!(commande::Entity =>
        UserId eq user_id,
        or(Statut eq StatutCommande::Termine, Statut eq StatutCommande::Livre),
    )
    .all(db)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|c| c.id)
    .collect();

    if commande_ids.is_empty() {
        return vec![];
    }

    let lignes = search!(commande_ligne::Entity => CommandeId in (commande_ids),)
        .all(db)
        .await
        .unwrap_or_default();

    let mut plat_ids: Vec<i32> = lignes.iter().filter_map(|l| l.plat_id).collect();
    plat_ids.sort_unstable();
    plat_ids.dedup();

    let mut entree_ids: Vec<i32> = lignes.iter().filter_map(|l| l.entree_id).collect();
    entree_ids.sort_unstable();
    entree_ids.dedup();

    let mut dessert_ids: Vec<i32> = lignes.iter().filter_map(|l| l.dessert_id).collect();
    dessert_ids.sort_unstable();
    dessert_ids.dedup();

    if plat_ids.is_empty() && entree_ids.is_empty() && dessert_ids.is_empty() {
        return vec![];
    }

    let avis_plats_map: HashMap<i32, avis_plat::Model> = if !plat_ids.is_empty() {
        search!(avis_plat::Entity => UserId eq user_id, PlatId in (plat_ids.clone()),)
            .all(db).await.unwrap_or_default()
            .into_iter().map(|a| (a.plat_id.unwrap_or(0), a)).collect()
    } else { HashMap::new() };

    let avis_entrees_map: HashMap<i32, avis_plat::Model> = if !entree_ids.is_empty() {
        search!(avis_plat::Entity => UserId eq user_id, EntreeId in (entree_ids.clone()),)
            .all(db).await.unwrap_or_default()
            .into_iter().map(|a| (a.entree_id.unwrap_or(0), a)).collect()
    } else { HashMap::new() };

    let avis_desserts_map: HashMap<i32, avis_plat::Model> = if !dessert_ids.is_empty() {
        search!(avis_plat::Entity => UserId eq user_id, DessertId in (dessert_ids.clone()),)
            .all(db).await.unwrap_or_default()
            .into_iter().map(|a| (a.dessert_id.unwrap_or(0), a)).collect()
    } else { HashMap::new() };

    let mut result: Vec<PlatCommande> = Vec::new();

    if !plat_ids.is_empty() {
        let plats = search!(plat::Entity => Id in (plat_ids), asc Titre,)
            .all(db).await.unwrap_or_default();
        for p in plats {
            let avis = avis_plats_map.get(&p.id);
            result.push(PlatCommande {
                id: p.id,
                titre: p.titre,
                image: p.image,
                avis_note: avis.map(|a| a.note),
                avis_statut: avis.map(|a| a.statut.to_string()),
                can_review: true,
                type_article: "plat".to_string(),
            });
        }
    }

    if !entree_ids.is_empty() {
        let entrees = search!(entree::Entity => Id in (entree_ids), asc Titre,)
            .all(db).await.unwrap_or_default();
        for e in entrees {
            let avis = avis_entrees_map.get(&e.id);
            result.push(PlatCommande {
                id: e.id,
                titre: e.titre,
                image: e.image,
                avis_note: avis.map(|a| a.note),
                avis_statut: avis.map(|a| a.statut.to_string()),
                can_review: true,
                type_article: "entree".to_string(),
            });
        }
    }

    if !dessert_ids.is_empty() {
        let desserts = search!(dessert::Entity => Id in (dessert_ids), asc Titre,)
            .all(db).await.unwrap_or_default();
        for d in desserts {
            let avis = avis_desserts_map.get(&d.id);
            result.push(PlatCommande {
                id: d.id,
                titre: d.titre,
                image: d.image,
                avis_note: avis.map(|a| a.note),
                avis_statut: avis.map(|a| a.statut.to_string()),
                can_review: true,
                type_article: "dessert".to_string(),
            });
        }
    }

    result
}
