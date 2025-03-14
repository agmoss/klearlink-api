use std::io::Error;

use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_stdout as stdout;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;

pub fn init_tracing_1() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .json()
        .init();

    Ok(())
}

pub fn init_tracing_2() {
    // Create a new OpenTelemetry trace pipeline that prints to stdout
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(stdout::SpanExporter::default())
        .build();
    let tracer = provider.tracer("klearlink_api");

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let _ = tracing_subscriber::registry()
        .with(fmt::layer())
        .with(telemetry)
        .try_init();
}
