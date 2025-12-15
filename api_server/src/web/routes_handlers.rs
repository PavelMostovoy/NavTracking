use crate::database::{GeoPoint, SimplifiedData, TrackerGeoData};
use crate::lora_data::payload::UplinkPayload;
use crate::parsers::{string_to_data, string_to_timestamp};
use axum::Json;
use axum::extract::{State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Result};
use axum_auth::{ AuthBearer};
use chrono::TimeZone;
use futures::stream::TryStreamExt as _;
use log::{info};
use mongodb::bson::{doc, DateTime as BsonDateTime};
use mongodb::{Collection, Database, bson};
use serde::{Deserialize};
use serde_json::{Value, json};

const SECRET_LORA_KEY: &str = "ZFj6GzdbLoLT3v2shaVq5iroGViEHglsx3pjXCc2eDbIgOib6sZrwF0q8ibxBIDS";

#[derive(Debug, Deserialize)]
pub(crate) struct TrackerPayload {
    tracker_id: String,
    tracker_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    tail: Option<i64>,
}

pub async fn handle_uplink(
    AuthBearer(bearer): AuthBearer,
    State(db): State<Database>,
    Json(payload): Json<UplinkPayload>,
) -> Json<Value> {
    if bearer != SECRET_LORA_KEY {
        return Json(json!({"error": "Authorization Error"}));
    }

    let time_received = string_to_timestamp(&*payload.time);
    let received_data = string_to_data(payload.data);
    // Time delay verification is 60 seconds, to not receive incorrect data
    // Time between mentioned in JSON and data from the device
    if (time_received - received_data.time) < 60 {
        let datetime = chrono::Utc
            .timestamp_opt(received_data.time as i64, 0)
            .unwrap();
        let data_to_send = TrackerGeoData {
            name: payload.device_info.device_name,
            timestamp: datetime,
            position: GeoPoint::new(received_data.latitude, received_data.longitude),
        };
        let collection = db.collection::<TrackerGeoData>(payload.device_info.dev_eui.as_str());

        let existing_timestamp = collection
            .find_one(doc! {"timestamp": BsonDateTime::from_millis(data_to_send.timestamp.timestamp_millis())})
            .await;
        match existing_timestamp {
            Ok(Some(..)) => {
                // println!("Duplicate found for {}", data_to_send.timestamp);
                let _ = collection
                    .find_one_and_update(
                        doc! {"timestamp": BsonDateTime::from_millis(data_to_send.timestamp.timestamp_millis())},
                        doc! { "$set": bson::to_document(&data_to_send).unwrap() },
                    )
                    .await;
            }
            Ok(None) => {
                let _ = collection.insert_one(data_to_send).await;
            }
            Err(_) => {
                info!("Error happens");
            }
        }
    }
    Json(json!({
        "status": "success",
        "message": "Uplink lora_data received"
    }))
}


pub async fn last_positions(
    State(db): State<Database>,
    payload: Result<Json<TrackerPayload>, axum::extract::rejection::JsonRejection>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let Json(data) = payload.map_err(|err| {
        info!("Error: {:?}", err);
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Bad request "})),
        )
    })?;

    let table_name = data.tracker_id;
    let tracker_name = data.tracker_name;

    let  tail = data.tail.unwrap_or(0);


    let collection: Collection<TrackerGeoData> = db.collection(&table_name);

    let mut cursor = collection
        .find(doc! {"name": &tracker_name})
        .sort(doc! { "timestamp": -1 })
        .limit(tail)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Collection or document not found" })),
            )
        })?;

    let mut tracker_data = Vec::new();

    while let Some(record) = cursor.try_next().await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to parse database response" })),
        )
    })? {
        info!("{:?}", record);
        tracker_data.push(SimplifiedData {
            position: record.position,
            time: record.timestamp,
        });
    }

    let body = json!({
        "result": {
            "tracker_name": tracker_name,
            "data": tracker_data
        }
    });


    Ok((StatusCode::OK, Json(body)))
}