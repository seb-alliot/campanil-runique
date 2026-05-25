use runique::prelude::*;

#[derive(Serialize)]
pub struct LigneService {
    pub titre: String,
    pub quantite: i32,
    pub cuisson: Option<String>,
    pub note: Option<String>,
    pub garnitures: Vec<String>,
    pub sans_sel: bool,
}

#[derive(Serialize)]
pub struct CommandeService {
    pub numero: String,
    pub client: String,
    pub heure: String,
    pub statut: String,
    pub statut_valeur: String,
    pub mode_paiement: String,
    pub prix_total: String,
    pub type_retrait: String,
    pub adresse_livraison: Option<String>,
    pub lignes: Vec<LigneService>,
    pub service: String,
}
