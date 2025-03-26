#[cfg(not(target_family = "wasm"))]
fn main() -> Result<(), rinf_cli::SetupError> {
  rinf_cli::run_command()
}

#[cfg(target_family = "wasm")]
fn main() {
  // This is a dummy function to make Clippy happy.
}
