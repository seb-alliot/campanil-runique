use runique::prelude::*;

pub async fn get_mongo(request: &Request) -> Result<mongodb::Database, mongodb::error::Error> {
    let client = request
        .engine
        .custom_db::<mongodb::Client>()
        .ok_or_else(|| mongodb::error::Error::custom("MongoDB client not initialized"))?;

    Ok(client.database("campanile"))
}
