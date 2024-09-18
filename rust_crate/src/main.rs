#[cfg(not(target_family = "wasm"))]
fn main() -> Result<(), String> {
    use std::env;
    use std::fs;
    use std::path;
    use std::process;

    // Ensure Protobuf compiler.
    let protoc_path = if let Ok(installed) = which::which("protoc") {
        // Get the path of Protobuf compiler that's already installed.
        println!("Detected `protoc`, skipping auto installation");
        installed
            .parent()
            .ok_or("Could not get the parent of `protoc` path")?
            .to_path_buf()
    } else {
        // Install Protobuf compiler and get the path.
        let home_path = home::home_dir()
            .ok_or("Could not get home directory for `protoc` installation")?;
        let out_path = home_path.join(".local").join("bin");
        fs::create_dir_all(&out_path).map_err(|_| {
            "Could not create the folder for `protoc` installation"
        })?;
        env::set_var(
            "OUT_DIR",
            out_path
                .to_str()
                .ok_or("Could not set the path for `protoc` installation")?,
        );
        let install_result = protoc_prebuilt::init("25.2");
        let (protoc_binary_path, _) = install_result.map_err(|_|
            "Automatic installation of `protoc` failed, try installing it manually"
        )?;
        protoc_binary_path
            .parent()
            .ok_or("Could not get the parent of newly installed `protoc` path")?
            .to_path_buf()
    };

    // Find the path where Dart executables are located.
    #[cfg(target_family = "windows")]
    let pub_cache_bin_path = path::PathBuf::from(
        env::var("LOCALAPPDATA")
            .map_err(|_| "Could not get `LOCALAPPDATA` path")?,
    )
    .join("Pub")
    .join("Cache")
    .join("bin");
    #[cfg(target_family = "unix")]
    let pub_cache_bin_path = path::PathBuf::from(
        env::var("HOME").map_err(|_| "Could get find `HOME` path")?,
    )
    .join(".pub-cache")
    .join("bin");

    // Add some folders to PATH for various commands to work correctly.
    let mut path_var = match env::var_os("PATH") {
        Some(val) => env::split_paths(&val).collect::<Vec<_>>(),
        None => Vec::new(),
    };
    path_var.push(protoc_path);
    path_var.push(pub_cache_bin_path);
    env::set_var(
        "PATH",
        env::join_paths(path_var)
            .map_err(|_| "Could not push required values to `PATH`")?,
    );

    // Get command-line arguments excluding the program name.
    let dart_command_args: Vec<String> = env::args().skip(1).collect();

    // Run the Dart script.
    let dart_path = which::which("dart")
        .map_err(|_| "Could not find where Dart is located")?;
    let mut command = process::Command::new(dart_path);
    command.args(["run", "rinf"]);
    command.args(&dart_command_args);
    let exit_status =
        command.status().map_err(|_| "Could not run Rinf command")?;
    let exit_code = exit_status
        .code()
        .ok_or("Could not get Rinf command exit code")?;
    if exit_code != 0 {
        process::exit(exit_code);
    }

    Ok(())
}

#[cfg(target_family = "wasm")]
fn main() -> Result<(), String> {
    // Dummy function to make the linter happy.
    Ok(())
}
