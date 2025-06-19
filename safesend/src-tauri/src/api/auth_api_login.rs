use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;
use std::env;
use dotenv::dotenv;

#[derive(Debug,Deserialize,Serialize)]
pub struct Login{
    email:String,
    password:String
}


#[tauri::command]
pub async  fn auth_login(data:Login)->Result<String,String>{
      dotenv().ok();
    
    let server_url = env::var("BACKEND_URL").map_err(|_| "BACKEND_URL missing".to_string())?;
    let endpoint = format!("{}/login", server_url);

    let client=reqwest::Client::new();
    let res = client.post(&endpoint).header("Content-Type", "application/msgpack").body(rmp_serde::to_vec_named(&data).map_err(|e| e.to_string())?).send().await.map_err(|e| format!("Request error {}",e));

       let status_code=res?.status();


match status_code.as_u16() {
    200 => Ok("Verification OK, code sent to email".to_string()),
    400 => Ok("Invalid email format".to_string()),
    401 => Ok("Wrong email or password".to_string()),
    500 => Ok("Internal server error, please try again later".to_string()),
    _ => Ok("Unknown error occurred".to_string()),
}

}