use std::cmp::min;
use std::error::Error;
use std::fs;
use std::ops::Range;

use crate::day5::type_conversion::TypeConversion;
use crate::string_functions::{split_and_clean, to_u128};
use crate::GenericError;

mod r#type;
mod type_conversion;

pub fn day5_challenge1(file_path: &str) -> Result<u128, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let lines = text.lines().collect::<Vec<&str>>();

    if lines.len() < 3 {
        return Err(Box::new(GenericError::new("unexpected puzzle input")));
    }

    let seed_ids = extract_seed_ids_challenge1(lines[0])?;
    let raw_mappings = extract_raw_mappings(&lines[2..])?;
    let mut conversions: Vec<TypeConversion> = Vec::new();
    for raw_mapping in raw_mappings {
        conversions.push(TypeConversion::parse(raw_mapping)?);
    }

    let mut min_location_id = u128::MAX;
    for seed_id in seed_ids {
        let maybe_location_id = apply_conversions(seed_id, &conversions);
        if maybe_location_id.is_err() {
            return Err(Box::new(GenericError::new(
                "unable to transform seed to location",
            )));
        }

        min_location_id = min(min_location_id, maybe_location_id.unwrap());
    }

    return Ok(min_location_id);
}

pub fn day5_challenge2(file_path: &str) -> Result<u128, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let lines = text.lines().collect::<Vec<&str>>();

    if lines.len() < 3 {
        return Err(Box::new(GenericError::new("unexpected puzzle input")));
    }

    let seed_ranges = extract_seed_ids_challenge2(lines[0])?;
    let raw_mappings = extract_raw_mappings(&lines[2..])?;
    let mut conversions: Vec<TypeConversion> = Vec::new();
    for raw_mapping in raw_mappings {
        conversions.push(TypeConversion::parse(raw_mapping)?);
    }

    let mut min_location_id = u128::MAX;
    for seed_range in seed_ranges {
        for seed_id in seed_range {
            let maybe_location_id = apply_conversions(seed_id, &conversions);
            if maybe_location_id.is_err() {
                return Err(Box::new(GenericError::new(
                    "unable to transform seed to location",
                )));
            }

            min_location_id = min(min_location_id, maybe_location_id.unwrap());
        }
    }

    return Ok(min_location_id);
}

fn extract_seed_ids_challenge1(input: &str) -> Result<Vec<u128>, GenericError> {
    let parts = split_and_clean(input, " ");
    if parts.len() < 2 {
        return Err(GenericError::new("unable to extract seed ids"));
    }

    let mut result: Vec<u128> = Vec::new();
    for part_index in 1..parts.len() {
        result.push(to_u128(parts[part_index])?);
    }

    return Ok(result);
}

fn extract_seed_ids_challenge2(input: &str) -> Result<Vec<Range<u128>>, GenericError> {
    let numbers = extract_seed_ids_challenge1(input)?;
    if numbers.len() % 2 != 0 {
        return Err(GenericError::new("uneven amount of seed numbers"));
    }

    let mut result: Vec<Range<u128>> = Vec::new();
    for index in (0..numbers.len()).step_by(2) {
        let start = numbers[index];
        let length = numbers[index + 1];

        result.push(start..start + length);
    }

    return Ok(result);
}

fn extract_raw_mappings<'a, 'b>(input: &'a [&'b str]) -> Result<Vec<&'a [&'b str]>, GenericError> {
    let mut current_slice_start: usize = 0;
    let mut result: Vec<&'a [&'b str]> = Vec::new();

    while current_slice_start < input.len() {
        while current_slice_start < input.len() && input[current_slice_start].is_empty() {
            // skip empty lines
            current_slice_start += 1;
        }

        let mut current_slice_end = current_slice_start;
        while current_slice_end < input.len() && !input[current_slice_end].is_empty() {
            // collect non-empty lines
            current_slice_end += 1;
        }

        result.push(&input[current_slice_start..current_slice_end]);
        current_slice_start = current_slice_end;
    }

    if current_slice_start < input.len() {
        result.push(&input[current_slice_start..]);
    }

    return Ok(result);
}

fn apply_conversions(
    source_id: u128,
    conversions: &Vec<TypeConversion>,
) -> Result<u128, GenericError> {
    let mut current_value = source_id;
    for conversion in conversions {
        current_value = conversion.convert(current_value)?;
    }

    return Ok(current_value);
}

#[cfg(test)]
mod tests {
    use crate::day5::extract_raw_mappings;

    #[test]
    fn test_extract_raw_mappings() {
        let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4"
            .lines()
            .collect::<Vec<&str>>();
        let actual = extract_raw_mappings(&input[2..]).unwrap();

        assert_eq!(
            actual,
            vec![
                vec!["seed-to-soil map:", "50 98 2", "52 50 48"],
                vec!["soil-to-fertilizer map:", "0 15 37", "37 52 2", "39 0 15"],
                vec![
                    "fertilizer-to-water map:",
                    "49 53 8",
                    "0 11 42",
                    "42 0 7",
                    "57 7 4",
                ],
            ]
        );
    }
}
