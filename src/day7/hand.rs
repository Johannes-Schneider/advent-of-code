use std::cmp::Ordering;
use std::collections::HashMap;
use crate::day7::card::Card;

pub struct Hand {
    cards: Vec<Card>,
    bid: u128,
    hand_type: HandType
}

impl Hand {
    pub fn new(cards: &Vec<Card>, bid: u128) -> Hand {
        let mut ordered_cards = cards.to_vec();
        ordered_cards.sort_by(|c1, c2| c1.value().cmp(&c2.value()));


        panic!("not implemented");
    }
}

#[derive(Debug, PartialEq)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard
}

impl HandType {
    fn from_cards(cards: &Vec<Card>) -> HandType {
        if cards.len() != 5 {
            panic!("each hand must consist of 5 cards exactly!");
        }

        let mut counts: HashMap<&Card, u32> = HashMap::new();
        for card in cards {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut sorted_counts: Vec<&u32> = counts.values().collect::<Vec<&u32>>();
        sorted_counts.sort_by(|c1, c2| c2.cmp(c1)); // sort in reverse order

        if *sorted_counts[0] == 5 {
            return HandType::FiveOfAKind;
        }

        if *sorted_counts[0] == 4 {
            return HandType::FourOfAKind;
        }

        if *sorted_counts[0] == 3 && *sorted_counts[1] == 2 {
            return HandType::FullHouse;
        }

        if *sorted_counts[0] == 3 {
            return HandType::ThreeOfAKind;
        }

        if *sorted_counts[0] == 2 && *sorted_counts[1] == 2 {
            return HandType::TwoPairs;
        }

        if *sorted_counts[0] == 2 {
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
            HandType::FiveOfAKind => 7
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::card::Card;
    use crate::day7::hand::HandType;

    #[test]
    fn test_hand_type_from_cards() {
        assert_eq!(HandType::from_cards(&vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]), HandType::FiveOfAKind);
        assert_eq!(HandType::from_cards(&vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Two]), HandType::FourOfAKind);
        assert_eq!(HandType::from_cards(&vec![Card::Ace, Card::Ace, Card::Ace, Card::Two, Card::Two]), HandType::FullHouse);
        assert_eq!(HandType::from_cards(&vec![Card::Ace, Card::Three, Card::Two, Card::Two, Card::Two]), HandType::ThreeOfAKind);
        assert_eq!(HandType::from_cards(&vec![Card::Ace, Card::Ace, Card::Two, Card::Two, Card::Three]), HandType::TwoPairs);
        assert_eq!(HandType::from_cards(&vec![Card::Ace, Card::Ace, Card::Two, Card::Four, Card::Three]), HandType::OnePair);
        assert_eq!(HandType::from_cards(&vec![Card::Ace, Card::King, Card::Two, Card::Four, Card::Three]), HandType::HighCard);
    }
}
