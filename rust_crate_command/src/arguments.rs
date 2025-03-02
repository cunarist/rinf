use crate::{
    generate_dart_code, load_verified_rinf_config, RinfCommandError,
    RinfConfigMessage,
};
use clap::{Arg, ArgAction, Command};
use owo_colors::OwoColorize;

// TODO: Remove string-based paths

pub fn run_command() -> Result<(), RinfCommandError> {
    // Check the internet connection status and remember it.
    check_connectivity();

    let matches = Command::new("rinf")
        .about("Helper commands for building apps using Rust in Flutter.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("config")
                .about("Show Rinf configuration resolved from `pubspec.yaml`."),
        )
        .subcommand(
            Command::new("template")
                .about("Apply Rust template to the current Flutter project."),
        )
        .subcommand(
            Command::new("gen")
                .about("Generate Dart code from Rust code with attributes.")
                .arg(
                    Arg::new("watch")
                        .short('w')
                        .long("watch")
                        .help("Continuously watches Rust files")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("wasm")
                .about("Build the WebAssembly module for the web.")
                .arg(
                    Arg::new("release")
                        .short('r')
                        .long("release")
                        .help("Builds in release mode")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("server")
                .about("Show how to run Flutter web server with web headers."),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("config", _)) => {
            let rinf_config = load_verified_rinf_config("pubspec.yaml")?;
            println!("{}", rinf_config.dimmed());
        }
        Some(("template", _)) => {
            let rinf_config = load_verified_rinf_config("pubspec.yaml")?;
            apply_rust_template(&rinf_config.message);
        }
        Some(("gen", sub_m)) => {
            let watch = sub_m.get_flag("watch");
            let rinf_config = load_verified_rinf_config("pubspec.yaml")?;
            if watch {
                watch_and_generate_dart_code(&rinf_config.message);
            } else {
                // TODO: Pass `rinf_config.message` to this function.
                generate_dart_code();
            }
        }
        Some(("wasm", sub_m)) => {
            let release = sub_m.get_flag("release");
            build_webassembly(release);
        }
        Some(("server", _)) => {
            let full_command = "flutter run \
                --web-header=Cross-Origin-Opener-Policy=same-origin \
                --web-header=Cross-Origin-Embedder-Policy=require-corp";
            println!("{}", full_command);
        }
        _ => unreachable!(), // TODO: Remove this unreachable
    }

    Ok(())
}

// TODO: Implement all of these.

fn check_connectivity() {
    // Implement internet connectivity check logic.
}

fn apply_rust_template(_message_config: &RinfConfigMessage) {
    // Implement Rust template application logic.
}

fn watch_and_generate_dart_code(_message_config: &RinfConfigMessage) {
    // Implement watching and message code generation logic.
}

fn build_webassembly(_release: bool) {
    // Implement WebAssembly build logic.
}
