use anyhow::Result;

pub mod email;
pub mod google_chat;
pub mod slack;

pub trait SinkNotifier: Send + Sync {
    fn send(&self, message: &str) -> Result<()>;
    fn type_name(&self) -> &str;
}
