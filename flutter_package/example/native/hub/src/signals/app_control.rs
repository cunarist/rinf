use rinf::{DartSignal, RustSignal};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, DartSignal)]
pub struct UnitTestStart;

#[derive(Serialize, RustSignal)]
pub struct UnitTestEnd;

#[derive(Serialize, RustSignal)]
pub struct ComplexSignalTestResult(pub bool);

#[derive(Deserialize, DartSignal)]
pub struct CreateActors;
