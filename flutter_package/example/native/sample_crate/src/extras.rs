//! This crate is written for Rinf demonstrations.

use crate::error::ExampleError;

// `machineid_rs` only supports desktop platforms.
#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
pub fn get_hardward_id() -> Result<String, ExampleError> {
  let mut builder = machineid_rs::IdBuilder::new(machineid_rs::Encryption::MD5);
  builder
    .add_component(machineid_rs::HWIDComponent::SystemID)
    .add_component(machineid_rs::HWIDComponent::CPUCores);
  let hwid = builder
    .build("mykey")
    .map_err(|_| ExampleError::HardwareId)?;
  Ok(hwid)
}
#[cfg(not(any(
  target_os = "windows",
  target_os = "macos",
  target_os = "linux"
)))]
pub fn get_hardward_id() -> Result<String, ExampleError> {
  Ok(String::from("UNSUPPORTED"))
}

// `chrono` supports all platforms, including web.
use chrono::{DateTime, offset};
pub fn get_current_time() -> DateTime<offset::Local> {
  offset::Local::now()
}

// `reqwest` supports all platforms, including web.
pub async fn fetch_from_web_api(url: &str) -> Result<String, ExampleError> {
  let fetched = reqwest::get(url)
    .await
    .map_err(|_| ExampleError::WebApi)?
    .text()
    .await
    .map_err(|_| ExampleError::WebApi)?;
  Ok(fetched)
}
