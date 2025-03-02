use rinf::{DartSignal, RustSignal, Signal};

#[derive(Clone, DartSignal)]
pub struct SampleNumberInput {
    pub letter: String,
    pub dummy_one: u32,
    pub dummy_two: Option<SampleSchema>,
    pub dummy_three: Vec<i32>,
}

#[derive(Clone, RustSignal)]
pub struct SampleNumberOutput {
    pub current_number: i32,
    pub dummy_one: u32,
    pub dummy_two: Option<SampleSchema>,
    pub dummy_three: Vec<i32>,
}

#[derive(Clone, Signal)]
pub struct SampleSchema {
    pub sample_field_one: bool,
    pub sample_field_two: bool,
}
