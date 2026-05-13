pub mod get_info_user;
pub use get_info_user::get_credentials;

pub mod connected;
pub use connected::authenticate_user;

pub mod inscription;
pub use inscription::register_user;

pub mod activate;
pub use activate::handle_activate;

pub mod auth;
pub use auth::{handle_inscription, handle_login};
