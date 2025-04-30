use tokio::sync::oneshot::Sender;
use veno_core::notifier::Notifier;

use crate::resources::v1::notifiers::model::NotifierResponse;

#[derive(Debug)]
pub struct GetAllCommand {
    notifier: Vec<Notifier>,
    responder: Sender<Result<Vec<NotifierResponse>, CommandError>>,
}

#[derive(Debug)]
pub struct NotifyCommand {
    notifier: Vec<Notifier>,
    responder: Sender<Result<Vec<NotifierResponse>, CommandError>>,
}

#[derive(Debug)]
pub struct GetByIdCommand {
    notifier: Vec<Notifier>,
    responder: Sender<Result<Vec<NotifierResponse>, CommandError>>,
}

#[derive(Debug)]
pub enum NotifierCommand {
    GetAll(GetAllCommand),
    Notify(NotifyCommand),
    GetById(GetByIdCommand),
}

#[derive(Debug)]
pub enum CommandError {
    NotifierNotFound,
    InternalServerError,
}
