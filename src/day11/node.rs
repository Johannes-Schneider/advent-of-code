use crate::GenericError;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Pixel {
    EmptySpace,
    Galaxy,
}

impl Pixel {
    pub fn parse(input: &u8) -> Result<Pixel, GenericError> {
        return match input {
            b'.' => Ok(Pixel::EmptySpace),
            b'#' => Ok(Pixel::Galaxy),
            _ => Err(GenericError::new("unknown node type")),
        };
    }
}
