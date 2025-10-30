use serde::{Deserialize, Serialize};
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
    pub(crate) timestamp:u32,
    pub(crate) position:GeoPoint,
}

#[derive(Debug, Serialize,Deserialize)]
pub(crate) struct SimplifiedData{
    pub(crate) position: GeoPoint,
    pub(crate) time: u32,
}
