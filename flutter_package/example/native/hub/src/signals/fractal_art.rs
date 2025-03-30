use crate::signals::SampleSchema;
use rinf::RustSignalBinary;
use serde::Serialize;

/// You can add your custom comments like this.
/// The generated Dart classes will have the same comments on them.
#[derive(Serialize, RustSignalBinary)]
pub struct SampleFractal {
  pub current_scale: f64,
  pub dummy: Option<SampleSchema>,
}
