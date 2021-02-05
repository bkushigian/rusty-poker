use crate::card::*;
use crate::hand_type::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;

/// Hand equity for a given hand or range played against another hand or range
pub struct HandEquity {
    /// Probability of winning
    pub pwin: f32,
    /// Probability of drawing
    pub pdraw: f32
}

pub fn hand_vs_hand(h1: &HoleCards, h2: &HoleCards, board: &Vec<Card>, num_trials: u32) -> [HandEquity; 2]
{
    let mut cards = all_cards();
    let mut dead_cards: Vec<Card> = Vec::default();
    let mut board: Vec<Card> = board.clone();
    dead_cards.extend_from_slice(h1);
    dead_cards.extend_from_slice(h2);
    for card in dead_cards {
        cards.remove(cards.iter().position(|x| *x == card).unwrap());
    }
    for card in &board {
        cards.remove(cards.iter().position(|x| x == card).unwrap());
    }

    let mut p1_wins = 0;
    let mut p2_wins = 0;
    let mut ties = 0;
    let mut rng = thread_rng();

    for _ in 0..num_trials {
        let new_cards: Vec<Card> = (&cards[..]).choose_multiple(&mut rng, 5 - board.len()).cloned().collect();
        for card in &new_cards {
            board.push(*card);
        }
        match hand_type(h1, &board).cmp(&hand_type(h2, &board)) {
            Ordering::Less => p2_wins +=1,
            Ordering::Equal => ties += 1,
            Ordering::Greater => p1_wins += 1
        }
        for card in &new_cards {
            board.remove(board.iter().position(|x| x == card).unwrap());
        }
    }

    [HandEquity{pwin: p1_wins as f32 / num_trials as f32, pdraw: ties as f32 / num_trials as f32},
     HandEquity{pwin: p2_wins as f32 / num_trials as f32, pdraw: ties as f32 / num_trials as f32}]
}
