use crate::card::{Card};
pub fn card_vec_to_card_array(cards: &Vec<Card>) -> Option<[Card; 5]> {
    if cards.len() >= 5 {
        Some([cards.get(0).unwrap().clone(),
              cards.get(1).unwrap().clone(),
              cards.get(2).unwrap().clone(),
              cards.get(3).unwrap().clone(),
              cards.get(4).unwrap().clone()])
    } else {
        None
    }
}
