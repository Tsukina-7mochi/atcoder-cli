use std::time::Duration;

use super::cookie;
use super::cookie::ureq::GetCookies;
use super::url;

fn get_csrf_token_and_session_cookie() -> (String, String) {
    let url = url::login();
    let res = ureq::get(&url)
        .timeout(Duration::from_secs(5))
        .call()
        .unwrap();
    let session_cookie = res
        .get_cookie(cookie::session::NAME)
        .unwrap()
        .value()
        .to_owned();
    let csrf_token = cookie::session::get_csrf_token(&session_cookie).unwrap();

    (csrf_token, session_cookie)
}

fn try_login(
    username: &str,
    password: &str,
    csrf_token: &str,
    session_cookie: &str,
) -> Option<String> {
    let url = url::login();
    let cookie_value = cookie::session::to_cookie_value(session_cookie);
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
    let cookies = res.get_cookies();
    let session_cookie = cookies
        .get(cookie::session::NAME)
        .unwrap()
        .value()
        .to_owned();
    let result_cookie = cookies
        .get(cookie::result::NAME)
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
