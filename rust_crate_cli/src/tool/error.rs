// TODO: Organize error messages
// TODO: Organize error conversion

use std::error::Error;
use std::ffi::OsString;

/// Error type for Rinf configuration loading.
#[derive(Debug)]
pub enum SetupError {
    Io(std::io::Error),
    Yaml(serde_yml::Error),
    Clipboard(arboard::Error),
    CodeSyntax(syn::Error),
    ReflectionModule(Box<dyn Error>),
    WatchingFile(notify::Error),
    PubConfig(String),
    BadFilePath(OsString),
    ProjectStructure(&'static str),
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupError::Io(e) => {
                write!(f, "Failed to operate on file: {}", e)
            }
            SetupError::Yaml(e) => {
                write!(f, "Failed to parse YAML: {}", e)
            }
            SetupError::Clipboard(e) => {
                write!(f, "Failed to use clipboard: {}", e)
            }
            SetupError::CodeSyntax(e) => {
                write!(f, "Invalid code syntax: {}", e)
            }
            SetupError::ReflectionModule(e) => {
                write!(f, "Failed to use reflection: {}", e)
            }
            SetupError::WatchingFile(e) => {
                write!(f, "Watching error: {}", e)
            }
            SetupError::PubConfig(msg) => {
                write!(f, "Invalid `pubspec.yaml` config: {}", msg)
            }
            SetupError::BadFilePath(name) => {
                write!(f, "Not a valid file path: `{}`", name.to_string_lossy())
            }
            SetupError::ProjectStructure(msg) => {
                write!(f, "Invalid project structure: {}", msg)
            }
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
        SetupError::Clipboard(err)
    }
}

impl From<syn::Error> for SetupError {
    fn from(err: syn::Error) -> Self {
        SetupError::CodeSyntax(err)
    }
}

impl From<notify::Error> for SetupError {
    fn from(err: notify::Error) -> Self {
        SetupError::WatchingFile(err)
    }
}
