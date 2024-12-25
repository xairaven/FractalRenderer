// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::AppConfig;

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

mod app;

mod config;
mod context;
mod graphics {
    pub mod grid;
}
mod geometry {
    pub mod line2d;
    pub mod point2d;
}
mod logging;
mod math {
    pub mod angle;
}
mod ui;
