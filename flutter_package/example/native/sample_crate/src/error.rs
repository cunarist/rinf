use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ExampleError(pub Box<dyn Error + Send + Sync>);

impl fmt::Display for ExampleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source = self.0.as_ref();
        write!(f, "An error occured inside the example code.\n{source}")
    }
}

impl Error for ExampleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.0.as_ref())
    }
}
