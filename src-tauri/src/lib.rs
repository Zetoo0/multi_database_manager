use domain::service::{parser_service, sys_info_service, system_service, user_settings_service};
use serde::{Deserialize, Serialize};
use tauri::Manager;
pub mod domain;
use crate::domain::commands::service_commands;
use crate::domain::service::database_service::DatabaseService;
use dashmap::DashMap;
use std::sync::Arc;
use tauri::Wry;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs;

#[tauri::command]
fn open_file(path: String) -> String {
    println!("Opening file: {}", path);
    return "File opened".to_string();
}

#[tauri::command]
async fn file_select(window: tauri::Window<Wry>) -> String {
    let f_path = window.dialog().file().pick_file(|path| {
        println!("Selected file: {}", path.unwrap());
    });

    return "".to_string();
}

#[derive(Default)]
pub struct AppData {
    pub service: DashMap<String, Arc<dyn DatabaseService>>, //DashMap<String,Arc<dyn DatabaseService>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseConnection {
    port: String,
    server: String,
    username: String,
    password: String,
    driver_type: String,
}

//service_commands::connect,service_commands::migrate_to,service_commands::query,service_commands::get_metadatas,service_commands::create_table, service_commands::add_column,service_commands::get_table

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_fs::init())
        .manage(tokio::sync::Mutex::new(AppData::default()))
        .invoke_handler(tauri::generate_handler![
            system_service::get_vram_usage,
            system_service::get_os_information,
            system_service::get_os_types,
            user_settings_service::get_user_settings,
            user_settings_service::get_default_user_settings,
            user_settings_service::save_user_settings,
            system_service::get_system_fonts,
            //parser_service::get_parsed_rtf,
            //parser_service::get_parsed_md,
            parser_service::get_parsed_docx,
            service_commands::connect,
            service_commands::migrate_to,
            service_commands::query,
            service_commands::get_metadatas,
            service_commands::create_table,
            service_commands::create_role,
            service_commands::create_sequence,
            service_commands::create_function,
            service_commands::create_schema,
            service_commands::create_index,
            service_commands::create_role,
            service_commands::create_database,
            service_commands::create_view,
            service_commands::create_trigger,
            service_commands::create_constraint,
            service_commands::add_column,
            service_commands::get_table,
            service_commands::edit_table_column,
            service_commands::base_delete,
            service_commands::delete_table_column,
            service_commands::edit_sequence,
            service_commands::edit_constraint,
            service_commands::edit_index,
            service_commands::edit_function,
            service_commands::edit_view,
            open_file,
            file_select
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            sys_info_service::save_sys_info(window);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
