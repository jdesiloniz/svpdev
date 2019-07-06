use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AssemblyError(pub String);

impl fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for AssemblyError {}
