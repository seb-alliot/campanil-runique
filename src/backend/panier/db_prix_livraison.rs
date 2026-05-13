use crate::entities::info_resto;
use runique::prelude::*;
use std::str::FromStr;

pub async fn get_prix_livraison(db: &sea_orm::DatabaseConnection) -> Decimal {
    info_resto::Entity::find()
        .one(db)
        .await
        .ok()
        .flatten()
        .and_then(|r| r.prix_livraison)
        .unwrap_or_else(|| Decimal::from_str("5.00").unwrap())
}
