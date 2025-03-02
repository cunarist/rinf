mod arguments;
mod common;
mod config;
mod generate;
mod template;
mod webassembly;

use common::*;
use config::*;
use generate::*;
use template::*;
use webassembly::*;

pub use arguments::run_command;
pub use config::RinfCommandError;

// TODO: Remove all panicking code like `unwrap` or `expect`.
