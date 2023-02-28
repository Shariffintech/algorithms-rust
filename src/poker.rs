// build a algorithm to calculate the odds of winning a poker hand bas off of the cards in your hand and the cards on the table

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Hand {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

impl Hand {
    fn new() -> Hand {
        Hand::HighCard
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct HandValue {
    hand: Hand,
    rank: Rank,
}

impl HandValue {
    fn new(hand: Hand, rank: Rank) -> HandValue {
        HandValue { hand, rank }
    }
}

fn main(flop: Vec<Card>, turn: Card, river: Card) {
    let mut deck = VecDeque::new();


    for suit in vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
        


    }