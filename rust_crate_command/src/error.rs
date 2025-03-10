// TODO: Organize error messages
// TODO: Organize error conversion

/// Error type for Rinf configuration loading.
#[derive(Debug)]
pub enum SetupError {
    IoError(std::io::Error),
    YamlError(serde_yml::Error),
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
