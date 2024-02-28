use std::{error, fmt};

use crate::commands::CommandError;

pub type Result<T = ()> = std::result::Result<T, CLIError>;

#[derive(Debug)]
pub enum CLIError {
    CommandError(CommandError),
    ConfigError(ConfigErrorKind),
}

impl fmt::Display for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandError(err) => write!(f, "{}", err),
            Self::ConfigError(err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for CLIError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::CommandError(err) => Some(err),
            Self::ConfigError(_) => None,
        }
    }
}

impl From<CommandError> for CLIError {
    fn from(err: CommandError) -> Self {
        Self::CommandError(err)
    }
}

impl From<ConfigErrorKind> for CLIError {
    fn from(err: ConfigErrorKind) -> Self {
        Self::ConfigError(err)
    }
}

#[derive(Debug)]
pub enum ConfigErrorKind {
    CWDNotProvided,
    WorkspacePathNotProvided,
    WorkspaceConfigNotProvided,
    SessionCookieNotProvided,
    ProfileNotProvided,
    ContestNameNotProvided,
    TaskNameNotProvided,
}

impl fmt::Display for ConfigErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CWDNotProvided => write!(f, "Current working directory not provided"),
            Self::WorkspacePathNotProvided => write!(f, "Workspace path not provided"),
            Self::WorkspaceConfigNotProvided => write!(f, "Workspace config not provided"),
            Self::SessionCookieNotProvided => write!(f, "Session cookie not provided"),
            Self::ProfileNotProvided => write!(f, "Profile not provided"),
            Self::ContestNameNotProvided => write!(f, "Contest name not provided"),
            Self::TaskNameNotProvided => write!(f, "Task name not provided"),
        }
    }
}
