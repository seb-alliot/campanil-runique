use crate::backend::menus::MenuFilters;
use crate::backend::stats::get_mongo;
use mongodb::bson::{DateTime as BsonDateTime, doc};
use runique::prelude::*;

pub async fn get_menu_filters(
    request: &Request,
    filters: &MenuFilters,
) -> Result<(), mongodb::error::Error> {
    let db = get_mongo(request).await?;
    let collection = db.collection::<mongodb::bson::Document>("menu_filters");
    let now = BsonDateTime::now();

    let mut updates: Vec<(&str, String)> = Vec::new();

    if let Some(pmin) = &filters.prix_min {
        updates.push(("prix_min", pmin.clone()));
    }
    if let Some(pmax) = &filters.prix_max {
        updates.push(("prix_max", pmax.clone()));
    }
    if let Some(theme) = &filters.theme {
        updates.push(("theme", theme.clone()));
    }
    if let Some(regime) = &filters.regime {
        updates.push(("regime", regime.clone()));
    }
    if let Some(np) = &filters.nb_personnes {
        updates.push(("nb_personnes", np.to_string()));
    }

    for (filtre, valeur) in updates {
        collection
            .update_one(
                doc! { "filtre": filtre, "valeur": &valeur },
                doc! { "$inc": { "count": 1 }, "$set": { "updated_at": now } },
            )
            .upsert(true)
            .await?;
    }

    Ok(())
}
