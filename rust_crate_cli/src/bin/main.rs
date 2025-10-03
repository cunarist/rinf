use std::process::ExitCode;

fn main() -> ExitCode {
  use owo_colors::OwoColorize;
  let result = rinf_cli::run_command();
  if let Err(err) = result {
    eprintln!("{}", format!("Error: {err}").red());
    return ExitCode::FAILURE;
  };
  ExitCode::SUCCESS
}
