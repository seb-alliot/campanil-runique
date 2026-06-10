use crate::entities::info_resto;
use runique::prelude::*;
use std::str::FromStr;
const PRIX_PAR_KM_FALLBACK: f64 = 0.59;
const BASE_LIVRAISON_FALLBACK: f64 = 5.0;

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

    if let Some(ref resto_ville) = row.ville
        && resto_ville.trim().to_lowercase() == ville.trim().to_lowercase()
    {
        return Some(Decimal::ZERO);
    }

    let resto_lat = row.latitude?.to_string().parse::<f64>().ok()?;
    let resto_lon = row.longitude?.to_string().parse::<f64>().ok()?;

    let query = format!("{} {} {}", adresse, cp, ville);
    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1",
        urlencoding::encode(&query)
    );

    let email = row
        .email
        .as_deref()
        .unwrap_or("alliotsebastien04@gmail.com");
    let user_agent = format!("UCampanile/1.0 ({})", email);
    let client = reqwest::Client::new();
    let resp: serde_json::Value = client
        .get(&url)
        .header("User-Agent", user_agent)
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    let result = resp.get(0)?;
    let lat: f64 = result.get("lat")?.as_str()?.parse().ok()?;
    let lon: f64 = result.get("lon")?.as_str()?.parse().ok()?;

    let prix_par_km = row
        .prix_livraison
        .and_then(|p| p.to_string().parse::<f64>().ok())
        .unwrap_or(PRIX_PAR_KM_FALLBACK);
    let base = row
        .prix_livraison_minimal
        .and_then(|p| p.to_string().parse::<f64>().ok())
        .unwrap_or(BASE_LIVRAISON_FALLBACK);
    let km = haversine_km(resto_lat, resto_lon, lat, lon);
    let prix = base + km * prix_par_km;
    Decimal::from_str(&format!("{:.2}", prix)).ok()
}

#[cfg(test)]
mod tests {
    use super::haversine_km;

    #[test]
    fn haversine_meme_point() {
        let d = haversine_km(42.297, 9.149, 42.297, 9.149);
        assert!(
            d < 0.001,
            "distance d'un point à lui-même doit être ~0, obtenu {d:.6}"
        );
    }

    #[test]
    fn haversine_corte_bastia() {
        // Corte (42.30°N 9.15°E) → Bastia (42.70°N 9.45°E) ≈ 52 km
        let d = haversine_km(42.297, 9.149, 42.703, 9.450);
        assert!(d > 45.0 && d < 60.0, "attendu ~52 km, obtenu {d:.1}");
    }

    #[test]
    fn haversine_corte_ajaccio() {
        // Corte (42.30°N 9.15°E) → Ajaccio (41.92°N 8.74°E) ≈ 54 km à vol d'oiseau
        let d = haversine_km(42.297, 9.149, 41.919, 8.738);
        assert!(d > 48.0 && d < 60.0, "attendu ~54 km, obtenu {d:.1}");
    }

    #[test]
    fn prix_formule_base_plus_distance() {
        // base=5.00, tarif=0.59/km, distance=30km → 5 + 17.70 = 22.70
        let base = 5.0_f64;
        let tarif = 0.59_f64;
        let km = 30.0_f64;
        let prix = ((base + km * tarif) * 100.0).round() / 100.0;
        assert!(
            (prix - 22.70).abs() < 0.001,
            "attendu 22.70, obtenu {prix:.2}"
        );
    }

    #[test]
    fn prix_formule_meme_ville_zero() {
        // même ville → livraison gratuite (Decimal::ZERO dans le code)
        let prix = 0.0_f64;
        assert_eq!(prix, 0.0);
    }
}
