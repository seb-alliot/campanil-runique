use crate::backend::compte::load_profil;
use crate::backend::panier::{get_prix_livraison, panier_get};
use crate::backend::utils::inject_auth;
use runique::prelude::*;

pub async fn vue_panier(request: &mut Request) -> AppResult<Response> {
    inject_auth(request).await;
    let panier = panier_get(&request.session).await;
    let total = panier.total();
    let prix_livraison = get_prix_livraison(&request.engine.db).await;

    let profil = if let Some(user) = &request.user {
        load_profil(&request.engine.db, user.id).await
    } else {
        None
    };

    context_update!(request => {
        "title"          => "Mon panier — U Campanile",
        "panier"         => &panier,
        "total"          => format!("{:.2}", total),
        "prix_livraison" => format!("{:.2}", prix_livraison),
        "profil"         => &profil,
    });
    request.render("panier/index.html")
}
