use rinf::{DartSignal, RustSignal};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, DartSignal, Clone)]
pub struct SmallText {
  pub text: String,
}

#[derive(Serialize, RustSignal)]
pub struct SmallNumber {
  pub number: i32,
}
