use crate::backend::commande::generer_numero;
use crate::backend::panier::{get_prix_livraison, panier_get, panier_vider};
use crate::entities::{
    commande_menu_choix, commande_menu_choix_garniture, commande_plat::CuissonViande,
    commande_plat_garniture, horaire,
    {
        commande,
        commande::{ModePaiement, StatutCommande, TypeCommande},
        commande_plat,
    },
};
use crate::formulaire::CommandeForm;
use chrono::{Datelike, NaiveTime, Weekday};
use runique::prelude::*;
use std::str::FromStr;

fn weekday_to_jour(wd: Weekday) -> &'static str {
    match wd {
        Weekday::Mon => "lundi",
        Weekday::Tue => "mardi",
        Weekday::Wed => "mercredi",
        Weekday::Thu => "jeudi",
        Weekday::Fri => "vendredi",
        Weekday::Sat => "samedi",
        Weekday::Sun => "dimanche",
    }
}

fn dans_plage(t: NaiveTime, ouverture: Option<NaiveTime>, fermeture: Option<NaiveTime>) -> bool {
    match (ouverture, fermeture) {
        (Some(o), Some(f)) => t >= o && t <= f,
        _ => false,
    }
}

pub async fn panier_valider(
    session: &Session,
    db: &sea_orm::DatabaseConnection,
    user_id: i32,
    form: CommandeForm,
    tz_str: &str,
) -> Result<String, String> {
    let panier = panier_get(session).await;

    if panier.lignes.is_empty() {
        return Err("Le panier est vide".to_string());
    }
    if let Some(panier_user_id) = panier.user_id {
        if panier_user_id != user_id {
            panier_vider(session).await;
            return Err("Session invalide, votre panier a été réinitialisé.".to_string());
        }
    }

    let parse_dt = |s: &str| {
        chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M").ok()
    };
    let dt_retrait = form.heure_retrait.as_deref().and_then(parse_dt);
    let dt_livraison = form.heure_livraison.as_deref().and_then(parse_dt);

    let tz: chrono_tz::Tz = tz_str.parse().unwrap_or(chrono_tz::UTC);
    let now = chrono::Utc::now().with_timezone(&tz).naive_local();

    let dt_cible = if form.avec_livraison { dt_livraison } else { dt_retrait };
    let dt_cible = match dt_cible {
        Some(t) => t,
        None => return Err("Veuillez indiquer une date et heure de retrait ou de livraison.".to_string()),
    };

    if dt_cible.signed_duration_since(now).num_seconds() < 30 * 60 {
        return Err("L'heure de retrait doit être dans au moins 30 minutes.".to_string());
    }

    let jour_str = weekday_to_jour(dt_cible.weekday());
    let horaire_opt = search!(horaire::Entity => Jour eq jour_str,)
        .first(db)
        .await
        .unwrap_or_default();

    match horaire_opt {
        Some(h) if h.ferme => {
            return Err("Le restaurant est fermé ce jour-là.".to_string());
        }
        Some(h) => {
            let t = dt_cible.time();
            let in_midi = dans_plage(t, h.ouverture_midi, h.fermeture_midi);
            let in_soir = dans_plage(t, h.ouverture_soir, h.fermeture_soir);
            if !in_midi && !in_soir {
                return Err(
                    "L'heure choisie est en dehors des horaires de service.".to_string(),
                );
            }
        }
        None => {
            return Err("Aucun horaire défini pour ce jour.".to_string());
        }
    }

    let mode_paiement = match form.mode_paiement.as_str() {
        "especes" => ModePaiement::Especes,
        "carte_bancaire" => ModePaiement::CarteBancaire,
        "en_ligne" => ModePaiement::EnLigne,
        _ => return Err("Mode de paiement invalide".to_string()),
    };

    let numero = generer_numero(db, tz_str).await;
    let prix_total = panier.total();
    let prix_livraison = if form.avec_livraison {
        get_prix_livraison(db).await
    } else {
        Decimal::ZERO
    };

    let commande_active = commande::ActiveModel {
        numero: Set(numero.clone()),
        user_id: Set(user_id),
        type_commande: Set(TypeCommande::Carte),
        statut: Set(StatutCommande::EnAttente),
        mode_paiement: Set(mode_paiement),
        prix_total: Set(prix_total + prix_livraison),
        avec_materiel: Set(Some(false)),
        heure_retrait: Set(dt_retrait),
        avec_livraison: Set(form.avec_livraison),
        adresse_livraison: Set(form.adresse_livraison),
        ville_livraison: Set(form.ville_livraison),
        cp_livraison: Set(form.cp_livraison),
        heure_livraison: Set(dt_livraison),
        prix_livraison: Set(Some(prix_livraison)),
        ..Default::default()
    };

    let commande_model = commande_active
        .insert(db)
        .await
        .map_err(|e| e.to_string())?;

    for ligne in &panier.lignes {
        let prix_unitaire = Decimal::from_str(&ligne.prix_unitaire).unwrap_or(Decimal::ZERO);
        let cuisson = ligne.cuisson.as_deref().and_then(|c| match c {
            "bleu" => Some(CuissonViande::Bleu),
            "saignant" => Some(CuissonViande::Saignant),
            "a_point" => Some(CuissonViande::APoint),
            "bien_cuit" => Some(CuissonViande::BienCuit),
            _ => None,
        });

        let ligne_active = if let Some(sid) = ligne.supplement_id {
            commande_plat::ActiveModel {
                commande_id: Set(commande_model.id),
                supplement_id: Set(Some(sid)),
                quantite: Set(ligne.quantite),
                prix_unitaire: Set(prix_unitaire),
                cuisson: Set(None),
                ..Default::default()
            }
        } else if let Some(bid) = ligne.boisson_id {
            commande_plat::ActiveModel {
                commande_id: Set(commande_model.id),
                boisson_id: Set(Some(bid)),
                quantite: Set(ligne.quantite),
                prix_unitaire: Set(prix_unitaire),
                cuisson: Set(None),
                ..Default::default()
            }
        } else if let Some(mid) = ligne.menu_id {
            commande_plat::ActiveModel {
                commande_id: Set(commande_model.id),
                menu_id: Set(Some(mid)),
                quantite: Set(ligne.quantite),
                prix_unitaire: Set(prix_unitaire),
                cuisson: Set(None),
                ..Default::default()
            }
        } else {
            commande_plat::ActiveModel {
                commande_id: Set(commande_model.id),
                plat_id: Set(Some(ligne.plat_id)),
                quantite: Set(ligne.quantite),
                prix_unitaire: Set(prix_unitaire),
                cuisson: Set(cuisson),
                avec_legumes: Set(Some(ligne.avec_legumes)),
                sans_sel: Set(Some(ligne.sans_sel)),
                note: Set(ligne.note.clone()),
                ..Default::default()
            }
        };
        let cp = ligne_active.insert(db).await.map_err(|e| e.to_string())?;

        for gid in &ligne.garniture_ids {
            commande_plat_garniture::ActiveModel {
                commande_plat_id: Set(cp.id),
                garniture_id: Set(*gid),
                ..Default::default()
            }
            .insert(db)
            .await
            .map_err(|e| e.to_string())?;
        }

        for choix in &ligne.menu_choix {
            let choix_active = commande_menu_choix::ActiveModel {
                commande_plat_id: Set(cp.id),
                cours: Set(choix.cours.clone()),
                plat_id: Set(choix.plat_id),
                cuisson: Set(choix.cuisson.clone()),
                avec_legumes: Set(Some(choix.avec_legumes)),
                sans_sel: Set(Some(choix.sans_sel)),
                note: Set(choix.note.clone()),
                ..Default::default()
            };
            let cm = choix_active.insert(db).await.map_err(|e| e.to_string())?;

            for gid in &choix.garniture_ids {
                commande_menu_choix_garniture::ActiveModel {
                    commande_menu_choix_id: Set(cm.id),
                    garniture_id: Set(*gid),
                    ..Default::default()
                }
                .insert(db)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
    }

    panier_vider(session).await;
    Ok(numero)
}
