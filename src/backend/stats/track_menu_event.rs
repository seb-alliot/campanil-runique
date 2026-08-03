use crate::backend::stats::get_mongo;
use mongodb::bson::{DateTime as BsonDateTime, doc};
use runique::prelude::*;
use sea_orm_migration::prelude::prelude::rust_decimal;

pub struct MenuEventParams {
    pub menu_id: Pk,
    pub titre: String,
    pub source: String,
    pub ca: rust_decimal::Decimal,
}

pub async fn get_menu_event(
    request: &Request,
    p: MenuEventParams,
) -> Result<(), mongodb::error::Error> {
    let db = get_mongo(request).await?;
    let event = doc! {
        "menu_id": p.menu_id,
        "titre": &p.titre,
        "source": &p.source,
        "ca": p.ca.to_string(),
        "created_at": BsonDateTime::now(),
    };
    db.collection::<mongodb::bson::Document>("menu_events")
        .insert_one(event)
        .await?;
    Ok(())
}
