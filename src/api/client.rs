use std::env;
use std::time::Duration;

use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue};

pub fn new_builder() -> ClientBuilder {
    let mut headers = HeaderMap::new();
    if let Ok(session_cookie) = env::var("ATCODER_SESSION_COOKIE") {
        let cookie_value = &format!("{}={}", "REVEL_SESSION", session_cookie);
        let cookie_value = HeaderValue::from_str(&cookie_value).unwrap();
        headers.insert(reqwest::header::COOKIE, cookie_value);
    }

    ClientBuilder::new()
        .timeout(Duration::from_secs(5))
        .default_headers(headers)
}

pub fn new_client() -> reqwest::Result<Client> {
    new_builder().build()
}
