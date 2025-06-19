// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

mod api;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            api::auth_api_signup::auth_signup,
            api::auth_api_login::auth_login,
            api::auth_api_verify_email::auth_verify_email,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// #[tauri::command]
// async fn auth_msg() -> Result<String, String> {
//     let response = tauri_plugin_http::reqwest::get("https://fakestoreapi.com/products/1")
//         .await
//         .map_err(|e| e.to_string())?;

//     let body = response.text().await.map_err(|e| e.to_string())?;

//     Ok(body)
// }
