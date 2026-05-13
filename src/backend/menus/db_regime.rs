use crate::entities::regime;
use runique::prelude::*;

pub async fn get_regimes_list(db: &DatabaseConnection) -> Vec<regime::Model> {
    search!(regime::Entity => asc Libelle,)
        .all(db)
        .await
        .unwrap_or_default()
}
