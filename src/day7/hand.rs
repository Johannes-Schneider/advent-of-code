use std::cmp::Ordering;
use std::collections::HashMap;

use crate::day7::card::Card;
use crate::string_functions::{split_and_clean, to_u128};
use crate::GenericError;

#[derive(Debug, Eq, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
    pub bid: u128,
    hand_type: HandType,
    card_value: u128,
}

impl Hand {
    pub fn parse_all(input: &str) -> Result<Vec<Hand>, GenericError> {
        let mut hands: Vec<Hand> = Vec::new();
        for line in input.lines() {
            let parts = split_and_clean(line, " ");
            if parts.len() != 2 {
                return Err(GenericError::new("unable to parse hand"));
            }

            let cards = Card::parse_all(parts[0])?;
            let bid = to_u128(parts[1])?;
            hands.push(Hand::new(cards, bid));
        }

        return Ok(hands);
    }

    fn new(cards: Vec<Card>, bid: u128) -> Hand {
        let hand_type = HandType::from_cards(&cards);
        let card_value = cards
            .iter()
            .enumerate()
            .map(|(i, c)| c.positional_value((cards.len() - 1 - i) as u32))
            .sum();

        return Hand {
            cards,
            bid,
            hand_type,
            card_value,
        };
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_order = self.hand_type.value().cmp(&other.hand_type.value());
        if type_order != Ordering::Equal {
            return type_order;
        }

        return self.card_value.cmp(&other.card_value);
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_cards(cards: &Vec<Card>) -> HandType {
        if cards.len() != 5 {
            panic!("each hand must consist of 5 cards exactly!");
        }

        let number_of_jokers = cards.iter().filter(|c| **c == Card::Jack).count() as u32;

        let mut counts: HashMap<&Card, u32> = HashMap::new();
        for card in cards.iter().filter(|c| **c != Card::Jack) {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut sorted_counts: Vec<&u32> = counts.values().collect::<Vec<&u32>>();
        sorted_counts.sort_by(|c1, c2| c2.cmp(c1)); // sort in reverse order

        if number_of_jokers == 5 || *sorted_counts[0] + number_of_jokers == 5 {
            return HandType::FiveOfAKind;
        }

        if number_of_jokers == 4 || *sorted_counts[0] + number_of_jokers == 4 {
            return HandType::FourOfAKind;
        }

        if sorted_counts.len() > 1 && *sorted_counts[0] + *sorted_counts[1] + number_of_jokers == 5
        {
            return HandType::FullHouse;
        }

        if *sorted_counts[0] + number_of_jokers == 3 {
            return HandType::ThreeOfAKind;
        }

        if *sorted_counts[0] + *sorted_counts[1] + number_of_jokers == 4 {
            return HandType::TwoPairs;
        }

        if *sorted_counts[0] + number_of_jokers == 2 {
            return HandType::OnePair;
        }

        return HandType::HighCard;
    }

    pub fn value(&self) -> u32 {
        match self {
            HandType::HighCard => 1,
            HandType::OnePair => 2,
            HandType::TwoPairs => 3,
            HandType::ThreeOfAKind => 4,
            HandType::FullHouse => 5,
            HandType::FourOfAKind => 6,
            HandType::FiveOfAKind => 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::day7::card::Card;
    use crate::day7::hand::{Hand, HandType};

    #[test]
    fn test_hand_card_value() {
        let first_hand = Hand::new(
            vec![Card::Three, Card::Two, Card::Two, Card::Two, Card::Two],
            28,
        );
        let second_hand = Hand::new(
            vec![Card::Two, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
            56,
        );

        assert!(first_hand.card_value > second_hand.card_value);
    }

    #[test]
    fn test_hand_ordering() {
        let first_hand = Hand::new(
            vec![Card::Four, Card::Two, Card::Two, Card::Two, Card::Two],
            28,
        );
        let second_hand = Hand::new(
            vec![Card::Three, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
            56,
        );
        let third_hand = Hand::new(
            vec![Card::Two, Card::Two, Card::Two, Card::Two, Card::Two],
            100,
        );

        assert_eq!(first_hand.cmp(&first_hand), Ordering::Equal);
        assert_eq!(second_hand.cmp(&second_hand), Ordering::Equal);
        assert_eq!(third_hand.cmp(&third_hand), Ordering::Equal);

        assert_eq!(first_hand.cmp(&second_hand), Ordering::Greater);
        assert_eq!(first_hand.cmp(&third_hand), Ordering::Less);

        assert_eq!(second_hand.cmp(&first_hand), Ordering::Less);
        assert_eq!(second_hand.cmp(&third_hand), Ordering::Less);

        assert_eq!(third_hand.cmp(&first_hand), Ordering::Greater);
        assert_eq!(third_hand.cmp(&second_hand), Ordering::Greater);
    }

    #[test]
    fn test_hand_type_from_cards() {
        assert_eq!(
            HandType::from_cards(&vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
            HandType::FiveOfAKind
        );
        assert_eq!(
            HandType::from_cards(&vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Two]),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_cards(&vec![Card::Ace, Card::Ace, Card::Ace, Card::Two, Card::Two]),
            HandType::FullHouse
        );
        assert_eq!(
            HandType::from_cards(&vec![
                Card::Ace,
                Card::Three,
                Card::Two,
                Card::Two,
                Card::Two,
            ]),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            HandType::from_cards(&vec![
                Card::Ace,
                Card::Ace,
                Card::Two,
                Card::Two,
                Card::Three,
            ]),
            HandType::TwoPairs
        );
        assert_eq!(
            HandType::from_cards(&vec![
                Card::Ace,
                Card::Ace,
                Card::Two,
                Card::Four,
                Card::Three,
            ]),
            HandType::OnePair
        );
        assert_eq!(
            HandType::from_cards(&vec![
                Card::Ace,
                Card::King,
                Card::Two,
                Card::Four,
                Card::Three,
            ]),
            HandType::HighCard
        );
    }

    #[test]
    fn test_hand_type_five_of_a_kind_with_jokers() {
        assert_eq!(
            HandType::from_cards(&merge(&[repeat(Card::Ace, 4), repeat(Card::Jack, 1)])),
            HandType::FiveOfAKind
        );
        assert_eq!(
            HandType::from_cards(&merge(&[repeat(Card::Ace, 3), repeat(Card::Jack, 2)])),
            HandType::FiveOfAKind
        );
        assert_eq!(
            HandType::from_cards(&merge(&[repeat(Card::Ace, 2), repeat(Card::Jack, 3)])),
            HandType::FiveOfAKind
        );
        assert_eq!(
            HandType::from_cards(&merge(&[repeat(Card::Ace, 1), repeat(Card::Jack, 4)])),
            HandType::FiveOfAKind
        );
        assert_eq!(
            HandType::from_cards(&merge(&[repeat(Card::Jack, 5)])),
            HandType::FiveOfAKind
        );
    }

    #[test]
    fn test_hand_type_four_of_a_kind_with_jokers() {
        assert_eq!(
            HandType::from_cards(&merge(&[
                repeat(Card::Ace, 3),
                repeat(Card::Jack, 1),
                repeat(Card::Two, 1)
            ])),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_cards(&merge(&[
                repeat(Card::Ace, 2),
                repeat(Card::Jack, 2),
                repeat(Card::Two, 1)
            ])),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_cards(&merge(&[
                repeat(Card::Ace, 1),
                repeat(Card::Jack, 3),
                repeat(Card::Two, 1)
            ])),
            HandType::FourOfAKind
        );
    }

    #[test]
    fn test_hand_type_full_house_with_jokers() {
        assert_eq!(
            HandType::from_cards(&merge(&[
                repeat(Card::Ace, 2),
                repeat(Card::King, 2),
                repeat(Card::Jack, 1)
            ])),
            HandType::FullHouse
        );
    }

    fn repeat(card: Card, count: u32) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::new();
        for _ in 0..count {
            result.push(card.clone());
        }

        return result;
    }

    fn merge(vecs: &[Vec<Card>]) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::new();
        for vec in vecs {
            for card in vec {
                result.push(card.clone());
            }
        }

        return result;
    }
}
