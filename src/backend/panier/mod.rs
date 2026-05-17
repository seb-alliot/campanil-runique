const SESSION_PANIER_KEY: &str = "panier";

pub mod struct_;
pub use struct_::{LignePanier, MenuChoixPanier, Panier};

pub mod session_get;
pub use session_get::panier_get;

pub mod session_save;
pub use session_save::panier_save;

pub mod ajouter_plat;
pub use ajouter_plat::{PanierAjouterParams, panier_ajouter};

pub mod retirer_plat;
pub use retirer_plat::{panier_retirer, panier_retirer_menu, panier_retirer_supplement};

pub mod ajouter_boisson;
pub use ajouter_boisson::panier_ajouter_boisson;

pub mod ajouter_supplement;
pub use ajouter_supplement::panier_ajouter_supplement;

pub mod ajouter_menu;
pub use ajouter_menu::panier_ajouter_menu;

pub mod retirer_boisson;
pub use retirer_boisson::panier_retirer_boisson;

pub mod vider;
pub use vider::panier_vider;

pub mod db_livraison;
pub use db_livraison::{get_prix_livraison, prix_livraison_distance};

pub mod valider;
pub use valider::panier_valider;

pub mod form_commande;
pub use form_commande::commander_form_from_body;

pub mod vue_panier;
pub use vue_panier::vue_panier;

pub mod vue_ajouter;
pub use vue_ajouter::vue_ajouter_panier;

pub mod vue_retirer;
pub use vue_retirer::vue_retirer_panier;

pub mod vue_commander;
pub use vue_commander::vue_panier_commander;

pub mod vue_confirmation;
pub use vue_confirmation::vue_commande_confirmation;

pub mod vue_livraison_prix;
pub use vue_livraison_prix::vue_livraison_prix;
