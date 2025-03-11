use crate::{
    SetupError, apply_rust_template, build_webassembly,
    check_internet_connection, generate_dart_code, load_verified_rinf_config,
    watch_and_generate_dart_code,
};
use arboard::Clipboard;
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use serde_yml::Value;
use std::env::current_dir;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "CLI tool for building apps using Rust in Flutter"
)]
struct CliInput {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    /// Show Rinf configuration resolved from `pubspec.yaml`
    Config,

    /// Apply Rust template to the current Flutter project
    Template,

    /// Generate Dart code from Rust structs with attributes
    Gen {
        /// Continuously watches Rust files
        #[arg(short, long)]
        watch: bool,
    },

    /// Build the WebAssembly module for the web
    Wasm {
        /// Builds in release mode
        #[arg(short, long)]
        release: bool,
    },

    /// Get full `flutter run` command with web headers
    Server,
}

pub fn run_command() -> Result<(), SetupError> {
    // Check the internet connection status and remember it.
    let is_internet_connected = check_internet_connection();

    // Check if the current directory is Flutter app's root.
    let root_dir = current_dir()?;
    if !is_flutter_app_project(&root_dir) {
        Err(SetupError::ProjectStructure(
            "This is not a Flutter app project",
        ))?;
    }

    // Run a command from user input.
    let cli = CliInput::parse();
    match &cli.command {
        CliCommand::Config => {
            let rinf_config = load_verified_rinf_config(&root_dir)?;
            println!("{}", rinf_config.dimmed());
        }
        CliCommand::Template => {
            let rinf_config = load_verified_rinf_config(&root_dir)?;
            apply_rust_template(&root_dir, &rinf_config.message)?;
        }
        CliCommand::Gen { watch } => {
            let rinf_config = load_verified_rinf_config(&root_dir)?;
            if *watch {
                watch_and_generate_dart_code(&root_dir, &rinf_config.message)?;
            } else {
                generate_dart_code(&root_dir, &rinf_config.message)?;
            }
        }
        CliCommand::Wasm { release } => {
            build_webassembly(&root_dir, *release, is_internet_connected)?;
        }
        CliCommand::Server => {
            provide_server_command()?;
        }
    }

    Ok(())
}

fn is_flutter_app_project(root_dir: &Path) -> bool {
    let spec_file = root_dir.join("pubspec.yaml");
    let Some(publish_to) = read_publish_to(&spec_file) else {
        // If the field is not readable,
        // just treat this directory as a Flutter app project.
        return true;
    };
    publish_to == "none"
}

fn read_publish_to(file_path: &PathBuf) -> Option<String> {
    let content = std::fs::read_to_string(file_path).ok()?;
    let yaml: Value = serde_yml::from_str(&content).ok()?;
    let field_value = yaml.as_mapping()?.get("publish_to")?;
    let config = field_value.as_str()?.to_string();
    Some(config)
}

fn provide_server_command() -> Result<(), SetupError> {
    let mut clipboard = Clipboard::new()?;
    let full_command = concat!(
        "flutter run",
        " --web-header=Cross-Origin-Opener-Policy=same-origin",
        " --web-header=Cross-Origin-Embedder-Policy=require-corp",
    );
    clipboard.set_text(full_command)?;
    let full_guide =
        "Full `flutter run` command for the web copied to clipboard";
    println!("{}", full_guide.dimmed());
    Ok(())
}
