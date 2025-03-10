// TODO: Organize error messages
// TODO: Organize error conversion

use std::error::Error;
use std::ffi::OsString;

/// Error type for Rinf configuration loading.
#[derive(Debug)]
pub enum SetupError {
    Io(std::io::Error),
    Yaml(serde_yml::Error),
    Clip(arboard::Error),
    Syntax(syn::Error),
    Reflection(Box<dyn Error>),
    WatchingFile(notify::Error),
    UnknownKey(String, String),
    BadFilePath(OsString),
    NotFlutterApp,
    Other, // TODO: Remove
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupError::Io(e) => {
                write!(f, "Failed to read YAML file: {}", e)
            }
            SetupError::Yaml(e) => {
                write!(f, "Failed to parse YAML: {}", e)
            }
            SetupError::Clip(e) => {
                write!(f, "Failed to use clipboard: {}", e)
            }
            SetupError::Syntax(e) => {
                write!(f, "Invalid syntax: {}", e)
            }
            SetupError::Reflection(e) => {
                write!(f, "Failed to use reflection: {}", e)
            }
            SetupError::WatchingFile(e) => {
                write!(f, "Watch error: {}", e)
            }
            SetupError::UnknownKey(key, available) => {
                write!(
                    f,
                    "Unknown key \"{}\" in rinf config. Available keys are: {}",
                    key, available
                )
            }
            SetupError::BadFilePath(name) => {
                write!(f, "\"{:?}\" is not a valid file path", name)
            }
            SetupError::NotFlutterApp => {
                write!(f, "This is not a Flutter app project")
            }
            SetupError::Other => write!(f, "Unknown error"),
        }
    }
}

impl std::error::Error for SetupError {}

impl From<std::io::Error> for SetupError {
    fn from(err: std::io::Error) -> Self {
        SetupError::Io(err)
    }
}

impl From<serde_yml::Error> for SetupError {
    fn from(err: serde_yml::Error) -> Self {
        SetupError::Yaml(err)
    }
}

impl From<arboard::Error> for SetupError {
    fn from(err: arboard::Error) -> Self {
        SetupError::Clip(err)
    }
}

impl From<syn::Error> for SetupError {
    fn from(err: syn::Error) -> Self {
        SetupError::Syntax(err)
    }
}

impl From<Box<dyn Error>> for SetupError {
    fn from(err: Box<dyn Error>) -> Self {
        SetupError::Reflection(err)
    }
}

impl From<notify::Error> for SetupError {
    fn from(err: notify::Error) -> Self {
        SetupError::WatchingFile(err)
    }
}
