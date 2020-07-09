#!/usr/bin/env python3

from random import shuffle
from typing import List, Dict, Optional, Tuple, Union

SPADES   = "♠"
CLUBS    = "♣"
HEARTS   = "♥"
DIAMONDS = "♦"

suits = [SPADES, CLUBS, HEARTS, DIAMONDS]
ranks = list(range(1,14))

def list_hand_strings(print_as_grid=False):
    """
    Give a list of all hand strings such as KTo, etc. These may be printed in a
    range grid
    """
    result = []
    ranks = '23456789TJQKA'[::-1]
    for (i,r) in enumerate(ranks):
        for (j,s) in enumerate(ranks):

            if i > j:
                result.append(s + r + 'o')
            elif i == j:
                result.append(r + s)
            else:
                result.append(r + s + 's')
    if print_as_grid:
        work_list = list(result)
        while work_list:
            to_print, work_list = work_list[:13], work_list[13:]
            print(' '.join(["{:3}".format(s) for s in to_print]))
    return result

HAND_STRINGS = set(list_hand_strings())

class Card:
    rank_strings = ['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K']
    def __init__(self, rank, suit):
        self.rank = rank
        self.suit = suit

    def __lt__(self, other):
        if not isinstance(other, Card):
            return False
        if self.rank == 1:
            return False
        if other.rank == 1:
            return True
        return self.rank < other.rank

    def __eq__(self, other):
        return isinstance(other, Card) and other.suit == self.suit and other.rank == self.rank

    @staticmethod
    def rank_as_str(rank):
        try:
            return Card.rank_strings[rank - 1]
        except Error as e:
            raise IndexError("Cannot lookup rank {}".format(self.rank))

    def __str__(self):
        return "{}{}".format(Card.rank_as_str(self.rank), self.suit)

    def __repr__(self):
        return str(self)

CARDS = [Card(r, s) for s in suits for r in ranks]
card_lookup = {}
rank_to_int = {
    'A': 1,
    'K': 13,
    'Q': 12,
    'J': 11,
    'T': 10,
    '10': 10,
    '9': 9,
    '8': 8,
    '7': 7,
    '6': 6,
    '5': 5,
    '4': 4,
    '3': 3,
    '2': 2
}

for c in CARDS:
    card_lookup[(c.rank, c.suit)] = c

def new_deck():
    return list(CARDS)

def get_card(rank, suit):
    return card_lookup[rank, suit]

def parse_card_str(s):
    return get_card(rank_to_int[s[:-1]], s[-1])

def parse_card_strs(ss):
    return [parse_card_str(s) for s in ss]

class RankClass:
    """
    A set of cards of the same rank
    """
    def __init__(self, rank, *suits):
        self.rank = rank
        self.suits = set(suits)

    def __len__(self):
        return len(self.suits)

    def __lt__(self, other):
        if not isinstance(other, RankClass):
            return False
        if self.rank == 1:
            return False
        if other.rank == 1:
            return True
        return self.rank < other.rank

    def __str__(self):
        return "RankClass<{},{}>".format(Card.rank_as_str(self.rank), self.suits)

    def __repr__(self):
        return str(self)

    def add_suit(self, suit):
        self.suits.add(suit)

    def pick_one(self):
        if not self.suits:
            raise RuntimeError("cannot pick from empty set")
        return get_card(self.rank, next(iter(self.suits)))

    def as_card_list(self):
        r = self.rank
        return [get_card(r, s) for s in self.suits]

class SuitClass:
    """
    A set of cards of the same suit
    """
    def __init__(self, suit, *ranks):
        self.suit = suit
        self.ranks = set(ranks)

    def __len__(self):
        return len(self.ranks)


    def __str__(self):
        return "SuitClass<{},[{}]>".format(self.suit, ', '.join([Card.rank_as_str(r) for r in self.ranks]))

    def __repr__(self):
        return str(self)

    def add_rank(self, rank):
        self.ranks.add(rank)

    def pick_one(self):
        if not self.ranks:
            raise RuntimeError("cannot pick from empty set")
        return get_card(next(iter(self.ranks)), self.suit)

    def as_card_list(self):
        s = self.suit
        return [get_card(r, s) for r in self.ranks]

    def to_rank_classes(self) -> List[RankClass]:
        return [RankClass(rank, self.suit) for rank in self.ranks]


def ranks_are_adjacent(r, s):
    """
    A helper function to determine if two ranks are adjacent. This us used to
    calculate straights, and handles the Ace being adjacent to both 2s and Kings
    """
    if r > s:
        r, s = s, r
    return s - r == 1 or s - r == 12

def gather_into_rank_classes(cards: List[Card]) -> List[RankClass]:
    """
    Given a list of cards, gather them into a sorted list of RankClasses
    """
    rank_map: Dict[int, RankClass] = {}
    for card in cards:
        rank_map.setdefault(card.rank, RankClass(card.rank))
        rank_map[card.rank].add_suit(card.suit)
    return sorted(list(rank_map.values()), reverse=True)

def gather_into_suit_classes(cards: List[Card]) -> List[SuitClass]:
    """
    Given a list of cards, gather them into a sorted list of RankClasses
    """
    suit_map: Dict[str, SuitClass] = {}
    for card in cards:
        suit_map.setdefault(card.suit, SuitClass(card.suit))
        suit_map[card.suit].add_rank(card.rank)
    return list(suit_map.values())

def compare_kickers_for_lt(cs, ds):
    for c, d in zip(cs, ds):
        if c < d:
            return True
        if c > d:
            return False
    return False

class HandRank:
    """
    Abstract class representing a hand rank
    """
    def ranking(self):
        return -1

    def __lt__(self, other):
        if self.ranking() < other.ranking():
            return True
        if self.ranking() > other.ranking():
            return False
        return self.break_lt_tie(other)

    def break_lt_tie(self, other):
        raise NotImplemented()

    def __str__(self):
        return "{}{}".format(self.name, self.cards)

    def __repr__(self):
        return str(self)

class StraightFlush(HandRank):
    def __init__(self, cards):
        self.cards = cards
        self.name = "StraightFlush"

    def ranking(self):
        return 8

    def break_lt_tie(self, other):
        return self.cards[-1] < other.cards[-1]


class Quads(HandRank):
    def __init__(self, quads, kicker):
        self.quads = quads
        self.kicker = kicker
        self.cards = quads + kicker
        self.name = "Quads"

    def ranking(self):
        return 7

    def break_lt_tie(self, other):
        return self.quads[0] < other.quads[0]


class FullHouse(HandRank):
    def __init__(self, trips, pair):
        self.trips = trips
        self.pair = pair
        self.cards = trips + pair
        self.name = "FullHouse"

    def ranking(self):
        return 6

    def break_lt_tie(self, other):
        t1, t2= self.trips[0], other.trips[0]
        if t1 < t2:
            return True
        if t1 > t2:
            return False
        return self.pair[0] < other.pair[0]

class Flush(HandRank):
    def __init__(self, cards):
        self.cards = cards
        self.name = "Flush"

    def ranking(self):
        return 5

    def break_lt_tie(self, other):
        for c1, c2 in zip(self.cards, other.cards):
            if c1 < c2:
                return True
            if c1 > c2:
                return False
        return False

class Straight(HandRank):
    def __init__(self, cards):
        self.cards = cards
        self.name = "Straight"

    def ranking(self):
        return 4

    def break_lt_tie(self, other):
        return self.cards[-1] < other.cards[-1]

class Set(HandRank):
    def __init__(self, trips, kickers):
        self.trips = trips
        self.kickers = kickers
        self.cards = trips + kickers
        self.name = "Set"

    def ranking(self):
        return 3

    def break_lt_tie(self, other):
        return self.trips[0] < other.trips[0]

class TwoPair(HandRank):
    def __init__(self, pair1, pair2, kicker):
        self.pair1 = pair1
        self.pair2 = pair2
        self.kicker = kicker
        self.cards = pair1 + pair2 + kicker
        self.name = "TwoPair"

    def ranking(self):
        return 2

    def break_lt_tie(self, other):
        my_p1 = self.pair1[0]
        my_p2 = self.pair2[0]
        their_p1 = other.pair1[0]
        their_p2 = other.pair2[0]
        # make sure my_p1 > my_p2
        if my_p1 < my_p2:
            my_p1, my_p2 = my_p2, my_p1

        # make sure their_p1 > their_p2
        if their_p1 < their_p2:
            their_p1, their_p2 = their_p2, their_p1

        if my_p1 < their_p1:
            return True
        if my_p1 > their_p1:
            return False
        if my_p2 < their_p2:
            return True
        if my_p2 > their_p2:
            return False
        return self.kicker[0] < other.kicker[0]

class Pair(HandRank):
    def __init__(self, pair, kickers):
        self.pair = pair
        self.kickers = kickers
        self.cards = pair + kickers
        self.name = "Pair"

    def ranking(self):
        return 1

    def break_lt_tie(self, other):
        c1, c2 = self.pair[0], other.pair[0]
        if c1 < c2:
            return True
        if c1 > c2:
            return False
        for c1, c2 in zip(self.kickers, other.kickers):
            if c1 < c2:
                return True
            if c1 > c2:
                return False
        return False

class HighCard(HandRank):
    def __init__(self, cards):
        self.cards = cards
        self.name = "HighCard"

    def ranking(self):
        return 0

    def break_lt_tie(self, other):
        return compare_kickers_for_lt(self.cards, other.cards)

def get_high_straight_flush(scs: List[SuitClass]) -> Optional[StraightFlush]:
    """
    Return the highest StraightFlush if possible, and None otherwise
    """
    for sc in scs:
        if len(sc) >= 5:
            s = get_high_straight(sc.to_rank_classes())
            if s:
                return StraightFlush(s.cards)
    return None

def get_high_straight(rcs: List[RankClass]) -> Optional[Straight]:
    """
    Return the highest Straight if possible, and None otherwise
    """
    rcs = list(rcs)
    if len(rcs) < 5:
        return None
    if len(rcs) > 7:
        raise RuntimeError("Cannot process more than 7 cards!")

    if rcs[0].rank == 1:        # Is an ace
        rcs.insert(0, rcs[0])   # Put it at the bottom as well

    # The algo works as follows: we know that there are at most 8 entries in
    # rcs. Therefore, any straight starting at the first card must use cards
    # 1,2,3,4,5, and any straight starting at the last card must use cards
    # 4,5,6,7,8. This means that cards 4 and 5 have to be included in any
    # straight. We start with expanding out from the 4th card both left and
    # right.
    l = len(rcs)
    left = 3
    while left > 0:
        if ranks_are_adjacent(rcs[left-1].rank, rcs[left].rank):
            left -= 1
        else:
            break
    right = 4    # noninclusive
    while right < l:
        if ranks_are_adjacent(rcs[right-1].rank, rcs[right].rank):
            right += 1
        else:
            break
    if right - left >= 5:
        try:
            return Straight([rc.pick_one() for rc in rcs[left: left + 5]])
        except AttributeError as ae:
            print(ae)
            print(rcs[left: left + 5])
            raise ae
    return None

def get_high_flush(scs: List[SuitClass]) -> Optional[Flush]:
    """
    Return the highest Flush if possible, and None otherwise
    """
    for sc in scs:
        if len(sc) >= 5:
            flush = sorted(sc.as_card_list())
            return Flush(flush[-5:])
    return None

def get_high_quads(rcs: List[RankClass]) -> Optional[Quads]:
    """
    Return the highest Quads if possible, and None otherwise
    """
    quads: List[RankClass] = []
    kickers: List[RankClass] = []
    for rc in rcs:
        if len(rc) == 4:
            quads.append(rc)
        else:
            kickers.append(rc)
    if quads:
        return Quads(quads[0].as_card_list(), [kickers[0].pick_one()])
    return None

def get_high_set(rcs: List[RankClass]) -> Optional[Set]:
    """
    Return the highest Set if possible, and None otherwise
    """
    kickers = []
    sets = None
    for rc in rcs:
        if len(rc) == 3 and not sets:
            sets = rc.as_card_list()
        else:
            kickers += rc.as_card_list()
    if sets:
        return Set(sets, kickers[:2])
    return None

def get_high_pair(rcs: List[RankClass]) -> Optional[Pair]:
    """
    Return the highest Pair if possible, and None otherwise
    """
    kickers = []
    pairs = None
    for rc in rcs:
        if len(rc) == 2 and not pairs:
            pairs = rc.as_card_list()
        else:
            kickers += rc.as_card_list()
    if pairs:
        return Pair(pairs, kickers[:3])
    return None

def get_high_two_pair(rcs: List[RankClass]) -> Optional[TwoPair]:
    """
    Return the highest Pair if possible, and None otherwise
    """
    kickers: List[RankClass] = []
    pairs: List[RankClass] = []
    for rc in rcs:
        if len(rc) == 2 and len(pairs) < 2:
            pairs.append(rc)
        else:
            kickers.append(rc)
    if len(pairs) >= 2:
        try:
            return TwoPair(pairs[0].as_card_list(), pairs[1].as_card_list(), kickers[0].as_card_list())
        except IndexError as e:
            print("ERROR:", e, "dumping locals()")
            locs = locals()
            for k in locs:
                print(k, locs[k])
    return None

def get_high_full_house(rcs: List[RankClass]) -> Optional[FullHouse]:
    """
    Return the highest FullHouse if possible, and None otherwise
    """
    trips = get_high_set(rcs)
    if trips:
        pair = get_high_pair(rcs)
        if pair:
            return FullHouse(trips.trips, pair.pair)
    return None

def get_high_high_card(cards: List[Card]) -> HighCard:
    """
    Return the highest FullHouse if possible, and None otherwise
    """
    return HighCard(sorted(cards)[-5:])

def rank_hand(hand: List[Card]) -> HandRank:
    if len(hand) != 7:
        raise RuntimeError("Hand must be length 7")
    rcs = gather_into_rank_classes(hand)
    scs = gather_into_suit_classes(hand)
    return (get_high_straight_flush(scs) or
            get_high_quads(rcs) or
            get_high_full_house(rcs) or
            get_high_flush(scs) or
            get_high_straight(rcs) or
            get_high_set(rcs) or
            get_high_two_pair(rcs) or
            get_high_pair(rcs) or
            get_high_high_card(hand))


### Helper Functions
###

def rank_random_hands_of_7(iters: int, write=False) -> List[Tuple[List[Card], HandRank]]:
    """
    This is a debug printing function
    """
    result = []
    deck = new_deck()
    for i in range(iters):
        shuffle(deck)
        hand = deck[0:7]
        ranked = rank_hand(hand)
        result.append((deck[0:7], rank_hand(hand)))
    result.sort(key=lambda tup: tup[1])
    if write:
        for (hand, ranking) in result:
            print("{:<32}: {}".format(str(sorted(hand)), ranking))
    return result

def compute_distributions(iters:int = 100000):
    names=['highcard', 'pair', 'two-pair', 'set', 'straight', 'flush', 'full-house', 'quads', 'straight-flush']
    rank_freqs = {}
    for bin in range(10):
        rank_freqs[bin] = 0
    xs = rank_random_hands_of_7(iters)
    for x in xs:
        rank_freqs[x[1].ranking()] += 1
    for rank in range(9):
        print("{:15}:  {}/{} ({}%)".format(names[rank], rank_freqs[rank], iters, 100 * rank_freqs[rank] / iters))
    rank_frequencies = { names[i] : rank_freqs[i] for i in range(9)}
    return rank_frequencies

def compare_hole_cards(hole1: List[Card], hole2: List[Card], iters:int=10000, print_results:bool=False) -> Dict[str, int]:
    """
    Given two holes, play them against each other a bunch of times
    """
    deck = new_deck()
    for c in hole1:
        deck.remove(c)
    for c in hole2:
        deck.remove(c)

    h1_wins = 0
    h2_wins = 0
    ties = 0
    print('iters=', iters)
    for i in range(iters):
        shuffle(deck)
        board = deck[:5]
        h1 = rank_hand(board + hole1)
        h2 = rank_hand(board + hole2)
        if h1 < h2:
            h2_wins += 1
        elif h1 > h2:
            h1_wins += 1
        else:
            ties += 1
    print("{}    beats  {}:     {}/{} ({}%) times".format(hole1, hole2, h1_wins, iters, 100 * h1_wins/iters))
    print("{}  loses to {}:  {}/{} ({}%) times".format(hole1, hole2, h2_wins, iters, 100 * h2_wins/iters))
    print("{} ties with {}: {}/{} ({}%) times".format(hole1, hole2, ties, iters, 100 * ties/iters))
    return {'h1': h1_wins, 'h2': h2_wins, 'ties': ties, 'total': iters}

def play_hole_against_field(hole: Union[str, List[Card]],
                            field_size:int=1,
                            the_board:Union[List[Card], str]='',
                            iters:int=10000,
                            print_results:bool=False) -> Tuple[float, float, float]:
    """
    Given a hole, or a hole string, play it against the field for `iters` iterations and collect results

    Parameters
    ----------
    :param Union[str, List[Card]] hole:
            either a list of two cards [c1, c2] that _do not occur in the deck_, or a hole string
    :param int field_size:
            number of players to play against
    :param int iters:
            number of iterations to run simulation
    :param bool print_results:
            should we print the results to stdout?

    This returns the analysis as a tuple of `(prob win, prob ties, prob losses)`.

    The deck will be unmodified save for ordering
    """
    deck = new_deck()
    if isinstance(hole, str):
        if ' ' in hole:
            hole = parse_cards(hole)
        elif hole in HAND_STRINGS:
            opt_hole = search_deck_for_hand(hole, deck, remove=True)
            if not opt_hole:
                raise RuntimeError("Cannot find hand {} in deck {}".format(hole, deck))
            hole = opt_hole
        else:
            raise RuntimeError("Unrecognized hole string", hole)
    elif isinstance(hole, list):
        for c in hole:
            if c in deck:
                deck.remove(c)

    if the_board is None:
        the_board = []

    elif isinstance(the_board, str):
        the_board = parse_cards(the_board)

    statuses = [0, 0, 0]    # wins, ties, losses
    for _ in range(iters):
        shuffle(deck)
        fields = [[deck.pop(), deck.pop()] for _ in range(field_size)]
        if the_board is None:
            board = deck[:5]
        else:
            board = the_board + deck[:5 - len(the_board)]
        # print('fields:', fields, 'board:', board)
        hole_rank = rank_hand(hole + board)
        field_ranks = [rank_hand(h + board) for h in fields]

        status = 0    # Winning
        for fr in field_ranks:
            if hole_rank < fr:
                status = 2    # loss
                break
            elif not hole_rank > fr:
                status = 1
        # print(hole_rank, fr, status)
        statuses[status] += 1
        for f in fields:
            deck += f

    for c in hole:
        deck.append(c)

    wins, ties, losses = statuses

    if print_results:
        print("Trials: {}, Field Size: {}".format(iters, field_size))
        print("Hole:  {}\nBoard: {}".format(' '.join([str(c) for c in hole]),' '.join([str(c) for c in the_board])))
        print("- wins   {}%".format(100 * wins/iters))
        print("- ties   {}%".format(100 * ties/iters))
        print("- loses  {}%".format(100 * losses/iters))

    return wins/iters, ties/iters, losses/iters

def parse_cards(cards):
    return [parse_card(c) for c in cards.split()]

def parse_card(card):
    card = card.strip()
    suit = card[-1]
    rank = card[:-1]
    if rank == '10':
        rank = 'T'
    rank = ' A23456789TJQK'.index(rank)
    if suit == 'H':
        suit = HEARTS
    elif suit == 'D':
        suit = DIAMONDS
    elif suit == 'S':
        suit = SPADES
    elif suit == 'C':
        suit = CLUBS
    return Card(rank, suit)


def search_deck_for_hand(hand_str: str, deck: List[Card], remove:bool=False) -> Optional[List[Card]]:
    """
    Given a hand string like "AKo", search the deck for a hand matching it and return it.
    Parameters
    ----------
    :param str hand_str:
        the hand string to search for
    :param List[Card] deck:
        the deck to search through
    :param bool remove:
        should we remove the cards from the deck? Defaults to False
    """
    if hand_str[-1] == 'o':
        #offsuite
        r1 = rank_to_int[hand_str[0]]
        r2 = rank_to_int[hand_str[1]]
        if r1 == r2:
            raise RuntimeError("Cannot parse:", hand_str, ": ranks are identical")
        c1s = [card for card in deck if card.rank == r1]
        c2s = [card for card in deck if card.rank == r2]
        for c1 in c1s:
            for c2 in c2s:
                if c1.suit != c2.suit:
                    if remove:
                        deck.remove(c1)
                        deck.remove(c2)
                    return [c1, c2]
        return None
    elif hand_str[-1] == 's':
        #suited
        r1 = rank_to_int[hand_str[0]]
        r2 = rank_to_int[hand_str[1]]
        if r1 == r2:
            raise RuntimeError("Cannot parse:", hand_str, ": ranks are identical")
        c1s = [card for card in deck if card.rank == r1]
        c2s = [card for card in deck if card.rank == r2]
        for c1 in c1s:
            for c2 in c2s:
                if c1.suit == c2.suit:
                    return [c1, c2]
        return None
    elif len(hand_str) == 2 and hand_str[0] == hand_str[1]:
        r = rank_to_int[hand_str[0]]
        cs = [card for card in deck if card.rank == r]
        if len(cs) < 2:
            return None
        return cs[:2]

    else:
        raise RuntimeError("Unrecognized hand string:", hand_str)


def play_hand_strings(hand_str1, hand_str2, iters=1000) -> Dict[str, int]:
    deck = new_deck()
    h1 = search_deck_for_hand(hand_str1, deck)
    if not h1:
        raise RuntimeError("Deck", deck, "doesn't contain", hand_str1)
    for c in h1:
        deck.remove(c)
    h2 = search_deck_for_hand(hand_str2, deck)
    if not h2:
        raise RuntimeError("Deck", deck, "doesn't contain", hand_str2)
    return compare_hole_cards(h1, h2, iters=iters, print_results=True)


def rank_all_hands(field=1, iters=100, print_results=True):
    deck = new_deck()
    hand_rank_map = {}
    # Check all AXs and AXo
    for hole in list_hand_strings():
        wins, ties, losses = play_hole_against_field(hole=hole, field_size=1, iters=iters, print_results=False)
        hand_rank_map[hole] = (wins, ties, losses)
    # Next, sort keys
    if print_results:
        keys = sorted(hand_rank_map, key=lambda x: hand_rank_map[x][0], reverse=True)
        for key in keys:
            win, tie, loss = hand_rank_map[key]
            print("{:3} {:1.3f} {:1.3f} {:1.3f}".format(key, win, tie, loss))
    return hand_rank_map
