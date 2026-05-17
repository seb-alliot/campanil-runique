pub mod ajax_commandes;
pub mod ajax_stats;
pub mod ajax_stock;
pub mod charger_commandes;
pub mod handle_service;
pub mod handle_service_statut;
pub mod struct_;

pub use ajax_commandes::ajax_commandes;
pub use ajax_stats::ajax_stats;
pub use ajax_stock::{ajax_stock_get, ajax_stock_update};
pub use handle_service::handle_service;
pub use handle_service_statut::handle_service_statut;
