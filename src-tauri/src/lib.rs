// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod db;
mod models;
mod commands;

use db::initialize_database;
use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if let Err(e) = initialize_database(app.handle()) {
                eprintln!("Failed to initialize database: {}", e);
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_firmen, create_firma, update_firma, delete_firma,
            get_ansprechspersonen, create_ansprechsperson, update_ansprechsperson, delete_ansprechsperson,
            get_kontakte, create_kontakt, update_kontakt, delete_kontakt
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
