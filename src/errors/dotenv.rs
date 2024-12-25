use thiserror::Error;

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
