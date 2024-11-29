use neveno_core::{
    sink::{create_custom_message, create_default_message, SinkNotifier},
    source::checker,
};
use std::collections::HashMap;

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

    let notifiers: HashMap<String, Box<dyn SinkNotifier<Output = String>>> = config
        .notifiers
        .into_iter()
        .map(|notifier| (notifier.name, notifier.sink.to_notifier()))
        .collect();

    for artifact in &config.artifacts {
        if let Some(latest_version) = checker::check(artifact).await? {
            for notifier_name in &artifact.notifier {
                if let Some(notifier) = notifiers.get(notifier_name) {
                    let response = match &artifact.message_prefix {
                        Some(prefix) => {
                            let message =
                                create_custom_message(prefix, &artifact.name, &latest_version);
                            notifier.send(&message).await
                        }
                        None => {
                            let message = create_default_message(&artifact.name, &latest_version);
                            notifier.send(&message).await
                        }
                    };
                    println!("Notifier response: {:?}", response);
                }
            }
        }
    }
    Ok(())
}
