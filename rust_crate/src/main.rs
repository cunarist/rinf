use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    dart_args: Vec<String>,
    #[arg(short, long)]
    protoc_path: Option<String>,
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    use std::env;
    use std::fs;
    use std::path;
    use std::path::PathBuf;
    use std::process;
    use std::str::FromStr;

    let args = Args::parse();

    let protoc_path;

    if let Some(p) = args.protoc_path {
        let provided = PathBuf::from_str(&p).unwrap();
        protoc_path = provided;
    } else {
        // Install Protobuf compiler and get the path.
        let home_path = home::home_dir().unwrap();
        let out_path = home_path.join(".local").join("bin");
        fs::create_dir_all(&out_path).unwrap();
        env::set_var("OUT_DIR", out_path.to_str().unwrap());
        let (protoc_binary_path, _) = protoc_prebuilt::init("25.2").unwrap();
        protoc_path = protoc_binary_path.parent().unwrap().to_path_buf();
    }

    // Find the path where Dart executables are located.
    #[cfg(target_family = "windows")]
    let pub_cache_bin_path = path::PathBuf::from(env::var("LOCALAPPDATA").unwrap())
        .join("Pub")
        .join("Cache")
        .join("bin");
    #[cfg(target_family = "unix")]
    let pub_cache_bin_path = path::PathBuf::from(env::var("HOME").unwrap())
        .join(".pub-cache")
        .join("bin");

    // Add some folders to PATH for various commands to work correctly.
    let mut path_var = match env::var_os("PATH") {
        Some(val) => env::split_paths(&val).collect::<Vec<_>>(),
        None => Vec::new(),
    };
    path_var.push(protoc_path);
    path_var.push(pub_cache_bin_path);
    env::set_var("PATH", env::join_paths(path_var).unwrap());

    // Build the command to run the Dart script.
    #[cfg(target_family = "windows")]
    let mut command = process::Command::new("dart.bat");
    #[cfg(target_family = "unix")]
    let mut command = process::Command::new("dart");
    command.args(["run", "rinf"]);
    command.args(&args.dart_args);

    // Execute the command
    let _ = command.status();
}

#[cfg(target_family = "wasm")]
fn main() {
    // Dummy function to make the linter happy.
}
