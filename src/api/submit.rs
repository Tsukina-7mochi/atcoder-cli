use std::time::Duration;

use super::cookie;
use super::url;
use crate::api;
use crate::api::error::SessionCookieErrorKind;
use crate::api::error::SubmitErrorKind;

pub fn submit(
    contest_name: &str,
    task_name: &str,
    language_id: u32,
    source_code: &str,
    session_cookie: &str,
) -> api::Result<()> {
    let csrf_token = cookie::session::get_csrf_token(session_cookie)
        .ok_or::<SubmitErrorKind>(SessionCookieErrorKind::InvalidSessionCookie.into())?;
    let language_id = language_id.to_string();

    let url = url::contest_submit(contest_name);
    let cookie_value = cookie::session::to_cookie_value(session_cookie);
    let form_data = [
        ("data.TaskScreenName", task_name),
        ("data.LanguageId", &language_id),
        ("sourceCode", source_code),
        ("csrf_token", &csrf_token),
    ];

    ureq::builder()
        .redirects(0)
        .timeout(Duration::from_secs(5))
        .build()
        .post(&url)
        .set("Cookie", &cookie_value)
        .send_form(&form_data)?;

    Ok(())
}
