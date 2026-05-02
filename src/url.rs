use crate::views::*;
use runique::prelude::*;

pub fn routes() -> Router {
    urlpatterns! {
        "/"             => view!{ index },       name = "index",
        "/connexion"    => view!{ connexion },   name = "connexion",
        "/deconnexion"  => view!{ deconnexion }, name = "deconnexion",
        "/inscription"  => view!{ inscription }, name = "inscription",
        "/activer/<token>/<encrypted_email>" => view!{ activer }, name = "activer",
    }
}
