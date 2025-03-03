use crate::SampleSchema;
use rinf::RustSignal;
use serde::Serialize;

/// You can add your custom comments like this.
#[derive(Serialize, RustSignal)]
pub struct SampleFractal {
    pub current_scale: f64,
    pub dummy: Option<SampleSchema>,
}
