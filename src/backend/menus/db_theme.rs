use crate::entities::theme;
use runique::prelude::*;

pub async fn get_themes_list(db: &DatabaseConnection) -> Vec<theme::Model> {
    search!(theme::Entity => asc Libelle,)
        .all(db)
        .await
        .unwrap_or_default()
}
