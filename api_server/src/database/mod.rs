use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use log::info;
use mongodb::bson::doc;
use mongodb::{Collection, Database};
use mongodb::options::IndexOptions;
use mongodb::IndexModel;
use mongodb::bson::DateTime as BsonDateTime;
pub const DB_URL: &str = "cluster0.8xcdaom.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";

pub const DB_USER: &str = "db_user_temp";

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoPoint {
    #[serde(rename = "type")]
    geo_type: String,
    pub(crate) coordinates: [f64; 2],
}
impl GeoPoint {
    pub(crate) fn new(lat: f64, lon: f64) -> Self {
        let factor = 1_000_000.0;
        let r_lat = (lat * factor).round() / factor;
        let r_lon = (lon * factor).round() / factor;
        Self {
            geo_type: "Point".to_string(),
            // from spec: If specifying latitude and longitude coordinates, list the longitude first, and then latitude.
            coordinates: [r_lon, r_lat],
        }
    }
}

#[derive(Debug, Serialize,Deserialize)]
pub(crate) struct TrackerGeoData{
    pub(crate) name:String,
    pub(crate) position:GeoPoint,
    pub(crate) timestamp:BsonDateTime,

}

#[derive(Debug, Serialize,Deserialize)]
pub(crate) struct SimplifiedData{
    pub(crate) position: GeoPoint,
    pub(crate) time: DateTime<Utc>,
}

// Ensure an ascending index on the `timestamp` field exists for a given collection
pub async fn ensure_timestamp_index<T: Send + Sync>(collection: &Collection<T>) -> mongodb::error::Result<()> {
    // Prefer a stable, deterministic index name
    let index_name = "timestamp_-1";

    // Fast path: if index already present, do nothing
    match collection.list_index_names().await {
        Ok(names) if names.iter().any(|n| n == index_name) => return Ok(()),
        Ok(_) => {}
        Err(_) => {
            // If listing fails, we still try to create; the server will dedupe if it already exists
        }
    }

    let mut opts = IndexOptions::default();
    opts.name = Some(index_name.to_string());
    let model = IndexModel::builder()
        .keys(doc! { "timestamp": -1 })
        .options(opts)
        .build();

    match collection.create_index(model).await {
        Ok(created) => {
            info!("Ensured index '{:?}' on collection '{}'", created, collection.name());
            Ok(())
        }
        Err(e) => Err(e),
    }
}

// Iterate all existing collections in the database and ensure the timestamp index.
pub async fn ensure_timestamp_index_for_all_collections(db: &Database) -> mongodb::error::Result<()> {
    let names = db.list_collection_names().await?;
    for name in names {
        // Use Document to avoid schema coupling
        let coll = db.collection::<mongodb::bson::Document>(&name);
        if let Err(e) = ensure_timestamp_index(&coll).await {
            // Log but continue with other collections
            info!("Failed to ensure index for collection '{}': {}", name, e);
        }
    }
    Ok(())
}
