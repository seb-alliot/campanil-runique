use crate::backend::carte::CarteSupplementItem;
use crate::entities::{garniture, supplement};
use runique::prelude::*;
use std::collections::HashMap;

pub async fn build_supplements(db: &sea_orm::DatabaseConnection) -> Vec<CarteSupplementItem> {
    let sups = search!(supplement::Entity => Disponible eq true, asc Ordre,)
        .all(db)
        .await
        .unwrap_or_default();

    if sups.is_empty() {
        return vec![];
    }

    let garn_ids: Vec<i32> = sups.iter().filter_map(|s| s.garniture_id).collect();
    let garnitures_map: HashMap<Pk, String> = if !garn_ids.is_empty() {
        search!(garniture::Entity => Id in (garn_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|g| (g.id, g.libelle))
            .collect()
    } else {
        HashMap::new()
    };

    let items: Vec<CarteSupplementItem> = sups
        .into_iter()
        .filter_map(|s| {
            let libelle = s.titre.clone().or_else(|| {
                s.garniture_id
                    .and_then(|gid| garnitures_map.get(&(gid as Pk)).cloned())
            })?;
            Some(CarteSupplementItem {
                id: s.id,
                libelle,
                prix: format!("{:.2}", s.prix),
            })
        })
        .collect();

    items
}
