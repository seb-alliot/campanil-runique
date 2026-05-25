use crate::entities::info_resto;
use runique::prelude::*;
use std::str::FromStr;
const PRIX_PAR_KM: f64 = 0.54;

pub async fn get_prix_livraison(db: &sea_orm::DatabaseConnection) -> Decimal {
    search!(info_resto::Entity)
        .first(db)
        .await
        .ok()
        .flatten()
        .and_then(|r| r.prix_livraison)
        .unwrap_or_else(|| Decimal::from_str("5.00").unwrap())
}

fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0_f64;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * r * a.sqrt().atan2((1.0 - a).sqrt())
}

pub async fn prix_livraison_distance(
    db: &sea_orm::DatabaseConnection,
    adresse: &str,
    cp: &str,
    ville: &str,
) -> Option<Decimal> {
    let row = search!(info_resto::Entity).first(db).await.ok().flatten()?;
    let resto_lat = row.latitude?.to_string().parse::<f64>().ok()?;
    let resto_lon = row.longitude?.to_string().parse::<f64>().ok()?;

    let query = format!("{} {} {}", adresse, cp, ville);
    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1",
        urlencoding::encode(&query)
    );

    let client = reqwest::Client::new();
    let resp: serde_json::Value = client.get(&url).send().await.ok()?.json().await.ok()?;
    let result = resp.get(0)?;
    let lat: f64 = result.get("lat")?.as_str()?.parse().ok()?;
    let lon: f64 = result.get("lon")?.as_str()?.parse().ok()?;

    let km = haversine_km(resto_lat, resto_lon, lat, lon);
    let prix = km * PRIX_PAR_KM;
    Decimal::from_str(&format!("{:.2}", prix)).ok()
}
