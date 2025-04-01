use owo_colors::OwoColorize;

use crate::SetupError;
use std::ffi::OsStr;
use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::process::Output;

#[cfg(target_family = "windows")]
pub static DART_BIN: &str = "dart.bat";
#[cfg(target_family = "unix")]
pub static DART_BIN: &str = "dart";

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

pub trait CaptureError {
  fn capture_err(self) -> Result<(), SetupError>;
}

impl CaptureError for Output {
  fn capture_err(self) -> Result<(), SetupError> {
    if self.status.success() {
      Ok(())
    } else {
      eprintln!("{}", String::from_utf8_lossy(&self.stderr).red());
      Err(SetupError::SubprocessError)
    }
  }
}
