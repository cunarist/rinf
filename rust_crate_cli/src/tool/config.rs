use crate::SetupError;
use serde::Deserialize;
use serde_yml::{Value, from_str, from_value};
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::Path;

/// Rinf message configuration structure.
#[derive(Deserialize)]
pub struct RinfConfig {
  #[serde(default = "create_default_input_crates")]
  pub input_crates: Vec<String>,
  #[serde(default = "create_default_dart_output_dir")]
  pub dart_output_dir: String,
}

impl Display for RinfConfig {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "message:\
      \n  input_crates: {}\
      \n  dart_output_dir: {}",
      self.input_crates.join(", "),
      self.dart_output_dir,
    )
  }
}

impl Default for RinfConfig {
  fn default() -> Self {
    Self {
      input_crates: create_default_input_crates(),
      dart_output_dir: create_default_dart_output_dir(),
    }
  }
}

fn create_default_input_crates() -> Vec<String> {
  vec!["hub".to_owned()]
}

fn create_default_dart_output_dir() -> String {
  "lib/src/bindings".to_owned()
}

/// Attempts to load the rinf configuration from the provided pubspec.yaml file.
/// If no rinf configuration is found, the default configuration is returned.
/// If the Rinf configuration is invalid, an exception is thrown.
/// Otherwise it loads all values found in the config, using defaults for missing values.
pub fn load_verified_rinf_config(
  root_dir: &Path,
) -> Result<RinfConfig, SetupError> {
  let file_path = root_dir.join("pubspec.yaml");
  let content = read_to_string(file_path)?;
  let file_yaml: Value = from_str(&content)?;
  let rinf_yaml = file_yaml
    .as_mapping()
    .ok_or(SetupError::PubConfig("Parsing failed".to_owned()))?
    .get("rinf")
    .cloned();
  let config = match rinf_yaml {
    Some(map) => from_value(map)?,
    None => RinfConfig::default(),
  };
  Ok(config)
}

pub fn read_publish_to(file_path: &Path) -> Option<String> {
  let content = std::fs::read_to_string(file_path).ok()?;
  let file_yaml: Value = from_str(&content).ok()?;
  let field_value = file_yaml.as_mapping()?.get("publish_to")?;
  let config = field_value.as_str()?.to_string();
  Some(config)
}
