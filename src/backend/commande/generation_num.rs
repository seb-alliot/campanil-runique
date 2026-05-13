use crate::entities::{commande, horaire, horaire::Jour};
use chrono::{Datelike, TimeZone as _, Weekday};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};

fn weekday_to_jour(wd: Weekday) -> Jour {
    match wd {
        Weekday::Mon => Jour::Lundi,
        Weekday::Tue => Jour::Mardi,
        Weekday::Wed => Jour::Mercredi,
        Weekday::Thu => Jour::Jeudi,
        Weekday::Fri => Jour::Vendredi,
        Weekday::Sat => Jour::Samedi,
        Weekday::Sun => Jour::Dimanche,
    }
}

pub async fn generer_numero(db: &sea_orm::DatabaseConnection, tz_str: &str) -> String {
    let tz: chrono_tz::Tz = tz_str.parse().unwrap_or(chrono_tz::UTC);
    let now_local = chrono::Utc::now().with_timezone(&tz);
    let jour = weekday_to_jour(now_local.weekday());

    let horaire_opt = horaire::Entity::find()
        .filter(horaire::Column::Jour.eq(jour))
        .one(db)
        .await
        .ok()
        .flatten();

    // Détermine le début du service courant
    let service_start_local = if let Some(h) = horaire_opt {
        let time_now = now_local.time();
        // Si ouverture_soir définie et on est après → service soir
        if let Some(ouv_soir) = h.ouverture_soir {
            if time_now >= ouv_soir {
                now_local.date_naive().and_time(ouv_soir)
            } else if let Some(ouv_midi) = h.ouverture_midi {
                now_local.date_naive().and_time(ouv_midi)
            } else {
                now_local.date_naive().and_hms_opt(0, 0, 0).unwrap()
            }
        } else if let Some(ouv_midi) = h.ouverture_midi {
            now_local.date_naive().and_time(ouv_midi)
        } else {
            now_local.date_naive().and_hms_opt(0, 0, 0).unwrap()
        }
    } else {
        now_local.date_naive().and_hms_opt(0, 0, 0).unwrap()
    };

    let service_start_utc = tz
        .from_local_datetime(&service_start_local)
        .single()
        .map(|dt| dt.naive_utc())
        .unwrap_or(service_start_local);

    let count = commande::Entity::find()
        .filter(commande::Column::CreatedAt.gte(service_start_utc))
        .count(db)
        .await
        .unwrap_or(0);

    format!("{}", count + 1)
}
