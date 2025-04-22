use artifact_commands::ArtifactCommand;
use artifact_handlers::handle_get_all_artifacts;
use notifier_commands::NotifierCommand;
use tokio::sync::mpsc::Receiver;
use tracing::debug;

pub mod artifact_commands;
pub mod artifact_handlers;
pub mod notifier_commands;
pub mod notifier_handlers;

#[derive(Debug)]
pub enum Command {
    Artifact(ArtifactCommand),
    Notifier(NotifierCommand),
}

pub async fn command_processor(mut rx: Receiver<Command>) {
    debug!("Command processor started");
    while let Some(command) = rx.recv().await {
        debug!("Received command: {:?}", command);
        let result = match command {
            Command::Artifact(command) => match command {
                ArtifactCommand::GetAll(command) => handle_get_all_artifacts(command),
                _ => Ok(()), // ArtifactCommand::Check(command) => {}
                             // ArtifactCommand::GetById(command) => {}
            },
            _ => Ok(()), // Command::Notifier(command) => match command {
                         //     NotifierCommand::GetAll(command) => {}
                         //     NotifierCommand::Notify(command) => {}
                         //     NotifierCommand::GetById(command) => {}
                         // },
        };

        match result {
            Ok(_) => {}
            Err(err) => {}
        }
    }
}
