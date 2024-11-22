use clap::Parser;
use config::AppConfig;
use endpoints::routes;

mod checker;
mod config;
mod endpoints;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = AppConfig::load(&cli.config)?;
    routes(config.clone()).await;
    Ok(())
}
