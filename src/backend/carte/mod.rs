pub mod boisson;
pub use boisson::build_boissons;

pub mod vue_boissons_type;
pub use vue_boissons_type::vue_boissons_type;

pub mod supplement;
pub use supplement::build_supplements;

pub mod carte_handler;
pub use carte_handler::vue_carte;

pub mod carte;
pub use carte::get_carte;

pub mod struct_;
pub use struct_::{
    CarteBoisson, CarteBoissonGroupe, CarteGarniture, CarteMenuResto, CartePage, CartePlat,
    CarteSupplementItem, CoursMenu,
};
