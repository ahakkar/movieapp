#![allow(dead_code)] 
// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]

mod constants;
mod db;
pub mod view;
pub mod app;
use app::MovieApp;
use db::connection::establish_connection;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let sql_conn = establish_connection();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(constants::WINDOW_SIZE)
            .with_min_inner_size(constants::WINDOW_MIN_SIZE),
        ..Default::default()
    };
    eframe::run_native(
        constants::APP_TITLE,
        native_options,
        Box::new(|cc| Ok(Box::new(MovieApp::new(cc, sql_conn)))),
    )
}