use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // Get the list of `.proto` files.
    let proto_folder = Path::new("../../messages");
    #[allow(clippy::if_same_then_else)]
    let proto_filenames: Vec<String> = fs::read_dir(proto_folder)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let entry = entry.expect("Error reading directory entry");
            let path = entry.path();
            if path.is_dir() {
                None
            } else if path.extension().map(|ext| ext != "proto").unwrap_or(true) {
                None
            } else {
                Some(entry.file_name().to_str().unwrap().to_owned())
            }
        })
        .collect();

    // Watch original `.proto` message files.
    println!("cargo:rerun-if-changed=../../messages");
    for proto_filename in &proto_filenames {
        println!("cargo:rerun-if-changed=../../messages/{proto_filename}");
    }

    // Generate Rust message files.
    let output_path = "src/messages";
    let _ = fs::create_dir(output_path);
    for result in fs::read_dir(output_path).unwrap() {
        let entry_path = result.unwrap().path();
        if entry_path.is_dir() {
            fs::remove_dir_all(&entry_path).unwrap();
        } else {
            fs::remove_file(&entry_path).unwrap();
        }
    }
    let result = tonic_build::configure()
        .out_dir("src/messages")
        .compile(&proto_filenames, &["../../messages"]);
    result.expect("Could not compile `.proto` files into Rust");

    // Generate `mod.rs` for `messages` module in Rust.
    let module_folder = Path::new("src/messages");
    #[allow(clippy::if_same_then_else)]
    let message_files = fs::read_dir(module_folder)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let entry = entry.expect("Error reading directory entry");
            let path = entry.path();
            if path.is_dir() {
                None
            } else if path.extension().map(|ext| ext != "rs").unwrap_or(true) {
                None
            } else if path.file_name().unwrap() == "mod.rs" {
                None
            } else {
                Some(path)
            }
        });
    let mod_rs_content = message_files
        .map(|path| {
            let file_stem = path.file_stem().unwrap().to_string_lossy();
            format!("pub mod {};", file_stem)
        })
        .collect::<Vec<_>>()
        .join("\n");
    let mod_rs_path = PathBuf::from(module_folder).join("mod.rs");
    fs::write(mod_rs_path, mod_rs_content).expect("Failed to write mod.rs");

    // Install `protoc_plugin` for Dart.
    let dart_path = which::which("dart").unwrap(); // https://github.com/rust-lang/rust/issues/37519
    let mut command = Command::new(dart_path);
    command.args(["pub", "global", "activate", "protoc_plugin"]);
    command
        .output()
        .expect("Cannot globally install `protoc_plugin` Dart package");

    // Generate Dart message files.
    let output_path = "../../lib/messages";
    let _ = fs::create_dir(output_path);
    for result in fs::read_dir(output_path).unwrap() {
        let entry_path = result.unwrap().path();
        if entry_path.is_dir() {
            fs::remove_dir_all(&entry_path).unwrap();
        } else {
            fs::remove_file(&entry_path).unwrap();
        }
    }
    let working_directory = env::current_dir().unwrap();
    let flutter_project_path = working_directory.parent().unwrap().parent().unwrap();
    let mut command = Command::new("protoc");
    command.current_dir(flutter_project_path);
    command.args([
        "--proto_path=messages",
        "--dart_out=lib/messages",
        "--fatal_warnings",
    ]);
    command.args(proto_filenames);
    let result = command.output();
    result.expect("Could not compile `.proto` files into Dart");
}
