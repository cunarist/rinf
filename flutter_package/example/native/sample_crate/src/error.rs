use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ExampleError {
    Fractal,
    HardwareId,
    WebApi,
}

impl Error for ExampleError {}

impl Display for ExampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fractal => {
                write!(f, "Failed to generate fractal")
            }
            Self::HardwareId => {
                write!(f, "Unable to retrieve hardware ID")
            }
            Self::WebApi => {
                write!(f, "Web API call failed")
            }
        }
    }
}
