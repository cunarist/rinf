use bincode::{Decode, Encode};
use rinf::{DartSignal, RustSignal};

#[derive(Debug, Decode, DartSignal)]
pub struct SmallText {
    pub text: String,
}

#[derive(Encode, RustSignal)]
pub struct SmallNumber {
    pub number: i32,
}
