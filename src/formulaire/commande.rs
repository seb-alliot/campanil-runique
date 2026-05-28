pub struct CommandeForm {
    pub mode_paiement: String,
    pub type_retrait: String,
    pub heure_retrait: Option<String>,
    pub adresse_livraison: Option<String>,
    pub ville_livraison: Option<String>,
    pub cp_livraison: Option<String>,
}
