pub struct CommandeForm {
    pub mode_paiement: String,
    pub heure_retrait: Option<String>,
    pub avec_livraison: bool,
    pub adresse_livraison: Option<String>,
    pub ville_livraison: Option<String>,
    pub cp_livraison: Option<String>,
    pub heure_livraison: Option<String>,
}
