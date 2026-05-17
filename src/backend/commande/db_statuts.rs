use crate::entities::commande::StatutCommande;
use sea_orm::Iterable;

pub fn all_statuts() -> Vec<(String, String)> {
    StatutCommande::iter()
        .map(|s: StatutCommande| (s.db_value().to_string(), s.to_string()))
        .collect()
}
