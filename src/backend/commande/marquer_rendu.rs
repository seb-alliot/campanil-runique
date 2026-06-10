use runique::prelude::*;
use sea_orm::{ActiveModelTrait, Set};

use crate::backend::service::charger_commandes::garde_acces;
use crate::entities::{commande, commande::StatutCommande};

pub async fn handle_marquer_rendu(request: &mut Request) -> AppResult<Response> {
    if !garde_acces(request) {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "403"})),
        )
            .into_response());
    }

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
        id: Set(cmd.id),
        pret_materiel: Set(true),
        ..Default::default()
    };
    if active.update(db).await.is_err() {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "erreur mise à jour"})),
        )
            .into_response());
    }

    Ok(Json(serde_json::json!({ "ok": true })).into_response())
}
