use runique::context;
use runique::prelude::*;
use sea_orm::Set;

use crate::entities::{commande, commande::StatutCommande, info_resto};

pub async fn handle_appliquer_penalite(request: &mut Request) -> AppResult<Response> {
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

    if cmd.penalite_envoyee {
        return Ok((
            StatusCode::CONFLICT,
            Json(serde_json::json!({"error": "pénalité déjà appliquée"})),
        )
            .into_response());
    }

    let montant = info_resto::Entity::find()
        .one(db)
        .await
        .ok()
        .flatten()
        .map(|r| r.penalite_materiel)
        .unwrap_or_else(|| Decimal::new(600, 0));

    let nouveau_total = cmd.prix_total + montant;

    let active = commande::ActiveModel {
        prix_total: Set(nouveau_total),
        penalite_envoyee: Set(true),
        ..Default::default()
    };
    if write::update(preuve, active, &**db).await.is_err() {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "erreur mise à jour"})),
        )
            .into_response());
    }

    let Ok(Some(user)) = runique_users::Entity::find_by_id(cmd.user_id).one(db).await else {
        return Ok(
            Json(serde_json::json!({ "ok": true, "montant": montant.to_string() })).into_response(),
        );
    };

    let montant_fmt = format!("{:.2} €", montant).replace('.', ",");
    let ctx = context! {
        "username" => &user.username,
        "numero"   => &cmd.numero,
        "montant"  => &montant_fmt,
    };

    if let Ok(msg) = Email::new()
        .to(user.email.clone())
        .subject("Supplément non-retour de matériel — U Campanile")
        .template(
            &request.engine.tera,
            "emails/penalite_materiel.html",
            ctx.into(),
        )
    {
        msg.send().await.ok();
    }

    Ok(Json(serde_json::json!({ "ok": true, "montant": montant_fmt })).into_response())
}
