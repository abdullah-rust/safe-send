use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::AppState;
use crate::auth::utils::helper::{generate_verification_code, is_strict_email};
use crate::auth::utils::verify_email::{EmailError, send_verification};
use axum_msgpack::MsgPack;
use redis::AsyncCommands;

// yeh struct hai jo ham ne use karna hai
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Login {
    #[validate(email(message = "Email format is invalid"))]
    email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisUser {
    pub id: i32,
    pub email: String,
    pub code: String,
}

//  yeh main function hai jahan se kaam hoga
pub async fn log_in(
    State(appstate): State<Arc<AppState>>,
    MsgPack(data): MsgPack<Login>,
) -> Result<MsgPack<String>, (StatusCode, String)> {
    let client = appstate.db.clone();
    let redis = appstate.redis.clone();

    // is se ham check karein ge ke connection establish hai ya nhi
    if client.is_closed() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database connection is closed".to_string(),
        ));
    }

    // is se validate karein ge ham ke data pora hai
    if let Err(e) = data.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Validation failed: {:?}", e.field_errors()),
        ));
    }

    // yeh aik extara leyer take email ache se check karein ham
    if !is_strict_email(&data.email) {
        return Err((StatusCode::BAD_REQUEST, "Invalid email format".to_string()));
    }

    // yahan se ham query chalayein ge ke user hai ya nhi db main
    match client
        .query_opt(
            "SELECT id,email,password FROM users WHERE email=$1",
            &[&data.email],
        )
        .await
    {
        Ok(Some(row)) => {
            let (id, email, password_hash): (i32, String, String) =
                (row.get("id"), row.get("email"), row.get("password"));
            let check = bcrypt::verify(&data.password, &password_hash.as_str());

            // yahan ham check karein ke user ka password match ho gaya hai theek se ya nhi
            match check {
                Ok(true) => {
                    let code = generate_verification_code().await;
                    let final_data = RedisUser {
                        id,
                        email: email.clone(),
                        code: code.clone(),
                    };

                    let key = final_data.email.clone();
                    let json_data = serde_json::to_string(&final_data).unwrap();
                    let mut redis_conn = redis.lock().await;
                    // yahan pe ham email par verification send karein ge
                    match send_verification(&final_data.email.as_str(), &code.as_str()).await {
                        // our yahan se ham redis main send karein ge taker user ke data ko wahan se le liye jai
                        Ok(_) => match redis_conn.set_ex::<_, _, ()>(key, json_data, 120).await {
                            Ok(_) => {
                                //  our yeh hamara end main jo ham ne fronted ko send karein ge final ok
                                Ok(MsgPack("Verification email sent successfully".to_string()))
                            }
                            Err(e) => {
                                eprintln!("{}", e);
                                Err((
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    "cheche data Error".to_string(),
                                ))
                            }
                        },
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
                Ok(false) => {
                    return Err((StatusCode::UNAUTHORIZED, "inavlid password".to_string()));
                }
                Err(e) => {
                    eprintln!("Bcrypt error: {}", e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Password verification failed".to_string(),
                    ))
                }
            }
        }
        Ok(None) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid email or password".to_string(),
            ));
        }
        Err(e) => {
            eprintln!("Database query error: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "databse query error".to_string(),
            ));
        }
    }
}
