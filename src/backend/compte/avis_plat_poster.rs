use crate::backend::utils::inject_auth;
use crate::entities::commande::StatutCommande;
use crate::entities::{avis_plat, commande, commande_ligne};
use crate::formulaire::AvisForm;
use runique::prelude::*;
use sea_orm::{Condition, QueryFilter};

pub async fn handle_avis_plat(request: Request) -> AppResult<Response> {
    let mut request = request;
    inject_auth(&mut request).await;

    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to("/connexion").into_response());
    };

    if !request.prisme.csrf_valid {
        return Ok(Redirect::to("/compte?tab=plats").into_response());
    }

    let plat_id: Pk = request.get_path_as::<Pk>("plat_id").unwrap_or(0);
    if plat_id == 0 {
        return Ok(Redirect::to("/compte?tab=plats").into_response());
    }

    // Sécurité IDOR : vérifier que l'user a commandé ce plat dans une commande terminée
    let commande_ids: Vec<Pk> = search!(commande::Entity => UserId eq user.id,)
        .filter(
            Condition::any()
                .add(commande::Column::Statut.eq(StatutCommande::Termine))
                .add(commande::Column::Statut.eq(StatutCommande::Livre)),
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
        search!(commande_ligne::Entity => CommandeId in (commande_ids),)
            .filter(commande_ligne::Column::PlatId.eq(plat_id))
            .count(request.db())
            .await
            .unwrap_or(0)
            > 0
    };

    if !a_commande {
        request
            .notices
            .error("Vous ne pouvez noter que des plats que vous avez commandés.".to_string())
            .await;
        return Ok(Redirect::to("/compte?tab=plats").into_response());
    }

    // Pas de doublon
    let existant = avis_plat::Entity::find()
        .filter(avis_plat::Column::PlatId.eq(plat_id))
        .filter(avis_plat::Column::UserId.eq(user.id))
        .one(request.db())
        .await
        .ok()
        .flatten();

    if existant.is_some() {
        request
            .notices
            .error("Vous avez déjà laissé un avis pour ce plat.".to_string())
            .await;
        return Ok(Redirect::to("/compte?tab=plats").into_response());
    }

    let note: i32 = request
        .prisme
        .data
        .get("note")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    if !(1..=5).contains(&note) {
        request.notices.error("Note invalide.".to_string()).await;
        return Ok(Redirect::to("/compte?tab=plats").into_response());
    }

    let mut form: AvisForm = request.form();
    if !form.is_valid().await {
        request
            .notices
            .error("Commentaire requis.".to_string())
            .await;
        return Ok(Redirect::to("/compte?tab=plats").into_response());
    }

    let commentaire = form.cleaned_string("commentaire").unwrap_or_default();

    let nouveau = avis_plat::ActiveModel {
        plat_id: Set(plat_id),
        user_id: Set(Some(user.id)),
        note: Set(note),
        commentaire: Set(commentaire.trim().to_string()),
        statut: Set(avis_plat::StatutAvisPlat::EnAttente),
        ..Default::default()
    };

    if let Err(e) = avis_plat::Entity::insert(nouveau).exec(request.db()).await {
        tracing::error!("Erreur insertion avis plat: {e}");
        request
            .notices
            .error("Une erreur est survenue.".to_string())
            .await;
        return Ok(Redirect::to("/compte?tab=plats").into_response());
    }

    request
        .notices
        .success("Votre avis a bien été envoyé. Il sera publié après modération.".to_string())
        .await;
    Ok(Redirect::to("/compte?tab=plats").into_response())
}
