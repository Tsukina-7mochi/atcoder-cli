use std::{collections::HashMap, time::Duration};

use reqwest::header::HeaderValue;

use super::url;
use crate::util::url_encoding;

const COOKIE_NAME_SESSION: &str = "REVEL_SESSION";

fn get_csrf_token(session_cookie: &str) -> Option<String> {
    for value in session_cookie.split("%00") {
        if value.starts_with("csrf_token") {
            return Some(url_encoding::encode(value[13..].to_owned()));
        }
    }
    None
}

pub fn submit(
    contest_name: &str,
    task_name: &str,
    language_id: u32,
    source_code: &str,
    session_cookie: &str,
) {
    let csrf_token = get_csrf_token(session_cookie).expect("Cannot get CSRF Token");
    let language_id = language_id.to_string();

    println!("{}", session_cookie);
    println!("{}", csrf_token);

    let url = url::contest_submit(contest_name);
    let cookie_value = &format!("{}={}", COOKIE_NAME_SESSION, session_cookie);
    let cookie_value = HeaderValue::from_str(&cookie_value).unwrap();
    let form_data = HashMap::from([
        ("data.TaskScreenName", task_name),
        ("data.LanguageId", &language_id),
        ("sourceCode", source_code),
        ("csrf_token", &csrf_token),
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

    println!("{:?}", res.text());
}
