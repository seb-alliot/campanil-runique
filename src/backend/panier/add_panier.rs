use crate::backend::panier::{LignePanier, panier_get, panier_save};
use crate::entities::plat;
use runique::prelude::*;

pub async fn panier_ajouter(
    session: &Session,
    db: &sea_orm::DatabaseConnection,
    plat_id: i32,
    quantite: i32,
    cuisson: Option<String>,
    note: Option<String>,
    garniture_ids: Vec<i32>,
    avec_legumes: bool,
    sans_sel: bool,
    user_id: Option<i32>,
) -> Result<(), &'static str> {
    let Some(plat_model) = plat::Entity::find_by_id(plat_id)
        .filter(plat::Column::Disponible.eq(true))
        .one(db)
        .await
        .ok()
        .flatten()
    else {
        return Err("Plat introuvable ou indisponible");
    };

    let mut panier = panier_get(session).await;
    // protection faille idor
    if panier.user_id.is_none() {
        panier.user_id = user_id;
    }
    // Les garnitures/options font partie de la commande — chaque combinaison est une ligne distincte
    panier.lignes.push(LignePanier {
        plat_id,
        boisson_id: None,
        menu_id: None,
        titre: plat_model.titre,
        prix_unitaire: format!("{:.2}", plat_model.prix),
        quantite,
        est_viande: plat_model.est_viande,
        cuisson,
        note,
        garniture_ids,
        avec_legumes,
        sans_sel,
        menu_choix: vec![],
        supplement_id: None,
    });

    panier_save(session, &panier).await;
    Ok(())
}
