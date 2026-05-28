use crate::backend::utils::inject_auth;
use crate::entities::commande;
use runique::prelude::*;

pub async fn vue_commande_confirmation(request: &mut Request) -> AppResult<Response> {
    inject_auth(request).await;
    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to("/connexion").into_response());
    };
    let numero = request.get_path("numero").unwrap_or("").to_string();
    let cmd = search!(commande::Entity => Numero eq &numero, UserId eq user.id,)
        .first(request.db())
        .await
        .ok()
        .flatten();
    if cmd.is_none() {
        return Ok(Redirect::to("/compte").into_response());
    }
    context_update!(request => {
        "title"  => "Commande confirmée — U Campanile",
        "numero" => &numero,
    });
    request.render("panier/confirmation.html")
}
