use runique::context;
use runique::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::{collections::HashMap, sync::Arc};

use crate::backend::commande::{
    CommandeDetail, LigneDetail, all_statuts, parse_statut, statut_info,
};
use crate::entities::{boisson, commande, commande_plat, menu_resto, plat};

pub async fn handle_admin_commande_detail(
    request: &mut Request,
    admin: &AdminState,
    proto: Option<Arc<PrototypeAdminState>>,
) -> AppResult<Response> {
    let path_param = request.get_path("numero").unwrap_or("").to_string();
    let db = request.db();

    let find = if let Ok(id) = path_param.parse::<i32>() {
        commande::Entity::find_by_id(id)
    } else {
        commande::Entity::find().filter(commande::Column::Numero.eq(&path_param))
    };
    let Some(cmd) = find.one(db).await.ok().flatten() else {
        request
            .notices
            .error("Commande introuvable.".to_string())
            .await;
        return Ok(
            Redirect::to(&format!("{}/commandes/list", admin.config.prefix)).into_response(),
        );
    };
    let numero = cmd.numero.clone();

    // Mise à jour statut via POST
    if request.is_post() {
        let new_statut_str = request
            .prisme
            .data
            .get("statut")
            .cloned()
            .unwrap_or_default();
        let motif = request
            .prisme
            .data
            .get("motif_annulation")
            .cloned()
            .filter(|v| !v.trim().is_empty());
        let mode_contact = request
            .prisme
            .data
            .get("mode_contact_annulation")
            .cloned()
            .filter(|v| !v.trim().is_empty());

        if let Some(statut) = parse_statut(&new_statut_str) {
            let active = commande::ActiveModel {
                id: Set(cmd.id),
                statut: Set(statut.clone()),
                motif_annulation: Set(motif.clone()),
                mode_contact_annulation: Set(mode_contact.clone()),
                ..Default::default()
            };
            if active.update(db).await.is_err() {
                request
                    .notices
                    .error("Erreur lors de la mise à jour.".to_string())
                    .await;
            } else {
                request
                    .notices
                    .success("Statut mis à jour.".to_string())
                    .await;

                // Envoi email si annulation
                if new_statut_str == "annule" && mailer_configured() {
                    if let Ok(Some(user)) =
                        runique_users::Entity::find_by_id(cmd.user_id).one(db).await
                    {
                        let motif_str = motif.as_deref().unwrap_or("");
                        let mode_contact_str = mode_contact.as_deref().unwrap_or("");
                        let ctx = context! {
                            "username"     => &user.username,
                            "numero"       => &cmd.numero,
                            "motif"        => motif_str,
                            "mode_contact" => mode_contact_str,
                        };
                        if let Ok(msg) = Email::new()
                            .to(user.email.clone())
                            .subject("Annulation de votre commande — U Campanile")
                            .template(
                                &request.engine.tera,
                                "emails/commande_annulee.html",
                                ctx.into(),
                            )
                        {
                            msg.send().await.ok();
                        }
                    }
                }
            }
            return Ok(Redirect::to(&format!(
                "{}/commandes/{}/detail",
                admin.config.prefix, numero
            ))
            .into_response());
        }
    }

    // Charger les lignes
    let lignes_db = search!(commande_plat::Entity => CommandeId eq cmd.id,)
        .all(db)
        .await
        .unwrap_or_default();

    let plat_ids: Vec<i32> = lignes_db.iter().filter_map(|l| l.plat_id).collect();
    let boisson_ids: Vec<i32> = lignes_db.iter().filter_map(|l| l.boisson_id).collect();
    let menu_ids: Vec<i32> = lignes_db.iter().filter_map(|l| l.menu_id).collect();

    let plats_map: HashMap<i32, String> = if !plat_ids.is_empty() {
        search!(plat::Entity => Id in (plat_ids))
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|p| (p.id, p.titre))
            .collect()
    } else {
        HashMap::new()
    };

    let boissons_map: HashMap<i32, String> = if !boisson_ids.is_empty() {
        search!(boisson::Entity => Id in (boisson_ids))
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|b| (b.id, b.titre))
            .collect()
    } else {
        HashMap::new()
    };

    let menus_map: HashMap<i32, String> = if !menu_ids.is_empty() {
        search!(menu_resto::Entity => Id in (menu_ids))
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|m| (m.id, m.titre))
            .collect()
    } else {
        HashMap::new()
    };

    let lignes: Vec<LigneDetail> = lignes_db
        .iter()
        .map(|l| {
            let (titre, est_boisson, est_menu) = if let Some(bid) = l.boisson_id {
                (
                    boissons_map
                        .get(&bid)
                        .cloned()
                        .unwrap_or_else(|| format!("Boisson #{}", bid)),
                    true,
                    false,
                )
            } else if let Some(mid) = l.menu_id {
                (
                    menus_map
                        .get(&mid)
                        .cloned()
                        .unwrap_or_else(|| format!("Menu #{}", mid)),
                    false,
                    true,
                )
            } else if let Some(pid) = l.plat_id {
                (
                    plats_map
                        .get(&pid)
                        .cloned()
                        .unwrap_or_else(|| format!("Plat #{}", pid)),
                    false,
                    false,
                )
            } else {
                ("Article inconnu".to_string(), false, false)
            };
            let cuisson = l.cuisson.as_ref().map(|c| c.to_string());

            LigneDetail {
                titre,
                quantite: l.quantite,
                cuisson,
                est_boisson,
                est_menu,
                note: l.note.clone(),
            }
        })
        .collect();

    let (statut_label, statut_valeur) = statut_info(&cmd.statut);
    let type_commande = cmd.type_commande.to_string();
    let mode_paiement = cmd.mode_paiement.to_string();

    let client = runique_users::Entity::find_by_id(cmd.user_id)
        .one(db)
        .await
        .ok()
        .flatten()
        .map(|u| u.username)
        .unwrap_or_else(|| format!("#{}", cmd.user_id));

    let detail = CommandeDetail {
        numero: cmd.numero.clone(),
        statut: statut_label,
        statut_valeur: statut_valeur.to_string(),
        type_commande,
        mode_paiement,
        prix_total: format!("{:.2}", cmd.prix_total),
        heure_retrait: cmd.heure_retrait.map(|t| t.format("%H:%M").to_string()),
        avec_livraison: cmd.avec_livraison,
        adresse_livraison: cmd.adresse_livraison,
        ville_livraison: cmd.ville_livraison,
        cp_livraison: cmd.cp_livraison,
        heure_livraison: cmd.heure_livraison.map(|t| t.format("%H:%M").to_string()),
        client,
        motif_annulation: cmd.motif_annulation,
        mode_contact_annulation: cmd.mode_contact_annulation,
        lignes,
    };

    // Injection contexte admin
    inject_admin_prefix(&mut request.context, &admin.config.prefix);
    insert_admin_messages(&mut request.context, "base");

    let resources: Vec<&AdminResource> = proto
        .as_ref()
        .map(|s| s.registry.all().map(|e| &e.meta).collect())
        .unwrap_or_default();

    context_update!(request => {
        "detail"           => detail,
        "statuts"          => all_statuts(),
        "title"            => format!("Commande {}", numero),
        "site_title"       => &admin.config.site_title,
        "site_url"         => &admin.config.site_url,
        "current_page"     => "commandes",
        "current_resource" => "commandes",
        "resources"        => resources,
    });
    request.render("admin/commande_detail.html")
}
