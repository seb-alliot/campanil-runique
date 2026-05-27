use crate::backend::menus::MenuFilters;
use crate::backend::stats::get_mongo;
use mongodb::bson::{DateTime as BsonDateTime, doc};
use runique::prelude::*;

fn prix_bucket(val: f64) -> &'static str {
    if val <= 20.0 {
        "≤ 20 €"
    } else if val <= 30.0 {
        "21–30 €"
    } else if val <= 40.0 {
        "31–40 €"
    } else if val <= 50.0 {
        "41–50 €"
    } else {
        "> 50 €"
    }
}

pub async fn get_menu_filters(
    request: &Request,
    filters: &MenuFilters,
) -> Result<(), mongodb::error::Error> {
    let db = get_mongo(request).await?;
    let collection = db.collection::<mongodb::bson::Document>("menu_filters");
    let now = BsonDateTime::now();

    if let Some(ref t) = filters.theme {
        collection
            .update_one(
                doc! { "filtre": "theme", "valeur": t },
                doc! { "$inc": { "count": 1 }, "$set": { "updated_at": now } },
            )
            .upsert(true)
            .await?;
    }

    if let Some(ref r) = filters.regime {
        collection
            .update_one(
                doc! { "filtre": "regime", "valeur": r },
                doc! { "$inc": { "count": 1 }, "$set": { "updated_at": now } },
            )
            .upsert(true)
            .await?;
    }

    if let Some(ref v) = filters.prix_min
        && let Ok(f) = v.parse::<f64>()
    {
        let bucket = prix_bucket(f);
        collection
            .update_one(
                doc! { "filtre": "prix_min", "valeur": bucket },
                doc! { "$inc": { "count": 1 }, "$set": { "updated_at": now } },
            )
            .upsert(true)
            .await?;
    }

    if let Some(ref v) = filters.prix_max
        && let Ok(f) = v.parse::<f64>()
    {
        let bucket = prix_bucket(f);
        collection
            .update_one(
                doc! { "filtre": "prix_max", "valeur": bucket },
                doc! { "$inc": { "count": 1 }, "$set": { "updated_at": now } },
            )
            .upsert(true)
            .await?;
    }

    if let Some(nb) = filters.nb_personnes {
        let valeur = format!("{} pers.", nb);
        collection
            .update_one(
                doc! { "filtre": "nb_personnes", "valeur": &valeur },
                doc! { "$inc": { "count": 1 }, "$set": { "updated_at": now } },
            )
            .upsert(true)
            .await?;
    }

    Ok(())
}
