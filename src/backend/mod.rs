pub mod avis;
pub mod carte;
pub mod commande;
pub mod compte;
pub mod contact;
pub mod menus;
pub mod panier;
pub mod service;
pub mod traiteur;
pub mod user;
pub mod utils;

pub use avis::{get_avis_valides, handle_avis, handle_avis_supprimer};
pub use carte::{vue_boissons_type, vue_carte};
pub use commande::handle_admin_commande_detail;
pub use compte::{handle_commande_annuler, handle_compte, handle_profil_post, handle_supprimer_compte};
pub use contact::handle_contact;
pub use menus::{vue_menu_details, vue_menus};
pub use panier::{
    vue_ajouter_panier, vue_commande_confirmation, vue_panier, vue_panier_commander,
    vue_retirer_panier,
};
pub use service::{handle_service, handle_service_json, handle_service_statut};
pub use traiteur::{handle_devis_confirmation, handle_devis_traiteur};
pub use user::{handle_activate, handle_inscription, handle_login};
pub use utils::inject_auth;
