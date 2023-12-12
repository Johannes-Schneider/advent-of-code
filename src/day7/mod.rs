use crate::day7::hand::Hand;
use std::error::Error;
use std::fs;

mod card;
mod hand;

pub fn day7_challenge1(file_path: &str) -> Result<u128, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let mut hands = Hand::parse_all(&text)?;
    hands.sort();

    let mut result: u128 = 0;
    for (index, hand) in hands.iter().enumerate() {
        let rank = (index + 1) as u128;
        result += hand.bid * rank;
    }

    return Ok(result);
}
