use crate::GenericError;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Spring {
    Functioning,
    Broken,
    Unknown,
}

impl Spring {
    pub fn parse(input: &u8) -> Result<Spring, GenericError> {
        return match input {
            b'.' => Ok(Spring::Functioning),
            b'#' => Ok(Spring::Broken),
            b'?' => Ok(Spring::Unknown),
            _ => Err(GenericError::new("unknown spring type")),
        };
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Spring::Functioning => ".",
            Spring::Broken => "#",
            Spring::Unknown => "?",
        };
        write!(f, "{symbol}")
    }
}
