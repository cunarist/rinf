use std::env;
use std::process::Command;

fn main() {
    // Get command-line arguments excluding the program name
    let dart_command_args: Vec<String> = env::args().skip(1).collect();

    // Build the command to run the Dart script
    #[cfg(target_family = "windows")]
    let mut command = Command::new("dart.bat");
    #[cfg(target_family = "unix")]
    let mut command = Command::new("dart");
    #[cfg(target_family = "wasm")]
    let mut command = Command::new("dart");
    command.args(["run", "rust_in_flutter"]);
    command.args(&dart_command_args);

    // Execute the command
    let _ = command.status();
}
