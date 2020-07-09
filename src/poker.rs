type Result<T> = std::result::Result<T, String>;

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
    suit: Suit,
    rank: Rank
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    /// Players name
    name: String,

    /// Their hole cards. These are wrapped in an Option since we may not be
    /// able to see their cards
    hole: Option<[Card; 2]>,

    /// Players current stack
    stack: u32,

    /// Are they alive in the hand?
    live: bool,

    /// Amount bet so far in the current street
    bet: u32,
}

/// The actions a player can take
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Check,
    Call ,
    Bet  (u32),
    Raise(u32),
    Fold
}

pub type Round = Vec<Action>;

/// A street involves community cards being revealed followed by a round of betting.
#[derive(Clone, Debug)]
pub struct Street {
    cards: Vec<Card>,
    round: Round,
}

/// A hand of poker, not to be confused to the two hole cards dealt to a player
/// at the start of play, includes all four rounds of betting. This can be
/// thought of as a single 'turn'.
///
/// The hand can be thought of as a VM, and each action as the bytecode of the game
#[derive(Clone)]
pub struct Hand {
    /// The players playing this hand
    players: Vec<Player>,

    /// Current pot
    pot: u32,

    /// Small and big blinds
    blinds: (u32,u32),

    /// index into the players of who is on the button
    button: usize,

    /// who is the action currently on?
    current_action: usize,

    /// All the actions that have happened so far in the hand---this lets us
    /// move back and forth easily
    history: Vec<Action>,

    /// The streets (pre-flop, flop, turn, river)
    streets: [Option<Street>; 4],

    /// which street are we on?
    current_street: usize,

    /// How much has been bet so far (amount needed to match)
    current_bet: u32,

    /// How much is the minimum bet?
    min_bet: u32,

    /// Last player to bet (None if no bets so far)
    last_player_to_bet: Option<usize>,

    /// How many bets since the last bet? This variable takes care of telling
    /// when a street is done. At the start of a street, this is initialzied to
    /// 0. Then, after any given bet, this is set to 1. After non-betting
    /// actions (bets or raises), this is increased by 1.
    bets_since_last_bet: usize,
}

impl Hand {
    pub fn big_blind(&self) -> usize {
        (self.button + 2) % self.players.len()
    }

    pub fn small_blind(&self) -> usize {
        (self.button + 1) % self.players.len()
    }

    pub fn utg(&self) -> usize {
        (self.button + 3) % self.players.len()
    }

    pub fn button(&self) -> usize {
        self.button
    }

    /// Who gets the final bet. This is computed as follows:
    /// - If there hasn't already been a bet (self.last_player_to_bet == None),
    ///   and the hand is currently preflop, the big-blind will be the final
    ///   better
    /// - If there hasn't already been a bet, and the hand is not currently
    ///   preflop (either on flop, turn, or river), then the button will be the
    ///   final better
    /// - If there has been a bet, the player preceeding the player that was
    ///   last to bet will be the finall better.
    pub fn final_better(&self) -> usize {
        match self.last_player_to_bet {
            None =>
                if self.current_street == 0 {
                    self.big_blind()
                } else {
                    self.button()
                },
            Some(last_player_to_bet) => (last_player_to_bet + self.players.len() - 1) % self.players.len()
        }
    }

    pub fn get_player(&self, player: usize) -> Result<&Player> {
        self.players.get(player).ok_or_else(|| "Index out of bounds".into())
    }

    pub fn get_current_player(&self) -> &Player {
        self.get_player(self.current_action).unwrap()
    }

    pub fn process_action(&mut self, action: Action) -> Result<()> {
        if self.bets_since_last_bet >= self.players.len() {
            Err("No more actions available on the current street".into())
        } else {
            match action {
                Action::Raise(amt) => self.raise(amt),
                Action::Bet(amt) => self.bet(amt),
                Action::Check => self.check(),
                Action::Call => self.call(),
                Action::Fold => self.fold(),
            }
        }
    }

    pub fn check(&mut self) -> Result<()> {
        let current_player = self.get_current_player();
        if current_player.bet != self.current_bet {
            Err("Cannot perform Check action: current player must call, raise, or fold".into())
        } else {
            self.current_action = (self.current_action + 1) % self.players.len();
            Ok(())
        }
    }

    pub fn raise(&mut self, amount: u32) -> Result<()> {
        let current_player = self.get_current_player();
        let current_bet = self.current_bet;
        let min_bet = self.min_bet;
        if amount == 0 {
            return Err("Must bet positive amount".into());
        }

        if amount == current_player.stack {

        }
        // Make sure current_bet is at least min raise
        if amount < current_bet + min_bet {
            return Err("Must bet at least minimum bet");
        }
        if amount > current_player.stack {
            Err("Player cannot bet current stack")
        }
    }

    pub fn bet(&mut self, amount: u32) -> Result<()> {
        panic!("todo")
    }

    pub fn call(&mut self) -> Result<()> {
        panic!("todo")
    }

    pub fn fold(&mut self) -> Result<()> {
        panic!("todo")
    }
}

pub fn apply_action_to_hand(hand: &mut Hand, action: &Action) -> bool {
    match action {
        // For a check to be valid, the current player must
        _ => false
    }
}

impl ToString for Suit {
    fn to_string(&self) -> String {
        match self {
            Suit::Spades    => String::from("♠"),
            Suit::Clubs     => String::from("♣"),
            Suit::Hearts    => String::from("♥"),
            Suit::Diamonds  => String::from("♦"),
        }
    }
}

static RANKS_STRING_ARRAY: [&str; 14] =
    ["ERROR",
     "A",
     "2",
     "3",
     "4",
     "5",
     "6",
     "7",
     "8",
     "9",
     "10",
     "J",
     "Q",
     "K",
    ];

impl ToString for Card {
    fn to_string(&self) -> String {
        assert!(1 <= self.rank && self.rank <= 13, "Ranks must be between 1 and 13, inclusive");
        String::from(RANKS_STRING_ARRAY[usize::from(self.rank)])
    }
}
