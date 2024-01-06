// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use env_logger::Env;
use log::{error, info};
use std::net::TcpListener;

#[tauri::command]
async fn is_port_taken(port: u16) {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => {
            info!("Port {} is free.", port);
        }
        Err(_) => {
            error!("Port {} is taken.", port);
            std::process::exit(1);
        }
    }
}

fn setup_logger() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .filter_level(log::LevelFilter::Debug)
        .init();
}

fn main() {
    ctrlc::set_handler(|| {
        std::process::exit(0);
    })
    .expect("Error setting Ctrl+C handler");

    setup_logger();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![is_port_taken])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
