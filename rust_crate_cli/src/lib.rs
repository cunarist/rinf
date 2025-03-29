#[cfg(not(target_family = "wasm"))]
mod tool;

#[cfg(not(target_family = "wasm"))]
pub use tool::{SetupError, run_command};
