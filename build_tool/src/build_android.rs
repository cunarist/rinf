use std::{fmt::Debug, fs, process::Command};

use anyhow::Result;
use log::{debug, info};

use super::utils::*;

#[derive(Debug)]
enum Target {
    ArmV7,
    Arm64,
    X86,
    X86_64,
}

impl Target {
    fn rust_target(&self) -> &'static str {
        match self {
            Target::ArmV7 => "armv7-linux-androideabi",
            Target::Arm64 => "aarch64-linux-android",
            Target::X86 => "i686-linux-android",
            Target::X86_64 => "x86_64-linux-android",
        }
    }

    fn ndk_prefix(&self) -> &'static str {
        match self {
            Target::ArmV7 => "armv-linux-androideabi",
            Target::Arm64 => "aarch64-linux-android",
            Target::X86 => "i686-linux-android",
            Target::X86_64 => "x86_64-linux-android",
        }
    }

    fn target_dir(&self) -> &'static str {
        match self {
            Target::ArmV7 => "armeabi-v7a",
            Target::Arm64 => "arm64-v8a",
            Target::X86 => "x86",
            Target::X86_64 => "x86_64",
        }
    }

    fn ndk_prefix_clang(&self) -> &'static str {
        match self {
            Target::ArmV7 => "armv7a-linux-androideabi",
            Target::Arm64 => "aarch64-linux-android",
            Target::X86 => "i686-linux-android",
            Target::X86_64 => "x86_64-linux-android",
        }
    }

    fn min_sdk_version(&self) -> i32 {
        match self {
            Target::ArmV7 => 16,
            Target::Arm64 => 21,
            Target::X86 => 16,
            Target::X86_64 => 21,
        }
    }

    fn from_flutter_target(s: &str) -> Option<Target> {
        match s {
            "android-arm" => Some(Target::ArmV7),
            "android-arm64" => Some(Target::Arm64),
            "android-x86" => Some(Target::X86),
            "android-x64" => Some(Target::X86_64),
            _ => None,
        }
    }
}

fn get_targets() -> Vec<Target> {
    let platforms = std::env::var("TOOLBOX_TARGET_PLATFORMS")
        .ok()
        .unwrap_or_else(|| "".into());
    platforms
        .split(',')
        .into_iter()
        .map(Target::from_flutter_target)
        .flatten()
        .collect()
}

fn is_release() -> bool {
    let configuration = std::env::var("TOOLBOX_BUILD_MODE")
        .ok()
        .unwrap_or_else(|| "release".into());
    configuration != "debug"
}

#[cfg(target_os = "macos")]
const ARCH: &str = "darwin-x86_64";
#[cfg(target_os = "linux")]
const ARCH: &str = "linux-x86_64";
#[cfg(target_os = "windows")]
const ARCH: &str = "windows-x86_64";

#[cfg(target_os = "windows")]
const CLANG_TOOL_EXTENSION: &str = ".cmd";

#[cfg(not(target_os = "windows"))]
const CLANG_TOOL_EXTENSION: &str = "";

fn build_for_target(target: &Target) -> Result<()> {
    let min_version = string_from_env("TOOLBOX_MIN_SDK_VERSION")?;
    let min_version: i32 = min_version.parse()?;
    let min_version = min_version.max(target.min_sdk_version());

    let toolchain_path = path_from_env("TOOLBOX_NDK_DIR")?
        .join("toolchains")
        .join("llvm")
        .join("prebuilt")
        .join(ARCH)
        .join("bin");

    let ar_key = format!("AR_{}", target.rust_target());
    let ar_value = toolchain_path.join(format!("{}-ar", target.ndk_prefix()));

    let cc_key = format!("CC_{}", target.rust_target());
    let cc_value = toolchain_path.join(format!(
        "{}{}-clang{}",
        target.ndk_prefix_clang(),
        min_version,
        CLANG_TOOL_EXTENSION
    ));

    let cxx_key = format!("CXX_{}", target.rust_target());
    let cxx_value = toolchain_path.join(format!(
        "{}{}-clang++{}",
        target.ndk_prefix_clang(),
        min_version,
        CLANG_TOOL_EXTENSION
    ));

    let linker_key = format!(
        "cargo_target_{}_linker",
        target.rust_target().replace("-", "_")
    )
    .to_ascii_uppercase();
    let linker_value = cc_value.clone();

    debug!("ENV {}={}", ar_key, ar_value.display());
    debug!("ENV {}={}", cc_key, cc_value.display());
    debug!("ENV {}={}", cxx_key, cxx_value.display());
    debug!("ENV {}={}", linker_key, linker_value.display());

    let build_dir = path_from_env("TOOLBOX_BUILD_DIR")?;
    let output_dir = path_from_env("TOOLBOX_OUTPUT_DIR")?;
    let lib_name = string_from_env("TOOLBOX_LIB_NAME")?;

    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    cmd.arg("--manifest-path");
    cmd.arg(path_from_env("TOOLBOX_MANIFEST_DIR")?.join("Cargo.toml"));
    cmd.arg("-p");
    cmd.arg(&lib_name);
    if is_release() {
        cmd.arg("--release");
    }
    cmd.arg("--target");
    cmd.arg(target.rust_target());
    cmd.arg("--target-dir");
    cmd.arg(&build_dir);

    cmd.env(ar_key, ar_value);
    cmd.env(cc_key, cc_value);
    cmd.env(cxx_key, cxx_value);
    cmd.env(linker_key, linker_value);

    run_command(cmd)?;

    let output_dir = output_dir.join(target.target_dir());
    fs::create_dir_all(&output_dir)?;

    let lib_name_full = format!("lib{}.so", lib_name);
    fs::copy(
        build_dir
            .join(target.rust_target())
            .join(if is_release() { "release" } else { "debug" })
            .join(&lib_name_full),
        output_dir.join(&lib_name_full),
    )?;

    Ok(())
}

pub fn build_android() -> Result<()> {
    let targets = get_targets();
    debug!("Building for targets: {:?}", targets);

    let installed_targets = installed_targets()?;
    for target in &targets {
        if !installed_targets.contains(&target.rust_target().to_owned()) {
            info!("Installing target {}...", target.rust_target());
            install_target(target.rust_target())?;
        }
    }

    for target in &targets {
        debug!("Building for {}...", target.rust_target());
        build_for_target(target)?;
    }

    Ok(())
}
