use crate::backend::menus::PlatDetail;
use crate::backend::panier::{LignePanier, panier_get, panier_save};
use crate::backend::stats::get_plat_views;
use crate::entities::{dessert, entree, plat, plat_garniture};
use runique::prelude::*;

async fn garnitures_valides_pour_plat(
    db: &sea_orm::DatabaseConnection,
    plat_id: Pk,
    garniture_ids: &[Pk],
) -> Vec<Pk> {
    if garniture_ids.is_empty() {
        return Vec::new();
    }
    match plat_garniture::Entity::find()
        .filter(plat_garniture::Column::PlatId.eq(plat_id))
        .filter(plat_garniture::Column::GarnitureId.is_in(garniture_ids.to_vec()))
        .all(db)
        .await
    {
        Ok(rows) => rows.into_iter().map(|pg| pg.garniture_id).collect(),
        Err(e) => {
            tracing::error!("Erreur vérification garnitures autorisées (plat {plat_id}): {e}");
            Vec::new()
        }
    }
}

pub struct PanierAjouterParams {
    pub plat_id: Pk,
    pub type_article: String,
    pub quantite: i32,
    pub cuisson: Option<String>,
    pub note: Option<String>,
    pub garniture_ids: Vec<Pk>,
    pub sans_sel: bool,
    pub user_id: Option<Pk>,
}

struct ArticleInfo {
    titre: String,
    description: Option<String>,
    image: Option<String>,
    prix: Decimal,
    est_viande: bool,
}

async fn find_article(
    db: &sea_orm::DatabaseConnection,
    id: Pk,
    type_article: &str,
) -> Option<ArticleInfo> {
    match type_article {
        "entree" => {
            let m = entree::Entity::find_by_id(id)
                .filter(entree::Column::Disponible.eq(true))
                .one(db)
                .await
                .ok()
                .flatten()?;
            Some(ArticleInfo {
                titre: m.titre,
                description: m.description,
                image: m.image,
                prix: m.prix,
                est_viande: false,
            })
        }
        "dessert" => {
            let m = dessert::Entity::find_by_id(id)
                .filter(dessert::Column::Disponible.eq(true))
                .one(db)
                .await
                .ok()
                .flatten()?;
            Some(ArticleInfo {
                titre: m.titre,
                description: m.description,
                image: m.image,
                prix: m.prix,
                est_viande: false,
            })
        }
        _ => {
            let m = plat::Entity::find_by_id(id)
                .filter(plat::Column::Disponible.eq(true))
                .one(db)
                .await
                .ok()
                .flatten()?;
            Some(ArticleInfo {
                titre: m.titre,
                description: m.description,
                image: m.image,
                prix: m.prix,
                est_viande: m.est_viande,
            })
        }
    }
}

pub async fn panier_ajouter(request: &Request, p: PanierAjouterParams) -> Result<(), &'static str> {
    let Some(article) = find_article(request.db(), p.plat_id, &p.type_article).await else {
        return Err("Article introuvable ou indisponible");
    };

    let garniture_ids =
        garnitures_valides_pour_plat(request.db(), p.plat_id, &p.garniture_ids).await;

    let mut panier = panier_get(&request.session).await;
    if panier.user_id.is_none() {
        panier.user_id = p.user_id;
    }
    panier.lignes.push(LignePanier {
        plat_id: p.plat_id,
        type_article: p.type_article.clone(),
        boisson_id: None,
        menu_id: None,
        supplement_id: None,
        titre: article.titre.clone(),
        prix_unitaire: format!("{:.2}", article.prix),
        quantite: p.quantite,
        est_viande: article.est_viande,
        cuisson: p.cuisson,
        note: p.note,
        garniture_ids,
        sans_sel: p.sans_sel,
        menu_choix: vec![],
    });

    panier_save(&request.session, &panier).await;

    let plat_detail = PlatDetail {
        id: p.plat_id,
        titre: article.titre,
        description: article.description,
        image: article.image,
        est_viande: article.est_viande,
        allergenes: vec![],
    };
    let request = request.clone();
    tokio::spawn(async move {
        if let Err(e) = get_plat_views(&request, &[plat_detail]).await {
            tracing::error!("Analytics Mongo error (plat views): {}", e);
        }
    });
    Ok(())
}
