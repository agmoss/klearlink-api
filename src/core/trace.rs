
use std::io::Error;

use tracing_subscriber::EnvFilter;



pub fn init_tracing() -> Result<(), Error> {
    tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env()) 
    .with_target(true)
    .init();

    Ok(())
}