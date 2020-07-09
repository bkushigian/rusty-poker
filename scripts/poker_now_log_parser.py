#!/usr/bin/env python3

from sys import argv
from typing import Tuple, Dict
import csv

log_file = argv[1]

def print_action_item(name, action, amt='', other=''):
    print("{:9}  {:10} {:10} {}".format(name, action, str(amt), other))

def print_cards(title, cards):
    print("{:10}                                     {}".format(title, cards))

def read_between(line:str, begin:str, end:str, start:int=0) -> Tuple[str, int]:
    start_idx = line.index(begin, start) + len(begin)
    end_idx = line.index(end, start_idx)
    return line[start_idx: end_idx], end_idx + len(end)

def parse_starting_hand(line: str):
    hand_no, start = read_between(line, '#', ' ')
    dealer, _ = read_between(line, '"', ' @', start)
    print("Hand {}: Dealer: {}".format(hand_no, dealer))
    return {'dealer': dealer}

def parse_player_stacks(line: str) -> Dict[str, int]:
    start = 0
    stacks = {}
    xs = []
    while '@' in line[start:]:
        name, start = read_between(line, '"', ' @', start)
        amount, start = read_between(line, '(', ')', start)
        stacks[name] = int(amount)
        xs.append("{}: {}".format(name, amount))
    print(' | '.join(xs))
    return stacks

def parse_your_hand(line:str) -> str:
    hand = line[len("Your hand is "):]
    print_cards('Hole', hand)
    return hand

def parse_small_blind(line:str) -> Tuple[str, int]:
    name, start = read_between(line, '"', ' @')
    blind = line.split()[-1].strip()
    print_action_item(name, "SB", blind)
    return name, int(blind)

def parse_big_blind(line:str) -> Tuple[str, int]:
    name, start = read_between(line, '"', ' @')
    blind = int(line.split()[-1])
    print_action_item(name, "BB", blind)
    return name, int(blind)

def parse_folds(line:str) -> str:
    name, _ = read_between(line, '"', ' @')
    print_action_item(name, "FOLDS")
    return name

def parse_raise(line:str) -> Tuple[str, int, bool]:
    name, start = read_between(line, '"', ' @')
    amt = int(line.split()[-1])
    all_in = 'all in' in line
    if all_in:
        print_action_item(name, "ALL IN", amt)
    else:
        print_action_item(name, "RAISES", amt)
    return name, int(amt), all_in


def parse_calls(line:str):
    name, start = read_between(line, '"', ' @')
    amt = int(line.split()[-1])
    print_action_item(name, "CALLS", amt)
    return name, int(amt)

def parse_checks(line:str):
    name, start = read_between(line, '"', ' @')
    print_action_item(name, "CHECKS")
    return name

def parse_flop(line:str):
    flop, _ = read_between(line, '[', ']' )
    print('-'*80)
    print_cards('Flop', flop.replace(',', ''))
    return flop.split(',')

def parse_turn(line:str):
    board, _ = read_between(line, "turn: ", " [")
    turn, _ = read_between(line, '[', ']' )
    board = board.replace(',', '')
    print('-'*80)
    print_cards('Turn', board + ' ' + turn.replace(',', ''))
    return turn

def parse_river(line:str):
    board, _ = read_between(line, "river: ", " [")
    river, _ = read_between(line, '[', ']' )
    board = board.replace(',', '')
    print('-'*80)
    print_cards('River', board + ' ' + river.replace(',', ''))
    return river

def parse_shows(line:str):
    name, _ = read_between(line, '"', ' @')
    hole, _ = read_between(line, '" shows a ', '.')
    print_action_item(name, 'SHOWS', hole)
    return name, hole

def parse_wins(line:str):
    name, _ = read_between(line, '"', ' @')
    wins, _ = read_between(line, '" wins ', ' with ')
    hand, _ = read_between(line, ' with ', '(')
    hole, _ = read_between(line, '(hand: ', ')')
    hole.replace(',', '')
    print_cards("{}'s HOLE".format(name), hole)
    print_action_item(name, 'WINS', wins, hand)
    return name, wins, hand, hole

def parse_gained(line:str):
    name, _ = read_between(line, '"', ' @')
    gained = int(line.split()[-1])
    print_action_item(name, 'GAINED', gained)
    return name, gained

with open(log_file) as f:
    reader = csv.reader(f, delimiter=',', quotechar='"')
    pot = 0
    so_far: Dict[str, int] = {}   # How much has the player bet so far on this street
    stacks: Dict[str, int] = {}
    board: str = ''
    hole: str = ''

    def finish_betting():
        """
        Finish a round of betting
        """
        for name, amt in so_far.items():
            stacks[name] -= amt
            pot += amt
        so_far = {}   # Reset for next round of betting

    button = None
    bb = None
    sb = None
    for row in reader:
        data = row[0]
        if data.startswith('-- starting hand '):
            print()
            print()
            parse_starting_hand(row[0])
        elif data.startswith("Players stacks: "):
            current_stacks = parse_player_stacks(data)
        elif data.startswith("Your hand is "):
            hand = parse_your_hand(data)
        elif 'posts a small blind of ' in data:
            sb, amt = parse_small_blind(data)
            so_far[sb] = amt
        elif 'posts a big blind of ' in data:
            bb, amt = parse_big_blind(data)
            so_far[bb] = amt
        elif 'folds' in data:
            parse_folds(data)
        elif 'raises' in data:
            parse_raise(data)
        elif 'calls' in data:
            parse_calls(data)
        elif 'checks' in data:
            parse_checks(data)
        elif data.startswith('flop: '):
            parse_flop(data)
        elif data.startswith('turn: '):
            parse_turn(data)
        elif data.startswith('river: '):
            parse_river(data)
        elif 'shows a' in data:
            parse_shows(data)
        elif ' wins ' in data:
            parse_wins(data)
        elif ' gained ' in data:
            parse_gained(data)
