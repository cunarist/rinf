mod arguments;
mod config;
mod generate;
mod template;

use config::*;
use generate::*;
use template::*;

pub use arguments::run_command;
pub use config::RinfCommandError;

// TODO: Remove all panicking code like `unwrap` or `expect`.
