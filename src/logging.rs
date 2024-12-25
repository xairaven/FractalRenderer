use crate::errors::logging::LogError;
use chrono::{Datelike, Local};
use log::LevelFilter;

pub fn init(log_level: LevelFilter, file_name: &str) -> Result<(), LogError> {
    let mut dispatcher = fern::Dispatch::new().level(log_level);

    if log_level != LevelFilter::Off {
        let file =
            fern::log_file(file_name).map_err(|_| LogError::CannotAccessLogFile)?;

        dispatcher = dispatcher
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    Local::now().format("%Y-%m-%d %H:%M"),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .chain(fern::Output::file(file, "\r\n"));
    }

    dispatcher.apply().map_err(LogError::SetLoggerError)
}

pub fn generate_log_name(crate_name: &str) -> String {
    let now = Local::now();
    let date = format!(
        "{year:04}-{month:02}-{day:02}",
        year = now.year(),
        month = now.month(),
        day = now.day(),
    );

    let crate_name_formatted = crate_name.replace(" ", "-");
    format!("{crate_name_formatted}_{date}.log")
}
