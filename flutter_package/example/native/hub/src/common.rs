use std::error::Error;

/// This `Result` type alias allows handling any error type
/// that implements the `Error` trait.
/// In practice, it is recommended to use custom solutions
/// or crates like `anyhow` dedicated to error handling.
pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
