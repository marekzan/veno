use neveno_core::sink::SinkNotifier;
use std::collections::HashMap;

use neveno_core::{checker, config::AppConfig};

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

    let notifiers: HashMap<String, Box<dyn SinkNotifier<Output = String>>> = config
        .notifiers
        .into_iter()
        .map(|notifier| (notifier.name, notifier.sink.to_notifier()))
        .collect();

    for artifact in &config.artifacts {
        if let Some(latest_version) = checker::check(artifact).await? {
            for notifier_name in &artifact.notifier {
                if let Some(notifier) = notifiers.get(notifier_name) {
                    let response = notifier.send(&latest_version).await;
                    println!("Notifier response: {:?}", response);
                }
            }
        }
    }
    Ok(())
}
