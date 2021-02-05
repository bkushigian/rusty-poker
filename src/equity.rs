use crate::card::*;

/// Hand equity for a given hand or range played against another hand or range
pub struct HandEquity {
    /// Probability of winning
    pub pwin: f32,
    /// Probability of drawing
    pub pdraw: f32
}

pub fn hand_vs_hand(h1: &HoleCards, h2: &HoleCards) -> [HandEquity; 2]
{
    let mut _cards = all_cards();
    let mut dead_cards: Vec<Card> = Vec::default();
    dead_cards.extend_from_slice(h1);
    dead_cards.extend_from_slice(h2);
    panic!()
}
