use crate::backend::panier::{LignePanier, panier_get, panier_save};
use crate::entities::{garniture, supplement};
use runique::prelude::*;

pub async fn panier_ajouter_supplement(
    session: &Session,
    db: &sea_orm::DatabaseConnection,
    supplement_id: Pk,
    quantite: i32,
    user_id: Option<Pk>,
) -> Result<(), &'static str> {
    let Some(sup) = supplement::Entity::find_by_id(supplement_id)
        .filter(supplement::Column::Disponible.eq(true))
        .one(db)
        .await
        .ok()
        .flatten()
    else {
        return Err("Supplément introuvable ou indisponible");
    };

    let label = if let Some(ref t) = sup.titre {
        t.clone()
    } else if let Some(gid) = sup.garniture_id {
        garniture::Entity::find_by_id(gid)
            .one(db)
            .await
            .ok()
            .flatten()
            .map(|g| g.libelle)
            .unwrap_or_else(|| "Supplément".to_string())
    } else {
        "Supplément".to_string()
    };

    let mut panier = panier_get(session).await;
    if panier.user_id.is_none() {
        panier.user_id = user_id;
    }
    if let Some(ligne) = panier
        .lignes
        .iter_mut()
        .find(|l| l.supplement_id == Some(supplement_id))
    {
        ligne.quantite += quantite;
    } else {
        panier.lignes.push(LignePanier {
            plat_id: 0,
            boisson_id: None,
            menu_resto_id: None,
            supplement_id: Some(supplement_id),
            titre: label,
            prix_unitaire: format!("{:.2}", sup.prix),
            quantite,
            est_viande: false,
            cuisson: None,
            garniture_ids: vec![],
            avec_legumes: false,
            sans_sel: false,
            note: None,
            menu_choix: vec![],
        });
    }

    panier_save(session, &panier).await;
    Ok(())
}
