use std::error::Error;
use std::fs;

use crate::day3::row_slice::RowSlice;
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

pub fn day3_challenge2(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let schema = Schema::parse(&text)?;

    let number_slices = schema.extract_number_slices();
    let mut result: u32 = 0;

    for row in 0..schema.rows {
        for (column, symbol) in schema.symbols[row].iter().enumerate() {
            if !symbol.is_gear() {
                continue;
            }

            let adjacent_slices = adjacent_slices(&number_slices, row, column);
            if adjacent_slices.len() != 2 {
                continue;
            }

            result += adjacent_slices[0].try_to_number()? * adjacent_slices[1].try_to_number()?;
        }
    }

    return Ok(result);
}

fn adjacent_slices<'a>(number_slices: &'a Vec<RowSlice>, row: usize, column: usize) -> Vec<&'a RowSlice<'a>> {
    number_slices.iter().filter(|slice| slice.is_adjacent_to_cell(row, column)).collect()
}
