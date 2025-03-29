#[cfg(not(target_family = "wasm"))]
fn main() {
  let result = rinf_cli::run_command();
  if let Err(err) = result {
    eprintln!("Error: {}", err);
  }
}

#[cfg(target_family = "wasm")]
fn main() {
  // This is a dummy function to make Clippy happy.
}
