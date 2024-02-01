pub use interface::*;
pub use macros::*;

pub mod externs;
mod interface;
#[cfg(not(target_family = "wasm"))]
mod interface_os;
#[cfg(target_family = "wasm")]
mod interface_web;
mod macros;
