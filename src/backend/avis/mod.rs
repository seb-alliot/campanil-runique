pub mod index_avis;
pub use index_avis::get_avis_valides;
use crate::backend::utils::inject_auth;
use crate::entities::{avis, commande};
use crate::entities::commande::StatutCommande;
use crate::formulaire::AvisForm;
use runique::prelude::*;

pub async fn handle_avis(request: Request) -> AppResult<Response> {
    let mut request = request;
    inject_auth(&mut request).await;

    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to("/connexion").into_response());
    };

    let commande_id: i32 = request.get_path_as::<i32>("commande_id").unwrap_or(0);

    // IDOR: load commande and verify ownership simultaneously
    let cmd_opt = commande::Entity::find()
        .filter(commande::Column::Id.eq(commande_id))
        .filter(commande::Column::UserId.eq(user.id))
        .one(request.db())
        .await
        .ok()
        .flatten();

    let Some(cmd) = cmd_opt else {
        request.notices.error("Commande introuvable.".to_string()).await;
        return Ok(Redirect::to("/compte").into_response());
    };

    if !matches!(cmd.statut, StatutCommande::Termine | StatutCommande::Livre) {
        request.notices.error("Un avis ne peut être laissé que pour une commande terminée.".to_string()).await;
        return Ok(Redirect::to("/compte").into_response());
    }

    let existing = avis::Entity::find()
        .filter(avis::Column::CommandeId.eq(commande_id))
        .one(request.db())
        .await
        .ok()
        .flatten();

    if existing.is_some() {
        request.notices.error("Vous avez déjà laissé un avis pour cette commande.".to_string()).await;
        return Ok(Redirect::to("/compte").into_response());
    }

    let note: i32 = request.prisme.data
        .get("note")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    if !(1..=5).contains(&note) {
        request.notices.error("Note invalide.".to_string()).await;
        return Ok(Redirect::to("/compte").into_response());
    }

    let mut form: AvisForm = request.form();

    if !form.is_valid().await {
        request.notices.error("Formulaire invalide.".to_string()).await;
        return Ok(Redirect::to("/compte").into_response());
    }

    let commentaire = form.cleaned_string("commentaire").unwrap_or_default();

    let new_avis = avis::ActiveModel {
        commande_id: Set(commande_id),
        user_id: Set(user.id),
        note: Set(note),
        commentaire: Set(commentaire.trim().to_string()),
        statut: Set(avis::StatutAvis::EnAttente),
        ..Default::default()
    };

    if let Err(e) = avis::Entity::insert(new_avis).exec(request.db()).await {
        tracing::error!("Erreur insertion avis: {e}");
        request.notices.error("Une erreur est survenue.".to_string()).await;
        return Ok(Redirect::to("/compte").into_response());
    }

    request.notices.success("Votre avis a bien été envoyé. Il sera publié après modération.".to_string()).await;
    Ok(Redirect::to("/compte").into_response())
}

pub async fn handle_avis_supprimer(request: Request) -> AppResult<Response> {
    let mut request = request;
    inject_auth(&mut request).await;

    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to("/connexion").into_response());
    };

    if !request.prisme.csrf_valid {
        return Ok(Redirect::to("/compte").into_response());
    }

    let commande_id: i32 = request.get_path_as::<i32>("commande_id").unwrap_or(0);

    let deleted = avis::Entity::delete_many()
        .filter(avis::Column::CommandeId.eq(commande_id))
        .filter(avis::Column::UserId.eq(user.id))
        .exec(request.db())
        .await
        .map(|r| r.rows_affected)
        .unwrap_or(0);

    if deleted > 0 {
        request.notices.success("Votre avis a été supprimé.".to_string()).await;
    }

    Ok(Redirect::to("/compte").into_response())
}
