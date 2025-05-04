// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use log::info;

mod config;

fn main() {
    config::load_config();
    info!("Starting app");
    writepad_lib::run()
}
