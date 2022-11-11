use std::{fs, process::Command};

use anyhow::Result;

use super::utils::*;
use log::info;

enum Target {
    WindowsX64,
    LinuxX64,
    LinuxArm64,
}

impl Target {
    fn rust_target(&self) -> &'static str {
        match self {
            Target::WindowsX64 => "x86_64-pc-windows-msvc",
            Target::LinuxX64 => "x86_64-unknown-linux-gnu",
            Target::LinuxArm64 => "aarch64-unknown-linux-gnu",
        }
    }

    fn from_flutter_target(s: &str) -> Option<Target> {
        match s {
            "windows-x64" => Some(Target::WindowsX64),
            "linux-x64" => Some(Target::LinuxX64),
            "linux-arm64" => Some(Target::LinuxArm64),
            _ => None,
        }
    }
}

fn is_release() -> bool {
    let configuration = std::env::var("CARGOKIT_CONFIGURATION")
        .ok()
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_else(|| "release".into());
    configuration != "debug"
}

pub fn build_cmake() -> Result<()> {
    let target = string_from_env("CARGOKIT_TARGET_PLATFORM")?;
    let target = Target::from_flutter_target(&target)
        .ok_or_else(|| anyhow::anyhow!("Invalid target {:?}", target))?;
    let installed_targets = installed_targets()?;
    if !installed_targets.contains(&target.rust_target().to_owned()) {
        info!("Installing target {}...", target.rust_target());
        install_target(target.rust_target())?;
    }

    let build_dir = path_from_env("CARGOKIT_BUILD_DIR")?;
    let output_dir = path_from_env("CARGOKIT_TARGET_DIR")?;
    let lib_name = string_from_env("CARGOKIT_LIB_NAME")?;
    let manifest_path = path_from_env("CARGOKIT_MANIFEST_DIR")?
        .join("Cargo.toml")
        .canonicalize()?;

    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    cmd.arg("--manifest-path");
    cmd.arg(manifest_path);
    cmd.arg("-p");
    cmd.arg(&lib_name);
    if is_release() {
        cmd.arg("--release");
    }
    cmd.arg("--target");
    cmd.arg(target.rust_target());
    cmd.arg("--target-dir");
    cmd.arg(&build_dir);

    run_command(cmd)?;

    let src_dir =
        build_dir
            .join(target.rust_target())
            .join(if is_release() { "release" } else { "debug" });

    let files = [
        format!("lib{}.so", lib_name),
        format!("{}.dll", lib_name),
        format!("{}.dll.lib", lib_name),
        format!("{}.pdb", lib_name),
    ];

    for file in files {
        let path = src_dir.join(&file);
        if path.exists() {
            fs::copy(path, output_dir.join(&file))?;
        }
    }

    Ok(())
}
