pub mod supprimer_compte;
pub use supprimer_compte::handle_supprimer_compte;

pub mod annuler;
pub use annuler::handle_commande_annuler;

pub mod vue_compte;
pub use vue_compte::handle_compte;

pub mod db_commandes;
pub use db_commandes::get_commandes_user;

pub mod profil;
pub use profil::{handle_profil_post, load_profil};

pub mod ajax_commandes;
pub use ajax_commandes::compte_commandes_ajax;

pub mod struct_;
pub use struct_::{CommandeResume, LigneResume, PlatCommande, StatutHistorique};

pub mod db_plats_commandes;
pub use db_plats_commandes::get_plats_commandes_user;

pub mod avis_plat_poster;
pub use avis_plat_poster::handle_avis_plat;
