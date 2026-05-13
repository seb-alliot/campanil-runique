use crate::backend::service::struct_::{CommandeService, LigneService, MenuChoixService};
use crate::entities::{
    boisson, commande, commande_menu_choix, commande_menu_choix_garniture, commande_plat,
    commande_plat_garniture, garniture, menu_resto, plat,
};
use chrono::Timelike;
use runique::prelude::*;
use std::collections::HashMap;

pub(super) fn cours_label(cours: &str) -> &'static str {
    match cours {
        "entree"  => "Entrée",
        "plat"    => "Plat",
        "dessert" => "Dessert",
        _         => "Cours",
    }
}

pub(super) fn garde_acces(request: &Request) -> bool {
    request
        .user
        .as_ref()
        .map(|u| u.is_staff || u.is_superuser)
        .unwrap_or(false)
}

pub(super) fn grouper_par_statut(
    list: Vec<CommandeService>,
) -> (Vec<CommandeService>, Vec<CommandeService>, Vec<CommandeService>, Vec<CommandeService>) {
    let (attente, rest): (Vec<_>, Vec<_>) =
        list.into_iter().partition(|c| c.statut_valeur == "en_attente");
    let (accepte, rest): (Vec<_>, Vec<_>) =
        rest.into_iter().partition(|c| c.statut_valeur == "accepte");
    let (preparation, pret): (Vec<_>, Vec<_>) =
        rest.into_iter().partition(|c| c.statut_valeur == "en_preparation");
    (attente, accepte, preparation, pret)
}

pub(super) async fn charger_commandes_jour(
    db: &sea_orm::DatabaseConnection,
    tz_str: &str,
) -> Vec<CommandeService> {
    use chrono::TimeZone as _;
    let tz: chrono_tz::Tz = tz_str.parse().unwrap_or(chrono_tz::UTC);
    let today = chrono::Utc::now().with_timezone(&tz).date_naive();
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
        TypeCommande eq "carte",
        CreatedAt range (debut, fin),
        Statut not_in ["termine", "annule"],
        asc CreatedAt,
    )
    .all(db)
    .await
    .unwrap_or_default();

    if commandes.is_empty() {
        return vec![];
    }

    let cmd_ids: Vec<i32> = commandes.iter().map(|c| c.id).collect();
    let user_ids: Vec<i32> = commandes.iter().map(|c| c.user_id).collect();

    let lignes_db = search!(commande_plat::Entity => CommandeId in (cmd_ids),)
        .all(db)
        .await
        .unwrap_or_default();

    let cp_ids: Vec<i32> = lignes_db.iter().map(|l| l.id).collect();
    let plat_ids: Vec<i32> = lignes_db.iter().filter_map(|l| l.plat_id).collect();
    let boisson_ids: Vec<i32> = lignes_db.iter().filter_map(|l| l.boisson_id).collect();
    let menu_ids: Vec<i32> = lignes_db.iter().filter_map(|l| l.menu_id).collect();

    let plats_map: HashMap<i32, String> = if !plat_ids.is_empty() {
        search!(plat::Entity => Id in (plat_ids),)
            .all(db).await.unwrap_or_default()
            .into_iter().map(|p| (p.id, p.label.unwrap_or(p.titre))).collect()
    } else { HashMap::new() };

    let boissons_map: HashMap<i32, String> = if !boisson_ids.is_empty() {
        search!(boisson::Entity => Id in (boisson_ids),)
            .all(db).await.unwrap_or_default()
            .into_iter().map(|b| (b.id, b.titre)).collect()
    } else { HashMap::new() };

    let menus_map: HashMap<i32, String> = if !menu_ids.is_empty() {
        search!(menu_resto::Entity => Id in (menu_ids),)
            .all(db).await.unwrap_or_default()
            .into_iter().map(|m| (m.id, m.titre)).collect()
    } else { HashMap::new() };

    let garn_links = search!(commande_plat_garniture::Entity => CommandePlatId in (cp_ids),)
        .all(db).await.unwrap_or_default();

    let garn_ids_plat: Vec<i32> = garn_links.iter().map(|g| g.garniture_id).collect();

    let menu_choix_db = search!(commande_menu_choix::Entity => CommandePlatId in (cp_ids),)
        .all(db).await.unwrap_or_default();

    let choix_ids: Vec<i32> = menu_choix_db.iter().map(|c| c.id).collect();
    let choix_plat_ids: Vec<i32> = menu_choix_db.iter().map(|c| c.plat_id).collect();

    let garn_choix_links = if !choix_ids.is_empty() {
        search!(commande_menu_choix_garniture::Entity => CommandeMenuChoixId in (choix_ids),)
            .all(db).await.unwrap_or_default()
    } else { vec![] };

    let mut all_garn_ids = garn_ids_plat.clone();
    all_garn_ids.extend(garn_choix_links.iter().map(|g| g.garniture_id));
    all_garn_ids.dedup();

    let garnitures_map: HashMap<i32, String> = if !all_garn_ids.is_empty() {
        search!(garniture::Entity => Id in (all_garn_ids),)
            .all(db).await.unwrap_or_default()
            .into_iter().map(|g| (g.id, g.libelle)).collect()
    } else { HashMap::new() };

    let choix_plats_map: HashMap<i32, String> = if !choix_plat_ids.is_empty() {
        search!(plat::Entity => Id in (choix_plat_ids),)
            .all(db).await.unwrap_or_default()
            .into_iter().map(|p| (p.id, p.label.unwrap_or(p.titre))).collect()
    } else { HashMap::new() };

    let mut garnitures_par_cp: HashMap<i32, Vec<String>> = HashMap::new();
    for link in garn_links {
        if let Some(label) = garnitures_map.get(&link.garniture_id) {
            garnitures_par_cp.entry(link.commande_plat_id).or_default().push(label.clone());
        }
    }

    let mut garnitures_par_choix: HashMap<i32, Vec<String>> = HashMap::new();
    for link in garn_choix_links {
        if let Some(label) = garnitures_map.get(&link.garniture_id) {
            garnitures_par_choix.entry(link.commande_menu_choix_id).or_default().push(label.clone());
        }
    }

    let mut menu_choix_par_cp: HashMap<i32, Vec<MenuChoixService>> = HashMap::new();
    for choix in menu_choix_db {
        let titre = choix_plats_map
            .get(&choix.plat_id)
            .cloned()
            .unwrap_or_else(|| format!("Plat #{}", choix.plat_id));
        let garnitures = garnitures_par_choix.remove(&choix.id).unwrap_or_default();
        menu_choix_par_cp.entry(choix.commande_plat_id).or_default().push(MenuChoixService {
            cours_label: cours_label(&choix.cours).to_string(),
            titre,
            cuisson: choix.cuisson,
            garnitures,
            avec_legumes: choix.avec_legumes.unwrap_or(false),
            sans_sel: choix.sans_sel.unwrap_or(false),
            note: choix.note,
        });
    }

    let clients_map: HashMap<i32, String> = search!(runique_users::Entity => Id in (user_ids),)
        .all(db).await.unwrap_or_default()
        .into_iter().map(|u| (u.id, u.username)).collect();

    let mut lignes_par_commande: HashMap<i32, Vec<LigneService>> = HashMap::new();
    for l in &lignes_db {
        let titre = if let Some(bid) = l.boisson_id {
            boissons_map.get(&bid).cloned().unwrap_or_else(|| format!("Boisson #{}", bid))
        } else if let Some(mid) = l.menu_id {
            menus_map.get(&mid).cloned().unwrap_or_else(|| format!("Menu #{}", mid))
        } else if let Some(pid) = l.plat_id {
            plats_map.get(&pid).cloned().unwrap_or_else(|| format!("Plat #{}", pid))
        } else {
            "Article inconnu".to_string()
        };
        lignes_par_commande.entry(l.commande_id).or_default().push(LigneService {
            titre,
            quantite: l.quantite,
            cuisson: l.cuisson.as_ref().map(|c| c.to_string()),
            note: l.note.clone(),
            garnitures: garnitures_par_cp.remove(&l.id).unwrap_or_default(),
            avec_legumes: l.avec_legumes.unwrap_or(false),
            sans_sel: l.sans_sel.unwrap_or(false),
            menu_choix: menu_choix_par_cp.remove(&l.id).unwrap_or_default(),
        });
    }

    let mut result: Vec<CommandeService> = commandes
        .into_iter()
        .map(|c| {
            use crate::backend::commande::statut_info;
            let heure = if c.avec_livraison {
                c.heure_livraison
                    .map(|t| format!("{:02}:{:02}", t.hour(), t.minute()))
                    .unwrap_or_else(|| "—".to_string())
            } else {
                c.heure_retrait
                    .map(|t| format!("{:02}:{:02}", t.hour(), t.minute()))
                    .unwrap_or_else(|| "—".to_string())
            };
            let adresse_livraison = if c.avec_livraison {
                let parts: Vec<String> = [
                    c.adresse_livraison.clone(),
                    c.cp_livraison.clone(),
                    c.ville_livraison.clone(),
                ]
                .into_iter()
                .flatten()
                .filter(|s| !s.is_empty())
                .collect();
                if parts.is_empty() { None } else { Some(parts.join(", ")) }
            } else {
                None
            };
            let (statut_label, statut_valeur) = statut_info(&c.statut);
            let lignes = lignes_par_commande.remove(&c.id).unwrap_or_default();
            let client = clients_map
                .get(&c.user_id)
                .cloned()
                .unwrap_or_else(|| format!("#{}", c.user_id));
            CommandeService {
                numero: c.numero,
                client,
                heure,
                statut: statut_label,
                statut_valeur: statut_valeur.to_string(),
                mode_paiement: c.mode_paiement.to_string(),
                prix_total: format!("{:.2}", c.prix_total),
                avec_livraison: c.avec_livraison,
                adresse_livraison,
                lignes,
            }
        })
        .collect();

    result.sort_by(|a, b| a.heure.cmp(&b.heure));
    result
}
