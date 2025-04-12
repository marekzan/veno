use anyhow::Result;
use veno_core::app::AppState;

use clap::Parser;
use endpoints::routes;

mod endpoints;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = AppState::init(&cli.config)?;
    routes(config.clone()).await;
    Ok(())
}
