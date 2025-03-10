#[cfg(not(target_family = "wasm"))]
fn main() -> Result<(), rinf_cli::SetupError> {
    rinf_cli::run_command()
}

#[cfg(target_family = "wasm")]
fn main() {
    // Dummy function to make the linter happy.
}
