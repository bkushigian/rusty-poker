/// This module is responsible for evaluationg of hand strength

use crate::card::*;
use crate::util::*;

#[derive(Eq, PartialEq, Debug )]
pub enum HandType {
    /// Ranks, sorted in descending order, of a high card hand
    HighCard([Card; 5]),
    /// Pairing card, three kickers sorted high to low
    Pair([Card; 2], [Card; 3]),
    /// TwoPair(High pair card, low pair card, kicker)
    TwoPair([Card; 2], [Card; 2], Card),
    /// Trips(Trip cards, kickers)
    Trips([Card; 3], [Card; 2]),
    /// Straight(High rank)
    Straight([Card; 5]),
    /// Flush(Suit, Sorted ranks)
    Flush([Card; 5]),
    /// FullHouse(trips rank, pair rank)
    FullHouse([Card; 3], [Card; 2]),
    /// quads , kicker rank
    Quads([Card; 4], Card),
    /// StraightFlush(Suit of flush, high rank of straight)
    StraightFlush([Card; 5])
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

/// Return an array, indexed by rank frequency, containing a vec of vecs of
/// cards that occur at the given frequency.
///
/// That is, the returned array has at index `f` a `Vec` which contains
/// `Vec<Card>`s of length `f` where each card has the same rank.
///
///
/// # Example
/// ```
/// use rusty_poker::card::*;
/// use rusty_poker::evaluation::*;
/// let cards: Vec<Card> = vec![KING_CLUBS,
///                             KING_SPADES,
///                             KING_HEARTS,
///                             QUEEN_CLUBS,
///                             QUEEN_SPADES,
///                             JACK_CLUBS,
///                             TEN_CLUBS];
/// let grouped = group_by_rank_freq(&cards);
/// assert_eq!(grouped[1], vec![vec![JACK_CLUBS], vec![TEN_CLUBS]]);
/// assert_eq!(grouped[2], vec![vec![QUEEN_CLUBS, QUEEN_SPADES]]);
/// assert_eq!(grouped[3], vec![vec![KING_CLUBS, KING_SPADES, KING_HEARTS]]);
/// ```
pub fn group_by_rank_freq(cards: &Vec<Card>) -> [Vec<Vec<Card>>; 5] {
    let mut grouped_by_rank: [Vec<Card>; 15] = Default::default();
    let mut grouped_by_rank_freq: [Vec<Vec<Card>>; 5] = Default::default();
    for card in cards {
        grouped_by_rank[card.rank as usize].push(*card);
    }
    for cv in grouped_by_rank.iter().rev() {
        assert!(cv.len() < 5);
        grouped_by_rank_freq[cv.len() as usize].push(cv.to_vec());
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
/// let cards: Vec<Card> = vec![ACE_CLUBS, KING_SPADES, QUEEN_CLUBS, JACK_SPADES, TEN_DIAMONDS, NINE_SPADES, EIGHT_HEARTS];
///
/// let straight = get_straight(&cards);
/// assert_eq!(straight, Some(HandType::Straight([ACE_CLUBS, KING_SPADES, QUEEN_CLUBS, JACK_SPADES, TEN_DIAMONDS])));
///
/// let cards: Vec<Card> = vec!["Ac".parse().unwrap(), "Ks".parse().unwrap(), "Tc".parse().unwrap(), "9s".parse().unwrap(), "7c".parse().unwrap(), "6s".parse().unwrap(), "3s".parse().unwrap()];
/// let cards: Vec<Card> = vec![ACE_CLUBS, KING_SPADES, QUEEN_CLUBS, TEN_DIAMONDS, NINE_SPADES, EIGHT_HEARTS, FIVE_DIAMONDS];
/// let no_straight = get_straight(&cards);
/// assert_eq!(no_straight, None);
///
/// let cards: Vec<Card> = vec![ACE_SPADES, QUEEN_CLUBS, TEN_SPADES, FIVE_CLUBS, FOUR_HEARTS, THREE_DIAMONDS, TWO_DIAMONDS];
/// let wheel = get_straight(&cards);
/// assert_eq!(wheel, Some(HandType::Straight([FIVE_CLUBS, FOUR_HEARTS, THREE_DIAMONDS, TWO_DIAMONDS, ACE_SPADES])));
/// ```
pub fn get_straight(cards: &Vec<Card>) -> Option<HandType> {
    if cards.len() >= 5 {
        // Last rank seen, initialized to dummy value
        let mut last_rank = Rank::Two;
        // Keep track of the cards in the current straight
        let mut straight = Vec::new();
        for card in cards {
            if card.rank == last_rank {
                continue;
            }
            if !card.rank.preceeds(&last_rank) {
                straight.clear();
            }
            straight.push(*card);
            last_rank = card.rank;
            if straight.len() == 5 {
                return Some(HandType::Straight(card_vec_to_card_array(&straight).unwrap()));
            }
        }
        // Now, test for ace-low straight
        if last_rank == Rank::Two && straight.len() ==  4 && cards.get(0).unwrap().rank == Rank::Ace {
            straight.push(*cards.get(0).unwrap());
            return Some(HandType::Straight(card_vec_to_card_array(&straight).unwrap()));
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
/// let cards: Vec<Card> = vec![ACE_CLUBS, KING_CLUBS, QUEEN_CLUBS, JACK_CLUBS, TEN_CLUBS, NINE_CLUBS, EIGHT_CLUBS];
///
/// let straight_flush = get_straight_flush(&cards);
/// assert_eq!(straight_flush, Some(HandType::StraightFlush([ACE_CLUBS, KING_CLUBS, QUEEN_CLUBS, JACK_CLUBS, TEN_CLUBS])));
///
/// let cards: Vec<Card> = vec![ACE_CLUBS, KING_CLUBS, QUEEN_CLUBS, JACK_SPADES, TEN_CLUBS, NINE_CLUBS, EIGHT_CLUBS];
/// // There's a straight, and there's a flush, but there's no straight flush
/// let no_straight_flush = get_straight_flush(&cards);
/// assert_eq!(no_straight_flush, None);
/// ```
pub fn get_straight_flush(cards: &Vec<Card>) -> Option<HandType> {
    let by_suit = group_by_suit(&cards);
    for suit in all_suits().iter() {
        let suited_cards = &by_suit[*suit as usize];
        match get_straight(suited_cards) {
            Some(HandType::Straight(cards)) => return Some(HandType::StraightFlush(cards)),
            _ => (),
        }
    }
    None
}

/// Find the highest possible flush
///
/// Cards _must_ be reverse sorted according to rank
///
/// # Example
/// ```
/// use rusty_poker::card::*;
/// use rusty_poker::evaluation::*;
/// let cards: Vec<Card> = vec![ACE_SPADES, KING_CLUBS, TEN_CLUBS, TEN_SPADES, NINE_CLUBS, FIVE_CLUBS, FOUR_CLUBS];
///
/// let flush = get_flush(&cards);
/// assert_eq!(flush, Some(HandType::Flush([KING_CLUBS, TEN_CLUBS, NINE_CLUBS, FIVE_CLUBS, FOUR_CLUBS])));
///
/// let cards: Vec<Card> = vec![ACE_CLUBS, ACE_SPADES, KING_CLUBS, QUEEN_CLUBS, TEN_CLUBS, NINE_CLUBS, EIGHT_CLUBS];
/// let flush = get_flush(&cards);
/// assert_eq!(flush, Some(HandType::Flush([ACE_CLUBS, KING_CLUBS, QUEEN_CLUBS, TEN_CLUBS, NINE_CLUBS])));
/// ```
pub fn get_flush(cards: &Vec<Card>) -> Option<HandType> {
    let by_suit = group_by_suit(&cards);
    for suit in all_suits().iter() {
        let suited_cards = &by_suit[*suit as usize];
        match card_vec_to_card_array(suited_cards) {
            Some(arr) => return Some(HandType::Flush(arr)),
            _ => (),
        }
    }
    None
}

pub fn get_quads(cards: &Vec<Card>) -> Option<HandType> {
    let grouped_by_rank = group_by_rank_freq(cards);
    //if grouped_by_rank[4].get(0)
    None
}

pub fn get_full_house(cards: &Vec<Card>) -> Option<HandType> {
    None
}

pub fn get_trips_or_pairs(cards: &Vec<Card>) -> Option<HandType> {
    None
}

pub fn get_high_card(cards: &Vec<Card>) -> Option<HandType> {
    None
}


pub fn hand_strength(hand: &HoleCards, board: &[Card; 5]) -> HandType {
    let mut cards = Vec::new();
    cards.extend_from_slice(hand);
    cards.extend_from_slice(board);
    cards.sort_by(|a, b| b.cmp(a));

    return get_straight_flush(&cards)
        .or_else(|| get_quads(&cards))
        .or_else(|| get_full_house(&cards))
        .or_else(|| get_flush(&cards))
        .or_else(|| get_straight(&cards))
        .or_else(|| get_trips_or_pairs(&cards))
        .or_else(|| get_high_card(&cards)).unwrap();
}
