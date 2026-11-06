use runique::prelude::*;
use sea_orm::Set;

use crate::entities::{commande, commande::StatutCommande};

pub async fn handle_marquer_rendu(request: &mut Request) -> AppResult<Response> {
    let id: Pk = match request.get_path("id").and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => {
            return Ok((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "id invalide"})),
            )
                .into_response());
        }
    };

    let Some(user) = request.user.clone() else {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "403"})),
        )
            .into_response());
    };
    let engine = request.engine.clone();
    let gate = GateCtx {
        db: &engine.db,
        user: &user,
    };
    let preuve = match authorize_row::<Update, commande::Entity>(&gate, id).await {
        Ok(preuve) => preuve,
        Err(refus) => {
            return Ok((
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({"error": refus.message()})),
            )
                .into_response());
        }
    };

    let db = request.db();
    let cmd = match search!(commande::Entity => Id eq id).first(db).await {
        Ok(Some(c)) => c,
        _ => {
            return Ok((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "commande introuvable"})),
            )
                .into_response());
        }
    };

    if cmd.statut != StatutCommande::Livre {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "statut invalide"})),
        )
            .into_response());
    }

    let active = commande::ActiveModel {
        pret_materiel: Set(true),
        ..Default::default()
    };
    if write::update(preuve, active, &**db).await.is_err() {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "erreur mise à jour"})),
        )
            .into_response());
    }

    Ok(Json(serde_json::json!({ "ok": true })).into_response())
}
