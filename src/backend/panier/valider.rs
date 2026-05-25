use crate::backend::commande::generer_numero;
use crate::backend::panier::{get_prix_livraison, panier_get, panier_vider};
use crate::backend::stats::{CommandeEventParams, get_commande_event};
use crate::entities::{
    commande,
    commande::{ModePaiement, StatutCommande, TypeRetrait},
    commande_ligne,
    commande_ligne::{CuissonViande, TypeArticle},
    commande_ligne_garniture, commande_menu_choix, commande_statut, horaire,
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

fn cours_label(cours: &str) -> &str {
    match cours {
        "entree" => "Entrée",
        "plat" => "Plat",
        "dessert" => "Dessert",
        _ => "Cours",
    }
}

pub async fn panier_valider(
    session: &Session,
    db: &sea_orm::DatabaseConnection,
    user_id: Pk,
    form: CommandeForm,
    tz_str: &str,
    request: &Request,
) -> Result<String, String> {
    let panier = panier_get(session).await;

    if panier.lignes.is_empty() {
        return Err("Le panier est vide".to_string());
    }
    if let Some(panier_user_id) = panier.user_id
        && panier_user_id != user_id
    {
        panier_vider(session).await;
        return Err("Session invalide, votre panier a été réinitialisé.".to_string());
    }

    let is_livraison = form.type_retrait == "livraison";
    let parse_dt = |s: &str| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M").ok();
    let dt_retrait = form.heure_retrait.as_deref().and_then(parse_dt);

    let dt_cible = match dt_retrait {
        Some(t) => t,
        None => {
            return Err(
                "Veuillez indiquer une date et heure de retrait ou de livraison.".to_string(),
            );
        }
    };

    let tz: chrono_tz::Tz = tz_str.parse().unwrap_or(chrono_tz::UTC);
    let now = chrono::Utc::now().with_timezone(&tz).naive_local();

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
                return Err("L'heure choisie est en dehors des horaires de service.".to_string());
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

    let type_retrait = if is_livraison {
        TypeRetrait::Livraison
    } else {
        TypeRetrait::SurPlace
    };

    let prix_total = panier.total();
    let prix_livraison = if is_livraison {
        get_prix_livraison(db).await
    } else {
        Decimal::ZERO
    };

    let commande_model = {
        let mut result = Err("Impossible de générer un numéro de commande.".to_string());
        for attempt in 0u8..5u8 {
            let numero = generer_numero(db, tz_str, dt_cible, attempt as u64).await;
            let active = commande::ActiveModel {
                numero: Set(numero.clone()),
                user_id: Set(user_id),
                statut: Set(StatutCommande::EnAttente),
                mode_paiement: Set(mode_paiement.clone()),
                prix_total: Set(prix_total + prix_livraison),
                type_retrait: Set(type_retrait.clone()),
                heure_retrait: Set(Some(dt_cible)),
                adresse_livraison: Set(form.adresse_livraison.clone()),
                ville_livraison: Set(form.ville_livraison.clone()),
                cp_livraison: Set(form.cp_livraison.clone()),
                prix_livraison: Set(Some(prix_livraison)),
                ..Default::default()
            };
            match active.insert(db).await {
                Ok(m) => {
                    result = Ok(m);
                    break;
                }
                Err(e) if e.to_string().contains("commandes_numero_key") => continue,
                Err(e) => {
                    result = Err(e.to_string());
                    break;
                }
            }
        }
        result?
    };
    let numero = commande_model.numero.clone();

    for ligne in &panier.lignes {
        let prix_unitaire = Decimal::from_str(&ligne.prix_unitaire).unwrap_or(Decimal::ZERO);
        let cuisson = ligne.cuisson.as_deref().and_then(|c| match c {
            "bleu" => Some(CuissonViande::Bleu),
            "saignant" => Some(CuissonViande::Saignant),
            "a_point" => Some(CuissonViande::APoint),
            "bien_cuit" => Some(CuissonViande::BienCuit),
            _ => None,
        });

        let (
            type_article_enum,
            plat_id,
            entree_id,
            dessert_id,
            menu_id,
            boisson_id,
            supplement_id,
            note,
        ) = if let Some(sid) = ligne.supplement_id {
            (
                TypeArticle::Supplement,
                None,
                None,
                None,
                None,
                None,
                Some(sid),
                None,
            )
        } else if let Some(bid) = ligne.boisson_id {
            (
                TypeArticle::Boisson,
                None,
                None,
                None,
                None,
                Some(bid),
                None,
                None,
            )
        } else if let Some(mid) = ligne.menu_id {
            let note_str = if ligne.menu_choix.is_empty() {
                None
            } else {
                let parts: Vec<String> = ligne
                    .menu_choix
                    .iter()
                    .map(|c| {
                        let mut s = format!("{} : {}", cours_label(&c.cours), c.plat_titre);
                        if let Some(ref cu) = c.cuisson {
                            s.push_str(&format!(" ({})", cu));
                        }
                        s
                    })
                    .collect();
                Some(parts.join(", "))
            };
            (
                TypeArticle::Menu,
                None,
                None,
                None,
                Some(mid),
                None,
                None,
                note_str,
            )
        } else {
            match ligne.type_article.as_str() {
                "entree" => (
                    TypeArticle::Entree,
                    None,
                    Some(ligne.plat_id),
                    None,
                    None,
                    None,
                    None,
                    ligne.note.clone(),
                ),
                "dessert" => (
                    TypeArticle::Dessert,
                    None,
                    None,
                    Some(ligne.plat_id),
                    None,
                    None,
                    None,
                    ligne.note.clone(),
                ),
                _ => (
                    TypeArticle::Plat,
                    Some(ligne.plat_id),
                    None,
                    None,
                    None,
                    None,
                    None,
                    ligne.note.clone(),
                ),
            }
        };

        let ligne_active = commande_ligne::ActiveModel {
            commande_id: Set(commande_model.id),
            type_article: Set(type_article_enum),
            plat_id: Set(plat_id),
            entree_id: Set(entree_id),
            dessert_id: Set(dessert_id),
            menu_id: Set(menu_id),
            boisson_id: Set(boisson_id),
            supplement_id: Set(supplement_id),
            cuisson: Set(cuisson),
            sans_sel: Set(ligne.sans_sel),
            note: Set(note),
            quantite: Set(ligne.quantite),
            prix_unitaire: Set(prix_unitaire),
            ..Default::default()
        };
        let cl = ligne_active.insert(db).await.map_err(|e| e.to_string())?;

        for gid in &ligne.garniture_ids {
            commande_ligne_garniture::ActiveModel {
                commande_ligne_id: Set(cl.id),
                garniture_id: Set(*gid),
                ..Default::default()
            }
            .insert(db)
            .await
            .map_err(|e| e.to_string())?;
        }

        for choix in &ligne.menu_choix {
            let (choix_plat_id, choix_entree_id, choix_dessert_id) = match choix.cours.as_str() {
                "entree" => (None, Some(choix.plat_id), None),
                "dessert" => (None, None, Some(choix.plat_id)),
                _ => (Some(choix.plat_id), None, None),
            };
            commande_menu_choix::ActiveModel {
                commande_ligne_id: Set(cl.id),
                cours: Set(choix.cours.clone()),
                plat_id: Set(choix_plat_id),
                entree_id: Set(choix_entree_id),
                dessert_id: Set(choix_dessert_id),
                ..Default::default()
            }
            .insert(db)
            .await
            .map_err(|e| e.to_string())?;
        }
    }

    commande_statut::ActiveModel {
        commande_id: Set(commande_model.id),
        statut: Set("en_attente".to_string()),
        ..Default::default()
    }
    .insert(db)
    .await
    .ok();

    let request = request.clone();
    let commande_id = commande_model.id;
    let numero_clone = numero.clone();
    let prix_total_clone = commande_model.prix_total;
    let statut = commande_model.statut.db_value().to_string();
    let user_id_clone = user_id;

    panier_vider(session).await;
    tokio::spawn(async move {
        if let Err(e) = get_commande_event(
            &request,
            CommandeEventParams {
                commande_id,
                numero: numero_clone,
                type_commande: "carte".to_string(),
                prix_total: prix_total_clone,
                date: chrono::Utc::now(),
                user_id: user_id_clone,
                statut,
            },
        )
        .await
        {
            eprintln!("Analytics Mongo error (commande event): {}", e);
        }
    });
    Ok(numero)
}
