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

    let notifiers: HashMap<String, Box<dyn SinkNotifier>> = config
        .notifiers
        .into_iter()
        .map(|notifier| (notifier.name, notifier.sink.to_notifier()))
        .collect();

    for app in &config.applications {
        if let Some(latest_version) = checker::check(app).await? {
            app.notifier
                .iter()
                .try_for_each(|notifier_name| -> Result<()> {
                    if let Some(notifier) = notifiers.get(notifier_name) {
                        println!("Sending notification to {}", notifier_name);
                        notifier.send(&latest_version)?;
                    }
                    Ok(())
                })?;
        }
    }
    Ok(())
}
