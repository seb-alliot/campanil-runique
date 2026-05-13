use crate::entities::commande::StatutCommande;

pub fn parse_statut(s: &str) -> Option<StatutCommande> {
    s.parse().ok()
}
