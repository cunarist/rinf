use crate::dimmedln;
use crate::tool::{SetupError, run_subprocess};
use std::path::Path;
use std::process::Command;

pub fn build_webassembly(
  root_dir: &Path,
  is_release_mode: bool,
  is_internet_connected: bool,
) -> Result<(), SetupError> {
  let total_steps = 3;
  let mut step = 0;

  step += 1;
  if is_internet_connected {
    dimmedln!(
      "[{}/{}] Installing Rust toolchain for the web",
      step,
      total_steps
    );
    install_wasm_toolchain()?;
  } else {
    dimmedln!(
      "[{}/{}] Skipping ensurement of Rust toolchain for the web",
      step,
      total_steps
    );
  }

  step += 1;
  dimmedln!(
    "[{}/{}] Preparing WebAssembly output path",
    step,
    total_steps
  );

  step += 1;
  dimmedln!(
    "[{}/{}] Compiling Rust with `wasm-pack` to `web` target",
    step,
    total_steps
  );
  compile_wasm(root_dir, is_release_mode)?;

  Ok(())
}

fn install_wasm_toolchain() -> Result<(), SetupError> {
  run_subprocess("rustup", &["toolchain", "install", "nightly"])?;
  run_subprocess("rustup", &["+nightly", "component", "add", "rust-src"])?;
  run_subprocess(
    "rustup",
    &["+nightly", "target", "add", "wasm32-unknown-unknown"],
  )?;
  run_subprocess("rustup", &["target", "add", "wasm32-unknown-unknown"])?;
  run_subprocess("cargo", &["install", "wasm-pack"])?;
  run_subprocess("cargo", &["install", "wasm-bindgen-cli"])?;
  Ok(())
}

fn compile_wasm(
  root_dir: &Path,
  is_release_mode: bool,
) -> Result<(), SetupError> {
  let out_path = root_dir.join("web").join("pkg");
  let out_string = out_path
    .to_str()
    .ok_or_else(|| SetupError::BadFilePath(out_path.clone()))?;
  let mut wasm_pack_args = vec![
    "--quiet",
    "build",
    "native/hub",
    "--out-dir",
    out_string,
    "--out-name",
    "hub",
    "--no-typescript",
    "--target",
    "web",
    "--",
    "-Z",
    "build-std=std,panic_abort",
  ];
  if !is_release_mode {
    wasm_pack_args.insert(7, "--dev");
  }

  let status = Command::new("wasm-pack")
    .args(&wasm_pack_args)
    .env("RUSTUP_TOOLCHAIN", "nightly")
    .env(
      "RUSTFLAGS",
      "-C target-feature=+atomics,+bulk-memory,+mutable-globals",
    )
    .status()?;

  if !status.success() {
    panic!("Wasm compilation failed");
  }

  Ok(())
}
