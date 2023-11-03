//! This crate is only for demonstration purposes.
//! You might want to remove this crate in production.

pub use mandelbrot::{mandelbrot, Point, Size};

mod mandelbrot;

// This is just a simple Rust function.

pub fn add_seven(before: i32) -> i32 {
    before + 7
}

// `machineid_rs` only supports desktop platforms.

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
pub fn get_hardward_id() -> Option<String> {
    let mut builder = machineid_rs::IdBuilder::new(machineid_rs::Encryption::MD5);
    builder
        .add_component(machineid_rs::HWIDComponent::SystemID)
        .add_component(machineid_rs::HWIDComponent::CPUCores);
    let hwid = builder.build("mykey").unwrap();
    Some(hwid)
}
#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn get_hardward_id() -> Option<String> {
    None
}

// `chrono` supports all platforms, including web.

use chrono::{offset, DateTime};
pub fn get_current_time() -> DateTime<offset::Local> {
    offset::Local::now()
}

// `reqwest` supports all platforms, including web.
// However, compiling it for Android on Windows can be challenging
// due to its dependency on the `openssl-sys` crate,
// which requires the corresponding C library to be installed on the system.
// Compiling `reqwest` for Android is possible with the right system setup,
// but it's intentionally disabled in our sample crate
// to ensure that the example app 'just works'.

#[cfg(any(target_os = "android"))]
pub async fn fetch_from_web_api(url: &str) -> String {
    String::from("API fetching is disabled on this platform.")
}
#[cfg(not(any(target_os = "android")))]
pub async fn fetch_from_web_api(url: &str) -> String {
    reqwest::get(url)
        .await
        .expect("Could not get the response from the example web API.")
        .text()
        .await
        .expect("Could not read body from the web response.")
}
