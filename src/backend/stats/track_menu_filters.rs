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

    Ok(())
}
