use crate::backend::panier::prix_livraison_distance;
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

    match prix_livraison_distance(&request.engine.db, &adresse, &cp, &ville).await {
        Some(prix) => Ok(Json(serde_json::json!({ "prix": prix.to_string() })).into_response()),
        None => Ok(
            Json(serde_json::json!({ "prix": null, "erreur": "Adresse introuvable ou coordonnées du restaurant non configurées" }))
                .into_response(),
        ),
    }
}
