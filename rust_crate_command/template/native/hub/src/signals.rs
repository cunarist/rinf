use rinf::{DartSignal, RustSignal};

#[derive(DartSignal)]
pub struct SmallText {
    pub text: String,
}

#[derive(RustSignal)]
pub struct SmallNumber {
    pub number: i32,
}
