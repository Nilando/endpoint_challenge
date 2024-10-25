use super::path::Path;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum FileSystemError {
    PathDoesNotExist(Path),
    BadPath(String),
    CmdNotFound(String),
    InvalidCmdArgs(String),
    FileAlreadyExists(Path),
}

impl Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathDoesNotExist(path) => {
                write!(f, "{} does not exist", String::from(path.clone()))
            }
            Self::CmdNotFound(cmd) => {
                write!(f, "command not found: {}", cmd)
            }
            Self::FileAlreadyExists(path) => {
                write!(f, "{} already exists", String::from(path.clone()))
            }
            Self::InvalidCmdArgs(err) => {
                write!(f, "{}", err)
            }
            Self::BadPath(err) => {
                write!(f, "invalid path {}", err)
            }
        }
    }
}
