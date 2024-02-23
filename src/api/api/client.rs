use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::blocking::Client;

pub fn new_client() -> reqwest::Result<Client> {
    let builder = Client::builder().timeout(Duration::from_secs(5));
    builder.build()
}
