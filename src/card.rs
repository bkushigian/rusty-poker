use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

/// Return an array of all suits in the order in which they are enumerated
pub fn all_suits() -> [Suit; 4]{
    [Suit::Spades, Suit::Clubs, Suit::Hearts, Suit::Diamonds]
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14
}

impl Rank {
    pub fn from_u32(rank: u32) -> Rank {
        match rank {
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            14 => Rank::Ace,
            _ => panic!("Invalid rank provided {}", rank)
        }
    }

    pub fn preceeds(&self, other: &Self) -> bool {
        (*self as u32) + 1 == (*other as u32)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseCardError {
    ParseSuitError(String),
    ParseRankError(String),
    ParseFormatError(String)
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ParseCardError::ParseFormatError(format!("Card string must be length 2: \"{}\"", s)));
        }
        let s = s.to_uppercase();
        let rank_char = s.chars().next().unwrap();
        let suit_char = s.chars().nth(1).unwrap();
        if ! "23456789TJQKA".contains(rank_char) {
            return Err(ParseCardError::ParseRankError(format!("Unrecognized rank: \"{}\"", rank_char)));
        }

        let rank: Rank = match "23456789TJQKA".find(rank_char) {
            Some(n) => Rank::from_u32((n + 2) as u32),
            None => return Err(ParseCardError::ParseRankError(format!("Unrecognized rank: \"{}\"", rank_char))),
        };

        let suit = match suit_char {
            'S' => Suit::Spades,
            'H' => Suit::Hearts,
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            _   => return Err(ParseCardError::ParseSuitError(format!("Unrecognized suit: \"{}\"", suit_char))),
        };
        Ok(Card{suit, rank})
    }
}

pub type HoleCards = [Card; 2];

pub fn ranks() -> [Rank; 13] {
    [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace]
}

pub fn suits() -> [Suit; 4] {
    [Suit::Spades, Suit::Clubs, Suit::Hearts, Suit::Diamonds]
}

/// Return a new Vec of all possible cards
pub fn all_cards() -> Vec<Card> {
    let mut cards : Vec<Card> = Vec::default();
    for suit in suits().iter() {
        for rank in ranks().iter() {
            cards.push(Card{suit: suit.clone(), rank: *rank});
        }
    }
    cards
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::card::*;
    #[test]
    fn parse_card() {
        assert_eq!(Card::from_str("Ac"), Ok(Card{suit: Suit::Clubs, rank: Rank::Ace}));
        assert_eq!(Card::from_str("Ks"), Ok(Card{suit: Suit::Spades, rank: Rank::King}));
        assert_eq!(Card::from_str("2d"), Ok(Card{suit: Suit::Diamonds, rank: Rank::Two}));
        assert_eq!(Card::from_str("6h"), Ok(Card{suit: Suit::Hearts, rank: Rank::Six}));
    }
}
