use std::process::Command;

#[tauri::command]
pub fn show_in_folder(path: &str) {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "START", "", path])
            .spawn()
            .unwrap();
    }
}

#[tauri::command]
pub fn show_in_code_editor(path: &str) {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "CODE", "", path])
            .spawn()
            .unwrap();
    }
}