use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RinfError {
    NoDartIsolate,
    NoSignalHandler,
    NoBindings,
}

impl fmt::Display for RinfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoDartIsolate => {
                write!(f, "Dart isolate for Rust signals was not created")
            }
            Self::NoSignalHandler => {
                write!(f, "Could not find the handler for Dart signal")
            }
            Self::NoBindings => {
                write!(f, "Rinf bindings are not ready")
            }
        }
    }
}

impl Error for RinfError {}
