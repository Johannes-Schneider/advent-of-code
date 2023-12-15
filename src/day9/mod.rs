use crate::day9::sequence::SequenceExtrapolation;
use std::error::Error;
use std::fs;

mod sequence;

pub fn day9_challenge1(file_path: &str) -> Result<i128, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let extrapolations = SequenceExtrapolation::parse_all(&text)?;

    let mut result = 0i128;
    for mut extrapolation in extrapolations {
        result += extrapolation.value_at_index(extrapolation.len());
    }

    return Ok(result);
}

pub fn day9_challenge2(file_path: &str) -> Result<i128, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let extrapolations = SequenceExtrapolation::parse_all(&text)?;

    let mut result = 0i128;
    for mut extrapolation in extrapolations {
        result += extrapolation.previous_value(1);
    }

    return Ok(result);
}
