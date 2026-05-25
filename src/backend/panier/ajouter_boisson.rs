use crate::backend::menus::PlatDetail;
use crate::backend::panier::{LignePanier, panier_get, panier_save};
use crate::backend::stats::get_plat_views;
use crate::entities::boisson;
use runique::prelude::*;

pub async fn panier_ajouter_boisson(
    request: &Request,
    session: &Session,
    db: &sea_orm::DatabaseConnection,
    boisson_id: Pk,
    quantite: i32,
    user_id: Option<Pk>,
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
            type_article: "boisson".to_string(),
            boisson_id: Some(boisson_id),
            menu_id: None,
            supplement_id: None,
            titre: boisson_model.titre.clone(),
            prix_unitaire: format!("{:.2}", boisson_model.prix),
            quantite,
            est_viande: false,
            cuisson: None,
            garniture_ids: vec![],
            sans_sel: false,
            note: None,
            menu_choix: vec![],
        });
    }

    panier_save(session, &panier).await;

    // Track boisson view
    let boisson_detail = PlatDetail {
        id: boisson_model.id,
        titre: boisson_model.titre,
        description: boisson_model.description,
        image: boisson_model.image,
        est_viande: false,
        allergenes: vec![],
    };
    let _ = get_plat_views(request, &[boisson_detail]).await;
    Ok(())
}
