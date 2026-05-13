pub mod generation_num;
pub use generation_num::generer_numero;

pub mod parser_status_commande;
pub use parser_status_commande::parse_statut;

pub mod info_status;
pub use info_status::statut_info;

pub mod recup_all_status;
pub use recup_all_status::all_statuts;

pub mod admin_commande;
pub use admin_commande::handle_admin_commande_detail;

pub mod struct_;
pub use struct_::{CommandeDetail, LigneDetail};
