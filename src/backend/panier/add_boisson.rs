use crate::backend::panier::{LignePanier, panier_get, panier_save};
use crate::entities::boisson;
use runique::prelude::*;

pub async fn panier_ajouter_boisson(
    session: &Session,
    db: &sea_orm::DatabaseConnection,
    boisson_id: i32,
    quantite: i32,
    user_id: Option<i32>,
) -> Result<(), &'static str> {
    let Some(boisson_model) = boisson::Entity::find_by_id(boisson_id)
        .filter(boisson::Column::Disponible.eq(true))
        .one(db)
        .await
        .ok()
        .flatten()
    else {
        return Err("Boisson introuvable ou indisponible");
    };

    let mut panier = panier_get(session).await;
    // protection idor
    if panier.user_id.is_none() {
        panier.user_id = user_id;
    }
    if let Some(ligne) = panier
        .lignes
        .iter_mut()
        .find(|l| l.boisson_id == Some(boisson_id))
    {
        ligne.quantite += quantite;
    } else {
        panier.lignes.push(LignePanier {
            plat_id: 0,
            boisson_id: Some(boisson_id),
            menu_id: None,
            titre: boisson_model.titre,
            prix_unitaire: format!("{:.2}", boisson_model.prix),
            quantite,
            est_viande: false,
            cuisson: None,
            garniture_ids: vec![],
            avec_legumes: false,
            sans_sel: false,
            note: None,
            menu_choix: vec![],
            supplement_id: None,
        });
    }

    panier_save(session, &panier).await;
    Ok(())
}
