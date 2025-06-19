
use tauri_plugin_http::reqwest; // make sure this plugin is enabled in main.rs
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

#[derive(Debug,Serialize,Deserialize)]
pub struct Signup{
    name:String,
    age:i32,
    gender:String,
    email:String,
    password:String,
  
}


#[tauri::command]
pub async fn auth_signup(data:Signup)->Result<String,String>{
    dotenv().ok();
    
    let server_url = env::var("BACKEND_URL").map_err(|_| "BACKEND_URL missing".to_string())?;
    let endpoint = format!("{}/signup", server_url);

     let client = reqwest::Client::new();
    let res = client
        .post(&endpoint)
        .header("Content-Type", "application/msgpack")
        .body(rmp_serde::to_vec_named(&data).map_err(|e| e.to_string())?)
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;



   

 match res.status().as_u16() {
    200 => Ok("Verification OK, code sent to email".to_string()),
    400 => Ok("Invalid email format".to_string()),
    409 => Ok("User already exists".to_string()),
    500 => Ok("Internal server error, please try again later".to_string()),
    _ => Ok("Unknown error occurred".to_string()),
}

}