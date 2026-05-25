use runique::prelude::*;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone)]
pub struct MenuChoixPanier {
    pub cours: String,
    pub plat_id: Pk,
    pub plat_titre: String,
    pub cuisson: Option<String>,
    #[serde(default)]
    pub garniture_ids: Vec<Pk>,
    #[serde(default)]
    pub sans_sel: bool,
    #[serde(default)]
    pub note: Option<String>,
}

fn default_type_article() -> String {
    "plat".to_string()
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LignePanier {
    pub plat_id: Pk,
    #[serde(default = "default_type_article")]
    pub type_article: String,
    pub titre: String,
    pub prix_unitaire: String,
    pub quantite: i32,
    pub est_viande: bool,
    pub cuisson: Option<String>,
    #[serde(default)]
    pub boisson_id: Option<Pk>,
    #[serde(default)]
    pub menu_id: Option<Pk>,
    #[serde(default)]
    pub supplement_id: Option<Pk>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub garniture_ids: Vec<Pk>,
    #[serde(default)]
    pub sans_sel: bool,
    #[serde(default)]
    pub menu_choix: Vec<MenuChoixPanier>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Panier {
    pub lignes: Vec<LignePanier>,
    pub user_id: Option<Pk>,
}

impl Panier {
    pub fn total(&self) -> Decimal {
        self.lignes.iter().fold(Decimal::ZERO, |acc, l| {
            let prix = Decimal::from_str(&l.prix_unitaire).unwrap_or(Decimal::ZERO);
            acc + prix * Decimal::from(l.quantite)
        })
    }

    pub fn nb_articles(&self) -> i32 {
        self.lignes.iter().map(|l| l.quantite).sum()
    }
}
