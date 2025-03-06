use chrono::Local;
use fern::Dispatch;
use std::env;

pub fn setup_logger() -> Result<(), fern::InitError> {
    let mut dispatch = Dispatch::new()
        // Set the minimum log level
        .level(log::LevelFilter::Info)
        // Filter logs for specific modules, e.g., Rocket
        .level_for("rocket", log::LevelFilter::Info)
        .level_for("klearlink-api", log::LevelFilter::Debug)
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
        .chain(std::io::stdout());

    // Check if the ROCKET_ENV is set to "development"
    if env::var("ROCKET_ENV").unwrap_or_default() == "development" {
        dispatch = dispatch.chain(fern::log_file("output.log")?);
    }

    dispatch.apply()?;
    Ok(())
}
