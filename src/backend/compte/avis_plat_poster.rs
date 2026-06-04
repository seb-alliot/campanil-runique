use crate::backend::utils::inject_auth;
use crate::entities::commande::StatutCommande;
use crate::entities::{avis_plat, commande, commande_ligne};
use crate::formulaire::AvisForm;
use runique::axum;
use runique::prelude::*;
use serde_json::json;

pub async fn handle_avis_plat(request: Request) -> AppResult<Response> {
    let mut request = request;
    inject_auth(&mut request).await;

    let Some(user) = request.user.clone() else {
        return Ok(axum::Json(json!({"ok": false, "error": "non authentifie"})).into_response());
    };

    let plat_id: Pk = request.get_path_as::<Pk>("plat_id").unwrap_or(0);
    if plat_id == 0 {
        return Ok(axum::Json(json!({"ok": false, "error": "plat invalide"})).into_response());
    }

    // Sécurité IDOR : vérifier que l'user a commandé ce plat dans une commande terminée
    let commande_ids: Vec<Pk> = search!(commande::Entity =>
        UserId eq user.id,
        or(Statut eq StatutCommande::Termine, Statut eq StatutCommande::Livre),
    )
    .all(request.db())
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|c| c.id)
    .collect();

    let a_commande = if commande_ids.is_empty() {
        false
    } else {
        search!(commande_ligne::Entity => CommandeId in (commande_ids), PlatId eq plat_id,)
            .count(request.db())
            .await
            .unwrap_or(0)
            > 0
    };

    if !a_commande {
        return Ok(axum::Json(json!({"ok": false, "error": "non commande"})).into_response());
    }

    let existant = search!(avis_plat::Entity => PlatId eq plat_id, UserId eq user.id,)
        .first(request.db())
        .await
        .ok()
        .flatten();

    if existant.is_some() {
        return Ok(axum::Json(json!({"ok": false, "error": "doublon"})).into_response());
    }

    let note: i32 = request
        .prisme
        .data
        .get("note")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    if !(1..=5).contains(&note) {
        return Ok(axum::Json(json!({"ok": false, "error": "note invalide"})).into_response());
    }

    let mut form: AvisForm = request.form();
    if !form.is_valid().await {
        return Ok(axum::Json(json!({"ok": false, "error": "commentaire requis"})).into_response());
    }

    let commentaire = form.cleaned_string("commentaire").unwrap_or_default();

    let nouveau = avis_plat::ActiveModel {
        plat_id: Set(Some(plat_id)),
        entree_id: Set(None),
        dessert_id: Set(None),
        user_id: Set(Some(user.id)),
        note: Set(note),
        commentaire: Set(commentaire.trim().to_string()),
        statut: Set(avis_plat::StatutAvisPlat::EnAttente),
        ..Default::default()
    };

    if let Err(e) = avis_plat::Entity::insert(nouveau).exec(request.db()).await {
        tracing::error!("Erreur insertion avis plat: {e}");
        return Ok(axum::Json(json!({"ok": false, "error": "db"})).into_response());
    }

    Ok(axum::Json(json!({"ok": true, "note": note})).into_response())
}
