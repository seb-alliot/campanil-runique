use runique::prelude::*;

#[derive(Serialize)]
pub struct MenuChoixService {
    pub cours_label: String,
    pub titre: String,
    pub cuisson: Option<String>,
    pub garnitures: Vec<String>,
    pub avec_legumes: bool,
    pub sans_sel: bool,
    pub note: Option<String>,
}

#[derive(Serialize)]
pub struct LigneService {
    pub titre: String,
    pub quantite: i32,
    pub cuisson: Option<String>,
    pub note: Option<String>,
    pub garnitures: Vec<String>,
    pub avec_legumes: bool,
    pub sans_sel: bool,
    pub menu_choix: Vec<MenuChoixService>,
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
    pub avec_livraison: bool,
    pub adresse_livraison: Option<String>,
    pub lignes: Vec<LigneService>,
}
