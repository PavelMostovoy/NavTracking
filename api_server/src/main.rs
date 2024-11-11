mod database;
mod web;

use axum::Router;
use axum::routing::{get, post};
use mongodb::bson::doc;
use mongodb::{Client, Collection};
use tokio::net::TcpListener;
use crate::database::{User, DB_URL, DB_USER};
use crate::web::routes_handlers::{get_pwd_hash, handler};

#[tokio::main]
async fn main() {
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
    println!("Pinged your database. Successfully connected to MongoDB!");


    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    axum::serve(listener, app(client))
        .await
        .unwrap();
}

// main app
fn app(client: Client) -> Router {
    let collection: Collection<User> = client.database("navigation").collection("users");
    Router::new().route("/", get(|| async { "API Endpoint" }))
        .route("/auth", get(handler)).with_state(collection).route("/hash", post(get_pwd_hash))
}

