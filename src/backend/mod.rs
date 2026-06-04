pub mod admin_menu_resto;
pub mod avis;
pub mod carte;
pub mod commande;
pub mod compte;
pub mod contact;
pub mod menus;
pub mod panier;
pub mod service;
pub mod stats;
pub mod traiteur;
pub mod user;
pub mod utils;

pub use admin_menu_resto::handle_menu_resto_composition;
pub use avis::{get_avis_valides, handle_avis, handle_avis_supprimer};
pub use carte::{ajax_avis_plat, vue_boissons_type, vue_carte};
pub use commande::handle_admin_commande_detail;
pub use compte::{
    compte_commandes_ajax, handle_avis_article, handle_avis_article_supprimer, handle_avis_plat,
    handle_avis_plat_supprimer, handle_commande_annuler, handle_compte, handle_modifier_ligne,
    handle_profil_post, handle_supprimer_compte, handle_supprimer_ligne,
};
pub use contact::handle_contact;
pub use menus::{vue_menu_details, vue_menus, vue_track_menu_filters};
pub use panier::{
    vue_ajouter_panier, vue_commande_confirmation, vue_livraison_prix, vue_panier,
    vue_panier_commander, vue_retirer_panier,
};
pub use service::{
    ajax_commandes, ajax_stats, ajax_stock_get, ajax_stock_update, handle_service,
    handle_service_statut,
};
pub use stats::load_stats;
pub use traiteur::{handle_devis_confirmation, handle_devis_traiteur};
pub use user::{handle_activate, handle_inscription, handle_login};
pub use utils::inject_auth;
