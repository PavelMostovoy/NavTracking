mod database;
mod web;
mod parsers;
mod lora_data;

use std::env;
use axum::{Router};
use axum::routing::{get, post};
use mongodb::bson::doc;
use mongodb::{Client, Database};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use crate::database::{DB_URL, DB_USER, ensure_timestamp_index_for_all_collections};
use crate::web::routes_handlers::{get_version, handle_uplink, last_positions};
use log::{ info};
use env_logger;


#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting server...");

    let db_full = env::var("DATABASE_URL");
    let db_pwd = env::var("DB_PASSWORD");
    let db_connection_str;
    match db_full {
        Err(_) => {
            match db_pwd {
                Err(_) => {
                    panic!("Password or DB connector not found in System Variables:")
                }
                Ok(value) => {
                    db_connection_str = format!("mongodb+srv://{DB_USER}:{value}@{DB_URL}").to_string();
                }
            }
        }
        Ok(value) => {
            db_connection_str = value;
        }
    }

    // connecting to mongodb
    let client;
    match Client::with_uri_str(db_connection_str).await {
        Err(_) => {
            panic!("Failed to connect to Server.")
        }
        Ok(result) => {
            client = result;
        }
    }

    // pinging the database
    client
        .database("navigation")
        .run_command(doc! { "ping": 1 })
        .await
        .unwrap();
    info!("Pinged your database. Successfully connected to MongoDB!");

    // Ensure timestamp index exists for all existing collections on startup
    let navigation_db = client.database("navigation");
    if let Err(e) = ensure_timestamp_index_for_all_collections(&navigation_db).await {
        info!("Index ensuring failed for some collections: {}", e);
    } else {
        info!("Ensured 'timestamp' index for all existing collections");
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], 3311));

    let listener = TcpListener::bind(addr).await.unwrap();

    info!("Server running on {addr}");

    axum::serve(listener, app(client))
        .await
        .unwrap();
}


fn app(client: Client) -> Router {
    let db_connector: Database = client.database("navigation");
    let db_connector_1: Database = client.database("navigation");
    Router::new().route("/", get(|| async { "API Endpoint" }))
        .route("/lora", post(handle_uplink)).with_state(db_connector)
        .route("/last_positions", post(last_positions)).with_state(db_connector_1)
        .route("/version", get(get_version))
}

