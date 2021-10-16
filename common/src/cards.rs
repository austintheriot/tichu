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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CardSuit {
    Sword,
    Jade,
    Pagoda,
    Star,
    MahJong,
    Dog,
    Phoenix,
    Dragon,
}

/// Enum of every possible card in Tichu
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Card {
    pub suit: CardSuit,
    pub value: CardValue,
}

impl Card {
    pub fn start_iter() -> Card {
        Card {
            suit: CardSuit::Sword,
            value: CardValue::Noop,
        }
    }
}

impl Iterator for Card {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let next_card = match &self {
            Card {
                suit: CardSuit::Sword,
                value: card_value,
            } => {
                let next_card_value = card_value.clone().next();
                match next_card_value {
                    Some(next_card_value) => Some(Card {
                        suit: CardSuit::Sword,
                        value: next_card_value,
                    }),
                    None => Some(Card {
                        suit: CardSuit::Jade,
                        value: CardValue::_2,
                    }),
                }
            }
            Card {
                suit: CardSuit::Jade,
                value: card_value,
            } => {
                let next_card_value = card_value.clone().next();
                match next_card_value {
                    Some(next_card_value) => Some(Card {
                        suit: CardSuit::Jade,
                        value: next_card_value,
                    }),
                    None => Some(Card {
                        suit: CardSuit::Pagoda,
                        value: CardValue::_2,
                    }),
                }
            }
            Card {
                suit: CardSuit::Pagoda,
                value: card_value,
            } => {
                let next_card_value = card_value.clone().next();
                match next_card_value {
                    Some(next_card_value) => Some(Card {
                        suit: CardSuit::Pagoda,
                        value: next_card_value,
                    }),
                    None => Some(Card {
                        suit: CardSuit::Star,
                        value: CardValue::_2,
                    }),
                }
            }
            Card {
                suit: CardSuit::Star,
                value: card_value,
            } => {
                let next_card_value = card_value.clone().next();
                match next_card_value {
                    Some(next_card_value) => Some(Card {
                        suit: CardSuit::Star,
                        value: next_card_value,
                    }),
                    None => Some(Card {
                        suit: CardSuit::MahJong,
                        value: CardValue::Noop,
                    }),
                }
            }
            Card {
                suit: CardSuit::MahJong,
                ..
            } => Some(Card {
                suit: CardSuit::Dog,
                value: CardValue::Noop,
            }),
            Card {
                suit: CardSuit::Dog,
                ..
            } => Some(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::Noop,
            }),
            Card {
                suit: CardSuit::Phoenix,
                ..
            } => Some(Card {
                suit: CardSuit::Dragon,
                value: CardValue::Noop,
            }),
            Card {
                suit: CardSuit::Dragon,
                ..
            } => None,
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

#[cfg(test)]
mod tests {
    mod test_deck {
        use crate::{Card, CardSuit, CardValue};

        use super::super::Deck;
        #[test]
        fn new_should_produce_a_vec_of_56_cards() {
            assert_eq!(Deck::new().0.len(), 56);
        }
        #[test]
        fn new_should_produce_a_sorted_deck() {
            let deck = Deck::new();
            assert_eq!(
                deck.0.get(0),
                Some(&Card {
                    suit: CardSuit::Sword,
                    value: CardValue::_2,
                })
            );
            assert_eq!(
                deck.0.get(12),
                Some(&Card {
                    suit: CardSuit::Sword,
                    value: CardValue::A,
                })
            );
            assert_eq!(
                deck.0.get(13),
                Some(&Card {
                    suit: CardSuit::Jade,
                    value: CardValue::_2,
                })
            );
            assert_eq!(
                deck.0.get(20),
                Some(&Card {
                    suit: CardSuit::Jade,
                    value: CardValue::_9,
                })
            );
            assert_eq!(
                deck.0.get(26),
                Some(&Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue::_2,
                })
            );
            assert_eq!(
                deck.0.get(39),
                Some(&Card {
                    suit: CardSuit::Star,
                    value: CardValue::_2,
                })
            );
            assert_eq!(
                deck.0.get(40),
                Some(&Card {
                    suit: CardSuit::Star,
                    value: CardValue::_3,
                })
            );
            assert_eq!(
                deck.0.get(52),
                Some(&Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::Noop,
                })
            );
            assert_eq!(
                deck.0.get(53),
                Some(&Card {
                    suit: CardSuit::Dog,
                    value: CardValue::Noop,
                })
            );
            assert_eq!(
                deck.0.get(54),
                Some(&Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::Noop,
                })
            );
            assert_eq!(
                deck.0.get(55),
                Some(&Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::Noop,
                })
            );
        }
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
