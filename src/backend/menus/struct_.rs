use runique::prelude::*;

#[derive(Serialize)]
pub struct MenuCard {
    pub id: i32,
    pub titre: String,
    pub description: String,
    pub prix_par_personne: String,
    pub nb_personnes_min: i32,
    pub theme_id: i32,
    pub theme: String,
    pub regime_id: i32,
    pub regime: String,
    pub image: Option<String>,
    pub stock: i32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MenuFilters {
    pub prix_min: Option<String>,
    pub prix_max: Option<String>,
    pub theme_id: Option<i32>,
    pub regime_id: Option<i32>,
    pub nb_personnes: Option<i32>,
}

#[derive(Serialize)]
pub struct PlatDetail {
    pub titre: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub est_viande: bool,
    pub allergenes: Vec<String>,
}

#[derive(Serialize)]
pub struct MenuDetail {
    pub id: i32,
    pub titre: String,
    pub description: String,
    pub conditions: Option<String>,
    pub prix_par_personne: String,
    pub nb_personnes_min: i32,
    pub theme: String,
    pub regime: String,
    pub stock: i32,
    pub images: Vec<String>,
    pub entrees: Vec<PlatDetail>,
    pub plats_principaux: Vec<PlatDetail>,
    pub desserts: Vec<PlatDetail>,
}
