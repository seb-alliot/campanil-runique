pub mod struct_;
pub use struct_::{MenuCard, MenuDetail, MenuFilters, PlatDetail};

pub mod db_menus;
pub use db_menus::get_menu_cards;

pub mod db_theme;
pub use db_theme::get_themes_static;

pub mod db_regime;
pub use db_regime::get_regimes_static;

pub mod db_detail;
pub use db_detail::get_menu_detail;

pub mod vue_menus;
pub use vue_menus::vue_menus;

pub mod vue_detail;
pub use vue_detail::vue_menu_details;

pub mod ajax_track_filtres;
pub use ajax_track_filtres::vue_track_menu_filters;
