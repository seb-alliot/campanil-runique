use crate::backend::stats::get_mongo;
use chrono::Utc;
use mongodb::bson::{DateTime as BsonDateTime, doc};
use runique::prelude::*;
use sea_orm_migration::prelude::prelude::rust_decimal;

pub struct CommandeEventParams {
    pub commande_id: Pk,
    pub numero: String,
    pub type_commande: String,
    pub prix_total: rust_decimal::Decimal,
    pub date: chrono::DateTime<Utc>,
    pub user_id: Pk,
    pub statut: String,
}

pub async fn get_commande_event(
    request: &Request,
    p: CommandeEventParams,
) -> Result<(), mongodb::error::Error> {
    let db = get_mongo(request).await?;
    let event = doc! {
        "commande_id": p.commande_id,
        "numero": &p.numero,
        "type_commande": &p.type_commande,
        "prix_total": p.prix_total.to_string(),
        "date": BsonDateTime::from_system_time(p.date.into()),
        "user_id": p.user_id,
        "statut": &p.statut,
        "created_at": BsonDateTime::now(),
    };
    db.collection::<mongodb::bson::Document>("commande_events")
        .insert_one(event)
        .await?;
    Ok(())
}
