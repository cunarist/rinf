use std::{env::args, process::exit};

use anyhow::Result;
use build_gradle::clang_linker_wrapper;
use log::{debug, LevelFilter};

mod build_gradle;
mod build_cmake;
mod build_pod;
mod logger;
mod utils;

fn init_logging() -> Result<()> {
    let silent = std::env::var("CARGOKIT_SILENT")
        .ok()
        .unwrap_or_else(|| "".into());
    let level = if silent == "1" {
        LevelFilter::Info
    } else {
        LevelFilter::Debug
    };
    logger::init_with_level(level)
        .map_err(|e| anyhow::format_err!("Failed to setup logger: {}", e))?;
    Ok(())
}

fn dump_environment() -> Result<()> {
    debug!("CargoKit environment:");
    for var in std::env::vars() {
        if var.0.to_ascii_lowercase().starts_with("cargokit_") {
            debug!("{}={}", var.0, var.1);
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    init_logging()?;
    dump_environment()?;

    if std::env::var("_CARGOKIT_NDK_LINK_TARGET").is_ok() {
        clang_linker_wrapper();
    }

    let mut args = args();
    args.next(); // executable

    let command = args
        .next()
        .ok_or_else(|| anyhow::format_err!("Missing argument"))?;

    match command.as_str() {
        "build_pod" => build_pod::build_pod(args),
        "build_gradle" => build_gradle::build_gradle(),
        "build_cmake" => build_cmake::build_cmake(),
        command => Err(anyhow::format_err!("Invalid command: {}", command)),
    }
}

fn main() {
    let res = run();
    if let Err(error) = res {
        eprintln!("Build tool failed:\n{:?}", error);
        exit(1);
    }
}
