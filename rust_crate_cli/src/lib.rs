mod common;
mod config;
mod entry;
mod error;
mod generate;
mod template;
mod webassembly;

use common::*;
use config::*;
use generate::*;
use template::*;
use webassembly::*;

pub use entry::run_command;
pub use error::SetupError;

// TODO: Remove all panicking code like `unwrap` or `expect`.
