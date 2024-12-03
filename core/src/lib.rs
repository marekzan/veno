pub mod artifact;
pub mod config;
pub mod notifier;

use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder};

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new()
        .user_agent("neveno-checker")
        .build()
        .expect("Could not create reqwest client")
});
