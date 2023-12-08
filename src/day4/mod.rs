use std::cmp::min;
use std::error::Error;
use std::fs;

use crate::day4::scratch_card::ScratchCard;
use crate::GenericError;

mod scratch_card;

pub fn day4_challenge1(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let cards = ScratchCard::parse_all(&text)?;

    return Ok(cards.iter().map(|g| g.points()).sum());
}

pub fn day4_challenge2(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let text = fs::read_to_string(file_path)?;
    let initial_cards = ScratchCard::parse_all(&text)?;
    assert_sequential_ids(&initial_cards)?;

    let mut remaining_cards: Vec<&ScratchCard> = initial_cards.iter().collect();

    let mut result: u32 = 0;
    while !remaining_cards.is_empty() {
        result += 1;
        let next_card = remaining_cards.pop().unwrap();

        let copy_start_index: usize = next_card.id as usize;
        let copy_end_index: usize = min(
            initial_cards.len(),
            copy_start_index + next_card.number_of_matches as usize,
        );
        for i in copy_start_index..copy_end_index {
            remaining_cards.push(&initial_cards[i]);
        }
    }

    return Ok(result);
}

fn assert_sequential_ids(cards: &Vec<ScratchCard>) -> Result<(), GenericError> {
    let mut expected_next_id: u32 = 1;
    for card in cards {
        if card.id != expected_next_id {
            return Err(GenericError::new("the card ids are not ascending"));
        }

        expected_next_id += 1;
    }

    return Ok(());
}
