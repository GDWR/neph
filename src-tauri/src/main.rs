#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]


use std::sync::Mutex;

mod commands;


pub struct Settings {
    current_namespace: Mutex<String>,
}

impl Default for Settings {
    fn default() -> Self {
        return Settings {
            current_namespace: Mutex::new("*".into()),
        };
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Settings::default())
        .invoke_handler(tauri::generate_handler![
            commands::get_all_nodes,
            commands::get_all_pods,
            commands::get_pod,
            commands::get_current_namespace,
            commands::set_current_namespace,
            commands::get_all_namespaces,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
