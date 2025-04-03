use crate::database::{TrackerGeoData, User};
use crate::lora_data::payload::UplinkPayload;
use crate::parsers::{string_to_data, string_to_timestamp};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{ErrorResponse, IntoResponse, Result};
use axum::Json;
use axum_auth::{AuthBasic, AuthBearer};
use jsonwebtoken::{DecodingKey, Validation};
use mongodb::bson::doc;
use mongodb::{bson, Collection, Database};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::{Duration, SystemTime};

const SECRET_SIGNING_KEY: &[u8] = b"keep_th1s_@_secret";
const SECRET_LORA_KEY: &str = "ZFj6GzdbLoLT3v2shaVq5iroGViEHglsx3pjXCc2eDbIgOib6sZrwF0q8ibxBIDS";

#[derive(Serialize, Deserialize)]
pub struct OurJwtPayload {
    pub subject: String,
    pub exp: usize,
}

impl OurJwtPayload {
    pub fn new(subject: String) -> Self {
        // expires by default in 60 minutes from now
        let exp = SystemTime::now()
            .checked_add(Duration::from_secs(60 * 60))
            .expect("valid timestamp")
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("valid duration")
            .as_secs() as usize;

        OurJwtPayload { subject, exp }
    }
}

/// Takes basic auth details and shows a message
pub async fn auth_check(
    AuthBasic((id, password)): AuthBasic,
    State(db): State<Collection<User>>,
) -> impl IntoResponse {
    let mut hasher = DefaultHasher::new();
    let pwd = password.unwrap_or(String::new());
    pwd.hash(&mut hasher);

    let user = db.find_one(doc! { "name": &id }).await;
    match user {
        Ok(Some(user)) => {
            if user.password == hasher.finish().to_string() {
                let Ok(jwt) = jsonwebtoken::encode(
                    &jsonwebtoken::Header::default(),
                    &OurJwtPayload::new(id),
                    &jsonwebtoken::EncodingKey::from_secret(SECRET_SIGNING_KEY),
                ) else {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Failed to generate token"})),
                    );
                };

                (StatusCode::OK, Json(json!({"jwt": jwt})))
            } else {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Unauthorized"})),
                )
            }
        }
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Internal Server Error"})),
        ),
    }
}

pub async fn get_pwd_hash(
    AuthBasic((id, password)): AuthBasic,
    payload: Json<UserPayload>,
) -> Result<Json<Value>> {
    let mut hasher = DefaultHasher::new();
    payload.password.hash(&mut hasher);

    let body = Json(serde_json::json!({
    "result": {
            "auth" : password,
            "success": true,
            "pwd_hash": hasher.finish()},
    }));

    Ok(body)
}

pub async fn create_user(
    AuthBasic((id, password)): AuthBasic,
    State(db): State<Collection<User>>,
    Json(body): Json<UserPayload>,
) -> Result<Json<Value>> {
    let mut hasher = DefaultHasher::new();
    let pwd = password.unwrap_or("".parse()?);
    pwd.hash(&mut hasher);
    let auth_user = db.find_one(doc! {"name": id}).await;
    match auth_user {
        Ok(Some(user)) => {
            if user.password.parse::<u64>().unwrap() != hasher.finish() {
                return Err(StatusCode::UNAUTHORIZED.into());
            }
        }
        _ => return Err(StatusCode::UNAUTHORIZED.into()),
    }

    let user = db.find_one(doc! { "name": &body.user_name }).await;

    match user {
        Ok(Some(user)) => Ok(Json(serde_json::json!({
        "result": {
                "user" : &body.user_name,
                "success": false}
        }))),

        Ok(None) => {
            let mut hasher = DefaultHasher::new();
            body.password.hash(&mut hasher);
            if body.user_name.contains(char::is_whitespace) {
                return Err(StatusCode::BAD_REQUEST.into());
            }
            let new_user = User {
                _id: bson::oid::ObjectId::new(),
                name: body.user_name,
                password: hasher.finish().to_string(),
            };

            let uid = db.insert_one(new_user).await;

            match uid {
                Ok(uid) => Ok(Json(serde_json::json!({"result": uid}))),
                Err(err) => Ok(Json(serde_json::json!({"error": format!("{}", err)}))),
            }
        }
        _ => Err(ErrorResponse::from(StatusCode::UNPROCESSABLE_ENTITY)),
    }
}

#[derive(Debug, Deserialize)]
pub struct UserPayload {
    user_name: String,
    password: String,
}

pub async fn token_visits(AuthBearer(bearer): AuthBearer) -> impl IntoResponse {
    let token = bearer;
    let decoding_key = DecodingKey::from_secret(SECRET_SIGNING_KEY);

    let Ok(jwt) =
        jsonwebtoken::decode::<OurJwtPayload>(&token, &decoding_key, &Validation::default())
    else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid token"})),
        );
    };

    let username = jwt.claims.subject;

    (
        StatusCode::OK,
        Json(json!({"ok": format_args!("Logged as {username}")})),
    )
}

pub async fn handle_uplink(
    AuthBearer(bearer): AuthBearer,
    State(db): State<Database>,
    Json(payload): Json<UplinkPayload>,
) -> Json<Value> {

    if bearer != SECRET_LORA_KEY{
        return Json(json!({"error": "Authorization Error"}));
    }
    // match payload.f_cnt {
    //     Some(cnt) => {
    //         println!("Count: {}", cnt)
    //     }
    //     _ => {
    //         println!("Incorrect lora_data ")
    //     }
    // }

    let time_received = string_to_timestamp(&*payload.time);
    let received_data = string_to_data(payload.data);
    // println!("Difference is : {}", (time_received - received_data.time));
    if (time_received - received_data.time) < 60 {
        // println!(
        //     "Time: {} lat {} lon {}",
        //     received_data.time, received_data.longitude, received_data.latitude
        // );
        // println!("device name {:?}", payload.device_info.device_name);
        let data_to_send = TrackerGeoData {
            name: payload.device_info.device_name,
            timestamp: received_data.time,
            latitude: (received_data.latitude * 1000000.0) as i32,
            longitude: (received_data.longitude * 1000000.0) as i32,
        };
        let collection = db.collection::<TrackerGeoData>(payload.device_info.dev_eui.as_str());

        let existing_timestamp = collection
            .find_one(doc! {"timestamp": data_to_send.timestamp})
            .await;
        match existing_timestamp {
            Ok(Some(..)) => {
                // println!("Duplicate found for {}", data_to_send.timestamp);
                let _ = collection
                    .find_one_and_update(
                        doc! {"timestamp": data_to_send.timestamp},
                        doc! { "$set": bson::to_document(&data_to_send).unwrap() },
                    )
                    .await;
            }
            Ok(None) => {
                let _ = collection.insert_one(data_to_send).await;
            }
            Err(_) => {
                println!("Error happens");
            }
        }
    }
    Json(json!({
        "status": "success",
        "message": "Uplink lora_data received"
    }))
}
