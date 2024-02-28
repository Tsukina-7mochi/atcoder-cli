use std::time::Duration;

use super::cookie;
use super::cookie::ureq::GetCookies;
use super::url;
use crate::api;
use crate::api::error::{LoginErrorKind, SessionCookieErrorKind};
use crate::util::url_encoding;

fn get_csrf_token_and_session_cookie() -> api::Result<(String, String)> {
    let url = url::login();
    let res = ureq::get(&url).timeout(Duration::from_secs(5)).call()?;
    let session_cookie = res
        .get_cookie(cookie::session::NAME)
        .ok_or::<LoginErrorKind>(SessionCookieErrorKind::NoSessionCookie.into())?
        .value()
        .to_owned();
    let csrf_token = cookie::session::get_csrf_token(&session_cookie)
        .ok_or::<LoginErrorKind>(SessionCookieErrorKind::InvalidSessionCookie.into())?;

    Ok((csrf_token, session_cookie))
}

fn try_login(
    username: &str,
    password: &str,
    csrf_token: &str,
    session_cookie: &str,
) -> api::Result<String> {
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
        .send_form(&form_data)?;
    let cookies = res.get_cookies();
    let session_cookie = cookies
        .get(cookie::session::NAME)
        .ok_or::<LoginErrorKind>(SessionCookieErrorKind::NoSessionCookie.into())?
        .value()
        .to_owned();
    let result_cookie = cookies
        .get(cookie::result::NAME)
        .ok_or(LoginErrorKind::NoResultCookie)?
        .value()
        .to_owned();

    if !result_cookie.contains("success") {
        let result = result_cookie
            .split("%00")
            .find(|s| s.starts_with("error"))
            .map(|s| url_encoding::decode(&s[8..]));
        let result = result.unwrap_or(url_encoding::decode(&result_cookie));
        Err(LoginErrorKind::LoginFailed(result).into())
    } else {
        Ok(session_cookie)
    }
}

pub fn login(username: &str, password: &str) -> api::Result<String> {
    let (csrf_token, session_cookie) = get_csrf_token_and_session_cookie()?;
    let session_cookie = try_login(username, password, &csrf_token, &session_cookie)?;

    Ok(session_cookie)
}
