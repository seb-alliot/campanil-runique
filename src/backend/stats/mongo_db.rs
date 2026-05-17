use runique::prelude::*;

pub async fn get_mongo(request: &Request) -> Result<mongodb::Database, mongodb::error::Error> {
    let guard = request
        .engine
        .custom_db
        .read()
        .map_err(|_| mongodb::error::Error::custom("custom_db lock poisoned"))?;

    let client = guard
        .as_ref()
        .and_then(|arc| arc.downcast_ref::<mongodb::Client>())
        .ok_or_else(|| mongodb::error::Error::custom("MongoDB client not initialized"))?;

    Ok(client.database("campanile"))
}
