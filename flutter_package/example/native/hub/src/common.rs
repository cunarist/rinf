use std::error::Error;

/// Using this `Result` type alias allows
/// handling any error type that implements the `Error` trait.
/// This approach eliminates the need
/// to depend on external crates for error handling.
pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
