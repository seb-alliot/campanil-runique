use crate::backend::menus::PlatDetail;
use crate::backend::stats::get_mongo;
use mongodb::bson::{DateTime as BsonDateTime, doc};
use runique::prelude::*;

pub async fn get_plat_views(
    request: &Request,
    plats: &[PlatDetail],
) -> Result<(), mongodb::error::Error> {
    let db = get_mongo(request).await?;
    let collection = db.collection::<mongodb::bson::Document>("plat_views");
    let now = BsonDateTime::now();

    for plat in plats {
        collection
            .update_one(
                doc! { "plat_id": plat.id },
                doc! {
                    "$inc": { "vues": 1 },
                    "$set": { "titre": &plat.titre, "derniere_vue": now },
                },
            )
            .upsert(true)
            .await?;
    }

    Ok(())
}
