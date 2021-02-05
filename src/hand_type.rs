/// This module is responsible for determining hand type

use crate::card::*;
use crate::util::*;

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
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
/// let grouped = rusty_poker::hand_type::group_by_suit(&cards);
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
/// use rusty_poker::hand_type::*;
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
/// use rusty_poker::hand_type::*;
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
/// use rusty_poker::hand_type::*;
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
/// use rusty_poker::hand_type::*;
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

/// Find quads if they exist
///
/// Cards _must_ be reverse sorted according to rank
///
/// # Example
/// ```
/// use rusty_poker::card::*;
/// use rusty_poker::hand_type::*;
///
/// let cards: Vec<Card> = vec![ACE_SPADES, ACE_CLUBS, ACE_DIAMONDS, ACE_HEARTS, NINE_CLUBS, FIVE_CLUBS, FOUR_CLUBS];
/// let quads = get_quads(&cards);
/// assert_eq!(quads, Some(HandType::Quads([ACE_SPADES, ACE_CLUBS, ACE_DIAMONDS, ACE_HEARTS], NINE_CLUBS)));
///
/// let cards: Vec<Card> = vec![ACE_SPADES, ACE_CLUBS, ACE_DIAMONDS, KING_HEARTS, NINE_CLUBS, FIVE_CLUBS, FOUR_CLUBS];
/// let no_quads = get_quads(&cards);
/// assert_eq!(no_quads, None);
/// ```
pub fn get_quads(cards: &Vec<Card>) -> Option<HandType> {
    let grouped_by_rank = group_by_rank_freq(cards);
    let quads: [Card; 4] = match grouped_by_rank[4].get(0) {
        Some(vec) => {
            assert!(vec.len() == 4);
            [vec.get(0).unwrap().clone(), vec.get(1).unwrap().clone(), vec.get(2).unwrap().clone(), vec.get(3).unwrap().clone()]
        }
        None => return None
    };
    let quad_rank = quads[0].rank;
    for card in cards {
        if card.rank != quad_rank {
            return Some(HandType::Quads(quads, *card));
        }
    }
    None
}

/// Find full house if it exists
///
/// Cards _must_ be reverse sorted according to rank
///
/// # Example
/// ```
/// use rusty_poker::card::*;
/// use rusty_poker::hand_type::*;
///
/// let cards: Vec<Card> = vec![ACE_SPADES, ACE_CLUBS, ACE_DIAMONDS, NINE_HEARTS, NINE_CLUBS, FIVE_CLUBS, FOUR_CLUBS];
/// let boat = get_full_house(&cards);
/// assert_eq!(boat, Some(HandType::FullHouse([ACE_SPADES, ACE_CLUBS, ACE_DIAMONDS], [NINE_HEARTS, NINE_CLUBS])));
///
/// let cards: Vec<Card> = vec![ACE_SPADES, ACE_CLUBS, ACE_DIAMONDS, KING_HEARTS, NINE_CLUBS, FIVE_CLUBS, FOUR_CLUBS];
/// let no_boat = get_full_house(&cards);
/// assert_eq!(no_boat, None);
///
/// assert!(get_full_house(&vec![ACE_CLUBS, TEN_DIAMONDS, TEN_HEARTS, NINE_DIAMONDS, NINE_HEARTS, EIGHT_SPADES, EIGHT_HEARTS]).is_none());
/// assert!(get_full_house(&vec![ACE_CLUBS, TEN_DIAMONDS, TEN_HEARTS, NINE_DIAMONDS, NINE_HEARTS, NINE_SPADES, EIGHT_HEARTS]).is_some());
/// ```
pub fn get_full_house(cards: &Vec<Card>) -> Option<HandType> {
    let grouped_by_rank = group_by_rank_freq(cards);
    let trips: [Card; 3] = match grouped_by_rank[3].get(0) {
        Some(vec) => {
            assert!(vec.len() == 3);
            [vec.get(0).unwrap().clone(), vec.get(1).unwrap().clone(), vec.get(2).unwrap().clone()]
        }
        None => return None
    };
    let pair: [Card; 2] = match grouped_by_rank[2].get(0) {
        Some(vec) => {
            assert!(vec.len() == 2);
            [vec.get(0).unwrap().clone(), vec.get(1).unwrap().clone()]
        }
        None => match grouped_by_rank[3].get(1) {
            Some(vec) => {
                assert!(vec.len() == 3);
                [vec.get(0).unwrap().clone(), vec.get(1).unwrap().clone()]
            }
            None => return None
        }
    };
    Some(HandType::FullHouse(trips, pair))
}

/// Find trips, two pair, or a pair if they exist
///
/// Cards _must_ be reverse sorted according to rank
///
/// # Example
/// ```
/// use rusty_poker::card::*;
/// use rusty_poker::hand_type::*;
///
/// let cards: Vec<Card> = vec![ACE_SPADES, ACE_CLUBS, ACE_DIAMONDS, NINE_HEARTS, EIGHT_CLUBS, FIVE_CLUBS, FOUR_CLUBS];
/// let trips = get_trips_or_pairs(&cards);
/// assert_eq!(trips, Some(HandType::Trips([ACE_SPADES, ACE_CLUBS, ACE_DIAMONDS], [NINE_HEARTS, EIGHT_CLUBS])));
///
/// let cards: Vec<Card> = vec![ACE_SPADES, KING_CLUBS, KING_DIAMONDS, KING_HEARTS, EIGHT_CLUBS, FIVE_CLUBS, FOUR_CLUBS];
/// let trips = get_trips_or_pairs(&cards);
/// assert_eq!(trips, Some(HandType::Trips([KING_CLUBS, KING_DIAMONDS, KING_HEARTS], [ACE_SPADES, EIGHT_CLUBS])));
///
/// let cards: Vec<Card> = vec![ACE_SPADES, KING_CLUBS, KING_DIAMONDS, EIGHT_HEARTS, EIGHT_CLUBS, FIVE_CLUBS, FOUR_CLUBS];
/// let two_pair = get_trips_or_pairs(&cards);
/// assert_eq!(two_pair, Some(HandType::TwoPair([KING_CLUBS, KING_DIAMONDS], [EIGHT_HEARTS, EIGHT_CLUBS], ACE_SPADES)));
///
/// let cards: Vec<Card> = vec![ACE_SPADES, KING_CLUBS, KING_DIAMONDS, EIGHT_HEARTS, EIGHT_CLUBS, FIVE_CLUBS, FIVE_CLUBS];
/// let two_pair = get_trips_or_pairs(&cards);
/// assert_eq!(two_pair, Some(HandType::TwoPair([KING_CLUBS, KING_DIAMONDS], [EIGHT_HEARTS, EIGHT_CLUBS], ACE_SPADES)));
///
/// let cards: Vec<Card> = vec![ACE_SPADES, KING_CLUBS, KING_DIAMONDS, EIGHT_HEARTS, SEVEN_CLUBS, FIVE_CLUBS, FOUR_CLUBS];
/// let pair = get_trips_or_pairs(&cards);
/// assert_eq!(pair, Some(HandType::Pair([KING_CLUBS, KING_DIAMONDS], [ACE_SPADES, EIGHT_HEARTS, SEVEN_CLUBS])));
/// ```
pub fn get_trips_or_pairs(cards: &Vec<Card>) -> Option<HandType> {
    let grouped_by_rank = group_by_rank_freq(cards);
    let trips = &grouped_by_rank[3];

    match trips.get(0) {
        Some(vec) => {
            assert!(vec.len() == 3);
            let trips = [vec.get(0).unwrap().clone(), vec.get(1).unwrap().clone(), vec.get(2).unwrap().clone()];
            let trips_rank = trips[0].rank;
            let mut kickers = Vec::new();
            for card in cards {
                if card.rank != trips_rank {
                    kickers.push(*card);
                }
                if kickers.len() >= 2 {

                    return Some(HandType::Trips(trips, [kickers.get(0).unwrap().clone(), kickers.get(1).unwrap().clone()]))
                }
            }
        }
        None => ()
    }
    let pairs = &grouped_by_rank[2];
    if pairs.len() >= 2 {
        let p1 = pairs.get(0).unwrap();
        let p1 = [p1.get(0).unwrap().clone(), p1.get(1).unwrap().clone()];
        let p1_rank = p1[0].rank;
        let p2 = pairs.get(1).unwrap();
        let p2 = [p2.get(0).unwrap().clone(), p2.get(1).unwrap().clone()];
        let p2_rank = p2[0].rank;

        for card in cards {
            if card.rank != p1_rank && card.rank != p2_rank {
                return Some(HandType::TwoPair(p1, p2, *card))
            }
        }
        panic!("Illegal State: found two pair ith no kicker");
    }
    if pairs.len() == 1 {
        let p = pairs.get(0).unwrap();
        let p = [p.get(0).unwrap().clone(), p.get(1).unwrap().clone()];
        let p_rank = p[0].rank;
        let mut kickers = Vec::new();
        for card in cards {
            if card.rank != p_rank {
                kickers.push(*card);
                if kickers.len() == 3 {
                    return Some(HandType::Pair(p, [kickers.get(0).unwrap().clone(), kickers.get(1).unwrap().clone(), kickers.get(2).unwrap().clone()]))
                }
            }
        }
        panic!("Illegal State: found Pair without enough kickers");
    }
    None
}

pub fn get_high_card(cards: &Vec<Card>) -> Option<HandType> {
    Some(HandType::HighCard(card_vec_to_card_array(cards).unwrap()))
}


/// Get the hand type of a given set of cards
///
/// # Example
/// ```
/// use rusty_poker::card::*;
/// use rusty_poker::hand_type::*;
///
/// let trips = hand_type(&[ACE_SPADES, ACE_CLUBS], &[ACE_DIAMONDS, NINE_HEARTS, EIGHT_CLUBS, FIVE_CLUBS, FOUR_CLUBS]);
/// assert_eq!(trips, HandType::Trips([ACE_SPADES, ACE_CLUBS, ACE_DIAMONDS], [NINE_HEARTS, EIGHT_CLUBS]));
/// let pair = hand_type(&[ACE_SPADES, ACE_CLUBS], &[KING_DIAMONDS, NINE_HEARTS, EIGHT_CLUBS, FIVE_CLUBS, FOUR_CLUBS]);
/// assert_eq!(pair, HandType::Pair([ACE_SPADES, ACE_CLUBS], [KING_DIAMONDS, NINE_HEARTS, EIGHT_CLUBS]));
/// ```
pub fn hand_type(hand: &HoleCards, board: &[Card; 5]) -> HandType {
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

#[cfg(test)]
mod tests {
    use crate::card::*;
    use super::*;
    #[test]
    fn full_house() {
        assert!(get_full_house(&vec![ACE_DIAMONDS, ACE_HEARTS, ACE_CLUBS, FIVE_DIAMONDS, FIVE_HEARTS, FOUR_CLUBS, EIGHT_SPADES]).is_some());
        assert!(get_full_house(&vec![FOUR_CLUBS, FOUR_DIAMONDS, FOUR_HEARTS, SEVEN_SPADES, SEVEN_DIAMONDS, QUEEN_HEARTS, EIGHT_CLUBS]).is_some());
        assert!(get_full_house(&vec![KING_DIAMONDS, KING_SPADES, KING_HEARTS, EIGHT_SPADES, EIGHT_CLUBS, JACK_HEARTS, FOUR_DIAMONDS]).is_some());
        assert!(get_full_house(&vec![ACE_CLUBS, ACE_DIAMONDS, ACE_SPADES, THREE_CLUBS, THREE_HEARTS, EIGHT_DIAMONDS, SEVEN_SPADES]).is_some());
        assert!(get_full_house(&vec![KING_DIAMONDS, KING_SPADES, KING_CLUBS, JACK_DIAMONDS, JACK_SPADES, SEVEN_HEARTS, TEN_CLUBS]).is_some());
        assert!(get_full_house(&vec![QUEEN_DIAMONDS, QUEEN_CLUBS, QUEEN_SPADES, NINE_SPADES, NINE_HEARTS, SEVEN_DIAMONDS, FOUR_CLUBS]).is_some());
        assert!(get_full_house(&vec![TEN_DIAMONDS, TEN_SPADES, TEN_HEARTS, SIX_HEARTS, SIX_SPADES, TWO_CLUBS, ACE_DIAMONDS]).is_some());
        assert!(get_full_house(&vec![ACE_SPADES, ACE_HEARTS, ACE_CLUBS, SIX_DIAMONDS, SIX_SPADES, EIGHT_HEARTS, KING_CLUBS]).is_some());
        assert!(get_full_house(&vec![SEVEN_HEARTS, SEVEN_SPADES, SEVEN_DIAMONDS, FIVE_HEARTS, FIVE_DIAMONDS, JACK_CLUBS, SIX_SPADES]).is_some());
        assert!(get_full_house(&vec![SIX_CLUBS, SIX_DIAMONDS, SIX_SPADES, THREE_SPADES, THREE_HEARTS, NINE_CLUBS, ACE_DIAMONDS]).is_some());
        assert!(get_full_house(&vec![TEN_SPADES, TEN_HEARTS, TEN_CLUBS, EIGHT_HEARTS, EIGHT_CLUBS, FOUR_DIAMONDS, SEVEN_SPADES]).is_some());
        assert!(get_full_house(&vec![FIVE_DIAMONDS, FIVE_CLUBS, FIVE_SPADES, FOUR_CLUBS, FOUR_SPADES, KING_DIAMONDS, EIGHT_HEARTS]).is_some());
        assert!(get_full_house(&vec![TEN_HEARTS, TEN_DIAMONDS, TEN_SPADES, TWO_DIAMONDS, TWO_HEARTS, THREE_SPADES, ACE_CLUBS]).is_some());
        assert!(get_full_house(&vec![NINE_DIAMONDS, NINE_HEARTS, NINE_CLUBS, JACK_DIAMONDS, JACK_CLUBS, THREE_HEARTS, SEVEN_SPADES]).is_some());
        assert!(get_full_house(&vec![NINE_DIAMONDS, NINE_HEARTS, NINE_SPADES, FOUR_DIAMONDS, FOUR_CLUBS, QUEEN_SPADES, TWO_HEARTS]).is_some());
        assert!(get_full_house(&vec![JACK_SPADES, JACK_HEARTS, JACK_CLUBS, FOUR_HEARTS, FOUR_CLUBS, TWO_DIAMONDS, TEN_SPADES]).is_some());
        assert!(get_full_house(&vec![SIX_DIAMONDS, SIX_SPADES, SIX_HEARTS, KING_CLUBS, KING_HEARTS, THREE_SPADES, TWO_DIAMONDS]).is_some());
        assert!(get_full_house(&vec![FIVE_DIAMONDS, FIVE_SPADES, FIVE_HEARTS, SEVEN_CLUBS, SEVEN_SPADES, THREE_DIAMONDS, EIGHT_HEARTS]).is_some());
        assert!(get_full_house(&vec![FOUR_SPADES, FOUR_CLUBS, FOUR_DIAMONDS, TWO_HEARTS, TWO_SPADES, NINE_CLUBS, SIX_DIAMONDS]).is_some());
        assert!(get_full_house(&vec![JACK_DIAMONDS, JACK_HEARTS, JACK_CLUBS, TWO_CLUBS, TWO_HEARTS, THREE_DIAMONDS, SEVEN_SPADES]).is_some());
        assert!(get_full_house(&vec![TWO_SPADES, TWO_CLUBS, TWO_DIAMONDS, THREE_SPADES, THREE_DIAMONDS, JACK_CLUBS, EIGHT_HEARTS]).is_some());
        assert!(get_full_house(&vec![THREE_CLUBS, THREE_SPADES, THREE_DIAMONDS, ACE_SPADES, ACE_CLUBS, SIX_DIAMONDS, TEN_HEARTS]).is_some());
        assert!(get_full_house(&vec![TEN_SPADES, TEN_CLUBS, TEN_HEARTS, KING_CLUBS, KING_SPADES, THREE_DIAMONDS, TWO_HEARTS]).is_some());
        assert!(get_full_house(&vec![NINE_HEARTS, NINE_DIAMONDS, NINE_CLUBS, JACK_HEARTS, JACK_DIAMONDS, SEVEN_CLUBS, THREE_SPADES]).is_some());
        assert!(get_full_house(&vec![SEVEN_CLUBS, SEVEN_DIAMONDS, SEVEN_HEARTS, FOUR_CLUBS, FOUR_HEARTS, THREE_DIAMONDS, EIGHT_SPADES]).is_some());
        assert!(get_full_house(&vec![ACE_HEARTS, ACE_SPADES, ACE_CLUBS, FOUR_CLUBS, FOUR_DIAMONDS, JACK_SPADES, TWO_HEARTS]).is_some());
        assert!(get_full_house(&vec![SIX_HEARTS, SIX_SPADES, SIX_DIAMONDS, TEN_CLUBS, TEN_HEARTS, JACK_DIAMONDS, KING_SPADES]).is_some());
        assert!(get_full_house(&vec![EIGHT_SPADES, EIGHT_DIAMONDS, EIGHT_CLUBS, JACK_SPADES, JACK_CLUBS, THREE_HEARTS, SIX_DIAMONDS]).is_some());
        assert!(get_full_house(&vec![SEVEN_CLUBS, SEVEN_DIAMONDS, SEVEN_SPADES, ACE_CLUBS, ACE_DIAMONDS, TEN_SPADES, QUEEN_HEARTS]).is_some());
        assert!(get_full_house(&vec![QUEEN_HEARTS, QUEEN_SPADES, QUEEN_DIAMONDS, FOUR_DIAMONDS, FOUR_SPADES, NINE_HEARTS, JACK_CLUBS]).is_some());
        assert!(get_full_house(&vec![JACK_SPADES, JACK_HEARTS, JACK_DIAMONDS, NINE_SPADES, NINE_DIAMONDS, FIVE_CLUBS, EIGHT_HEARTS]).is_some());
        assert!(get_full_house(&vec![TWO_DIAMONDS, TWO_SPADES, TWO_HEARTS, SEVEN_HEARTS, SEVEN_CLUBS, NINE_DIAMONDS, TEN_SPADES]).is_some());
        assert!(get_full_house(&vec![NINE_DIAMONDS, NINE_CLUBS, NINE_SPADES, TEN_CLUBS, TEN_HEARTS, KING_SPADES, EIGHT_DIAMONDS]).is_some());
        assert!(get_full_house(&vec![EIGHT_SPADES, EIGHT_HEARTS, EIGHT_DIAMONDS, JACK_HEARTS, JACK_CLUBS, NINE_SPADES, SEVEN_DIAMONDS]).is_some());
        assert!(get_full_house(&vec![FOUR_SPADES, FOUR_DIAMONDS, FOUR_CLUBS, SIX_SPADES, SIX_CLUBS, SEVEN_HEARTS, JACK_DIAMONDS]).is_some());
        assert!(get_full_house(&vec![TWO_SPADES, TWO_DIAMONDS, TWO_HEARTS, ACE_DIAMONDS, ACE_CLUBS, FOUR_HEARTS, SIX_SPADES]).is_some());
        assert!(get_full_house(&vec![THREE_CLUBS, THREE_DIAMONDS, THREE_HEARTS, JACK_CLUBS, JACK_HEARTS, KING_DIAMONDS, NINE_SPADES]).is_some());
        assert!(get_full_house(&vec![NINE_SPADES, NINE_HEARTS, NINE_CLUBS, FOUR_CLUBS, FOUR_HEARTS, SIX_SPADES, QUEEN_DIAMONDS]).is_some());
        assert!(get_full_house(&vec![FOUR_CLUBS, FOUR_SPADES, FOUR_DIAMONDS, ACE_HEARTS, ACE_DIAMONDS, TWO_CLUBS, QUEEN_SPADES]).is_some());
        assert!(get_full_house(&vec![SEVEN_HEARTS, SEVEN_DIAMONDS, SEVEN_CLUBS, TEN_SPADES, TEN_CLUBS, QUEEN_DIAMONDS, FIVE_HEARTS]).is_some());
        assert!(get_full_house(&vec![JACK_HEARTS, JACK_SPADES, JACK_DIAMONDS, TEN_HEARTS, TEN_SPADES, SEVEN_CLUBS, EIGHT_DIAMONDS]).is_some());
        assert!(get_full_house(&vec![FIVE_DIAMONDS, FIVE_SPADES, FIVE_HEARTS, NINE_CLUBS, NINE_SPADES, JACK_DIAMONDS, FOUR_HEARTS]).is_some());
        assert!(get_full_house(&vec![SIX_SPADES, SIX_DIAMONDS, SIX_HEARTS, EIGHT_CLUBS, EIGHT_DIAMONDS, SEVEN_HEARTS, FOUR_SPADES]).is_some());
        assert!(get_full_house(&vec![SEVEN_DIAMONDS, SEVEN_SPADES, SEVEN_CLUBS, SIX_CLUBS, SIX_HEARTS, JACK_SPADES, THREE_DIAMONDS]).is_some());
        assert!(get_full_house(&vec![THREE_HEARTS, THREE_CLUBS, THREE_SPADES, FOUR_CLUBS, FOUR_SPADES, KING_DIAMONDS, TEN_HEARTS]).is_some());
        assert!(get_full_house(&vec![ACE_DIAMONDS, ACE_SPADES, ACE_HEARTS, SIX_CLUBS, SIX_HEARTS, TEN_SPADES, TWO_DIAMONDS]).is_some());
        assert!(get_full_house(&vec![NINE_HEARTS, NINE_SPADES, NINE_CLUBS, TEN_CLUBS, TEN_SPADES, THREE_DIAMONDS, ACE_HEARTS]).is_some());
        assert!(get_full_house(&vec![SEVEN_DIAMONDS, SEVEN_CLUBS, SEVEN_SPADES, TWO_HEARTS, TWO_DIAMONDS, NINE_CLUBS, JACK_SPADES]).is_some());
        assert!(get_full_house(&vec![JACK_HEARTS, JACK_CLUBS, JACK_DIAMONDS, ACE_HEARTS, ACE_DIAMONDS, SIX_CLUBS, FIVE_SPADES]).is_some());
        assert!(get_full_house(&vec![SIX_HEARTS, SIX_SPADES, SIX_DIAMONDS, TEN_SPADES, TEN_HEARTS, QUEEN_DIAMONDS, TWO_CLUBS]).is_some());
    }


    #[test]
    fn two_pair() {

    }

}
