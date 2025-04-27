use std::{env, sync::Arc};

use anyhow::Result;
use commands::{command_processor, Command};
use tokio::sync::mpsc::{self, Sender};
use tracing::{error, level_filters::LevelFilter};
use veno_core::app::AppState;

use clap::Parser;

mod commands;
mod resources;
mod server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    config: String,

    #[arg(short, long("log-level"))]
    log_level: Option<String>,
}

struct App {
    config: AppState,
    command_tx: Sender<Command>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    init_tracing_subscriber(&cli.log_level);
    let config = AppState::init(&cli.config)?;

    let (command_tx, command_rx) = mpsc::channel(100);
    tokio::spawn(command_processor(command_rx));
    let app = Arc::new(App {
        config: config.clone(),
        command_tx,
    });

    server::start(app).await?;
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
        .compact()
        .with_max_level(log_level)
        .with_thread_ids(true)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_target(true)
        .init();
}

fn parse_level_str(level_str: &str, source_description: &str) -> LevelFilter {
    match level_str.parse::<LevelFilter>() {
        Ok(level) => level,
        Err(_) => {
            error!(
                "WARN: Invalid value for {}. Valid log levels are 'trace', 'debug', 'info', 'warn', 'error'. Found: '{}'. Defaulting to 'info'",
                source_description, level_str
            );
            LevelFilter::INFO
        }
    }
}
