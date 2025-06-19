use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use axum_msgpack::MsgPack;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::AppState;
use crate::auth::handler::sign_up::SignUp;
use crate::auth::utils::create_jwt::create_jwt;
use crate::auth::utils::helper::is_strict_email;
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SignupHandleInsert {
    #[validate(email(message = "Email format is invalid"))]
    email: String,
    code: String,
}

pub async fn signup_handle_insert(
    State(appstate): State<Arc<AppState>>,
    MsgPack(data): MsgPack<SignupHandleInsert>,
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
            let parsed_data = match serde_json::from_str::<SignUp>(&json_str) {
                Ok(data) => data,
                Err(e) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to parse JSON: {}", e),
                    ));
                }
            };

            // ab check karte hain ke code match kar rha hai ya nhi 
            match parsed_data.code.clone() {
                Some(stored_code) if stored_code == data.code => {
                    let _: () = redis_conn.del(&data.email).await.unwrap_or_default();
                    parsed_data
                }
                Some(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid code".to_string())),
                None => return Err((StatusCode::UNAUTHORIZED, "Code missing".to_string())),
            }
        }
        None => return Err((StatusCode::UNAUTHORIZED, "Code not found".to_string())),
    };

    // ab ham user ko db main insert karein ge 
    match client
        .execute(
            "INSERT INTO users(name,age,gender,email,password) VALUES ($1,$2,$3,$4,$5)",
            &[
                &data2.name,
                &data2.age,
                &data2.gender,
                &data2.email,
                &data2.password,
            ],
        )
        .await
    {
        Ok(_) => {
            // ab ham user ki id ko fatch karein ge take jwt bana sakein 
            let fetch_result = client
                .query_opt("SELECT id FROM users WHERE email = $1", &[&data.email])
                .await;

            let id = match fetch_result {
                Ok(Some(row)) => row.get::<_, i32>("id"),
                Ok(None) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "User not found after insertion".to_string(),
                    ));
                }
                Err(e) => {
                    eprintln!("DB error (fetch ID): {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Database fetch error".to_string(),
                    ));
                }
            };

            // ab ham jwt banyein ge 
            match create_jwt(id.to_string()).await {

                 // ager jwt ban jata hai toh usse return karein ge 
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
        Err(e) => {
            eprintln!("database error {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database Error Insert".to_string(),
            ))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tokken {
    pub token: String,
}
