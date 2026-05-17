use crate::backend::menus::PlatDetail;
use crate::backend::panier::{LignePanier, panier_get, panier_save};
use crate::backend::stats::get_plat_views;
use crate::entities::plat;
use runique::prelude::*;

pub struct PanierAjouterParams {
    pub plat_id: Pk,
    pub quantite: i32,
    pub cuisson: Option<String>,
    pub note: Option<String>,
    pub garniture_ids: Vec<Pk>,
    pub avec_legumes: bool,
    pub sans_sel: bool,
    pub user_id: Option<Pk>,
}

pub async fn panier_ajouter(request: &Request, p: PanierAjouterParams) -> Result<(), &'static str> {
    let Some(plat_model) = plat::Entity::find_by_id(p.plat_id)
        .filter(plat::Column::Disponible.eq(true))
        .one(request.db())
        .await
        .ok()
        .flatten()
    else {
        return Err("Plat introuvable ou indisponible");
    };

    let mut panier = panier_get(&request.session).await;
    if panier.user_id.is_none() {
        panier.user_id = p.user_id;
    }
    panier.lignes.push(LignePanier {
        plat_id: p.plat_id,
        boisson_id: None,
        menu_resto_id: None,
        supplement_id: None,
        titre: plat_model.titre.clone(),
        prix_unitaire: format!("{:.2}", plat_model.prix),
        quantite: p.quantite,
        est_viande: plat_model.est_viande,
        cuisson: p.cuisson,
        note: p.note,
        garniture_ids: p.garniture_ids,
        avec_legumes: p.avec_legumes,
        sans_sel: p.sans_sel,
        menu_choix: vec![],
    });

    panier_save(&request.session, &panier).await;

    let plat_detail = PlatDetail {
        id: plat_model.id,
        titre: plat_model.titre,
        description: plat_model.description,
        image: plat_model.image,
        est_viande: plat_model.est_viande,
        allergenes: vec![],
    };
    let _ = get_plat_views(request, &[plat_detail]).await;
    Ok(())
}
