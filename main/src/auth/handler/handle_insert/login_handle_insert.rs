use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use axum_msgpack::MsgPack;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    auth::{
        handler::{handle_insert::signup_handle_insert::Tokken, log_in::RedisUser},
        utils::{create_jwt::create_jwt, helper::is_strict_email},
    },
};

// yeh struct jo ham ne fronted se lena hai 
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginHandleInsert {
    #[validate(email(message = "Email format is invalid"))]
    email: String,
    code: String,
}

// yeh main function jahan se ham login handle insert karein ge
pub async fn login_hanle_insert(
    State(appstate): State<Arc<AppState>>,
    MsgPack(data): MsgPack<LoginHandleInsert>,
) -> Result<MsgPack<Tokken>, (StatusCode, String)> {
    let client = appstate.db.clone();
    let redis = appstate.redis.clone();
    let mut redis_conn = redis.lock().await;

    // ham check karte hain ke clinet postgresaql ka connect hai optional hai lakin phir bhi
    if client.is_closed() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database connection is closed".to_string(),
        ));
    }

    // Validate karte hain ke data pura hai ya nhi
    if let Err(e) = data.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Validation failed: {:?}", e.field_errors()),
        ));
    }

    // ab validate karte hain ke email ka format theek hai ya nhi
    // kyon ke validator macro jo hai wo email ko ko us level ka validate nhi karta hai
    if !is_strict_email(&data.email) {
        return Err((StatusCode::BAD_REQUEST, "Invalid email format".to_string()));
    }
 // ab ham redis se data lain ge jo chech  kiya tha
    let stored_json: Option<String> = match redis_conn.get(&data.email).await {
        Ok(val) => val,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Redis error: {}", e),
            ));
        }
    };


// ager data milta hai to usay parse karein ge
    let data2 = match stored_json {
        Some(json_str) => {
            let parsed_data = match serde_json::from_str::<RedisUser>(&json_str) {
                Ok(data) => data,
                Err(e) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to parse JSON: {}", e),
                    ));
                }
            };
     
            // ab check karte hain ke code match kar rha hai ya nhi
            if &parsed_data.code == &data.code {
                let _: () = redis_conn.del(&data.email).await.unwrap_or_default();
                parsed_data
            } else {
                return Err((StatusCode::UNAUTHORIZED, "Invalid code".to_string()));
            }
        }
        None => return Err((StatusCode::UNAUTHORIZED, "Code not found".to_string())),
    };
// ab ham JWT create karein ge
    match create_jwt(data2.id.to_string()).await {
//AB HAM isse retun karein ge 
        Ok(token) => Ok(MsgPack(Tokken { token })),
        Err(e) => {
            eprintln!("JWT creation failed: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "JWT creation failed".to_string(),
            ))
        }
    }
}
