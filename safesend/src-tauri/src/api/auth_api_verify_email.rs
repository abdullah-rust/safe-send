use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use tauri_plugin_http::reqwest;

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    email: String,
    code: String,
}

#[derive(Debug, serde::Deserialize)]
struct Tokken {
    token: String,
}

#[tauri::command]
pub async fn auth_verify_email(data: State, types: String) -> Result<String, String> {
    dotenv().ok();

    let server_url = env::var("BACKEND_URL").map_err(|_| "BACKEND_URL missing".to_string())?;
    let endpoint = if types == "login" {
        format!("{}/login/code", server_url)
    } else {
        format!("{}/signup/code", server_url)
    };
    let client = reqwest::Client::new();
    let res = client
        .post(&endpoint)
        .header("Content-Type", "application/msgpack")
        .body(rmp_serde::to_vec_named(&data).map_err(|e| e.to_string())?)
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    let status_code = res.status();
    // 🔥 Yeh line JWT extract karegi from msgpack binary:
    let bytes = res.bytes().await.map_err(|e| e.to_string())?;
   match status_code.as_u16() {
    200 => {
        let token: Tokken = rmp_serde::from_slice(&bytes)
            .map_err(|e| format!("Deserialization error: {}", e))?;

        Ok(token.token)
    }
    400 => Ok("Invalid email format".to_string()),
    401 => Ok("Code invalid".to_string()),
    500 => Ok("Internal server error, please try again later".to_string()),
    _ => Ok("Unknown error".to_string()),
}
}
