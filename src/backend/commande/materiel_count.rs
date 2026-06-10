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

#[cfg(test)]
mod tests {
    use super::add_jours_ouvres;
    use chrono::NaiveDateTime;

    fn dt(s: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap()
    }

    #[test]
    fn cinq_jours_depuis_lundi() {
        // Lundi 2026-06-08 + 5 jours ouvrés = Lundi 2026-06-15
        assert_eq!(
            add_jours_ouvres(dt("2026-06-08 10:00:00"), 5),
            dt("2026-06-15 10:00:00")
        );
    }

    #[test]
    fn un_jour_depuis_vendredi_saute_weekend() {
        // Vendredi 2026-06-12 + 1 jour ouvré = Lundi 2026-06-15
        assert_eq!(
            add_jours_ouvres(dt("2026-06-12 10:00:00"), 1),
            dt("2026-06-15 10:00:00")
        );
    }

    #[test]
    fn dix_jours_traverse_deux_weekends() {
        // Lundi 2026-06-08 + 10 jours ouvrés = Lundi 2026-06-22
        assert_eq!(
            add_jours_ouvres(dt("2026-06-08 10:00:00"), 10),
            dt("2026-06-22 10:00:00")
        );
    }

    #[test]
    fn zero_jours_inchange() {
        let start = dt("2026-06-10 08:00:00");
        assert_eq!(add_jours_ouvres(start, 0), start);
    }

    #[test]
    fn depuis_samedi_saute_weekend() {
        // Samedi 2026-06-13 + 1 jour ouvré = Lundi 2026-06-15
        assert_eq!(
            add_jours_ouvres(dt("2026-06-13 10:00:00"), 1),
            dt("2026-06-15 10:00:00")
        );
    }
}
