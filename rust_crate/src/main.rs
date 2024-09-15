#[cfg(not(target_family = "wasm"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use std::fs;
    use std::path;
    use std::process;

    // Ensure Protobuf compiler.
    let protoc_path = if let Ok(installed) = which::which("protoc") {
        // Get the path of Protobuf compiler that's already installed.
        println!("Detected `protoc`, skipping auto installation.");
        installed
            .parent()
            .ok_or("Could not get the parent of `protoc` path.")?
            .to_path_buf()
    } else {
        // Install Protobuf compiler and get the path.
        let home_path = home::home_dir()
            .ok_or("Could not get home directory for `protoc` installation.")?;
        let out_path = home_path.join(".local").join("bin");
        fs::create_dir_all(&out_path)?;
        env::set_var(
            "OUT_DIR",
            out_path
                .to_str()
                .ok_or("Could not set the path for `protoc` installation.")?,
        );
        let install_result = protoc_prebuilt::init("25.2");
        let (protoc_binary_path, _) = install_result.map_err(|_| {
            format!(
                "{}\n{}",
                "Automatic installation of `protoc` failed.",
                "Try installing `protoc` manually and adding it to PATH."
            )
        })?;
        protoc_binary_path
            .parent()
            .ok_or("Could not get the parent of installed `protoc` path.")?
            .to_path_buf()
    };

    // Find the path where Dart executables are located.
    #[cfg(target_family = "windows")]
    let pub_cache_bin_path = path::PathBuf::from(env::var("LOCALAPPDATA")?)
        .join("Pub")
        .join("Cache")
        .join("bin");
    #[cfg(target_family = "unix")]
    let pub_cache_bin_path = path::PathBuf::from(env::var("HOME")?)
        .join(".pub-cache")
        .join("bin");

    // Add some folders to PATH for various commands to work correctly.
    let mut path_var = match env::var_os("PATH") {
        Some(val) => env::split_paths(&val).collect::<Vec<_>>(),
        None => Vec::new(),
    };
    path_var.push(protoc_path);
    path_var.push(pub_cache_bin_path);
    env::set_var("PATH", env::join_paths(path_var)?);

    // Get command-line arguments excluding the program name.
    let dart_command_args: Vec<String> = env::args().skip(1).collect();

    // Run the Dart script.
    let dart_path = which::which("dart")?;
    let mut command = process::Command::new(dart_path);
    command.args(["run", "rinf"]);
    command.args(&dart_command_args);
    command.status()?;

    Ok(())
}

#[cfg(target_family = "wasm")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Dummy function to make the linter happy.
    Ok(())
}
