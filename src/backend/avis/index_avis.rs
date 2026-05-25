use crate::entities::avis;
use runique::prelude::*;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct AvisPublic {
    pub note: i32,
    pub etoiles: String,
    pub commentaire: String,
    pub auteur: String,
}

pub async fn get_avis_valides(db: &DatabaseConnection) -> Vec<AvisPublic> {
    let avis_list = search!(avis::Entity => Statut eq avis::StatutAvis::Valide, desc CreatedAt,)
        .limit(8)
        .all(db)
        .await
        .unwrap_or_default();

    if avis_list.is_empty() {
        return vec![];
    }

    let user_ids: Vec<i32> = avis_list.iter().map(|a| a.user_id).collect();

    let user_name_map: HashMap<Pk, String> = search!(runique_users::Entity => Id in (user_ids),)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|u| (u.id, u.username))
        .collect();

    avis_list
        .into_iter()
        .map(|a| {
            let username = user_name_map
                .get(&(a.user_id as Pk))
                .cloned()
                .unwrap_or_default();
            let auteur = if username.is_empty() {
                "Un client".to_string()
            } else {
                username
            };
            let etoiles = "★".repeat(a.note as usize) + &"☆".repeat((5 - a.note) as usize);
            AvisPublic {
                note: a.note,
                etoiles,
                commentaire: a.commentaire,
                auteur,
            }
        })
        .collect()
}
