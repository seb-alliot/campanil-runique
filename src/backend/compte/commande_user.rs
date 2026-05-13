use crate::backend::compte::{CommandeResume, LigneResume};
use crate::entities::commande;
use crate::entities::commande::StatutCommande;
use crate::entities::{avis, boisson, commande_plat, menu_resto, plat, supplement};
use runique::prelude::*;
use std::collections::HashMap;

const PAGE_SIZE: u64 = 10;

pub async fn get_commandes_user(
    db: &sea_orm::DatabaseConnection,
    user_id: i32,
    page: u64,
) -> (Vec<CommandeResume>, u64, u64) {
    let page = page.max(1);
    let total: u64 = search!(commande::Entity => UserId eq user_id,)
        .count(db)
        .await
        .unwrap_or(0);
    let total_pages = (total + PAGE_SIZE - 1) / PAGE_SIZE;
    let commandes = search!(commande::Entity => UserId eq user_id, desc Id,)
        .offset((page - 1) * PAGE_SIZE)
        .limit(PAGE_SIZE)
        .all(db)
        .await
        .unwrap_or_default();

    if commandes.is_empty() {
        return (vec![], page, total_pages);
    }

    let ids: Vec<i32> = commandes.iter().map(|c| c.id).collect();

    let lignes_raw = search!(commande_plat::Entity => CommandeId in (ids),)
        .all(db)
        .await
        .unwrap_or_default();

    let plat_ids: Vec<i32> = lignes_raw.iter().filter_map(|l| l.plat_id).collect();
    let menu_ids: Vec<i32> = lignes_raw.iter().filter_map(|l| l.menu_id).collect();
    let boisson_ids: Vec<i32> = lignes_raw.iter().filter_map(|l| l.boisson_id).collect();
    let supplement_ids: Vec<i32> = lignes_raw.iter().filter_map(|l| l.supplement_id).collect();

    let plats_map: HashMap<i32, String> = if !plat_ids.is_empty() {
        search!(plat::Entity => Id in (plat_ids),)
            .all(db).await.unwrap_or_default()
            .into_iter().map(|p| (p.id, p.titre)).collect()
    } else { HashMap::new() };

    let menus_map: HashMap<i32, String> = if !menu_ids.is_empty() {
        search!(menu_resto::Entity => Id in (menu_ids),)
            .all(db).await.unwrap_or_default()
            .into_iter().map(|m| (m.id, m.titre)).collect()
    } else { HashMap::new() };

    let boissons_map: HashMap<i32, String> = if !boisson_ids.is_empty() {
        search!(boisson::Entity => Id in (boisson_ids),)
            .all(db).await.unwrap_or_default()
            .into_iter().map(|b| (b.id, b.titre)).collect()
    } else { HashMap::new() };

    let supplements_map: HashMap<i32, String> = if !supplement_ids.is_empty() {
        search!(supplement::Entity => Id in (supplement_ids),)
            .all(db).await.unwrap_or_default()
            .into_iter().map(|s| (s.id, s.libelle)).collect()
    } else { HashMap::new() };

    let avis_list = search!(avis::Entity => CommandeId in (ids),)
        .all(db)
        .await
        .unwrap_or_default();

    let avis_map: HashMap<i32, avis::Model> = avis_list
        .into_iter()
        .map(|a| (a.commande_id, a))
        .collect();

    let mut lignes_map: HashMap<i32, Vec<LigneResume>> = HashMap::new();

    for cp in lignes_raw {
        let titre = if let Some(sid) = cp.supplement_id {
            supplements_map.get(&sid).cloned().unwrap_or_else(|| format!("Supplément #{}", sid))
        } else if let Some(bid) = cp.boisson_id {
            boissons_map.get(&bid).cloned().unwrap_or_else(|| format!("Boisson #{}", bid))
        } else if let Some(mid) = cp.menu_id {
            menus_map.get(&mid).cloned().unwrap_or_else(|| format!("Menu #{}", mid))
        } else if let Some(pid) = cp.plat_id {
            plats_map.get(&pid).cloned().unwrap_or_else(|| format!("Plat #{}", pid))
        } else {
            "Article inconnu".to_string()
        };
        let cuisson = cp.cuisson.map(|c| c.to_string());
        lignes_map
            .entry(cp.commande_id)
            .or_default()
            .push(LigneResume {
                titre,
                quantite: cp.quantite,
                prix_unitaire: format!("{:.2}", cp.prix_unitaire),
                cuisson,
            });
    }

    let mut result = Vec::with_capacity(commandes.len());
    let page_courante = page;
    for c in commandes {
        let statut_label = c.statut.to_string();
        let statut_css = match c.statut {
            StatutCommande::EnAttente
            | StatutCommande::Accepte
            | StatutCommande::EnPreparation
            | StatutCommande::Pret => "en-cours",
            StatutCommande::EnCoursLivraison => "livraison",
            StatutCommande::Livre | StatutCommande::Termine => "termine",
            StatutCommande::EnAttenteRetourMateriel => "retour",
            StatutCommande::Annule => "annule",
        };
        let type_commande = c.type_commande.to_string();
        let mode_paiement = c.mode_paiement.to_string();
        let date = c
            .created_at
            .map(|dt| dt.format("%d/%m/%Y %H:%M").to_string())
            .unwrap_or_default();
        let date_iso = c
            .created_at
            .map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            .unwrap_or_default();
        let heure_retrait = c.heure_retrait.map(|t| t.format("%H:%M").to_string());
        let heure_livraison = c.heure_livraison.map(|t| t.format("%H:%M").to_string());
        let date_annulation = c.date_annulation.map(|dt| dt.format("%d/%m/%Y %H:%M").to_string());

        let can_cancel = matches!(
            c.statut,
            StatutCommande::EnAttente | StatutCommande::Accepte
        ) && {
            let heure = if c.avec_livraison {
                c.heure_livraison
            } else {
                c.heure_retrait
            };
            heure.map_or(true, |t| {
                let now = chrono::Utc::now().naive_utc();
                t.signed_duration_since(now).num_seconds() > 15 * 60
            })
        };

        let eligible_for_review = matches!(
            c.statut,
            StatutCommande::Termine | StatutCommande::Livre
        );
        let existing_avis = avis_map.get(&c.id);
        let can_review = eligible_for_review && existing_avis.is_none();
        let avis_statut = existing_avis.map(|a| a.statut.to_string());
        let avis_note = existing_avis.map(|a| a.note);
        let avis_commentaire = existing_avis.map(|a| a.commentaire.clone());
        let commande_id = c.id;

        let lignes = lignes_map.remove(&c.id).unwrap_or_default();

        result.push(CommandeResume {
            numero: c.numero,
            statut_label,
            statut_css: statut_css.to_string(),
            type_commande,
            mode_paiement,
            prix_total: format!("{:.2}", c.prix_total),
            heure_retrait,
            avec_livraison: c.avec_livraison,
            adresse_livraison: c.adresse_livraison,
            ville_livraison: c.ville_livraison,
            cp_livraison: c.cp_livraison,
            heure_livraison,
            date,
            date_iso,
            date_annulation,
            can_cancel,
            commande_id,
            can_review,
            avis_statut,
            avis_note,
            avis_commentaire,
            lignes,
        });
    }
    (result, page_courante, total_pages)
}
