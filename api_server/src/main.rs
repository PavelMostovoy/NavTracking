mod database;
mod web;
mod parsers;
mod lora_data;

use axum::{Router};
use axum::routing::{get, post};
use mongodb::bson::doc;
use mongodb::{Client, Collection, Database};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use crate::database::{User, DB_URL, DB_USER};
use crate::web::routes_handlers::{create_user, get_pwd_hash, auth_check, token_visits, handle_uplink, get_single_track, get_last_positions};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
        .with_env_filter("info") // Set default level to info
        .init();
    
    info!("Starting server...");
    let db_full = std::env::var("DATABASE_URL");
    let db_pwd = std::env::var("DB_PASSWORD");
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

    let addr = SocketAddr::from(([0, 0, 0, 0], 3311));

    let listener = TcpListener::bind(addr).await.unwrap();

    info!("Server running on http://{addr}");

    axum::serve(listener, app(client))
        .await
        .unwrap();
}

// main app
fn app(client: Client) -> Router {
    let db_connector: Database = client.database("navigation");
    let db_connector_1: Database = client.database("navigation");
    let users: Collection<User> = client.database("navigation").collection("users");
    Router::new().route("/", get(|| async { "API Endpoint" }))
        .route("/auth", get(auth_check)).with_state(users.clone())
        .route("/hash", post(get_pwd_hash))
        .route("/token", post(token_visits))
        .route("/user/create", post(create_user)).with_state(users)
        .route("/lora", post(handle_uplink)).with_state(db_connector)
        .route("/get_single_track", post(get_single_track)).with_state(db_connector_1.clone())
        .route("/get_last_positions/{count}", post(get_last_positions)).with_state(db_connector_1)
}

