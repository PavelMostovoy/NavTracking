use mongodb::bson;
use serde::{Deserialize, Serialize};
pub const DB_URL: &str = "cluster0.8xcdaom.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";

pub const DB_USER: &str = "dbuser";

#[derive(Debug, Serialize,Deserialize)]
pub struct User{
    pub _id: bson::oid::ObjectId,
    pub name:String,
    pub password:String,
}

#[derive(Debug, Serialize,Deserialize)]
enum GeoTypes {
    Point,
    LineString,
    Polygon,
    MultiPoint
}

#[derive(Debug, Serialize,Deserialize)]
struct GeoData<T>{
    r#type: GeoTypes,
    coordinates: Vec<T>,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct Location {
    _id:bson::oid::ObjectId,
    owner:String,
    date_time: bson::DateTime,
    altitude:f32,
    sog:f32,
    location: GeoData<f32>,
}