//! This `hub` crate is the
//! entry point of the Rust logic.
//!
#![allow(unused_imports)]

mod messages;
mod sample_functions;

use libvips;
use tokio;
// use tokio_with_wasm::tokio; // Uncomment this line to target the web

rinf::write_interface!();

async fn main() {
    tokio::spawn(sample_functions::tell_numbers());
    tokio::spawn(sample_functions::stream_fractal());
    tokio::spawn(sample_functions::run_debug_tests());
    tokio::spawn(sample_functions::libvips_add());
}
