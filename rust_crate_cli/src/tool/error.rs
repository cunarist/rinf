// TODO: Organize error messages
// TODO: Organize error conversion

use std::error::Error;

/// Error type for Rinf configuration loading.
#[derive(Debug)]
pub enum SetupError {
    IoError(std::io::Error),
    YamlError(serde_yml::Error),
    ClipError(arboard::Error),
    SyntaxError(syn::Error),
    ReflectionError(Box<dyn Error>),
    WatchError(notify::Error),
    UnknownKey(String, String),
    NotFlutterApp,
    Other, // TODO: Remove
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupError::IoError(e) => {
                write!(f, "Failed to read YAML file: {}", e)
            }
            SetupError::YamlError(e) => {
                write!(f, "Failed to parse YAML: {}", e)
            }
            SetupError::ClipError(e) => {
                write!(f, "Failed to use clipboard: {}", e)
            }
            SetupError::SyntaxError(e) => {
                write!(f, "Invalid syntax: {}", e)
            }
            SetupError::ReflectionError(e) => {
                write!(f, "Failed to use reflection: {}", e)
            }
            SetupError::WatchError(e) => {
                write!(f, "Watch error: {}", e)
            }
            SetupError::UnknownKey(key, available) => {
                write!(
                    f,
                    "Unknown key '{}' in rinf config. Available keys are: {}",
                    key, available
                )
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
        SetupError::IoError(err)
    }
}

impl From<serde_yml::Error> for SetupError {
    fn from(err: serde_yml::Error) -> Self {
        SetupError::YamlError(err)
    }
}

impl From<arboard::Error> for SetupError {
    fn from(err: arboard::Error) -> Self {
        SetupError::ClipError(err)
    }
}

impl From<syn::Error> for SetupError {
    fn from(err: syn::Error) -> Self {
        SetupError::SyntaxError(err)
    }
}

impl From<Box<dyn Error>> for SetupError {
    fn from(err: Box<dyn Error>) -> Self {
        SetupError::ReflectionError(err)
    }
}

impl From<notify::Error> for SetupError {
    fn from(err: notify::Error) -> Self {
        SetupError::WatchError(err)
    }
}
