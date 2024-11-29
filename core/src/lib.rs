use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder};

pub mod config;
pub mod sink;
pub mod source;

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new()
        .user_agent("neveno-checker")
        .build()
        .expect("Could not create reqwest client")
});
