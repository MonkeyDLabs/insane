#![allow(non_camel_case_types)]

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::env;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use crate::error::Result;
use crate::config::TraceConfig;

#[derive(Copy, Clone, Debug, ValueEnum, Serialize, Deserialize)]
pub enum TraceFormat {
    standard,
    json,
    pretty,
}

impl Default for TraceFormat {
    fn default() -> Self {
        TraceFormat::standard
    }
}

pub fn init(config: &TraceConfig) -> Result<()> {
    let mut filter = EnvFilter::new(config.filter());

    // because tokio_util is too verbose
    filter = filter
        .add_directive("tokio_util=info".parse().expect("Failed to parse filter directive"));


    if let Ok(rust_log) = env::var(EnvFilter::DEFAULT_ENV) {
        filter = filter.add_directive(rust_log.parse()?);
    }

    match config.format() {
        TraceFormat::standard => {
            tracing_subscriber::registry()
                .with(fmt::layer().with_ansi(atty::is(atty::Stream::Stdout)))
                .with(filter)
                .init();
        }
        TraceFormat::json => {
            tracing_subscriber::registry()
                .with(fmt::layer().json().flatten_event(true))
                .with(filter)
                .init();
        }
        TraceFormat::pretty => {
            tracing_subscriber::registry()
                .with(fmt::layer().pretty().with_ansi(atty::is(atty::Stream::Stdout)))
                .with(filter)
                .init();
        }
    };

    Ok(())
}
