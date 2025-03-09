use crate::SampleSchema;
use bincode::Encode;
use rinf::RustSignalBinary;

/// You can add your custom comments like this.
#[derive(Encode, RustSignalBinary)]
pub struct SampleFractal {
    pub current_scale: f64,
    pub dummy: Option<SampleSchema>,
}
