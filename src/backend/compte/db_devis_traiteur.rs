use crate::backend::compte::struct_::DevisCard;
use crate::entities::{devis_traiteur, menu_traiteur};
use runique::prelude::*;
use sea_orm::ColumnTrait;

pub async fn get_devis_user(db: &DatabaseConnection, user_id: i32) -> Vec<DevisCard> {
    let devis = search!(devis_traiteur::Entity => UserId eq user_id, desc Id,)
        .all(db)
        .await
        .unwrap_or_default();

    let menu_ids: Vec<i32> = devis.iter().filter_map(|d| d.menu_id).collect();

    let menus = if !menu_ids.is_empty() {
        menu_traiteur::Entity::find()
            .filter(menu_traiteur::Column::Id.is_in(menu_ids))
            .all(db)
            .await
            .unwrap_or_default()
    } else {
        vec![]
    };

    let menu_map: std::collections::HashMap<i32, String> =
        menus.into_iter().map(|m| (m.id, m.titre)).collect();

    devis
        .into_iter()
        .map(|d| {
            let menu_titre = d
                .menu_id
                .and_then(|id| menu_map.get(&id))
                .cloned()
                .unwrap_or_else(|| "Non spécifié".to_string());
            let remise_appliquee = d
                .remise_appliquee
                .filter(|r| *r > Decimal::ZERO)
                .map(|r| format!("{:.0}%", r));

            let (statut_label, statut_css) = match d.statut {
                devis_traiteur::StatutDevis::EnAttente => ("En attente", "en_attente"),
                devis_traiteur::StatutDevis::EnCours => ("En cours", "en_cours"),
                devis_traiteur::StatutDevis::Accepte => ("Accepté", "accepte"),
                devis_traiteur::StatutDevis::Refuse => ("Refusé", "refuse"),
            };

            DevisCard {
                id: d.id,
                menu_titre,
                date_evenement: d.date_evenement.to_string(),
                nb_personnes: d.nb_personnes,
                prix_total: d.prix_total.map(|p| format!("{:.2}", p)),
                remise_appliquee,
                statut_label: statut_label.to_string(),
                statut_css: statut_css.to_string(),
                created_at: d
                    .created_at
                    .map(|dt| dt.format("%d/%m/%Y").to_string())
                    .unwrap_or_default(),
            }
        })
        .collect()
}
