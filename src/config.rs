use dotenvy::dotenv;
use egui::ThemePreference;
use log::LevelFilter;
use std::env;
use std::str::FromStr;
use thiserror::Error;

pub struct AppConfig {
    pub name: String,
    pub log_level: LevelFilter,
    pub theme: ThemePreference,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, DotenvError> {
        // Loading .env from crate folder.
        if dotenv().is_err() {
            return Err(DotenvError::ConfigNotLoaded);
        }

        // Loading crate name (used in window title)
        let name = env::var("CRATE_NAME").map_err(|_| DotenvError::NameNotLoaded)?;

        // Loading log level
        let log_level =
            env::var("LOG_LEVEL").map_err(|_| DotenvError::LogLevelNotLoaded)?;
        let log_level = LevelFilter::from_str(&log_level)
            .map_err(|_| DotenvError::LogLevelUndefined)?;

        // Loading theme
        let theme = env::var("THEME").map_err(|_| DotenvError::ThemeNotLoaded)?;
        let theme = match theme.to_lowercase().as_str() {
            "dark" => ThemePreference::Dark,
            "light" => ThemePreference::Light,
            _ => return Err(DotenvError::ThemeUndefined),
        };

        Ok(Self {
            name,
            log_level,
            theme,
        })
    }
}

#[derive(Error, Debug)]
pub enum DotenvError {
    #[error("Config is not loaded.")]
    ConfigNotLoaded,

    #[error("Log level is not loaded.")]
    LogLevelNotLoaded,

    #[error("Log level is undefined.")]
    LogLevelUndefined,

    #[error("Crate name is not loaded.")]
    NameNotLoaded,

    #[error("Theme is not loaded.")]
    ThemeNotLoaded,

    #[error("Theme is undefined.")]
    ThemeUndefined,
}
