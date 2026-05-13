const SESSION_PANIER_KEY: &str = "panier";

pub mod struct_;
pub use struct_::{LignePanier, MenuChoixPanier, Panier};

pub mod db_panier;
pub use db_panier::panier_get;

pub mod panier_persistant;
pub use panier_persistant::panier_save;

pub mod add_panier;
pub use add_panier::panier_ajouter;

pub mod delete_du_panier;
pub use delete_du_panier::{panier_retirer, panier_retirer_menu};

pub mod add_boisson;
pub use add_boisson::panier_ajouter_boisson;

pub mod add_supplement;
pub use add_supplement::panier_ajouter_supplement;

pub mod add_menu;
pub use add_menu::panier_ajouter_menu;

pub mod delete_boisson;
pub use delete_boisson::panier_retirer_boisson;

pub mod reset_panier;
pub use reset_panier::panier_vider;

pub mod db_prix_livraison;
pub use db_prix_livraison::get_prix_livraison;

pub mod validation_panier;
pub use validation_panier::panier_valider;

pub mod display_form_commande;
pub use display_form_commande::commander_form_from_body;

pub mod panier;
pub use panier::vue_panier;

pub mod display_add_panier;
pub use display_add_panier::vue_ajouter_panier;

pub mod display_delete_panier;
pub use display_delete_panier::vue_retirer_panier;

pub mod validation_commande;
pub use validation_commande::vue_panier_commander;

pub mod confirmation_commande;
pub use confirmation_commande::vue_commande_confirmation;
