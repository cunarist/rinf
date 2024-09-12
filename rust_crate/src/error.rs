use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RinfError {
    LockDartIsolate,
    NoDartIsolate,
    LockShutdownReceiver,
    NoShutdownReceiver,
    DecodeMessage,
    NoSignalHandler,
}

impl fmt::Display for RinfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RinfError::LockDartIsolate => {
                write!(f, "Could not acquire the Dart isolate lock.")
            }
            RinfError::NoDartIsolate => {
                write!(f, "Dart isolate for Rust signals was not created.")
            }
            RinfError::LockShutdownReceiver => {
                write!(f, "Could not acquire the shutdown receiver lock.")
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
