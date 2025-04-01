use std::process::ExitCode;

#[cfg(not(target_family = "wasm"))]
fn main() -> ExitCode {
  use owo_colors::OwoColorize;
  let result = rinf_cli::run_command();
  if let Err(err) = result {
    eprintln!("{}", format!("Error: {}", err).red());
    return ExitCode::FAILURE;
  };
  ExitCode::SUCCESS
}

#[cfg(target_family = "wasm")]
fn main() -> ExitCode {
  // This is a dummy function to make Clippy happy.
  ExitCode::SUCCESS
}
