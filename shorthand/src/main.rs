use std::env;
use std::process::Command;

fn main() {
    // Get command-line arguments excluding the program name
    let dart_command_args: Vec<String> = env::args().skip(1).collect();

    // Build the command to run the Dart script
    let dart_path = which::which("dart").unwrap(); // https://github.com/rust-lang/rust/issues/37519
    let mut command = Command::new(dart_path);
    command.args(["run", "rust_in_flutter"]);
    command.args(&dart_command_args);

    // Execute the command
    let _ = command.status();
}
