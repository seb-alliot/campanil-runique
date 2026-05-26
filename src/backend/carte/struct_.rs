use runique::prelude::*;

#[derive(Serialize, Clone)]
pub struct CarteGarniture {
    pub id: Pk,
    pub libelle: String,
    pub type_garniture: String,
    pub est_defaut: bool,
}

#[derive(Serialize, Clone)]
pub struct CarteGarnitureGroupe {
    pub label: String,
    pub items: Vec<CarteGarniture>,
}

#[derive(Serialize, Clone)]
pub struct CartePlat {
    pub id: Pk,
    pub titre: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub prix: String,
    pub image: Option<String>,
    pub est_viande: bool,
    pub disponible: bool,
    pub allergenes: Vec<String>,
    pub garnitures: Vec<CarteGarnitureGroupe>,
    pub supplements: Vec<CarteSupplementItem>,
}

#[derive(Serialize)]
pub struct CarteBoisson {
    pub id: Pk,
    pub titre: String,
    pub description: Option<String>,
    pub prix: String,
    pub image: Option<String>,
    pub disponible: bool,
}

#[derive(Serialize)]
pub struct CarteBoissonGroupe {
    pub type_label: String,
    pub type_slug: String,
    pub items: Vec<CarteBoisson>,
}

#[derive(Serialize, Clone)]
pub struct CarteSupplementItem {
    pub id: Pk,
    pub libelle: String,
    pub prix: String,
}

#[derive(Serialize, Clone)]
pub struct CoursMenu {
    pub label: String,
    pub key: String,
    pub plats: Vec<CartePlat>,
}

#[derive(Serialize, Clone)]
pub struct CarteMenuResto {
    pub id: Pk,
    pub nom: String,
    pub type_menu: String,
    pub prix: String,
    pub description: Option<String>,
    pub entree_libre: Option<String>,
    pub plat_libre: Option<String>,
    pub dessert_libre: Option<String>,
    pub cours: Vec<CoursMenu>,
}

#[derive(Serialize)]
pub struct CarteMenuSection {
    pub label: String,
    pub key: String,
    pub menus: Vec<CarteMenuResto>,
}

#[derive(Serialize)]
pub struct CartePage {
    pub entrees: Vec<CartePlat>,
    pub specialites: Vec<CartePlat>,
    pub viandes: Vec<CartePlat>,
    pub poissons: Vec<CartePlat>,
    pub desserts: Vec<CartePlat>,
    pub menus: Vec<CarteMenuSection>,
    pub supplements: Vec<CarteSupplementItem>,
    pub boissons: Vec<CarteBoissonGroupe>,
}
