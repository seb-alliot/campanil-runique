use chrono::{Datelike, Duration, NaiveDateTime, Utc, Weekday};
use runique::context;
use runique::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, QueryFilter, Set};

use crate::backend::service::charger_commandes::garde_acces;
use crate::entities::{commande, commande::StatutCommande};

fn add_jours_ouvres(start: NaiveDateTime, n: u32) -> NaiveDateTime {
    let mut dt = start;
    let mut counted = 0u32;
    while counted < n {
        dt += Duration::days(1);
        match dt.weekday() {
            Weekday::Sat | Weekday::Sun => {}
            _ => counted += 1,
        }
    }
    dt
}

/// Envoie les mails de pénalité pour les commandes livrées sans retour
/// de matériel depuis plus de 10 jours ouvrés. Idempotent via `penalite_envoyee`.
/// Retourne le nombre de commandes en attente (avant envoi).
pub async fn process_penalites(db: &DatabaseConnection, tera: &ATera) -> usize {
    let now = Utc::now().naive_utc();

    let commandes_en_attente = commande::Entity::find()
        .filter(commande::Column::Statut.eq(StatutCommande::Livre))
        .filter(commande::Column::PretMateriel.eq(false))
        .all(db)
        .await
        .unwrap_or_default();

    let count = commandes_en_attente.len();

    for cmd in &commandes_en_attente {
        if cmd.penalite_envoyee {
            continue;
        }
        let echeance = add_jours_ouvres(cmd.updated_at.unwrap_or(now), 10);
        if now < echeance {
            continue;
        }

        let Ok(Some(user)) = runique_users::Entity::find_by_id(cmd.user_id).one(db).await else {
            continue;
        };

        let ctx = context! {
            "username" => &user.username,
            "numero"   => &cmd.numero,
            "montant"  => "600,00 €",
        };

        if let Ok(msg) = Email::new()
            .to(user.email.clone())
            .subject("Supplément non-retour de matériel — U Campanile")
            .template(tera, "emails/penalite_materiel.html", ctx.into())
        {
            msg.send().await.ok();
        }

        let active = commande::ActiveModel {
            id: Set(cmd.id),
            penalite_envoyee: Set(true),
            ..Default::default()
        };
        active.update(db).await.ok();
    }

    count
}

pub async fn handle_materiel_count(request: &Request) -> AppResult<Response> {
    if !garde_acces(request) {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "403"})),
        )
            .into_response());
    }

    let count = process_penalites(request.db(), &request.engine.tera).await;
    Ok(Json(serde_json::json!({ "count": count })).into_response())
}
