use crate::backend::utils::inject_auth;
use runique::prelude::*;

pub async fn vue_commande_confirmation(request: &mut Request) -> AppResult<Response> {
    inject_auth(request).await;
    let numero = request.get_path("numero").unwrap_or("").to_string();
    context_update!(request => {
        "title"  => "Commande confirmée — U Campanile",
        "numero" => &numero,
    });
    request.render("panier/confirmation.html")
}
