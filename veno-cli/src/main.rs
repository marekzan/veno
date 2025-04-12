use anyhow::Result;
use veno_core::app::AppState;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let app = AppState::init(&cli.config)?;

    app.notify().await;

    Ok(())
}
