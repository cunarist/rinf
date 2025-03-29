use crate::tool::{
  SetupError, apply_rust_template, build_webassembly,
  check_internet_connection, generate_dart_code, load_verified_rinf_config,
  provide_server_command, read_publish_to, watch_and_generate_dart_code,
};
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use std::env::current_dir;
use std::path::Path;

#[derive(Parser)]
#[command(about = "CLI tool for building apps using Rust in Flutter")]
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
  match cli.command {
    CliCommand::Config => {
      let rinf_config = load_verified_rinf_config(&root_dir)?;
      println!("{}", rinf_config.dimmed());
    }
    CliCommand::Template => {
      let rinf_config = load_verified_rinf_config(&root_dir)?;
      apply_rust_template(&root_dir, &rinf_config)?;
      println!("Rust template is now ready 🎉");
    }
    CliCommand::Gen { watch } => {
      let rinf_config = load_verified_rinf_config(&root_dir)?;
      if watch {
        watch_and_generate_dart_code(&root_dir, &rinf_config)?;
      } else {
        generate_dart_code(&root_dir, &rinf_config)?;
        println!("Dart signal classes were generated successfully 🎉");
      }
    }
    CliCommand::Wasm { release } => {
      build_webassembly(&root_dir, release, is_internet_connected)?;
      println!(
        "WebAssembly module has been built and is ready in `web/pkg` 🎉"
      );
      println!(
        "{}",
        "To get the Flutter web server command, run `rinf server`".dimmed()
      );
    }
    CliCommand::Server => {
      provide_server_command()?;
      let full_guide = concat!(
        "Full `flutter run` command for the web",
        " has been copied to the clipboard"
      );
      println!("{}", full_guide.dimmed());
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
