use crate::backend::panier::{get_prix_livraison, prix_livraison_distance};
use runique::prelude::*;

pub async fn vue_livraison_prix(request: Request) -> AppResult<Response> {
    let adresse = request.get_query("adresse").unwrap_or("").to_string();
    let cp = request.get_query("cp").unwrap_or("").to_string();
    let ville = request.get_query("ville").unwrap_or("").to_string();

    if adresse.trim().is_empty() || ville.trim().is_empty() {
        return Ok(
            Json(serde_json::json!({ "prix": null, "erreur": "Adresse incomplète" }))
                .into_response(),
        );
    }

    let prix = match prix_livraison_distance(&request.engine.db, &adresse, &cp, &ville).await {
        Some(p) => p,
        None => get_prix_livraison(&request.engine.db).await,
    };
    Ok(Json(serde_json::json!({ "prix": prix.to_string() })).into_response())
}
