use crate::backend::utils::inject_auth;
use crate::entities::commande::StatutCommande;
use crate::entities::{avis_plat, commande, commande_ligne};
use crate::formulaire::AvisForm;
use runique::axum;
use runique::prelude::*;
use serde_json::json;

pub async fn handle_avis_article(request: Request) -> AppResult<Response> {
    let mut request = request;
    inject_auth(&mut request).await;

    let Some(user) = request.user.clone() else {
        return Ok(axum::Json(json!({"ok": false, "error": "non authentifie"})).into_response());
    };

    let type_article = request.get_path("type_article").unwrap_or("").to_string();
    let article_id: Pk = request.get_path_as::<Pk>("id").unwrap_or(0);
    if article_id == 0 {
        return Ok(axum::Json(json!({"ok": false, "error": "article invalide"})).into_response());
    }

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

    if commande_ids.is_empty() {
        return Ok(axum::Json(json!({"ok": false, "error": "non commande"})).into_response());
    }

    // Vérification IDOR : l'utilisateur a bien commandé cet article
    let a_commande = match type_article.as_str() {
        "plat" => {
            search!(commande_ligne::Entity => CommandeId in (commande_ids), PlatId eq article_id,)
                .count(request.db()).await.unwrap_or(0) > 0
        }
        "entree" => {
            search!(commande_ligne::Entity => CommandeId in (commande_ids), EntreeId eq article_id,)
                .count(request.db()).await.unwrap_or(0) > 0
        }
        "dessert" => {
            search!(commande_ligne::Entity => CommandeId in (commande_ids), DessertId eq article_id,)
                .count(request.db()).await.unwrap_or(0) > 0
        }
        _ => false,
    };

    if !a_commande {
        return Ok(axum::Json(json!({"ok": false, "error": "non commande"})).into_response());
    }

    // Vérification doublon
    let existant = match type_article.as_str() {
        "plat" => search!(avis_plat::Entity => PlatId eq article_id, UserId eq user.id,)
            .first(request.db()).await.ok().flatten(),
        "entree" => search!(avis_plat::Entity => EntreeId eq article_id, UserId eq user.id,)
            .first(request.db()).await.ok().flatten(),
        "dessert" => search!(avis_plat::Entity => DessertId eq article_id, UserId eq user.id,)
            .first(request.db()).await.ok().flatten(),
        _ => None,
    };

    if existant.is_some() {
        return Ok(axum::Json(json!({"ok": false, "error": "doublon"})).into_response());
    }

    let note: i32 = request
        .prisme.data.get("note")
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

    let nouveau = match type_article.as_str() {
        "plat" => avis_plat::ActiveModel {
            plat_id: Set(Some(article_id)),
            entree_id: Set(None),
            dessert_id: Set(None),
            user_id: Set(Some(user.id)),
            note: Set(note),
            commentaire: Set(commentaire.trim().to_string()),
            statut: Set(avis_plat::StatutAvisPlat::EnAttente),
            ..Default::default()
        },
        "entree" => avis_plat::ActiveModel {
            plat_id: Set(None),
            entree_id: Set(Some(article_id)),
            dessert_id: Set(None),
            user_id: Set(Some(user.id)),
            note: Set(note),
            commentaire: Set(commentaire.trim().to_string()),
            statut: Set(avis_plat::StatutAvisPlat::EnAttente),
            ..Default::default()
        },
        "dessert" => avis_plat::ActiveModel {
            plat_id: Set(None),
            entree_id: Set(None),
            dessert_id: Set(Some(article_id)),
            user_id: Set(Some(user.id)),
            note: Set(note),
            commentaire: Set(commentaire.trim().to_string()),
            statut: Set(avis_plat::StatutAvisPlat::EnAttente),
            ..Default::default()
        },
        _ => return Ok(axum::Json(json!({"ok": false, "error": "type invalide"})).into_response()),
    };

    if let Err(e) = avis_plat::Entity::insert(nouveau).exec(request.db()).await {
        tracing::error!("Erreur insertion avis article: {e}");
        return Ok(axum::Json(json!({"ok": false, "error": "db"})).into_response());
    }

    Ok(axum::Json(json!({"ok": true, "note": note})).into_response())
}
