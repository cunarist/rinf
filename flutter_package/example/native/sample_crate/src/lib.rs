//! This crate is written for Rinf demonstrations.

mod error;
mod extras;
mod fractal;

pub use extras::{fetch_from_web_api, get_current_time, get_hardward_id};
pub use fractal::{ImageInfo, draw_fractal_image};
