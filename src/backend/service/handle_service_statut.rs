use crate::backend::commande::parse_statut;
use crate::backend::service::charger_commandes::garde_acces;
use crate::entities::{commande, commande_statut};
use runique::context;
use runique::prelude::*;
use sea_orm::{ActiveModelTrait, Set};

pub async fn handle_service_statut(request: &mut Request) -> AppResult<Response> {
    if !garde_acces(request) {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "403"})),
        )
            .into_response());
    }

    if !request.prisme.csrf_valid {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "csrf"})),
        )
            .into_response());
    }

    let numero = request.get_path("numero").unwrap_or("").to_string();
    let nouveau_statut = request
        .prisme
        .data
        .get("statut")
        .cloned()
        .unwrap_or_default();

    let Some(statut) = parse_statut(&nouveau_statut) else {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "statut invalide"})),
        )
            .into_response());
    };

    let db = request.db();

    let Some(cmd) = search!(commande::Entity => Numero eq &numero,)
        .first(db)
        .await
        .ok()
        .flatten()
    else {
        return Ok((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "introuvable"})),
        )
            .into_response());
    };

    let mut active = commande::ActiveModel {
        id: Set(cmd.id),
        statut: Set(statut.clone()),
        ..Default::default()
    };
    if nouveau_statut == "en_preparation" {
        active.modifiable = Set(false);
    }
    if active.update(db).await.is_err() {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "db"})),
        )
            .into_response());
    }

    commande_statut::ActiveModel {
        commande_id: Set(cmd.id),
        statut: Set(nouveau_statut.clone()),
        ..Default::default()
    }
    .insert(db)
    .await
    .ok();

    let statut_str = nouveau_statut.as_str();
    if mailer_configured()
        && let Ok(Some(user)) = runique_users::Entity::find_by_id(cmd.user_id).one(db).await
    {
        if statut_str == "pret" || statut_str == "livre" {
            let base_url = request
                .headers
                .get("host")
                .and_then(|v| v.to_str().ok())
                .map(|h| format!("http://{h}"))
                .unwrap_or_else(|| "http://localhost:3000".to_string());
            let (statut_titre, statut_message) = if statut_str == "livre" {
                ("Votre commande est livrée", "a bien été livrée.")
            } else {
                (
                    "Votre commande est prête",
                    "est prête et peut être récupérée.",
                )
            };
            let ctx = context! {
                "username"       => &user.username,
                "numero"         => &cmd.numero,
                "prix_total"     => &format!("{:.2}", cmd.prix_total),
                "statut_titre"   => statut_titre,
                "statut_message" => statut_message,
                "compte_url"     => &format!("{}/compte", base_url),
            };
            if let Ok(msg) = Email::new()
                .to(user.email.clone())
                .subject("Votre commande U Campanile")
                .template(
                    &request.engine.tera,
                    "emails/commande_terminee.html",
                    ctx.into(),
                )
            {
                msg.send().await.ok();
            }
        } else if statut_str == "annule" {
            let motif_str = request
                .prisme
                .data
                .get("motif_annulation")
                .map(|s| s.as_str())
                .unwrap_or("Non disponible");
            let mode_contact_str = request
                .prisme
                .data
                .get("mode_contact_annulation")
                .map(|s| s.as_str())
                .unwrap_or("Non disponible");
            let ctx = context! {
                "username"     => &user.username,
                "numero"       => &cmd.numero,
                "motif"        => motif_str,
                "mode_contact" => mode_contact_str,
            };
            if let Ok(msg) = Email::new()
                .to(user.email.clone())
                .subject("Annulation de votre commande — U Campanile")
                .template(
                    &request.engine.tera,
                    "emails/commande_annulee.html",
                    ctx.into(),
                )
            {
                msg.send().await.ok();
            }
        }
    }

    Ok(Json(serde_json::json!({"ok": true})).into_response())
}
