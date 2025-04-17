use std::sync::Arc;

use anyhow::Result;
use resources::serve_api;
use veno_core::app::AppState;

use clap::Parser;

mod resources;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let app = Arc::new(AppState::init(&cli.config)?);
    serve_api(app).await;
    Ok(())
}
