
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

pub type Rank = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}
