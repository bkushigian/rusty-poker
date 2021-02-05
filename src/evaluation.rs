/// This module is responsible for evaluationg of hand strength

use crate::card::*;

#[derive(Eq, PartialEq, Debug )]
pub enum HandType {
    /// Ranks, sorted in descending order, of a high card hand
    HighCard([Rank; 5]),
    /// Pairing card, three kickers sorted high to low
    Pair(Card, [Rank; 3]),
    /// TwoPair(High pair card, low pair card, kicker)
    TwoPair(Rank, Rank, Rank),
    /// Trips(Trip card rank, kickers)
    Trips(Rank, [Rank; 4]),
    /// Straight(High rank)
    Straight(Rank),
    /// Flush(Suit, Sorted ranks)
    Flush(Suit, [Rank; 5]),
    /// FullHouse(trips rank, pair rank)
    FullHouse(Rank, Rank),
    /// quad rank, kicker rank
    Quads(Rank, Rank),
    /// StraightFlush(Suit of flush, high rank of straight)
    StraightFlush(Suit, Rank)
}

/// Group the cards by suit, ordered by rank.
///
/// Return an array of four `Vec<Card>`s, where each Vec contains the sorted
/// cards of the corresponding rank. The index into the returned array for a
/// particular suit is determined by `Suit::SUIT as usize`. For instance,
/// `Suit::Spades as usize` will give the index into the returned array for the
/// ordered Vec containing the Spades cards.
///
/// # Examples
///
/// ```
/// use rusty_poker::card::{Card, Suit};
/// let cards: Vec<Card> = vec!["Ac".parse().unwrap(),
///                             "Kc".parse().unwrap(),
///                             "Qc".parse().unwrap(),
///                             "Jc".parse().unwrap(),
///                             "Tc".parse().unwrap(),
///                             "As".parse().unwrap(),
///                             "Kd".parse().unwrap()];
/// let grouped = rusty_poker::evaluation::group_by_suit(&cards);
/// assert_eq!(grouped[Suit::Spades   as usize].len(), 1);
/// assert_eq!(grouped[Suit::Clubs    as usize].len(), 5);
/// assert_eq!(grouped[Suit::Diamonds as usize].len(), 1);
/// assert_eq!(grouped[Suit::Hearts   as usize].len(), 0);
/// ```
pub fn group_by_suit(cards: &Vec<Card>) -> [Vec<Card>; 4] {
    let mut grouped_by_suit: [Vec<Card>; 4] = Default::default();
    for card in cards {
        grouped_by_suit[card.suit as usize].push(*card);
    }
    grouped_by_suit
}

/// Return an array indexed by rank frequency, containing a vec of ranks that
/// occur at the given frequency.
///
/// Given n cards, a given card rank can occur at most 4 times. This function
/// takes that list of cards and returns a length 5 array of vecs, where the vec
/// at index i contains all ranks that occured i times in the `cards` input.
///
/// # Example
/// ```
/// use rusty_poker::card::{Card, Rank};
/// let cards: Vec<Card> = vec!["Ac".parse().unwrap(),
///                             "Kc".parse().unwrap(),
///                             "Qc".parse().unwrap(),
///                             "Jc".parse().unwrap(),
///                             "Tc".parse().unwrap(),
///                             "As".parse().unwrap(),
///                             "Kd".parse().unwrap()];
///
/// let grouped = rusty_poker::evaluation::group_by_rank_freq(cards);
/// assert_eq!(grouped[0].len(), 8);  // There are 8 cards that don't appear (2-9)
/// assert!(grouped[0].contains(&Rank::Two));
/// assert!(grouped[0].contains(&Rank::Three));
/// assert!(grouped[0].contains(&Rank::Four));
/// assert!(grouped[0].contains(&Rank::Five));
/// assert!(grouped[0].contains(&Rank::Six));
/// assert_eq!(grouped[1].len(), 3);  // There are three cards that appear once (T-Q)
/// assert!(grouped[1].contains(&Rank::Ten));
/// assert!(grouped[1].contains(&Rank::Jack));
/// assert!(grouped[1].contains(&Rank::Queen));
/// assert_eq!(grouped[2].len(), 2);  // There are two cards that appear twice (A, K)
/// assert!(grouped[2].contains(&Rank::King));
/// assert!(grouped[2].contains(&Rank::Ace));
/// ```
pub fn group_by_rank_freq(cards: Vec<Card>) -> [Vec<Rank>; 5] {
    let mut grouped_by_rank: [u32; 15] = [0;15];
    let mut grouped_by_rank_freq: [Vec<Rank>; 5] = Default::default();
    for card in cards {
        grouped_by_rank[card.rank as usize] += 1;
    }
    for rank in 2..15 {
        let n_of_rank = grouped_by_rank[rank];
        assert!(n_of_rank < 5);
        grouped_by_rank_freq[n_of_rank as usize].push(Rank::from_u32(rank as u32));
    }
    grouped_by_rank_freq
}

/// Find the highest possible straight.
///
/// Cards _must_ be reverse sorted according to rank
///
/// # Example
/// ```
/// use rusty_poker::card::*;
/// use rusty_poker::evaluation::*;
/// let cards: Vec<Card> = vec!["Ac".parse().unwrap(), "Ks".parse().unwrap(), "Qc".parse().unwrap(), "Js".parse().unwrap(), "Tc".parse().unwrap(), "9s".parse().unwrap(), "8s".parse().unwrap()];
///
/// let straight = get_straight(&cards);
/// assert_eq!(straight, Some(HandType::Straight(Rank::Ace)));
///
/// let cards: Vec<Card> = vec!["Ac".parse().unwrap(), "Ks".parse().unwrap(), "Tc".parse().unwrap(), "9s".parse().unwrap(), "7c".parse().unwrap(), "6s".parse().unwrap(), "3s".parse().unwrap()];
/// let no_straight = get_straight(&cards);
/// assert_eq!(no_straight, None);
///
/// let cards: Vec<Card> = vec!["Ac".parse().unwrap(), "Ks".parse().unwrap(), "Tc".parse().unwrap(), "5s".parse().unwrap(), "4c".parse().unwrap(), "3s".parse().unwrap(), "2s".parse().unwrap()];
/// let wheel = get_straight(&cards);
/// assert_eq!(wheel, Some(HandType::Straight(Rank::Five)));
/// ```
pub fn get_straight(cards: &Vec<Card>) -> Option<HandType> {
    if cards.len() >= 5 {
        // Last rank seen, initialized to dummy value
        let mut last_rank = Rank::Two;
        // Top of straight we are in, initialized to dummy value
        let mut top_rank = Rank::Two;
        // Size of straight we are in
        let mut straight_size = 0;

        for card in cards {
            if card.rank == last_rank {
                continue;
            }
            if !card.rank.preceeds(&last_rank) {
                straight_size = 0;
                top_rank = card.rank;
            }
            straight_size += 1;
            last_rank = card.rank;
            if straight_size == 5 {
                return Some(HandType::Straight(top_rank))
            }
        }
        // Now, test for ace-low straight
        if last_rank == Rank::Two && straight_size == 4 && cards.get(0).unwrap().rank == Rank::Ace {
            return Some(HandType::Straight(Rank::Five));
        }
    }
    None
}

/// Find the highest possible straight flush
///
/// Cards _must_ be reverse sorted according to rank
///
/// # Example
/// ```
/// use rusty_poker::card::*;
/// use rusty_poker::evaluation::*;
/// let cards: Vec<Card> = vec!["Ac".parse().unwrap(), "Kc".parse().unwrap(), "Qc".parse().unwrap(), "Jc".parse().unwrap(), "Tc".parse().unwrap(), "9c".parse().unwrap(), "8c".parse().unwrap()];
///
/// let straight_flush = get_straight_flush(&cards);
/// assert_eq!(straight_flush, Some(HandType::StraightFlush(Suit::Clubs, Rank::Ace)));
///
/// let cards: Vec<Card> = vec!["Ac".parse().unwrap(), "Kc".parse().unwrap(), "Qc".parse().unwrap(), "Js".parse().unwrap(), "Tc".parse().unwrap(), "9c".parse().unwrap(), "8c".parse().unwrap()];
/// // There's a straight, and there's a flush, but there's no straight flush
/// let no_straight_flush = get_straight_flush(&cards);
/// assert_eq!(no_straight_flush, None);
/// ```
pub fn get_straight_flush(cards: &Vec<Card>) -> Option<HandType> {
    let by_suit = group_by_suit(&cards);
    for suit in all_suits().iter() {
        let suited_cards = &by_suit[*suit as usize];
        match get_straight(suited_cards) {
            Some(HandType::Straight(rank)) => return Some(HandType::StraightFlush(*suit, rank)),
            _ => (),
        }
    }
    None
}

pub fn hand_strength(hand: &HoleCards, board: &[Card; 5]) {
    let mut cards = Vec::new();
    cards.extend_from_slice(hand);
    cards.extend_from_slice(board);
    cards.sort_by(|a, b| b.cmp(a));
}
