use crate::backend::service::struct_::{CommandeService, LigneService};
use crate::entities::{
    boisson, commande, commande::TypeRetrait, commande_ligne, commande_ligne_garniture, dessert,
    entree, garniture, horaire, menu, plat, supplement,
};
use chrono::{Datelike, NaiveTime, Timelike, Weekday};
use runique::prelude::*;
use std::collections::HashMap;

fn weekday_str(wd: Weekday) -> &'static str {
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

fn service_pour_heure(
    t: NaiveTime,
    ouv_midi: Option<NaiveTime>,
    fer_midi: Option<NaiveTime>,
    ouv_soir: Option<NaiveTime>,
    fer_soir: Option<NaiveTime>,
) -> &'static str {
    if let (Some(os), Some(fs)) = (ouv_soir, fer_soir)
        && t >= os
        && t <= fs
    {
        return "soir";
    }
    if let (Some(om), Some(fm)) = (ouv_midi, fer_midi)
        && t >= om
        && t <= fm
    {
        return "midi";
    }
    if let Some(os) = ouv_soir
        && t >= os
    {
        return "soir";
    }
    "midi"
}

pub(super) fn grouper_par_service(
    list: Vec<CommandeService>,
) -> (Vec<CommandeService>, Vec<CommandeService>) {
    list.into_iter().partition(|c| c.service == "midi")
}

pub fn garde_acces(request: &Request) -> bool {
    request
        .user
        .as_ref()
        .map(|u| u.is_staff || u.is_superuser)
        .unwrap_or(false)
}

pub(super) fn grouper_par_statut(
    list: Vec<CommandeService>,
) -> (
    Vec<CommandeService>,
    Vec<CommandeService>,
    Vec<CommandeService>,
    Vec<CommandeService>,
) {
    let (attente, rest): (Vec<_>, Vec<_>) = list
        .into_iter()
        .partition(|c| c.statut_valeur == "en_attente");
    let (accepte, rest): (Vec<_>, Vec<_>) =
        rest.into_iter().partition(|c| c.statut_valeur == "accepte");
    let (preparation, rest): (Vec<_>, Vec<_>) = rest
        .into_iter()
        .partition(|c| c.statut_valeur == "en_preparation");
    let pret: Vec<_> = rest
        .into_iter()
        .filter(|c| c.statut_valeur == "pret" || c.statut_valeur == "en_cours_livraison")
        .collect();
    (attente, accepte, preparation, pret)
}

pub(super) async fn charger_commandes_jour(
    db: &sea_orm::DatabaseConnection,
    tz_str: &str,
) -> Vec<CommandeService> {
    use chrono::TimeZone as _;
    let tz: chrono_tz::Tz = tz_str.parse().unwrap_or(chrono_tz::UTC);
    let today = chrono::Utc::now().with_timezone(&tz).date_naive();

    let jour_str = weekday_str(today.weekday());
    let horaire_today: Option<horaire::Model> = search!(horaire::Entity => Jour eq jour_str,)
        .first(db)
        .await
        .ok()
        .flatten();
    let debut_local = today.and_hms_opt(0, 0, 0).unwrap();
    let fin_local = today.and_hms_opt(23, 59, 59).unwrap();
    let debut = tz
        .from_local_datetime(&debut_local)
        .single()
        .map(|dt| dt.naive_utc())
        .unwrap_or(debut_local);
    let fin = tz
        .from_local_datetime(&fin_local)
        .single()
        .map(|dt| dt.naive_utc())
        .unwrap_or(fin_local);

    let commandes = search!(commande::Entity =>
        CreatedAt range (debut, fin),
        Statut not_in ["termine", "annule", "livre"],
        asc CreatedAt,
    )
    .all(db)
    .await
    .unwrap_or_default();

    if commandes.is_empty() {
        return vec![];
    }

    let cmd_ids: Vec<Pk> = commandes.iter().map(|c| c.id).collect();
    let user_ids: Vec<i32> = commandes.iter().map(|c| c.user_id).collect();

    let lignes_db = search!(commande_ligne::Entity => CommandeId in (cmd_ids),)
        .all(db)
        .await
        .unwrap_or_default();

    let cl_ids: Vec<Pk> = lignes_db.iter().map(|l| l.id).collect();
    let plat_ids: Vec<i32> = lignes_db.iter().filter_map(|l| l.plat_id).collect();
    let entree_ids_svc: Vec<i32> = lignes_db.iter().filter_map(|l| l.entree_id).collect();
    let dessert_ids_svc: Vec<i32> = lignes_db.iter().filter_map(|l| l.dessert_id).collect();
    let boisson_ids: Vec<i32> = lignes_db.iter().filter_map(|l| l.boisson_id).collect();
    let menu_ids: Vec<i32> = lignes_db.iter().filter_map(|l| l.menu_id).collect();
    let supplement_ids: Vec<i32> = lignes_db.iter().filter_map(|l| l.supplement_id).collect();

    let plats_map: HashMap<Pk, String> = if !plat_ids.is_empty() {
        search!(plat::Entity => Id in (plat_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|p| (p.id, p.label.unwrap_or(p.titre)))
            .collect()
    } else {
        HashMap::new()
    };

    let entrees_map_svc: HashMap<Pk, String> = if !entree_ids_svc.is_empty() {
        search!(entree::Entity => Id in (entree_ids_svc),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|e| (e.id, e.label.unwrap_or(e.titre)))
            .collect()
    } else {
        HashMap::new()
    };

    let desserts_map_svc: HashMap<Pk, String> = if !dessert_ids_svc.is_empty() {
        search!(dessert::Entity => Id in (dessert_ids_svc),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|d| (d.id, d.label.unwrap_or(d.titre)))
            .collect()
    } else {
        HashMap::new()
    };

    let boissons_map: HashMap<Pk, String> = if !boisson_ids.is_empty() {
        search!(boisson::Entity => Id in (boisson_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|b| (b.id, b.titre))
            .collect()
    } else {
        HashMap::new()
    };

    let menus_map: HashMap<Pk, String> = if !menu_ids.is_empty() {
        search!(menu::Entity => Id in (menu_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|m| (m.id, m.nom))
            .collect()
    } else {
        HashMap::new()
    };

    let supplements_map: HashMap<Pk, String> = if !supplement_ids.is_empty() {
        let sups = search!(supplement::Entity => Id in (supplement_ids),)
            .all(db)
            .await
            .unwrap_or_default();
        let garn_ids: Vec<i32> = sups.iter().filter_map(|s| s.garniture_id).collect();
        let garnitures_sup: HashMap<Pk, String> = if !garn_ids.is_empty() {
            search!(garniture::Entity => Id in (garn_ids),)
                .all(db)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|g| (g.id, g.libelle))
                .collect()
        } else {
            HashMap::new()
        };
        sups.into_iter()
            .map(|s| {
                let label = s
                    .titre
                    .clone()
                    .or_else(|| {
                        s.garniture_id
                            .and_then(|gid| garnitures_sup.get(&(gid as Pk)).cloned())
                    })
                    .unwrap_or_else(|| "Supplément".to_string());
                (s.id, label)
            })
            .collect()
    } else {
        HashMap::new()
    };

    let garn_links = search!(commande_ligne_garniture::Entity => CommandeLigneId in (cl_ids),)
        .all(db)
        .await
        .unwrap_or_default();

    let garn_link_ids: Vec<i32> = garn_links.iter().map(|g| g.garniture_id).collect();
    let garnitures_map: HashMap<Pk, String> = if !garn_link_ids.is_empty() {
        search!(garniture::Entity => Id in (garn_link_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|g| (g.id, g.libelle))
            .collect()
    } else {
        HashMap::new()
    };

    let mut garnitures_par_cl: HashMap<Pk, Vec<String>> = HashMap::new();
    for link in garn_links {
        if let Some(label) = garnitures_map.get(&(link.garniture_id as Pk)) {
            garnitures_par_cl
                .entry(link.commande_ligne_id as Pk)
                .or_default()
                .push(label.clone());
        }
    }

    let clients_map: HashMap<Pk, String> = search!(runique_users::Entity => Id in (user_ids),)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|u| (u.id, u.username))
        .collect();

    let mut lignes_par_commande: HashMap<Pk, Vec<LigneService>> = HashMap::new();
    for l in &lignes_db {
        let titre = if let Some(sid) = l.supplement_id {
            supplements_map
                .get(&(sid as Pk))
                .cloned()
                .unwrap_or_else(|| format!("Supplément #{}", sid))
        } else if let Some(bid) = l.boisson_id {
            boissons_map
                .get(&(bid as Pk))
                .cloned()
                .unwrap_or_else(|| format!("Boisson #{}", bid))
        } else if let Some(mid) = l.menu_id {
            menus_map
                .get(&(mid as Pk))
                .cloned()
                .unwrap_or_else(|| format!("Menu #{}", mid))
        } else if let Some(eid) = l.entree_id {
            entrees_map_svc
                .get(&(eid as Pk))
                .cloned()
                .unwrap_or_else(|| format!("Entrée #{}", eid))
        } else if let Some(did) = l.dessert_id {
            desserts_map_svc
                .get(&(did as Pk))
                .cloned()
                .unwrap_or_else(|| format!("Dessert #{}", did))
        } else if let Some(pid) = l.plat_id {
            plats_map
                .get(&(pid as Pk))
                .cloned()
                .unwrap_or_else(|| format!("Plat #{}", pid))
        } else {
            "Article inconnu".to_string()
        };
        lignes_par_commande
            .entry(l.commande_id as Pk)
            .or_default()
            .push(LigneService {
                titre,
                quantite: l.quantite,
                cuisson: l.cuisson.as_ref().map(|c| c.to_string()),
                note: l.note.clone(),
                garnitures: garnitures_par_cl.remove(&l.id).unwrap_or_default(),
                sans_sel: l.sans_sel,
            });
    }

    let mut result: Vec<CommandeService> = commandes
        .into_iter()
        .map(|c| {
            use crate::backend::commande::statut_info;
            let is_livraison = c.type_retrait == TypeRetrait::Livraison;
            let heure = c
                .heure_retrait
                .map(|t| format!("{:02}:{:02}", t.hour(), t.minute()))
                .unwrap_or_else(|| "—".to_string());
            let adresse_livraison = if is_livraison {
                let parts: Vec<String> = [
                    c.adresse_livraison.clone(),
                    c.cp_livraison.clone(),
                    c.ville_livraison.clone(),
                ]
                .into_iter()
                .flatten()
                .filter(|s| !s.is_empty())
                .collect();
                if parts.is_empty() {
                    None
                } else {
                    Some(parts.join(", "))
                }
            } else {
                None
            };
            let (statut_label, statut_valeur) = statut_info(&c.statut);
            let lignes = lignes_par_commande.remove(&c.id).unwrap_or_default();
            let client = clients_map
                .get(&(c.user_id as Pk))
                .cloned()
                .unwrap_or_else(|| format!("#{}", c.user_id));
            let time_cible = c.heure_retrait.map(|d| d.time());
            let service = time_cible
                .map(|t| {
                    service_pour_heure(
                        t,
                        horaire_today.as_ref().and_then(|h| h.ouverture_midi),
                        horaire_today.as_ref().and_then(|h| h.fermeture_midi),
                        horaire_today.as_ref().and_then(|h| h.ouverture_soir),
                        horaire_today.as_ref().and_then(|h| h.fermeture_soir),
                    )
                })
                .unwrap_or("midi")
                .to_string();
            CommandeService {
                numero: c.numero,
                client,
                heure,
                statut: statut_label,
                statut_valeur: statut_valeur.to_string(),
                mode_paiement: c.mode_paiement.to_string(),
                prix_total: format!("{:.2}", c.prix_total),
                type_retrait: c.type_retrait.db_value().to_string(),
                adresse_livraison,
                lignes,
                service,
            }
        })
        .collect();

    result.sort_by(|a, b| a.heure.cmp(&b.heure));
    result
}
