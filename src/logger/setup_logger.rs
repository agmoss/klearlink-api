use chrono::Local;
use fern::Dispatch;
use log::{error, info, warn};
use rocket::serde::json::Json;

pub fn setup_logger() -> Result<(), fern::InitError> {
    Dispatch::new()
        // Set the minimum log level
        .level(log::LevelFilter::Info)
        // Filter logs for specific modules, e.g., Rocket
        .level_for("rocket", log::LevelFilter::Info)
        .level_for("my_app", log::LevelFilter::Debug)
        // Format the output
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        // Output to stdout
        .chain(std::io::stdout())
        // Optionally output to a file
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
