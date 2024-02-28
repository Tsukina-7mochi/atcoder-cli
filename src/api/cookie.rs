pub mod session {
    use crate::util::url_encoding;

    pub const NAME: &str = "REVEL_SESSION";

    pub fn get_csrf_token(session_cookie: &str) -> Option<String> {
        for value in session_cookie.split("%00") {
            if value.starts_with("csrf_token") {
                return Some(url_encoding::encode(value[13..].to_owned()));
            }
        }
        None
    }

    pub fn to_cookie_value(session_cookie: &str) -> String {
        format!("{}={}", NAME, session_cookie)
    }
}

pub mod result {
    pub const NAME: &str = "REVEL_FLASH";
}

pub mod ureq {
    use ureq::{Cookie, Response};

    pub struct Cookies<'a> {
        value: Vec<Cookie<'a>>,
    }

    impl<'a> Cookies<'a> {
        pub fn get(&self, name: &str) -> Option<&Cookie<'a>> {
            self.value.iter().find(|c| c.name() == name)
        }
    }

    impl<'a> Into<Vec<Cookie<'a>>> for Cookies<'a> {
        fn into(self) -> Vec<Cookie<'a>> {
            self.value
        }
    }

    impl<'a> From<Vec<Cookie<'a>>> for Cookies<'a> {
        fn from(value: Vec<Cookie<'a>>) -> Self {
            Cookies { value }
        }
    }

    pub trait GetCookies {
        fn get_cookie(&self, name: &str) -> Option<Cookie>;
        fn get_cookies(&self) -> Cookies;
    }

    impl GetCookies for Response {
        fn get_cookie(&self, name: &str) -> Option<Cookie> {
            self.all("set-cookie")
                .iter()
                .map(|s| ureq::Cookie::parse(*s))
                .inspect(|r| {
                    if r.is_err() {
                        eprintln!("Failed to prase cookie, ignore it.");
                    }
                })
                .filter_map(|r| r.ok())
                .find(|c| c.name() == name)
        }

        fn get_cookies(&self) -> Cookies {
            self.all("set-cookie")
                .iter()
                .map(|s| ureq::Cookie::parse(*s))
                .inspect(|r| {
                    if r.is_err() {
                        eprintln!("Failed to prase cookie, ignore it.");
                    }
                })
                .filter_map(|r| r.ok())
                .collect::<Vec<_>>()
                .into()
        }
    }
}
