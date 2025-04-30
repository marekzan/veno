use artifact_commands::ArtifactCommand;
use artifact_handlers::{handle_check, handle_get_all_artifacts, handle_get_by_id};
use notifier_commands::NotifierCommand;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error};

pub mod artifact_commands;
pub mod artifact_handlers;
pub mod notifier_commands;
pub mod notifier_handlers;

#[derive(Debug)]
pub enum Command<'a> {
    Artifact(ArtifactCommand<'a>),
    Notifier(NotifierCommand),
}

pub async fn command_processor(mut rx: Receiver<Command<'_>>) {
    debug!("Command processor started");
    while let Some(command) = rx.recv().await {
        tokio::spawn(async move {
            match command {
                Command::Artifact(command) => match command {
                    ArtifactCommand::GetAll(command) => handle_get_all_artifacts(command),
                    ArtifactCommand::Check(command) => handle_check(command).await,
                    ArtifactCommand::GetById(command) => handle_get_by_id(command),
                },
                _ => Ok(()), // Command::Notifier(command) => match command {
                             //     NotifierCommand::GetAll(command) => {}
                             //     NotifierCommand::Notify(command) => {}
                             //     NotifierCommand::GetById(command) => {}
                             // },
            };
        });
    }
}
