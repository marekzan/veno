use std::{env, sync::Arc};

use anyhow::Result;
use tracing::{debug, error, info, level_filters::LevelFilter, trace, warn};
use veno_core::app::AppState;

use clap::Parser;

mod resources;
mod server;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(short, long)]
    config: String,

    #[arg(short, long("log-level"))]
    log_level: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    init_tracing_subscriber(&cli.log_level);

    let app = Arc::new(AppState::init(&cli.config)?);
    trace!("this is a trace");
    debug!("this is a debug");
    info!("this is an info");
    warn!("this is a warn");
    error!("this is an error");
    server::run(app).await;
    Ok(())
}

fn init_tracing_subscriber(log_level: &Option<String>) {
    let log_level = match log_level {
        Some(level) => parse_level_str(level, "'log-level' argument"),
        None => match env::var("VENO_LOG_LEVEL") {
            Ok(level) => parse_level_str(&level, "'VENO_LOG_LEVEL' environment variable"),
            Err(_) => LevelFilter::INFO,
        },
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .init();
}

fn parse_level_str(level_str: &str, source_description: &str) -> LevelFilter {
    match level_str.parse::<LevelFilter>() {
        Ok(level) => level,
        Err(_) => {
            eprintln!(
                "WARN: Invalid value for {}. Valid log levels are 'trace', 'debug', 'info', 'warn', 'error'. Found: '{}'. Defaulting to 'info'",
                source_description, level_str
            );
            LevelFilter::INFO
        }
    }
}
