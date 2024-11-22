use std::hash::{DefaultHasher, Hash, Hasher};
use axum::{Json};
use axum::extract::State;
use axum::http::StatusCode;
use axum_auth::AuthBasic;
use serde::Deserialize;
use serde_json::Value;
use axum::response::{ErrorResponse, Result};
use mongodb::{bson, Collection};
use mongodb::bson::doc;
use crate::database::User;

/// Takes basic auth details and shows a message
pub async fn handler(AuthBasic((id, password)): AuthBasic) -> String {
    if let Some(password) = password {
        format!("User '{}' with password '{}'", id, password)
    } else {
        format!("User '{}' without password", id)
    }
}


pub async fn get_pwd_hash(AuthBasic((id, password)): AuthBasic, payload: Json<UserPayload>) -> Result<Json<Value>> {
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

pub async fn create_user(AuthBasic((id, password)): AuthBasic, State(db): State<Collection<User>>, Json(body): Json<UserPayload>) -> Result<Json<Value>> {
    let mut hasher = DefaultHasher::new();
    let pwd = password.unwrap_or("".parse()?);
    pwd.hash(&mut hasher);
    let auth_user = db.find_one(doc! {"name": id}).await;
    match auth_user {
        Ok(Some(user)) => {
            if  user.password.parse::<u64>().unwrap() != hasher.finish() {
                return Err(StatusCode::UNAUTHORIZED.into());
            }
        },
        _ => {return Err(StatusCode::UNAUTHORIZED.into())}
    }

    let user = db
        .find_one(doc! { "name": &body.user_name })
        .await;

    match user {
        Ok(Some(user)) =>
            Ok(
            Json(serde_json::json!({
        "result": {
                "user" : &body.user_name,
                "success": false}
        }))),

        Ok(None) => {
            let mut hasher = DefaultHasher::new();
            body.password.hash(&mut hasher);
            if body.user_name.contains(char::is_whitespace){
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
        _ => Err(ErrorResponse::from(StatusCode::UNPROCESSABLE_ENTITY))
    }
}

#[derive(Debug, Deserialize)]
pub struct UserPayload {
    user_name: String,
    password: String,
}