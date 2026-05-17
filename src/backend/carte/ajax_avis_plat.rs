use crate::entities::avis_plat::{self, StatutAvisPlat};
use runique::axum;
use runique::prelude::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct AvisPlatPublic {
    pub note: i32,
    pub etoiles: String,
    pub commentaire: String,
    pub auteur: String,
}

pub async fn get_avis_plat(
    db: &sea_orm::DatabaseConnection,
    plat_id: Pk,
    n: u64,
) -> Vec<AvisPlatPublic> {
    let avis_list = search!(avis_plat::Entity =>
        PlatId eq plat_id,
        Statut eq StatutAvisPlat::Valide,
    )
    .order_by_random()
    .limit(n)
    .all(db)
    .await
    .unwrap_or_default();

    if avis_list.is_empty() {
        return vec![];
    }

    let user_ids: Vec<i32> = avis_list.iter().filter_map(|a| a.user_id).collect();

    let user_name_map: HashMap<Pk, String> = if !user_ids.is_empty() {
        runique_users::Entity::find()
            .filter(runique_users::Column::Id.is_in(user_ids))
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|u| (u.id, u.username))
            .collect()
    } else {
        HashMap::new()
    };

    avis_list
        .into_iter()
        .map(|a| {
            let auteur = a
                .user_id
                .and_then(|uid| user_name_map.get(&(uid as Pk)).cloned())
                .filter(|s: &String| !s.is_empty())
                .unwrap_or_else(|| "Un client".to_string());
            let etoiles = "★".repeat(a.note as usize) + &"☆".repeat((5 - a.note) as usize);
            AvisPlatPublic {
                note: a.note,
                etoiles,
                commentaire: a.commentaire,
                auteur,
            }
        })
        .collect()
}

pub async fn ajax_avis_plat(request: Request) -> AppResult<Response> {
    let plat_id: Pk = request.get_path_as::<Pk>("id").unwrap_or(0);
    let n: u64 = request
        .get_query("n")
        .and_then(|v| v.parse().ok())
        .unwrap_or(3)
        .clamp(1, 7);

    if plat_id == 0 {
        return Ok(axum::Json(Vec::<AvisPlatPublic>::new()).into_response());
    }

    let avis = get_avis_plat(request.db(), plat_id, n).await;
    Ok(axum::Json(avis).into_response())
}
