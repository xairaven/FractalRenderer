// Project lints
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
use crate::config::AppConfig;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app_config = AppConfig::from_env().unwrap_or_else(|err| {
        println!("Error: {err}");
        std::process::exit(1);
    });

    logging::init(
        app_config.log_level,
        &logging::generate_log_name(&app_config.name),
    )
    .unwrap_or_else(|err| {
        println!("Logger initialization failed. Error: {}", err);
        std::process::exit(1);
    });

    ui::start(app_config.name, app_config.theme).unwrap_or_else(|err| {
        log::error!("{}", err);
        std::process::exit(1);
    });
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    ui::start();
}

mod app;
mod context;
mod fractals;
mod graphics {
    pub mod grid;
}
mod geometry {
    pub mod dot;
    pub mod line2d;
    pub mod point2d;
}
mod io {
    pub mod filter;
    pub mod operations;
    pub mod screenshot;
}
mod math {
    pub mod angle;
}
mod ui;

#[cfg(not(target_arch = "wasm32"))]
mod config;
#[cfg(not(target_arch = "wasm32"))]
mod logging;
