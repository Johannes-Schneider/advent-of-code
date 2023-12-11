use std::cmp::Ordering;
use phf::phf_map;

use crate::GenericError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

static CARDS: phf::Map<u8, Card> = phf_map! {
    b'2' => Card::Two,
    b'3' => Card::Three,
    b'4' => Card::Four,
    b'5' => Card::Five,
    b'6' => Card::Six,
    b'7' => Card::Seven,
    b'8' => Card::Eight,
    b'9' => Card::Nine,
    b'T' => Card::Ten,
    b'J' => Card::Jack,
    b'Q' => Card::Queen,
    b'K' => Card::King,
    b'A' => Card::Ace
};

impl Card {
    pub fn parse_all(input: &str) -> Result<Vec<Card>, GenericError> {
        let mut result: Vec<Card> = Vec::new();
        for byte in input.as_bytes() {
            let card = CARDS.get(byte).ok_or_else(|| GenericError::new("unable to parse card"))?;
            result.push(*card);
        }

        return Ok(result);
    }

    pub fn value(&self) -> u32 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::card::Card;

    #[test]
    fn test_parse_all() {
        let input = "23456789TJQKA";
        let actual = Card::parse_all(input).unwrap();

        assert_eq!(actual, vec![Card::Two, Card::Three, Card::Four, Card::Five, Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten, Card::Jack, Card::Queen, Card::King, Card::Ace]);
    }
}