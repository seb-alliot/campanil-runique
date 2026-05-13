use crate::backend::carte::CarteSupplementItem;
use crate::entities::supplement;
use runique::prelude::*;

pub async fn build_supplements(db: &sea_orm::DatabaseConnection) -> Vec<CarteSupplementItem> {
    search!(supplement::Entity => Disponible eq true, asc Libelle,)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|s| CarteSupplementItem {
            id: s.id,
            libelle: s.libelle,
            prix: format!("{:.2}", s.prix),
        })
        .collect()
}
