mod arguments;
mod config;
mod generate;

use config::*;
use generate::*;

pub use arguments::run_command;
pub use config::RinfCommandError;

// TODO: Remove all panicking code like `unwrap` or `expect`.
