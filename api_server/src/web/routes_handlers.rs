use std::hash::{DefaultHasher, Hash, Hasher};
use axum::{Json};
use axum_auth::AuthBasic;
use serde::Deserialize;
use serde_json::Value;
use axum::response::Result;

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

#[derive(Debug, Deserialize)]
pub struct UserPayload {
    password: String,
}