use std::collections::HashMap;
use std::time::Duration;

use reqwest::header::HeaderValue;
use scraper::Html;

use super::client;
use super::url;

const COOKIE_NAME_SESSION: &str = "REVEL_SESSION";
const COOKIE_NAME_RESULT: &str = "REVEL_FLASH";

mod selectors {
    use once_cell::sync::Lazy;
    use scraper::Selector;

    pub const INPUT_CSRF_TOKEN: Lazy<Selector> =
        Lazy::new(|| Selector::parse("input[name=csrf_token]").unwrap());
}

fn get_csrf_token_and_session_cookie() -> (String, String) {
    let url = url::login();
    let res = client::new_client().unwrap().get(url).send().unwrap();
    let session_cookie = res
        .cookies()
        .find(|c| c.name() == COOKIE_NAME_SESSION)
        .unwrap()
        .value()
        .to_owned();
    let body = res.text().unwrap();
    let document = Html::parse_document(&body);
    let csrf_token = document
        .select(&selectors::INPUT_CSRF_TOKEN)
        .next()
        .map(|el| el.attr("value").unwrap())
        .unwrap()
        .to_owned();

    (csrf_token, session_cookie)
}

fn try_login(
    username: &str,
    password: &str,
    csrf_token: &str,
    session_cookie: &str,
) -> Option<String> {
    let url = url::login();
    let cookie_value = &format!("{}={}", COOKIE_NAME_SESSION, session_cookie);
    let cookie_value = HeaderValue::from_str(&cookie_value).unwrap();
    let form_data = HashMap::from([
        ("username", username),
        ("password", password),
        ("csrf_token", csrf_token),
    ]);

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    let res = client
        .post(url)
        .header(reqwest::header::COOKIE, cookie_value)
        .form(&form_data)
        .send()
        .unwrap();

    let session_cookie = res
        .cookies()
        .find(|c| c.name() == COOKIE_NAME_SESSION)
        .unwrap()
        .value()
        .to_owned();
    let result_cookie = res
        .cookies()
        .find(|c| c.name() == COOKIE_NAME_RESULT)
        .unwrap();
    let result_cookie = result_cookie.value();

    if !result_cookie.contains("success") {
        None
    } else {
        Some(session_cookie)
    }
}

pub fn login(username: &str, password: &str) -> Option<String> {
    let (csrf_token, session_cookie) = get_csrf_token_and_session_cookie();
    try_login(username, password, &csrf_token, &session_cookie)
}
