use rinf::RustSignal;

/// You can add your custom comments like this.
#[derive(Clone, RustSignal)]
pub struct SampleFractal {
    pub current_scale: f64,
    pub dummy: Option<crate::SampleSchema>,
}
