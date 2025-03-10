mod common;
mod config;
mod entry;
mod generate;
mod template;
mod webassembly;

use common::*;
use config::*;
use generate::*;
use template::*;
use webassembly::*;

pub use config::SetupError;
pub use entry::run_command;

// TODO: Remove all panicking code like `unwrap` or `expect`.
