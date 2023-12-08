use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct GenericError {
    message: &'static str,
}

impl GenericError {
    pub fn new(message: &'static str) -> GenericError {
        GenericError { message }
    }

    pub fn not_implemented() -> GenericError {
        GenericError {
            message: "not implemented",
        }
    }
}

impl Error for GenericError {}

impl Display for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Debug for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred: {}", self.message)
    }
}
