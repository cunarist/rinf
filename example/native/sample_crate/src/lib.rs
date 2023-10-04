//! This crate is only for demonstration purposes.
//! You might want to remove this crate in production.

use chrono::{offset, DateTime};

pub use mandelbrot::{mandelbrot, Point, Size};

mod mandelbrot;

pub fn add_seven(before: i32) -> i32 {
    before + 7
}

// Some crates only support desktop platforms.
// That's why we are doing the compilation test
// only on desktop platforms.
#[allow(unused_imports)]
#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
pub mod compilation_test {
    use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
    pub fn get_hardward_id() -> Option<String> {
        let mut builder = IdBuilder::new(Encryption::MD5);
        builder
            .add_component(HWIDComponent::SystemID)
            .add_component(HWIDComponent::CPUCores);
        let hwid = builder.build("mykey").unwrap();
        Some(hwid)
    }
}
#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub mod compilation_test {
    pub fn get_hardward_id() -> Option<String> {
        None
    }
}

pub fn get_current_time() -> DateTime<offset::Local> {
    offset::Local::now()
}
