use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::blocking::Client;

pub const HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    let builder = Client::builder().timeout(Duration::from_secs(5));
    builder.build().unwrap()
});
