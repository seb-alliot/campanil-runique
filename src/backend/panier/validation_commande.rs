use crate::backend::panier::{
    commander_form_from_body, get_prix_livraison, panier_get, panier_valider,
};
use crate::backend::utils::inject_auth;
use runique::prelude::*;

pub async fn vue_panier_commander(request: &mut Request) -> AppResult<Response> {
    if !request.is_post() {
        return Ok(Redirect::to("/panier").into_response());
    }
    inject_auth(request).await;
    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to("/connexion?next=/panier").into_response());
    };

    let panier = panier_get(&request.session).await;
    if panier.lignes.is_empty() {
        request
            .notices
            .warning("Votre panier est vide.".to_string())
            .await;
        return Ok(Redirect::to("/panier").into_response());
    }

    let form = commander_form_from_body(&request.prisme.data.clone());
    let tz_str = request.engine.config.timezone.clone();
    match panier_valider(&request.session, &request.engine.db, user.id, form, &tz_str).await {
        Ok(numero) => {
            Ok(Redirect::to(&format!("/commande/{}/confirmation", numero)).into_response())
        }
        Err(e) => {
            let total = format!("{:.2}", panier.total());
            let prix_livraison = format!("{:.2}", get_prix_livraison(&request.engine.db).await);
            context_update!(request => {
                "title"          => "Mon panier — U Campanile",
                "erreur"         => e,
                "panier"         => &panier,
                "total"          => total,
                "prix_livraison" => prix_livraison,
            });
            request.render("panier/index.html")
        }
    }
}
