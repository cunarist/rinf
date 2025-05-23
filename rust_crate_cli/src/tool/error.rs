use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

/// Error type for Rinf configuration loading.
#[derive(Debug)]
pub enum SetupError {
  // Below are automatically converted variants.
  Io(std::io::Error),
  Yaml(serde_yml::Error),
  Clipboard(arboard::Error),
  WatchingFile(notify::Error),
  // Below are manually constructed variants.
  ReflectionModule,
  PubConfig(String),
  BadFilePath(PathBuf),
  NotFlutterApp,
  TemplateApplied,
  DuplicatedSignal(String),
  CodeSyntax(String),
  SubprocessError,
}

impl Error for SetupError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      Self::Io(e) => Some(e),
      Self::Yaml(e) => Some(e),
      Self::Clipboard(e) => Some(e),
      Self::WatchingFile(e) => Some(e),
      _ => None,
    }
  }
}

impl Display for SetupError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Io(e) => {
        write!(f, "Failed to operate on file: {}", e)
      }
      Self::Yaml(e) => {
        write!(f, "Failed to parse YAML: {}", e)
      }
      Self::Clipboard(e) => {
        write!(f, "Failed to use clipboard: {}", e)
      }
      Self::WatchingFile(e) => {
        write!(f, "Failed to watch files: {}", e)
      }
      Self::ReflectionModule => {
        write!(f, "Failed to write reflection modules")
      }
      Self::PubConfig(s) => {
        write!(f, "Invalid `pubspec.yaml` config: {}", s)
      }
      Self::BadFilePath(p) => {
        write!(f, "Invalid file path: `{}`", p.display())
      }
      Self::NotFlutterApp => {
        write!(f, "This is not a Flutter app project")
      }
      Self::TemplateApplied => {
        write!(f, "Rust template has already been applied")
      }
      Self::DuplicatedSignal(n) => {
        write!(f, "Duplicated signals named `{}` were found", n)
      }
      Self::CodeSyntax(n) => {
        write!(f, "Invalid syntax in file `{}`", n)
      }
      Self::SubprocessError => {
        write!(f, "A subprocess did not exit successfully")
      }
    }
  }
}

impl From<std::io::Error> for SetupError {
  fn from(err: std::io::Error) -> Self {
    Self::Io(err)
  }
}

impl From<serde_yml::Error> for SetupError {
  fn from(err: serde_yml::Error) -> Self {
    Self::Yaml(err)
  }
}

impl From<arboard::Error> for SetupError {
  fn from(err: arboard::Error) -> Self {
    Self::Clipboard(err)
  }
}

impl From<notify::Error> for SetupError {
  fn from(err: notify::Error) -> Self {
    Self::WatchingFile(err)
  }
}
