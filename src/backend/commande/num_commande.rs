use crate::entities::{commande, horaire, horaire::Jour};
use chrono::{Datelike, NaiveDateTime, Weekday};
use runique::prelude::*;

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

/// Génère un numéro de commande basé sur le service (M/S) de l'heure de retrait/livraison,
/// pas sur l'heure de passation — évite les conflits entre services.
pub async fn generer_numero(
    db: &sea_orm::DatabaseConnection,
    tz_str: &str,
    dt_cible: NaiveDateTime,
    offset: u64,
) -> String {
    let _tz: chrono_tz::Tz = tz_str.parse().unwrap_or(chrono_tz::UTC);
    let date_cible = dt_cible.date();
    let time_cible = dt_cible.time();
    let jour = weekday_to_jour(date_cible.weekday());

    let horaire_opt = search!(horaire::Entity => Jour eq jour,)
        .first(db)
        .await
        .ok()
        .flatten();

    // Fenêtre de service déterminée par l'heure de retrait/livraison (pas l'heure courante)
    let (service_start, service_end, service_code) = if let Some(ref h) = horaire_opt {
        if let (Some(ouv_soir), Some(fer_soir)) = (h.ouverture_soir, h.fermeture_soir) {
            if time_cible >= ouv_soir {
                (
                    date_cible.and_time(ouv_soir),
                    date_cible.and_time(fer_soir),
                    "S",
                )
            } else if let (Some(ouv_midi), Some(fer_midi)) = (h.ouverture_midi, h.fermeture_midi) {
                (
                    date_cible.and_time(ouv_midi),
                    date_cible.and_time(fer_midi),
                    "M",
                )
            } else {
                (
                    date_cible.and_hms_opt(0, 0, 0).unwrap(),
                    date_cible.and_hms_opt(23, 59, 59).unwrap(),
                    "",
                )
            }
        } else if let (Some(ouv_midi), Some(fer_midi)) = (h.ouverture_midi, h.fermeture_midi) {
            (
                date_cible.and_time(ouv_midi),
                date_cible.and_time(fer_midi),
                "M",
            )
        } else {
            (
                date_cible.and_hms_opt(0, 0, 0).unwrap(),
                date_cible.and_hms_opt(23, 59, 59).unwrap(),
                "",
            )
        }
    } else {
        (
            date_cible.and_hms_opt(0, 0, 0).unwrap(),
            date_cible.and_hms_opt(23, 59, 59).unwrap(),
            "",
        )
    };

    // Compte les commandes dont le retrait tombe dans la même fenêtre de service
    let count = search!(commande::Entity => HeureRetrait range (service_start, service_end),)
        .count(db)
        .await
        .unwrap_or(0);

    let date_prefix = date_cible.format("%d%m%y").to_string();
    format!("{}{}-{}", date_prefix, service_code, count + 1 + offset)
}
