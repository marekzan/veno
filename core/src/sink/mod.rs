use anyhow::Result;

pub mod email;
pub mod google_chat;
pub mod slack;

pub trait SinkNotifier {
    fn send(&self, message: &str) -> Result<()>;
}
