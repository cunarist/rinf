use crate::dimmedln;
use crate::tool::{CaptureError, SetupError};
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
  Command::new("rustup")
    .args(["toolchain", "install", "nightly"])
    .output()?
    .capture_err()?;
  Command::new("rustup")
    .args(["+nightly", "component", "add", "rust-src"])
    .output()?
    .capture_err()?;
  Command::new("rustup")
    .args(["+nightly", "target", "add", "wasm32-unknown-unknown"])
    .output()?
    .capture_err()?;
  Command::new("rustup")
    .args(["target", "add", "wasm32-unknown-unknown"])
    .output()?
    .capture_err()?;
  Command::new("cargo")
    .args(["install", "wasm-pack"])
    .output()?
    .capture_err()?;
  Command::new("cargo")
    .args(["install", "wasm-bindgen-cli"])
    .output()?
    .capture_err()?;
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
    "-Zbuild-std=std,panic_abort",
  ];
  if !is_release_mode {
    wasm_pack_args.insert(7, "--dev");
  }

  Command::new("wasm-pack")
    .args(&wasm_pack_args)
    .env("RUSTUP_TOOLCHAIN", "nightly")
    .env(
      "RUSTFLAGS",
      concat!(
        "-C target-feature=+atomics,+bulk-memory,+mutable-globals ",
        "-C link-arg=--shared-memory ",
        "-C link-arg=--max-memory=1073741824 ",
        "-C link-arg=--import-memory ",
        "-C link-arg=--export=__wasm_init_tls ",
        "-C link-arg=--export=__tls_size ",
        "-C link-arg=--export=__tls_align ",
        "-C link-arg=--export=__tls_base",
      ),
    )
    .output()?
    .capture_err()?;

  Ok(())
}
