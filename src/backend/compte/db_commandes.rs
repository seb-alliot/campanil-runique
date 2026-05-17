use crate::backend::compte::{CommandeResume, LigneResume, StatutHistorique};
use crate::entities::commande;
use crate::entities::commande::{StatutCommande, TypeRetrait};
use crate::entities::{
    avis, boisson, commande_ligne, commande_statut, garniture, menu_resto, plat, supplement,
};
use runique::prelude::*;
use sea_orm::{ColumnTrait, Condition};
use std::collections::HashMap;

const PAGE_SIZE: u64 = 10;

fn statut_label(s: &str) -> &'static str {
    match s {
        "en_attente" => "En attente",
        "accepte" => "Accepté",
        "en_preparation" => "En préparation",
        "pret" => "Prêt",
        "en_cours_livraison" => "En cours de livraison",
        "livre" => "Livré",
        "termine" => "Terminé",
        "annule" => "Annulé",
        _ => "Inconnu",
    }
}

fn statut_condition(filtre: &str) -> Option<Condition> {
    match filtre {
        "en_cours" => Some(
            Condition::any()
                .add(commande::Column::Statut.eq(StatutCommande::EnAttente))
                .add(commande::Column::Statut.eq(StatutCommande::Accepte))
                .add(commande::Column::Statut.eq(StatutCommande::EnPreparation))
                .add(commande::Column::Statut.eq(StatutCommande::Pret))
                .add(commande::Column::Statut.eq(StatutCommande::EnCoursLivraison)),
        ),
        "termine" => Some(
            Condition::any()
                .add(commande::Column::Statut.eq(StatutCommande::Livre))
                .add(commande::Column::Statut.eq(StatutCommande::Termine)),
        ),
        "annule" => Some(Condition::any().add(commande::Column::Statut.eq(StatutCommande::Annule))),
        _ => None,
    }
}

pub async fn get_commandes_user(
    db: &sea_orm::DatabaseConnection,
    user_id: Pk,
    page: u64,
    filtre: &str,
    service: &str,
) -> (Vec<CommandeResume>, u64, u64) {
    let page = page.max(1);

    let mut base_count = search!(commande::Entity => UserId eq user_id,);
    if let Some(cond) = statut_condition(filtre) {
        base_count = base_count.filter(cond);
    }
    if service == "midi" {
        base_count = base_count.filter(commande::Column::Numero.like("%M-%"));
    } else if service == "soir" {
        base_count = base_count.filter(commande::Column::Numero.like("%S-%"));
    }
    let total: u64 = base_count.count(db).await.unwrap_or(0);
    let total_pages = total.div_ceil(PAGE_SIZE);

    let mut base_query = search!(commande::Entity => UserId eq user_id, desc Id,);
    if let Some(cond) = statut_condition(filtre) {
        base_query = base_query.filter(cond);
    }
    if service == "midi" {
        base_query = base_query.filter(commande::Column::Numero.like("%M-%"));
    } else if service == "soir" {
        base_query = base_query.filter(commande::Column::Numero.like("%S-%"));
    }
    let commandes = base_query
        .offset((page - 1) * PAGE_SIZE)
        .limit(PAGE_SIZE)
        .all(db)
        .await
        .unwrap_or_default();

    if commandes.is_empty() {
        return (vec![], page, total_pages);
    }

    let ids: Vec<Pk> = commandes.iter().map(|c| c.id).collect();

    let lignes_raw = search!(commande_ligne::Entity => CommandeId in (ids),)
        .all(db)
        .await
        .unwrap_or_default();

    let plat_ids: Vec<i32> = lignes_raw.iter().filter_map(|l| l.plat_id).collect();
    let menu_ids: Vec<i32> = lignes_raw.iter().filter_map(|l| l.menu_resto_id).collect();
    let boisson_ids: Vec<i32> = lignes_raw.iter().filter_map(|l| l.boisson_id).collect();
    let supplement_ids: Vec<i32> = lignes_raw.iter().filter_map(|l| l.supplement_id).collect();

    let plats_map: HashMap<Pk, String> = if !plat_ids.is_empty() {
        search!(plat::Entity => Id in (plat_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|p| (p.id, p.titre))
            .collect()
    } else {
        HashMap::new()
    };

    let menus_map: HashMap<Pk, String> = if !menu_ids.is_empty() {
        search!(menu_resto::Entity => Id in (menu_ids),)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|m| (m.id, m.nom))
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

    let statuts_raw = search!(commande_statut::Entity => CommandeId in (ids),)
        .all(db)
        .await
        .unwrap_or_default();
    let mut statuts_map: HashMap<Pk, Vec<StatutHistorique>> = HashMap::new();
    for s in statuts_raw {
        let heure = s
            .created_at
            .map(|dt| dt.format("%d/%m %H:%M").to_string())
            .unwrap_or_default();
        statuts_map
            .entry(s.commande_id as Pk)
            .or_default()
            .push(StatutHistorique {
                statut: statut_label(&s.statut).to_string(),
                heure,
            });
    }

    let avis_list = search!(avis::Entity => CommandeId in (ids),)
        .all(db)
        .await
        .unwrap_or_default();
    let avis_map: HashMap<Pk, avis::Model> = avis_list
        .into_iter()
        .map(|a| (a.commande_id as Pk, a))
        .collect();

    let mut lignes_map: HashMap<Pk, Vec<LigneResume>> = HashMap::new();
    for cl in lignes_raw {
        let titre = if let Some(sid) = cl.supplement_id {
            supplements_map
                .get(&(sid as Pk))
                .cloned()
                .unwrap_or_else(|| format!("Supplément #{}", sid))
        } else if let Some(bid) = cl.boisson_id {
            boissons_map
                .get(&(bid as Pk))
                .cloned()
                .unwrap_or_else(|| format!("Boisson #{}", bid))
        } else if let Some(mid) = cl.menu_resto_id {
            menus_map
                .get(&(mid as Pk))
                .cloned()
                .unwrap_or_else(|| format!("Menu #{}", mid))
        } else if let Some(pid) = cl.plat_id {
            plats_map
                .get(&(pid as Pk))
                .cloned()
                .unwrap_or_else(|| format!("Plat #{}", pid))
        } else {
            "Article inconnu".to_string()
        };
        let cuisson = cl.cuisson.map(|c| c.to_string());
        lignes_map
            .entry(cl.commande_id as Pk)
            .or_default()
            .push(LigneResume {
                titre,
                quantite: cl.quantite,
                prix_unitaire: format!("{:.2}", cl.prix_unitaire),
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
            StatutCommande::Annule => "annule",
        };
        let is_livraison = c.type_retrait == TypeRetrait::Livraison;
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
        let date_annulation = c
            .date_annulation
            .map(|dt| dt.format("%d/%m/%Y %H:%M").to_string());

        let can_cancel = matches!(
            c.statut,
            StatutCommande::EnAttente | StatutCommande::Accepte
        ) && c.heure_retrait.is_none_or(|t| {
            let now = chrono::Utc::now().naive_utc();
            t.signed_duration_since(now).num_seconds() > 15 * 60
        });

        let eligible_for_review =
            matches!(c.statut, StatutCommande::Termine | StatutCommande::Livre);
        let existing_avis = avis_map.get(&c.id);
        let can_review = eligible_for_review && existing_avis.is_none();
        let avis_statut = existing_avis.map(|a| a.statut.to_string());
        let avis_note = existing_avis.map(|a| a.note);
        let avis_commentaire = existing_avis.map(|a| a.commentaire.clone());
        let commande_id = c.id;

        let lignes = lignes_map.remove(&c.id).unwrap_or_default();
        let statuts = statuts_map.remove(&c.id).unwrap_or_default();

        result.push(CommandeResume {
            numero: c.numero,
            statut_label,
            statut_css: statut_css.to_string(),
            mode_paiement,
            prix_total: format!("{:.2}", c.prix_total),
            heure_retrait,
            type_retrait: c.type_retrait.db_value().to_string(),
            adresse_livraison: if is_livraison {
                c.adresse_livraison
            } else {
                None
            },
            ville_livraison: if is_livraison {
                c.ville_livraison
            } else {
                None
            },
            cp_livraison: if is_livraison { c.cp_livraison } else { None },
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
            statuts,
        });
    }
    (result, page_courante, total_pages)
}
