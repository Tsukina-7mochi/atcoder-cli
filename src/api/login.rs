use std::time::Duration;

use scraper::Html;

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
    let res = ureq::get(&url)
        .timeout(Duration::from_secs(5))
        .call()
        .unwrap();
    let cookies: Vec<_> = res
        .all("set-cookie")
        .iter()
        .map(|s| ureq::Cookie::parse(*s).unwrap())
        .collect();
    let session_cookie = cookies
        .iter()
        .find(|c| c.name() == COOKIE_NAME_SESSION)
        .unwrap()
        .value()
        .to_owned();
    let body = res.into_string().unwrap();
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
    let form_data = [
        ("username", username),
        ("password", password),
        ("csrf_token", csrf_token),
    ];

    let res = ureq::builder()
        .redirects(0)
        .timeout(Duration::from_secs(5))
        .build()
        .post(&url)
        .set("Cookie", &cookie_value)
        .send_form(&form_data)
        .unwrap();
    let cookies: Vec<_> = res
        .all("set-cookie")
        .iter()
        .map(|s| ureq::Cookie::parse(*s).unwrap())
        .collect();
    let session_cookie = cookies
        .iter()
        .find(|c| c.name() == COOKIE_NAME_SESSION)
        .unwrap()
        .value()
        .to_owned();
    let result_cookie = cookies
        .iter()
        .find(|c| c.name() == COOKIE_NAME_RESULT)
        .unwrap()
        .value()
        .to_owned();

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
