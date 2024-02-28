use std::error;
use std::{fmt, io};

pub type Result<T> = std::result::Result<T, APIError>;

#[derive(Debug)]
pub enum APIError {
    UreqError(ureq::Error),
    IOError(io::Error),
    GetTaskTestsError(GetTaskTestsErrorKind),
    LoginError(LoginErrorKind),
    SubmitError(SubmitErrorKind),
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UreqError(err) => match err {
                ureq::Error::Status(403, _) => write!(f, "Server responded with error 403 (Forbidden). You may not logged in or session may have expired. Try re-logging in with `atcoder login`."),
                ureq::Error::Status(code, _) => write!(f, "Server responded with error {}.", code),
                _ => write!(f, "HTTP Request failed: {}", err),
            },
            Self::IOError(err) => write!(f, "{}", err),
            Self::GetTaskTestsError(err) => write!(f, "Failed to get tests: {}", err),
            Self::LoginError(err) => write!(f, "Failed to login: {}", err),
            Self::SubmitError(err) => write!(f, "Failed to submit: {}", err),
        }
    }
}

impl error::Error for APIError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::UreqError(err) => Some(err),
            APIError::IOError(err) => Some(err),
            APIError::GetTaskTestsError(_) => None,
            APIError::LoginError(_) => None,
            APIError::SubmitError(_) => None,
        }
    }
}

impl From<ureq::Error> for APIError {
    fn from(err: ureq::Error) -> Self {
        Self::UreqError(err)
    }
}

impl From<io::Error> for APIError {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<GetTaskTestsErrorKind> for APIError {
    fn from(err: GetTaskTestsErrorKind) -> Self {
        Self::GetTaskTestsError(err)
    }
}

impl From<LoginErrorKind> for APIError {
    fn from(err: LoginErrorKind) -> Self {
        Self::LoginError(err)
    }
}

impl From<SubmitErrorKind> for APIError {
    fn from(err: SubmitErrorKind) -> Self {
        Self::SubmitError(err)
    }
}

#[derive(Debug)]
pub enum GetTaskTestsErrorKind {
    IncorrectTestNumber(usize),
}

impl fmt::Display for GetTaskTestsErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncorrectTestNumber(n) => write!(f, "Incorrect number of tests ({}).", n),
        }
    }
}

#[derive(Debug)]
pub enum LoginErrorKind {
    SessionCookieError(SessionCookieErrorKind),
    NoResultCookie,
    LoginFailed(String),
}

impl fmt::Display for LoginErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SessionCookieError(err) => err.fmt(f),
            Self::NoResultCookie => write!(f, "No result cookie set."),
            Self::LoginFailed(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<SessionCookieErrorKind> for LoginErrorKind {
    fn from(err: SessionCookieErrorKind) -> Self {
        Self::SessionCookieError(err)
    }
}

#[derive(Debug)]
pub enum SubmitErrorKind {
    SessionCookieError(SessionCookieErrorKind),
}

impl fmt::Display for SubmitErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SessionCookieError(err) => err.fmt(f),
        }
    }
}

impl From<SessionCookieErrorKind> for SubmitErrorKind {
    fn from(err: SessionCookieErrorKind) -> Self {
        Self::SessionCookieError(err)
    }
}

#[derive(Debug)]
pub enum SessionCookieErrorKind {
    NoSessionCookie,
    InvalidSessionCookie,
}

impl fmt::Display for SessionCookieErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSessionCookie => write!(f, "No session cookie set."),
            Self::InvalidSessionCookie => write!(f, "Invalid session cookie."),
        }
    }
}
