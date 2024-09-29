use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ExampleError {
    Fractal,
    HardwareId,
    WebApi,
}

impl fmt::Display for ExampleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl Error for ExampleError {}
