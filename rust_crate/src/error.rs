use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RinfError {
    NoDartIsolate,
    NoShutdownReceiver,
    DecodeMessage,
    NoSignalHandler,
}

impl fmt::Display for RinfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RinfError::NoDartIsolate => {
                write!(f, "Dart isolate for Rust signals was not created.")
            }
            RinfError::NoShutdownReceiver => {
                write!(f, "Shutdown receiver was not created.")
            }
            RinfError::DecodeMessage => {
                write!(f, "Could not decode the message.")
            }
            RinfError::NoSignalHandler => {
                write!(f, "Could not find the handler for Dart signal.")
            }
        }
    }
}

impl Error for RinfError {}
