#[cfg(not(target_family = "wasm"))]
fn main() {
    use std::env;
    use std::fs;
    use std::process;

    // Install and remember Protobuf compiler's path
    let home_path = home::home_dir().unwrap();
    let out_path = home_path.join(".local").join("bin");
    fs::create_dir_all(&out_path).unwrap();
    env::set_var("OUT_DIR", out_path.to_str().unwrap());
    let (protoc_path, _) = protoc_prebuilt::init("22.0").unwrap();
    let mut path_var = match env::var_os("PATH") {
        Some(val) => env::split_paths(&val).collect::<Vec<_>>(),
        None => Vec::new(),
    };
    path_var.push(protoc_path.parent().unwrap().to_path_buf());
    env::set_var("PATH", env::join_paths(path_var).unwrap());

    // Get command-line arguments excluding the program name
    let dart_command_args: Vec<String> = env::args().skip(1).collect();

    // Build the command to run the Dart script
    #[cfg(target_family = "windows")]
    let mut command = process::Command::new("dart.bat");
    #[cfg(target_family = "unix")]
    let mut command = process::Command::new("dart");
    command.args(["run", "rinf"]);
    command.args(&dart_command_args);

    // Execute the command
    let _ = command.status();
}

#[cfg(target_family = "wasm")]
fn main() {
    // Separate dummy function to make the linter happy
}
