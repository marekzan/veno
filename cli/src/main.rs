use neveno_core::config::AppConfig;

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

    let new_versions = config.check_artifacts().await?;
    println!("{}", new_versions);

    for artifact in &config.artifacts {
        if let Some(latest_version) = artifact.check_version().await? {
            artifact.send(&latest_version).await?;
        };
    }
    Ok(())
}
