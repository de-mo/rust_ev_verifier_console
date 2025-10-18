// Copyright © 2025 Denis Morel
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License and
// a copy of the GNU General Public License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

use rust_ev_verifier_lib::VerifierConfig;
use std::fs::File;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, Layer, fmt::format::FmtSpan, prelude::*};

/// Init the subscriber with or without stdout
pub fn init_subscriber(config: &'static VerifierConfig) -> Vec<WorkerGuard> {
    // Get the logile
    let log_file = File::options()
        .create(true)
        .append(true)
        .open(config.log_file_path())
        .unwrap();

    // Define which span evens will be logged (new and clode)
    let span_events = FmtSpan::NEW | FmtSpan::CLOSE;

    // Define the writers for output and file, using non_blocking
    let (mk_writer_output, guard_output) = tracing_appender::non_blocking(std::io::stdout());
    let (mk_writer_file, guard_file) = tracing_appender::non_blocking(log_file);

    // Define the layer for output
    let layer_output = tracing_subscriber::fmt::layer()
        .with_span_events(span_events.clone())
        .with_writer(mk_writer_output)
        .with_filter(EnvFilter::from_default_env());

    // Define the layer for file
    // USe the EnvFilter to read the value "RUST_LOG" in .env
    let layer_file = tracing_subscriber::fmt::layer()
        .with_span_events(span_events)
        .with_writer(mk_writer_file)
        .with_filter(EnvFilter::from_default_env());

    // Combine the layers in a subcriber
    // USe the EnvFilter to read the value "RUST_LOG" in .env
    let subscriber = tracing_subscriber::registry()
        .with(layer_output)
        .with(layer_file);

    // Set the subscriber as global
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Return the guards, in order to ensure that the logs will be written
    // See https://docs.rs/tracing-appender/latest/tracing_appender/non_blocking/struct.WorkerGuard.html
    vec![guard_output, guard_file]
}
