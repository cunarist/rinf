use crate::SetupError;
use std::ffi::OsStr;
use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn run_dart_command(args: &[&str]) -> Result<(), SetupError> {
  #[cfg(target_family = "windows")]
  let cmd = "dart.bat";
  #[cfg(target_family = "unix")]
  let cmd = "dart";
  run_subprocess(cmd, args)
}

pub fn run_subprocess(cmd: &str, args: &[&str]) -> Result<(), SetupError> {
  Command::new(cmd)
    .args(args)
    .stdout(Stdio::null())
    .output()?;
  Ok(())
}

pub fn check_internet_connection() -> bool {
  "pub.dev:80"
    .to_socket_addrs()
    .map(|mut addrs| addrs.next().is_some())
    .unwrap_or(false)
}

#[macro_export]
macro_rules! dimmedln {
  ($($arg:tt)*) => {
      println!("{}", owo_colors::OwoColorize::dimmed(&format!($($arg)*)));
  };
}

pub trait CleanFileName {
  fn clean_file_name(&self) -> Result<String, SetupError>;
}

impl CleanFileName for PathBuf {
  fn clean_file_name(&self) -> Result<String, SetupError> {
    let file_name = self
      .file_name()
      .and_then(OsStr::to_str)
      .ok_or_else(|| SetupError::BadFilePath(self.to_owned()))?
      .to_owned();
    Ok(file_name)
  }
}
