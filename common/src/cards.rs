use std::time::{SystemTime, UNIX_EPOCH};

use crate::rand::SeedableRng;
use rand::prelude::SliceRandom;
use rand::rngs::SmallRng;
use serde::{Deserialize, Serialize};

pub const NUM_PLAYERS: usize = 4;
pub const TOTAL_CARDS: usize = 56;
pub const NUM_CARDS_BEFORE_GRAND_TICHU: usize = 9;
pub const NUM_CARDS_AFTER_GRAND_TICHU: usize =
    TOTAL_CARDS / NUM_PLAYERS - NUM_CARDS_BEFORE_GRAND_TICHU;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CardValue {
    Noop,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    J,
    Q,
    K,
    A,
}

impl Iterator for CardValue {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = match &self {
            CardValue::Noop => Some(CardValue::_2),
            CardValue::_2 => Some(CardValue::_3),
            CardValue::_3 => Some(CardValue::_4),
            CardValue::_4 => Some(CardValue::_5),
            CardValue::_5 => Some(CardValue::_6),
            CardValue::_6 => Some(CardValue::_7),
            CardValue::_7 => Some(CardValue::_8),
            CardValue::_8 => Some(CardValue::_9),
            CardValue::_9 => Some(CardValue::_10),
            CardValue::_10 => Some(CardValue::J),
            CardValue::J => Some(CardValue::Q),
            CardValue::Q => Some(CardValue::K),
            CardValue::K => Some(CardValue::A),
            CardValue::A => None,
        };
        if let Some(next_value) = &next_value {
            *self = next_value.clone()
        }
        next_value
    }
}

/// Enum of every possible card in Tichu
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Card {
    Sword(CardValue),
    Jade(CardValue),
    Pagoda(CardValue),
    Star(CardValue),
    MahJong,
    Dog,
    Phoenix,
    Dragon,
}

impl Card {
    pub fn start_iter() -> Card {
        Card::Sword(CardValue::Noop)
    }

    pub fn get_value(&self) -> Option<&CardValue> {
        match self {
            Card::Sword(card_value) => Some(card_value),
            Card::Jade(card_value) => Some(card_value),
            Card::Pagoda(card_value) => Some(card_value),
            Card::Star(card_value) => Some(card_value),
            _ => None,
        }
    }
}

impl Iterator for Card {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let next_card = match &self {
            Card::Sword(card_value) => {
                let next_card_value = card_value.clone().next();
                match next_card_value {
                    Some(next_card_value) => Some(Card::Sword(next_card_value)),
                    None => Some(Card::Jade(CardValue::_2)),
                }
            }
            Card::Jade(card_value) => {
                let next_card_value = card_value.clone().next();
                match next_card_value {
                    Some(next_card_value) => Some(Card::Jade(next_card_value)),
                    None => Some(Card::Pagoda(CardValue::_2)),
                }
            }
            Card::Pagoda(card_value) => {
                let next_card_value = card_value.clone().next();
                match next_card_value {
                    Some(next_card_value) => Some(Card::Pagoda(next_card_value)),
                    None => Some(Card::Star(CardValue::_2)),
                }
            }
            Card::Star(card_value) => {
                let next_card_value = card_value.clone().next();
                match next_card_value {
                    Some(next_card_value) => Some(Card::Star(next_card_value)),
                    None => Some(Card::MahJong),
                }
            }
            Card::MahJong => Some(Card::Dog),
            Card::Dog => Some(Card::Phoenix),
            Card::Phoenix => Some(Card::Dragon),
            Card::Dragon => None,
        };

        if let Some(next_card) = &next_card {
            *self = next_card.clone();
        }
        next_card
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Deck(Vec<Card>);

impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::with_capacity(56);

        for card in Card::start_iter() {
            cards.push(card);
        }

        Deck(cards)
    }
}

impl Deck {
    /// Creates a new, full, sorted Deck (i.e. it is NOT shuffled)
    pub fn new() -> Deck {
        Deck::default()
    }

    pub fn shuffle(&mut self) -> &mut Self {
        let pseudo_rand_num = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        self.0
            .shuffle(&mut SmallRng::seed_from_u64(pseudo_rand_num));
        self
    }

    pub fn draw(&mut self, number: usize) -> Vec<Card> {
        // limit draws to size of deck
        let number = if number > self.0.len() {
            self.0.len()
        } else {
            number
        };

        let mut cards = Vec::with_capacity(number);
        for _ in 0..number {
            let popped_card = self.0.pop();
            if let Some(popped_card) = popped_card {
                cards.push(popped_card);
            }
        }

        cards
    }
}

/// a single card
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Single(Card);

/// a pair of cards of equal value
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Pair {
    value: CardValue,
    cards: Vec<Card>,
}

/// a sequence of pairs of adjacent value
/// u8 = number of pairs
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SequenceOfPairs {
    starting_value: CardValue,
    number_of_pairs: u8,
    cards: Vec<Card>,
}

/// a trio of cards of equal value
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Trio {
    value: CardValue,
    cards: Vec<Card>,
}

/// a bomb (4 of the same value)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct BombOf4 {
    value: CardValue,
    cards: Vec<Card>,
}

/// a bomb (sequence of 5+ of all the same suit)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct BombOfSequence {
    suit: Card,
    starting_value: CardValue,
    number_of_cards: u8,
    cards: Vec<Card>,
}

/// a full house (trio + pair)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FullHouse {
    trio_value: CardValue,
    pair_value: CardValue,
    lowest_value: CardValue,
    cards: Vec<Card>,
}

/// a sequence of length at least 5
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Sequence {
    starting_value: CardValue,
    number_of_cards: u8,
    cards: Vec<Card>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ValidCardCombos {
    /// a single card
    Single(Single),

    /// a pair of cards of equal value
    Pair(Pair),

    /// a sequence of pairs of adjacent value
    /// u8 = number of pairs
    SequenceOfPairs(SequenceOfPairs),

    /// a trio of cards of equal value
    Trio(Trio),

    /// a bomb (4 of the same value)
    BombOf4(BombOf4),

    /// a bomb (sequence of 5+ of all the same suit)
    BombOfSequence(BombOfSequence),

    /// a full house (trio + pair)
    FullHouse(FullHouse),

    /// a sequence of length at least 5
    Sequence(Sequence),
}
