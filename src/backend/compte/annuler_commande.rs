use crate::backend::utils::inject_auth;
use crate::entities::commande;
use crate::entities::commande::StatutCommande;
use runique::prelude::*;

pub async fn handle_commande_annuler(request: &mut Request) -> AppResult<Response> {
    if !request.is_post() {
        return Ok(Redirect::to("/compte").into_response());
    }
    inject_auth(request).await;
    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to("/connexion?next=/compte").into_response());
    };

    let numero = request.get_path("numero").unwrap_or("").to_string();

    // IDOR : filtre user_id + numero simultanément — message générique si non trouvé
    let cmd_opt = commande::Entity::find()
        .filter(commande::Column::Numero.eq(&numero))
        .filter(commande::Column::UserId.eq(user.id))
        .one(request.db())
        .await
        .ok()
        .flatten();

    let Some(cmd) = cmd_opt else {
        request
            .notices
            .error("Commande introuvable.".to_string())
            .await;
        return Ok(Redirect::to("/compte").into_response());
    };

    if !matches!(
        cmd.statut,
        StatutCommande::EnAttente | StatutCommande::Accepte
    ) {
        request
            .notices
            .error("Cette commande est déjà en préparation ou terminée.".to_string())
            .await;
        return Ok(Redirect::to("/compte").into_response());
    }

    let tz: chrono_tz::Tz = request.engine.config.timezone.parse().unwrap_or(chrono_tz::UTC);
    let dt_cible = if cmd.avec_livraison { cmd.heure_livraison } else { cmd.heure_retrait };
    if let Some(dt) = dt_cible {
        let now = chrono::Utc::now().with_timezone(&tz).naive_local();
        if dt.signed_duration_since(now).num_seconds() <= 15 * 60 {
            request
                .notices
                .error("Annulation impossible : moins de 15 minutes avant le retrait.".to_string())
                .await;
            return Ok(Redirect::to("/compte").into_response());
        }
    }

    let active = commande::ActiveModel {
        id: Set(cmd.id),
        statut: Set(StatutCommande::Annule),
        date_annulation: Set(Some(chrono::Utc::now().with_timezone(&tz).naive_local())),
        ..Default::default()
    };
    if active.update(request.db()).await.is_err() {
        request
            .notices
            .error("Erreur lors de l'annulation.".to_string())
            .await;
    } else {
        request
            .notices
            .success("Commande annulée.".to_string())
            .await;
    }
    Ok(Redirect::to("/compte").into_response())
}
