use runique::prelude::*;

#[derive(Serialize)]
pub struct LigneDetail {
    pub titre: String,
    pub quantite: i32,
    pub cuisson: Option<String>,
    pub est_boisson: bool,
    pub est_menu: bool,
    pub note: Option<String>,
}

#[derive(Serialize)]
pub struct CommandeDetail {
    pub numero: String,
    pub statut: String,
    pub statut_valeur: String,
    pub type_commande: String,
    pub mode_paiement: String,
    pub prix_total: String,
    pub heure_retrait: Option<String>,
    pub avec_livraison: bool,
    pub adresse_livraison: Option<String>,
    pub cp_livraison: Option<String>,
    pub ville_livraison: Option<String>,
    pub heure_livraison: Option<String>,
    pub client: String,
    pub motif_annulation: Option<String>,
    pub mode_contact_annulation: Option<String>,
    pub lignes: Vec<LigneDetail>,
}
