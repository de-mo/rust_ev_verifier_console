use rust_verifier::Config as VerifierConfig;
use std::fs::File;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*, EnvFilter, Layer};

/// Init the subscriber with or without stdout
pub fn init_subscriber(config: &'static VerifierConfig) -> Vec<WorkerGuard> {
    let log_file = File::options()
        .create(true)
        .append(true)
        .open(config.log_file_path())
        .unwrap();

    let span_events = FmtSpan::NEW | FmtSpan::CLOSE;

    let (mk_writer_output, guard_output) = tracing_appender::non_blocking(std::io::stdout());
    let (mk_writer_file, guard_file) = tracing_appender::non_blocking(log_file);

    let layer_output = tracing_subscriber::fmt::layer()
        .with_span_events(span_events.clone())
        .with_writer(mk_writer_output)
        .with_filter(EnvFilter::from_default_env());
    let layer_file = tracing_subscriber::fmt::layer()
        .with_span_events(span_events)
        .with_writer(mk_writer_file)
        .with_filter(EnvFilter::from_default_env());

    let subscriber = tracing_subscriber::registry()
        .with(layer_output)
        .with(layer_file);

    tracing::subscriber::set_global_default(subscriber).unwrap();
    vec![guard_output, guard_file]
}
