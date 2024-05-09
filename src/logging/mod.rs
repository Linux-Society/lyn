use std::io::{self};
use tracing::{info, warn};
use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::fmt::format::DefaultFields;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::config::CONFIG;

mod result_split;
use result_split::SplitIntoOptions;

pub fn setup_logging() {
    // Standard error logging layer
    let std_layer = fmt::layer().with_writer(io::stderr);

    // Write to log file layer
    let log_layer = file_layer();
    let (log_ok, log_err) = log_layer.split();

    // Setup registry for tracing subscriber into all output forms.
    let registry = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(std_layer)
        .with(log_ok);

    // Add journald to tracing
    match tracing_journald::layer() {
        Ok(layer) => {
            // Add journald layer to registry.
            registry.with(layer).init();
        }
        Err(error) => {
            // handle error gracefully if it happened
            registry.init();
            warn!("Unable to connect to journald: {}", error);
        }
    }

    // Print log file error if there was one
    match log_err {
        Some(error) => {
            warn!("Unable to setup file logging: {}", error);
        }
        _ => {}
    }

    info!("Logging successfully initialised");
}

pub fn file_layer<T>() -> anyhow::Result<
    tracing_subscriber::fmt::Layer<
        T,
        DefaultFields,
        tracing_subscriber::fmt::format::Format,
        NonBlocking,
    >,
> {
    // Shell expand the logging string, allowing for use of $SOMETHING & ~
    let logging_dir = shellexpand::full(&CONFIG.log_dir)?;

    // Setup file logging
    let file_appender = tracing_appender::rolling::hourly(logging_dir.as_ref(), &CONFIG.log_file);
    let (log_out, _guard) = tracing_appender::non_blocking(file_appender);

    Ok(fmt::layer().with_writer(log_out))
}
