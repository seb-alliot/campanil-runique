pub mod db_boissons;
pub use db_boissons::build_boissons;

pub mod vue_boissons_type;
pub use vue_boissons_type::vue_boissons_type;

pub mod db_supplements;
pub use db_supplements::build_supplements;

pub mod vue_carte;
pub use vue_carte::vue_carte;

pub mod db_carte;
pub use db_carte::get_carte;

pub mod ajax_avis_plat;
pub use ajax_avis_plat::ajax_avis_plat;

pub mod struct_;
pub use struct_::*;
