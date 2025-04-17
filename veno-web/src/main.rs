use std::sync::Arc;

use anyhow::Result;
use tracing::info;
use veno_core::app::AppState;

use clap::Parser;

mod resources;
mod server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let app = Arc::new(AppState::init(&cli.config)?);
    info!("Loaded app config");
    server::run(app).await;
    Ok(())
}
