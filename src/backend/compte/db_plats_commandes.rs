use crate::backend::compte::PlatCommande;
use crate::entities::commande::StatutCommande;
use crate::entities::{avis_plat, commande, commande_ligne, plat};
use runique::prelude::*;
use sea_orm::{Condition, QueryFilter};
use std::collections::HashMap;

pub async fn get_plats_commandes_user(
    db: &sea_orm::DatabaseConnection,
    user_id: Pk,
) -> Vec<PlatCommande> {
    let commande_ids: Vec<Pk> = search!(commande::Entity => UserId eq user_id,)
        .filter(
            Condition::any()
                .add(commande::Column::Statut.eq(StatutCommande::Termine))
                .add(commande::Column::Statut.eq(StatutCommande::Livre)),
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

    if plat_ids.is_empty() {
        return vec![];
    }

    let plats = search!(plat::Entity => Id in (plat_ids.clone()), asc Titre,)
        .all(db)
        .await
        .unwrap_or_default();

    let avis_map: HashMap<i32, avis_plat::Model> = avis_plat::Entity::find()
        .filter(avis_plat::Column::UserId.eq(user_id))
        .filter(avis_plat::Column::PlatId.is_in(plat_ids))
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|a| (a.plat_id, a))
        .collect();

    plats
        .into_iter()
        .map(|p| {
            let avis = avis_map.get(&p.id);
            PlatCommande {
                id: p.id,
                titre: p.titre,
                image: p.image,
                avis_note: avis.map(|a| a.note),
                avis_statut: avis.map(|a| a.statut.to_string()),
            }
        })
        .collect()
}
