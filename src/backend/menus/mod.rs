pub mod struct_;
pub use struct_::{MenuCard, MenuDetail, MenuFilters, PlatDetail};

pub mod db_menu;
pub use db_menu::get_menu_cards;

pub mod db_theme;
pub use db_theme::get_themes_list;

pub mod db_regime;
pub use db_regime::get_regimes_list;

pub mod db_menu_details;
pub use db_menu_details::get_menu_detail;

pub mod display_menu;
pub use display_menu::vue_menus;

pub mod display_menu_details;
pub use display_menu_details::vue_menu_details;
