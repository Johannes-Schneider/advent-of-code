use std::error::Error;
use std::fs;

use crate::day6::race::Race;

mod race;

pub fn day6_challenge1(file_path: &str) -> Result<u128, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let races = Race::parse_all(&text)?;

    let mut result: u128 = 1;
    for race in races {
        result *= race.number_of_ways_to_win() as u128;
    }

    return Ok(result);
}
