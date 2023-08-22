use std::{
    fmt::Debug,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result};
use log::{debug, info};
use semver::Version;

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

    fn target_dir(&self) -> &'static str {
        match self {
            Target::ArmV7 => "armeabi-v7a",
            Target::Arm64 => "arm64-v8a",
            Target::X86 => "x86",
            Target::X86_64 => "x86_64",
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
    let platforms = std::env::var("CARGOKIT_TARGET_PLATFORMS")
        .ok()
        .unwrap_or_else(|| "".into());
    platforms
        .split(',')
        .filter_map(Target::from_flutter_target)
        .collect()
}

fn is_release() -> bool {
    let configuration = std::env::var("CARGOKIT_BUILD_MODE")
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

// Workaround for libgcc missing in NDK23, inspired by cargo-ndk
fn libgcc_workaround(build_dir: &Path, ndk_version: &Version) -> Result<String> {
    let workaround_dir = build_dir
        .join("cargokit")
        .join("libgcc_workaround")
        .join(ndk_version.major.to_string());
    fs::create_dir_all(&workaround_dir)?;
    if ndk_version.major >= 23 {
        let mut file = std::fs::File::create(workaround_dir.join("libgcc.a"))?;
        file.write_all(b"INPUT(-lunwind)")?;
    } else {
        // Other way around, untested, forward libgcc.a from libunwind once Rust
        // gets updated for NDK23+.
        let mut file = std::fs::File::create(workaround_dir.join("libunwind.a"))?;
        file.write_all(b"INPUT(-lgcc)")?;
    }

    let mut rustflags = match std::env::var("CARGO_ENCODED_RUSTFLAGS") {
        Ok(val) => val,
        Err(std::env::VarError::NotPresent) => "".to_string(),
        Err(std::env::VarError::NotUnicode(_)) => {
            log::error!("RUSTFLAGS environment variable contains non-unicode characters");
            std::process::exit(1);
        }
    };

    if !rustflags.is_empty() {
        rustflags.push('\x1f');
    }
    rustflags.push_str("-L\x1f");
    rustflags.push_str(&workaround_dir.to_string_lossy());

    Ok(rustflags)
}

fn pick_existing(paths: Vec<PathBuf>) -> Option<PathBuf> {
    paths.into_iter().find(|p| p.exists())
}

fn install_ndk(sdk_path: &Path, ndk_version: &str, java_home: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    const SDK_MANAGER_EXTENSION: &str = ".bat";
    #[cfg(not(target_os = "windows"))]
    const SDK_MANAGER_EXTENSION: &str = "";

    let sdk_manager = sdk_path
        .join("cmdline-tools")
        .join("latest")
        .join("bin")
        .join(format!("sdkmanager{}", SDK_MANAGER_EXTENSION));

    let mut cmd = Command::new(sdk_manager);
    cmd.arg("--install")
        .arg(format!("ndk;{}", ndk_version))
        .env("JAVA_HOME", java_home);

    run_command(cmd)?;

    Ok(())
}

fn build_for_target(target: &Target) -> Result<()> {
    let min_version = string_from_env("CARGOKIT_MIN_SDK_VERSION")?;
    let min_version: i32 = min_version.parse()?;
    let min_version = min_version.max(target.min_sdk_version());

    let ndk_version = string_from_env("CARGOKIT_NDK_VERSION")?;

    let sdk_path = path_from_env("CARGOKIT_SDK_DIR")?;
    let ndk_path = sdk_path.join("ndk").join(&ndk_version);

    let ndk_package_xml = ndk_path.join("package.xml");

    if !ndk_package_xml.is_file() {
        info!("Installing NDK {}...", ndk_version);
        let java_home = string_from_env("CARGOKIT_JAVA_HOME")?;
        install_ndk(&sdk_path, &ndk_version, &java_home)?;
    }

    if !ndk_package_xml.is_file() {
        return Err(anyhow::format_err!(
            "NDK version {} failed to install into {}",
            ndk_version,
            ndk_path.display()
        ));
    }

    let toolchain_path = ndk_path
        .join("toolchains")
        .join("llvm")
        .join("prebuilt")
        .join(ARCH)
        .join("bin");

    let ar_key = format!("AR_{}", target.rust_target());
    let ar_value = pick_existing(vec![
        toolchain_path.join(format!("{}-ar", target.rust_target())),
        toolchain_path.join("llvm-ar"),
        toolchain_path.join("llvm-ar.exe"),
    ])
    .expect("Did not find ar tool");

    let target_arg = format!("--target={}{}", target.rust_target(), min_version);

    let cc_key = format!("CC_{}", target.rust_target());
    let cc_value = toolchain_path.join("clang");
    let cflags_key = format!("CFLAGS_{}", target.rust_target());
    let cflags_value = target_arg.clone();

    let cxx_key = format!("CXX_{}", target.rust_target());
    let cxx_value = toolchain_path.join("clang++");
    let cxx_flags_key = format!("CXXFLAGS_{}", target.rust_target());
    let cxx_flags_value = target_arg.clone();

    let rustflags_key = "CARGO_ENCODED_RUSTFLAGS";

    let linker_key = format!(
        "cargo_target_{}_linker",
        target.rust_target().replace('-', "_")
    )
    .to_ascii_uppercase();

    let ranlib_key = format!("RANLIB_{}", target.rust_target());
    let ranlib_value = toolchain_path.join("llvm-ranlib");

    let build_dir = path_from_env("CARGOKIT_BUILD_DIR")?;
    let output_dir = path_from_env("CARGOKIT_OUTPUT_DIR")?;
    let lib_name = string_from_env("CARGOKIT_LIB_NAME")?;

    let ndk_version = Version::parse(&ndk_version)?;
    let rust_flags_value = libgcc_workaround(&build_dir, &ndk_version)?;

    // Workaround for https://github.com/android/ndk/issues/1856
    // based on cargo-ndk solution.
    // https://github.com/bbqsrc/cargo-ndk/commit/d6cdbf4feef48ebea5eee8958e9c98431c3c5f32
    let self_path = std::fs::canonicalize(std::env::args().next().unwrap())
        .expect("Failed to canonicalize absolute path to build_gradle");

    debug!("ENV {}={}", ar_key, ar_value.display());
    debug!("ENV {}={}", cc_key, cc_value.display());
    debug!("ENV {}={}", cxx_key, cxx_value.display());
    debug!("ENV {}={}", linker_key, self_path.display());
    debug!("ENV {}={}", rustflags_key, rust_flags_value);
    debug!("ENV {}={}", ranlib_key, ranlib_value.display());
    debug!("ENV {}={}", cflags_key, cflags_value);
    debug!("ENV {}={}", cxx_flags_key, cxx_flags_value);

    let mut cmd = Command::new("rustup");
    cmd.arg("run");
    cmd.arg("stable");
    cmd.arg("cargo");
    cmd.arg("build");
    cmd.arg("--manifest-path");
    cmd.arg(path_from_env("CARGOKIT_MANIFEST_DIR")?.join("Cargo.toml"));
    cmd.arg("-p");
    cmd.arg(&lib_name);
    if is_release() {
        cmd.arg("--release");
    }
    cmd.arg("--target");
    cmd.arg(target.rust_target());
    cmd.arg("--target-dir");
    cmd.arg(&build_dir);

    cmd.env(ar_key, &ar_value);
    cmd.env(cc_key, &cc_value);
    cmd.env(cxx_key, &cxx_value);
    cmd.env(linker_key, &self_path);
    cmd.env(rustflags_key, &rust_flags_value);
    cmd.env(cflags_key, &cflags_value);
    cmd.env(cxx_flags_key, &cxx_flags_value);
    cmd.env("_CARGOKIT_NDK_LINK_TARGET", target_arg); // Recognized by main() so we know when we're acting as a wrapper
    cmd.env("_CARGOKIT_NDK_LINK_CLANG", &cc_value);

    run_command(cmd)?;

    let output_dir = output_dir.join(target.target_dir());
    fs::create_dir_all(&output_dir)?;

    let lib_name_full = format!("lib{}.so", lib_name);
    let src = build_dir
        .join(target.rust_target())
        .join(if is_release() { "release" } else { "debug" })
        .join(&lib_name_full);
    let dst = output_dir.join(&lib_name_full);
    fs::copy(&src, &dst)
        .with_context(|| format!("dst: {:?}", dst))
        .with_context(|| format!("src: {:?}", src))?;

    Ok(())
}

pub fn clang_linker_wrapper() -> ! {
    let args = std::env::args_os().skip(1);
    let clang = std::env::var("_CARGOKIT_NDK_LINK_CLANG")
        .expect("cargo-ndk rustc linker: didn't find _CARGOKIT_NDK_LINK_CLANG env var");
    let target = std::env::var("_CARGOKIT_NDK_LINK_TARGET")
        .expect("cargo-ndk rustc linker: didn't find _CARGOKIT_NDK_LINK_TARGET env var");

    let mut child = std::process::Command::new(&clang)
        .arg(target)
        .args(args)
        .spawn()
        .unwrap_or_else(|err| {
            eprintln!("cargokit: Failed to spawn {clang:?} as linker: {err}");
            std::process::exit(1)
        });
    let status = child.wait().unwrap_or_else(|err| {
        eprintln!("cargokit (as linker): Failed to wait for {clang:?} to complete: {err}");
        std::process::exit(1);
    });

    std::process::exit(status.code().unwrap_or(1))
}

pub fn build_gradle() -> Result<()> {
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
