use std::error::Error;
use std::fs;

use crate::day3::schema::Schema;

mod row_slice;
mod schema;
mod symbol;

pub fn day3_challenge1(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let schema = Schema::parse(&text)?;

    return Ok(schema
        .extract_number_slices()
        .iter()
        .filter(|slice| slice.is_part_number())
        .map(|slice| slice.to_number())
        .sum());
}
