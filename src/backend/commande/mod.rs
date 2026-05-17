pub mod num_commande;
pub use num_commande::generer_numero;

pub mod parse_statut;
pub use parse_statut::parse_statut;

pub mod statut_info;
pub use statut_info::statut_info;

pub mod db_statuts;
pub use db_statuts::all_statuts;

pub mod vue_admin_detail;
pub use vue_admin_detail::handle_admin_commande_detail;

pub mod struct_;
pub use struct_::{CommandeDetail, LigneDetail};
