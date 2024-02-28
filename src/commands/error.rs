use std::error;
use std::{fmt, io};

use subprocess::{ExitStatus, PopenError};

use crate::api::APIError;

pub type Result<T = ()> = std::result::Result<T, CommandError>;

#[derive(Debug)]
pub enum CommandError {
    APIError(APIError),
    IOError(io::Error),
    InitTaskDirectoryError(InitTaskDirectoryErrorKind),
    RunTestError(RunTestErrorKind),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::APIError(err) => write!(f, "{}", err),
            Self::IOError(err) => write!(f, "{}", err),
            Self::InitTaskDirectoryError(err) => write!(f, "{}", err),
            Self::RunTestError(err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for CommandError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::APIError(err) => Some(err),
            Self::IOError(err) => Some(err),
            Self::InitTaskDirectoryError(err) => err.source(),
            Self::RunTestError(err) => err.source(),
        }
    }
}

impl From<APIError> for CommandError {
    fn from(err: APIError) -> Self {
        Self::APIError(err)
    }
}

impl From<io::Error> for CommandError {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<InitTaskDirectoryErrorKind> for CommandError {
    fn from(err: InitTaskDirectoryErrorKind) -> Self {
        Self::InitTaskDirectoryError(err)
    }
}

impl From<RunTestErrorKind> for CommandError {
    fn from(err: RunTestErrorKind) -> Self {
        Self::RunTestError(err)
    }
}

#[derive(Debug)]
pub enum InitTaskDirectoryErrorKind {
    PathIsNotDirectory(String),
    FailedToCreateDirectory(io::Error),
    InitCommandFailed(SubprocessError),
}

impl fmt::Display for InitTaskDirectoryErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PathIsNotDirectory(path) => write!(f, "Target path {} is not directory.", path),
            Self::FailedToCreateDirectory(err) => {
                write!(f, "Failed to create directory :{}", err)
            }
            Self::InitCommandFailed(status) => write!(f, "{}", status),
        }
    }
}

impl InitTaskDirectoryErrorKind {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::FailedToCreateDirectory(err) => Some(err),
            Self::InitCommandFailed(err) => err.source(),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum RunTestErrorKind {
    BuildCommandFailed(SubprocessError),
    RunCommandFailed(SubprocessError),
}

impl fmt::Display for RunTestErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BuildCommandFailed(err) => write!(f, "Build process failed: {}", err),
            Self::RunCommandFailed(err) => write!(f, "Process failed: {}", err),
        }
    }
}

impl RunTestErrorKind {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::BuildCommandFailed(err) => err.source(),
            Self::RunCommandFailed(err) => err.source(),
        }
    }
}

#[derive(Debug)]
pub enum SubprocessError {
    PopenError(PopenError),
    ProcessFailed(ExitStatus),
}

impl fmt::Display for SubprocessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PopenError(err) => write!(f, "{}", err),
            Self::ProcessFailed(status) => match status {
                ExitStatus::Exited(code) => write!(f, "Process exited with code {}.", code),
                ExitStatus::Signaled(signal) => {
                    write!(f, "Process was killed by signal {}.", signal)
                }
                ExitStatus::Other(err) => write!(f, "Process failed: {}", err),
                ExitStatus::Undetermined => write!(f, "Process failed for some reason."),
            },
        }
    }
}

impl SubprocessError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::PopenError(err) => Some(err),
            Self::ProcessFailed(_) => None,
        }
    }
}

impl From<PopenError> for SubprocessError {
    fn from(err: PopenError) -> Self {
        Self::PopenError(err)
    }
}

impl From<ExitStatus> for SubprocessError {
    fn from(err: ExitStatus) -> Self {
        Self::ProcessFailed(err)
    }
}
