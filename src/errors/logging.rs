use log::SetLoggerError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LogError {
    #[error("Cannot access log file.")]
    CannotAccessLogFile,

    #[error("Logger initialization error.")]
    SetLoggerError(SetLoggerError),
}
