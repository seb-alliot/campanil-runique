use runique::prelude::*;
use sea_orm::FromQueryResult;

/// Chargé via raw SQL — inclut les colonnes built-in + celles ajoutées par extend!{}.
#[derive(Debug, Serialize, FromQueryResult)]
pub struct UserComplet {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    // extend!{} eihwaz_users
    pub telephone: Option<String>,
    pub adresse: Option<String>,
    pub ville: Option<String>,
    pub code_postal: Option<String>,
    pub pays: Option<String>,
}

#[derive(Serialize)]
pub struct LigneResume {
    pub titre: String,
    pub quantite: i32,
    pub prix_unitaire: String,
    pub cuisson: Option<String>,
}

#[derive(Serialize)]
pub struct CommandeResume {
    pub numero: String,
    pub statut_label: String,
    pub statut_css: String,
    pub type_commande: String,
    pub mode_paiement: String,
    pub prix_total: String,
    pub heure_retrait: Option<String>,
    pub avec_livraison: bool,
    pub adresse_livraison: Option<String>,
    pub ville_livraison: Option<String>,
    pub cp_livraison: Option<String>,
    pub heure_livraison: Option<String>,
    pub date: String,
    pub date_iso: String,
    pub date_annulation: Option<String>,
    pub can_cancel: bool,
    pub commande_id: i32,
    pub can_review: bool,
    pub avis_statut: Option<String>,
    pub avis_note: Option<i32>,
    pub avis_commentaire: Option<String>,
    pub lignes: Vec<LigneResume>,
}
