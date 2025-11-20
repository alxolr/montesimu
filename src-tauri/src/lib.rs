use shaku::HasComponent;
use tauri::{Manager, State};

use crate::commands::histogram;
use crate::services::{greet::GreetService, Container};

#[tauri::command]
fn greet(state: State<'_, Container>, name: &str) -> String {
    let greet_service: &dyn GreetService = state.resolve_ref();

    greet_service.greet(name)
}

pub mod commands;
pub mod dtos;
pub mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let container = Container::builder().build();

    tauri::Builder::default()
        .setup(move |app| {
            app.manage(container);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            histogram::generate_histogram
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
