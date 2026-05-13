use crate::backend::service::charger_commandes::{charger_commandes_jour, garde_acces, grouper_par_statut};
use runique::prelude::*;

pub async fn handle_service_json(request: &Request) -> AppResult<Response> {
    if !garde_acces(request) {
        return Ok(
            (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "403"}))).into_response(),
        );
    }
    let db = request.db();
    let commandes = charger_commandes_jour(db, &request.engine.config.timezone).await;

    let (retraits, livraisons): (Vec<_>, Vec<_>) =
        commandes.into_iter().partition(|c| !c.avec_livraison);

    let (ra, rac, rp, rpr) = grouper_par_statut(retraits);
    let (la, lac, lp, lpr) = grouper_par_statut(livraisons);

    Ok(Json(serde_json::json!({
        "retraits_attente":       ra,
        "retraits_accepte":       rac,
        "retraits_preparation":   rp,
        "retraits_pret":          rpr,
        "livraisons_attente":     la,
        "livraisons_accepte":     lac,
        "livraisons_preparation": lp,
        "livraisons_pret":        lpr,
    }))
    .into_response())
}
