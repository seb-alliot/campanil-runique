use runique::prelude::*;

#[derive(Serialize)]
pub struct StatutHistorique {
    pub statut: String,
    pub heure: String,
}

#[derive(Serialize)]
pub struct LigneResume {
    pub id: Pk,
    pub titre: String,
    pub quantite: i32,
    pub prix_unitaire: String,
    pub cuisson: Option<String>,
    pub sans_sel: bool,
    pub note: Option<String>,
    pub type_article: String,
}

#[derive(Serialize)]
pub struct PlatCommande {
    pub id: Pk,
    pub titre: String,
    pub image: Option<String>,
    pub avis_note: Option<i32>,
    pub avis_statut: Option<String>,
}

#[derive(Serialize)]
pub struct CommandeResume {
    pub numero: String,
    pub statut_label: String,
    pub statut_css: String,
    pub mode_paiement: String,
    pub prix_total: String,
    pub heure_retrait: Option<String>,
    pub type_retrait: String,
    pub adresse_livraison: Option<String>,
    pub ville_livraison: Option<String>,
    pub cp_livraison: Option<String>,
    pub date: String,
    pub date_iso: String,
    pub date_annulation: Option<String>,
    pub modifiable: bool,
    pub can_cancel: bool,
    pub commande_id: Pk,
    pub can_review: bool,
    pub avis_statut: Option<String>,
    pub avis_note: Option<i32>,
    pub avis_commentaire: Option<String>,
    pub lignes: Vec<LigneResume>,
    pub statuts: Vec<StatutHistorique>,
}
