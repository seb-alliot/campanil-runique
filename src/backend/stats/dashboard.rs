use crate::backend::stats::get_mongo;
use mongodb::bson::{Bson, Document, doc};
use runique::prelude::*;
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct StatsDashboard {
    pub nb_commandes: i64,
    pub nb_carte: i64,
    pub nb_traiteur: i64,
    pub ca_total: f64,
    pub jours: Vec<JourCA>,
    pub top_plats: Vec<PlatViewStat>,
    pub top_filtres: Vec<FilterStat>,
}

#[derive(Serialize)]
pub struct JourCA {
    pub date: String,
    pub ca: f64,
    pub nb: i64,
}

#[derive(Serialize)]
pub struct PlatViewStat {
    pub titre: String,
    pub vues: i64,
}

#[derive(Serialize)]
pub struct FilterStat {
    pub filtre: String,
    pub valeur: String,
    pub count: i64,
}

fn bson_str(doc: &Document, key: &str) -> String {
    doc.get_str(key).unwrap_or("").to_string()
}

fn bson_i64(doc: &Document, key: &str) -> i64 {
    match doc.get(key) {
        Some(Bson::Int32(v)) => *v as i64,
        Some(Bson::Int64(v)) => *v,
        Some(Bson::Double(v)) => *v as i64,
        _ => 0,
    }
}

fn bson_f64(doc: &Document, key: &str) -> f64 {
    match doc.get(key) {
        Some(Bson::Int32(v)) => *v as f64,
        Some(Bson::Int64(v)) => *v as f64,
        Some(Bson::Double(v)) => *v,
        Some(Bson::String(s)) => s.parse().unwrap_or(0.0),
        _ => 0.0,
    }
}

pub async fn load_stats(request: &Request, periode_jours: u32) -> StatsDashboard {
    let Ok(db) = get_mongo(request).await else {
        return StatsDashboard::default();
    };

    let mut stats = StatsDashboard::default();
    let col_events = db.collection::<Document>("commande_events");

    let cutoff = chrono::Utc::now() - chrono::Duration::days(periode_jours as i64);
    let cutoff_bson = mongodb::bson::DateTime::from_system_time(cutoff.into());
    let match_periode = doc! { "$match": { "created_at": { "$gte": cutoff_bson } } };

    // Totaux sur la période
    let pipeline_types = vec![
        match_periode.clone(),
        doc! { "$group": {
            "_id": "$type_commande",
            "nb": { "$sum": 1 },
            "ca": { "$sum": { "$toDouble": "$prix_total" } },
        }},
    ];
    if let Ok(mut cur) = col_events.aggregate(pipeline_types).await {
        while cur.advance().await.unwrap_or(false) {
            if let Ok(doc) = cur.deserialize_current() {
                let nb = bson_i64(&doc, "nb");
                let ca = bson_f64(&doc, "ca");
                stats.nb_commandes += nb;
                stats.ca_total += ca;
                match bson_str(&doc, "_id").as_str() {
                    "carte" => stats.nb_carte = nb,
                    "traiteur" => stats.nb_traiteur = nb,
                    _ => {}
                }
            }
        }
    }

    // Groupement par jour (≤90j) ou par mois (>90j)
    let par_mois = periode_jours > 90;
    let group_id = if par_mois {
        doc! { "y": { "$year": "$created_at" }, "m": { "$month": "$created_at" } }
    } else {
        doc! { "y": { "$year": "$created_at" }, "m": { "$month": "$created_at" }, "d": { "$dayOfMonth": "$created_at" } }
    };
    let sort_key = if par_mois {
        doc! { "_id.y": 1, "_id.m": 1 }
    } else {
        doc! { "_id.y": 1, "_id.m": 1, "_id.d": 1 }
    };

    let pipeline_jours = vec![
        match_periode,
        doc! { "$group": {
            "_id": group_id,
            "nb": { "$sum": 1 },
            "ca": { "$sum": { "$toDouble": "$prix_total" } },
        }},
        doc! { "$sort": sort_key },
    ];
    if let Ok(mut cur) = col_events.aggregate(pipeline_jours).await {
        while cur.advance().await.unwrap_or(false) {
            if let Ok(doc) = cur.deserialize_current()
                && let Some(Bson::Document(id)) = doc.get("_id")
            {
                let date = if par_mois {
                    format!("{:02}/{}", bson_i64(id, "m"), bson_i64(id, "y"))
                } else {
                    format!("{:02}/{:02}", bson_i64(id, "d"), bson_i64(id, "m"))
                };
                stats.jours.push(JourCA {
                    date,
                    ca: bson_f64(&doc, "ca"),
                    nb: bson_i64(&doc, "nb"),
                });
            }
        }
    }

    let opts_p = mongodb::options::FindOptions::builder()
        .sort(doc! { "vues": -1 })
        .limit(10)
        .build();
    if let Ok(mut cur) = db
        .collection::<Document>("plat_views")
        .find(doc! {})
        .with_options(opts_p)
        .await
    {
        while cur.advance().await.unwrap_or(false) {
            if let Ok(doc) = cur.deserialize_current() {
                stats.top_plats.push(PlatViewStat {
                    titre: bson_str(&doc, "titre"),
                    vues: bson_i64(&doc, "vues"),
                });
            }
        }
    }

    let opts_f = mongodb::options::FindOptions::builder()
        .sort(doc! { "filtre": 1, "count": -1 })
        .limit(50)
        .build();
    if let Ok(mut cur) = db
        .collection::<Document>("menu_filters")
        .find(doc! {})
        .with_options(opts_f)
        .await
    {
        while cur.advance().await.unwrap_or(false) {
            if let Ok(doc) = cur.deserialize_current() {
                stats.top_filtres.push(FilterStat {
                    filtre: bson_str(&doc, "filtre"),
                    valeur: bson_str(&doc, "valeur"),
                    count: bson_i64(&doc, "count"),
                });
            }
        }
    }

    stats
}
