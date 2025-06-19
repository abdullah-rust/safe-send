use crate::{
    AppState,
    auth::utils::{
        helper::{generate_verification_code, is_strict_email},
        verify_email::{EmailError, send_verification},
    },
};
use axum::{extract::State, http::StatusCode};
use axum_msgpack::MsgPack;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

// yeh strcut hai for SignUp
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SignUp {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub name: String,

    #[validate(range(min = 13, max = 120, message = "Age must be between 13 and 120"))]
    pub age: i32,

    #[validate(length(min = 1, message = "Gender is required"))]
    pub gender: String,

    #[validate(email(message = "Email format is invalid"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    pub code: Option<String>,
}

// yeh signup handler hai jo run hoga jahan se code ko call kiya jai ga
#[axum::debug_handler]
pub async fn sign_up(
    State(appstate): State<Arc<AppState>>,
    MsgPack(data): MsgPack<SignUp>,
) -> Result<MsgPack<String>, (StatusCode, String)> {
    let client = appstate.db.clone();
    let redis = appstate.redis.clone();

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

    // check karte hain ke user alrady hai to nhi
    let user_exists = client
        .query_opt("SELECT 1 FROM users WHERE email = $1", &[&data.email])
        .await;

    match user_exists {
        Ok(Some(_)) => {
            return Err((StatusCode::CONFLICT, "User already exists".to_string()));
        }
        Err(e) => {
            eprintln!("DB error (check exist): {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ));
        }
        _ => {}
    }

    // Hash the password using bcrypt
    let hashed_password = match bcrypt::hash(&data.password, 12) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Password hashing error: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Password hashing failed".to_string(),
            ));
        }
    };

    // genrate a verfication code
    let code = generate_verification_code().await;

    // send verification to user email
    match send_verification(&data.email, &code).await {
        Ok(_) => {
            // create final data jo ham ne redis main store karna hai
            let data_final = SignUp {
                name: data.name,
                age: data.age,
                gender: data.gender,
                email: data.email,
                password: hashed_password,
                code: Some(code.clone()),
            };

            let key = data_final.email.clone();
            let json_data = serde_json::to_string(&data_final).unwrap();
            let mut redis_conn = redis.lock().await;

            // ab ham redis main data store karte hain
            match redis_conn.set_ex::<_, _, ()>(key, json_data, 120).await {
                Ok(_) => Ok(MsgPack("Verification email sent successfully".to_string())),
                Err(e) => {
                    eprintln!("{}", e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "check data Error".to_string(),
                    ))
                }
            }
        }

        // emails send kane main jo erros aayein ge unhein handle karte hain
        Err(e) => match e {
            EmailError::EmailFormatError => {
                Err((StatusCode::BAD_REQUEST, "Email Format Error".to_string()))
            }
            EmailError::InvalidEmail => {
                Err((StatusCode::BAD_REQUEST, "Invalid Email Address".to_string()))
            }
            EmailError::SmtpError(msg) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("SMTP Error: {}", msg),
            )),
        },
    }
}
