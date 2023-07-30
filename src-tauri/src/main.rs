// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn show_in_folder(path: &str) {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "START", "", path])
            .spawn()
            .unwrap();
    }
}

#[tauri::command]
fn show_in_code_editor(path: &str) {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "CODE", "", path])
            .spawn()
            .unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![show_in_folder, show_in_code_editor])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
