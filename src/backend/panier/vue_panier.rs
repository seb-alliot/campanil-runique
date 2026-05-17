use crate::backend::panier::{get_prix_livraison, panier_get};
use crate::backend::utils::inject_auth;
use runique::prelude::*;

pub async fn vue_panier(request: &mut Request) -> AppResult<Response> {
    inject_auth(request).await;
    let panier = panier_get(&request.session).await;
    let total = panier.total();
    let prix_livraison = get_prix_livraison(&request.engine.db).await;
    context_update!(request => {
        "title"          => "Mon panier — U Campanile",
        "panier"         => &panier,
        "total"          => format!("{:.2}", total),
        "prix_livraison" => format!("{:.2}", prix_livraison),
    });
    request.render("panier/index.html")
}
