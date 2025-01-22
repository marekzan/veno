use veno_core::{config::AppConfig, pretty_json};

use anyhow::Result;
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
    let config = AppConfig::load(&cli.config)?;

    // let new_versions = config.check_artifacts().await?;
    // println!("{}", pretty_json(&new_versions)?);

    for artifact in &config.artifacts {
        artifact.notify_on_latest_version().await?;
    }
    Ok(())
}
