// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api; 

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api::show_in_folder, api::show_in_code_editor])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
