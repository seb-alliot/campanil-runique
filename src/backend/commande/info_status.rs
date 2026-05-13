use crate::entities::commande::StatutCommande;

pub fn statut_info(s: &StatutCommande) -> (String, &'static str) {
    (s.to_string(), s.db_value())
}
