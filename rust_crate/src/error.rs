use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RinfError {
    LockDartIsolate,
    NoDartIsolate,
    BuildRuntime,
    LockMessageChannel,
    NoMessageChannel,
    MessageReceiverTaken,
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
            RinfError::BuildRuntime => {
                write!(f, "Could not build the tokio runtime.")
            }
            RinfError::LockMessageChannel => {
                write!(f, "Could not acquire the message channel lock.")
            }
            RinfError::NoMessageChannel => {
                write!(f, "Message channel was not created.",)
            }
            RinfError::MessageReceiverTaken => {
                write!(f, "Each Dart signal receiver can be taken only once.")
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
