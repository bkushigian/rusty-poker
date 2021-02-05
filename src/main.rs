use std::env;
use rusty_poker::card::HoleCards;
use rusty_poker::equity::hand_vs_hand;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    if args.len() != 2 {
        panic!("Usage: hand-v-hand h1 h2")
    }

    let h1 = args.get(0).unwrap();
    let h2 = args.get(1).unwrap();

    let h1_cards: HoleCards = [h1[0..2].parse().unwrap(), h1[2..].parse().unwrap()];
    let h2_cards: HoleCards = [h2[0..2].parse().unwrap(), h2[2..].parse().unwrap()];

    let result = hand_vs_hand(&h1_cards, &h2_cards, &vec![], 10000);
    println!("{}: {}\n{}: {}\ntie:  {}", h1, result[0].pwin, h2, result[1].pwin, result[0].pdraw);
}
