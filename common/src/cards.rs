use crate::rand::SeedableRng;
use rand::prelude::SliceRandom;
use rand::rngs::SmallRng;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

pub const NUM_PLAYERS: usize = 4;
pub const TOTAL_CARDS: usize = 56;
pub const NUM_CARDS_BEFORE_GRAND_TICHU: usize = 9;
pub const NUM_CARDS_AFTER_GRAND_TICHU: usize =
    TOTAL_CARDS / NUM_PLAYERS - NUM_CARDS_BEFORE_GRAND_TICHU;

pub const CARD_VALUE_NOOP: u8 = 0;
pub const CARD_VALUE_START_ITER: u8 = 1;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CardValue(pub u8);

impl CardValue {
    pub fn noop() -> Self {
        CardValue(CARD_VALUE_NOOP)
    }

    pub fn is_noop(card_value: &Self) -> bool {
        card_value.0 == CARD_VALUE_NOOP
    }

    pub fn start_iter() -> Self {
        CardValue(CARD_VALUE_START_ITER)
    }
}

impl Iterator for CardValue {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = match &self {
            CardValue(14) => None,
            CardValue(card_value) => Some(CardValue(card_value + 1)),
        };
        if let Some(next_value) = &next_value {
            *self = next_value.clone();
        };
        next_value
    }
}

#[cfg(test)]
mod test_card_value {
    use crate::CardValue;
    #[test]
    fn it_should_iterate_correctly() {
        let cards: Vec<CardValue> = CardValue::start_iter().into_iter().collect();

        // correct length
        assert_eq!(cards.len(), 13);

        // sorted correctly
        assert_eq!(cards.get(0), Some(&CardValue(2)));
        assert_eq!(cards.get(7), Some(&CardValue(9)));
        assert_eq!(cards.get(12), Some(&CardValue(14)));
        assert_eq!(cards.get(13), None);
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

pub const CARD_SUIT_START_ITER: CardSuit = CardSuit::Sword;

/// Enum of every possible card in Tichu
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Card {
    pub suit: CardSuit,
    pub value: CardValue,
}

impl Card {
    pub fn start_iter() -> Card {
        Card {
            suit: CARD_SUIT_START_ITER,
            value: CardValue::start_iter(),
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
                        value: CardValue(2),
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
                        value: CardValue(2),
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
                        value: CardValue(2),
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
                        value: CardValue::noop(),
                    }),
                }
            }
            Card {
                suit: CardSuit::MahJong,
                ..
            } => Some(Card {
                suit: CardSuit::Dog,
                value: CardValue::noop(),
            }),
            Card {
                suit: CardSuit::Dog,
                ..
            } => Some(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }),
            Card {
                suit: CardSuit::Phoenix,
                ..
            } => Some(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
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
pub struct Deck(pub Vec<Card>);

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
mod test_deck {
    use crate::{Card, CardSuit, CardValue, Deck};

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
                value: CardValue(2),
            })
        );
        assert_eq!(
            deck.0.get(12),
            Some(&Card {
                suit: CardSuit::Sword,
                value: CardValue(14),
            })
        );
        assert_eq!(
            deck.0.get(13),
            Some(&Card {
                suit: CardSuit::Jade,
                value: CardValue(2),
            })
        );
        assert_eq!(
            deck.0.get(20),
            Some(&Card {
                suit: CardSuit::Jade,
                value: CardValue(9),
            })
        );
        assert_eq!(
            deck.0.get(26),
            Some(&Card {
                suit: CardSuit::Pagoda,
                value: CardValue(2),
            })
        );
        assert_eq!(
            deck.0.get(39),
            Some(&Card {
                suit: CardSuit::Star,
                value: CardValue(2),
            })
        );
        assert_eq!(
            deck.0.get(40),
            Some(&Card {
                suit: CardSuit::Star,
                value: CardValue(3),
            })
        );
        assert_eq!(
            deck.0.get(52),
            Some(&Card {
                suit: CardSuit::MahJong,
                value: CardValue::noop(),
            })
        );
        assert_eq!(
            deck.0.get(53),
            Some(&Card {
                suit: CardSuit::Dog,
                value: CardValue::noop(),
            })
        );
        assert_eq!(
            deck.0.get(54),
            Some(&Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            })
        );
        assert_eq!(
            deck.0.get(55),
            Some(&Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            })
        );
    }
}

const COMPARE_IDENTICAL_SPECIAL_CARDS: &str = "Can't compare identical special cards to each other";

/// a single card
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Single(pub Card);
impl PartialOrd for Single {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // can't compare identical special cards
        if CardValue::is_noop(&self.0.value)
            && CardValue::is_noop(&other.0.value)
            && self.0.suit == other.0.suit
        {
            panic!("{COMPARE_IDENTICAL_SPECIAL_CARDS}");
        }

        // dragon beats everything
        if let CardSuit::Dragon = &self.0.suit {
            return Some(Ordering::Greater);
        }
        if let CardSuit::Dragon = &other.0.suit {
            return Some(Ordering::Less);
        }

        // Phoenix beats any standard card
        if let CardSuit::Phoenix = &self.0.suit {
            return Some(Ordering::Greater);
        }
        if let CardSuit::Phoenix = &other.0.suit {
            return Some(Ordering::Less);
        }

        Some([&self.0.value].cmp(&[&other.0.value]))
    }
}
impl Ord for Single {
    fn cmp(&self, other: &Self) -> Ordering {
        // can't compare identical special cards
        if CardValue::is_noop(&self.0.value)
            && CardValue::is_noop(&other.0.value)
            && self.0.suit == other.0.suit
        {
            panic!("{COMPARE_IDENTICAL_SPECIAL_CARDS}");
        }

        // dragon beats everything
        if let CardSuit::Dragon = &self.0.suit {
            return Ordering::Greater;
        }
        if let CardSuit::Dragon = &other.0.suit {
            return Ordering::Less;
        }

        // Phoenix beats any standard card
        if let CardSuit::Phoenix = &self.0.suit {
            return Ordering::Greater;
        }
        if let CardSuit::Phoenix = &other.0.suit {
            return Ordering::Less;
        }

        [&self.0.value].cmp(&[&other.0.value])
    }
}
impl PartialEq for Single {
    fn eq(&self, other: &Self) -> bool {
        if CardValue::is_noop(&self.0.value) && CardValue::is_noop(&other.0.value) {
            if self.0.suit == other.0.suit {
                // can't compare two identical special cards to each other
                panic!("{COMPARE_IDENTICAL_SPECIAL_CARDS}");
            } else {
                // only compare suits if they are special cards
                return self.0.suit == other.0.suit;
            }
        }

        self.0.value == other.0.value
    }
}
impl Eq for Single {}
#[cfg(test)]
mod test_single_card {
    use super::super::Single;
    use crate::{Card, CardSuit, CardValue};
    #[test]
    fn it_should_compare_std_cards_of_same_suit_correctly() {
        // different suit, is less than
        assert_eq!(
            Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(2)
            }) < Single(Card {
                suit: CardSuit::Jade,
                value: CardValue(3)
            }),
            true
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(2)
            }) == Single(Card {
                suit: CardSuit::Jade,
                value: CardValue(3)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(2)
            }) > Single(Card {
                suit: CardSuit::Jade,
                value: CardValue(3)
            }),
            false
        );

        // different suit, is equal to
        assert_eq!(
            Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(11)
            }) < Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(11)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(11)
            }) == Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(11)
            }),
            true
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(11)
            }) > Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(11)
            }),
            false
        );

        // different suit, is greater than
        assert_eq!(
            Single(Card {
                suit: CardSuit::Star,
                value: CardValue(14),
            }) < Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(11)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Star,
                value: CardValue(14),
            }) == Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(11)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Star,
                value: CardValue(14),
            }) > Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(11)
            }),
            true
        );
    }

    #[test]
    fn it_should_compare_std_cards_of_different_suits_correctly() {
        // different suit, is less than
        assert_eq!(
            Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(2)
            }) < Single(Card {
                suit: CardSuit::Jade,
                value: CardValue(3)
            }),
            true
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(2)
            }) == Single(Card {
                suit: CardSuit::Jade,
                value: CardValue(3)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(2)
            }) > Single(Card {
                suit: CardSuit::Jade,
                value: CardValue(3)
            }),
            false
        );

        // different suit, is equal to
        assert_eq!(
            Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(11)
            }) < Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(11)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(11)
            }) == Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(11)
            }),
            true
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(11)
            }) > Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(11)
            }),
            false
        );

        // different suit, is greater than
        assert_eq!(
            Single(Card {
                suit: CardSuit::Star,
                value: CardValue(14),
            }) < Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(11)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Star,
                value: CardValue(14),
            }) == Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(11)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Star,
                value: CardValue(14),
            }) > Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(11)
            }),
            true
        );
    }

    #[test]
    fn it_should_compare_dragon_correctly() {
        // Dragon to standard
        assert_eq!(
            Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }) < Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }) == Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }) > Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14)
            }),
            true
        );

        // standard to Dragon
        assert_eq!(
            Single(Card {
                suit: CardSuit::Star,
                value: CardValue(2),
            }) < Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }),
            true
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(7),
            }) == Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(13),
            }) > Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }),
            false
        );
    }

    #[test]
    #[should_panic(expected = "identical special cards")]
    fn comparing_two_dragons_for_equality_should_panic() {
        // suppress panic logs
        let original_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        assert_eq!(
            Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }) == Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }),
            true
        );
        std::panic::set_hook(original_panic_hook);
    }

    #[test]
    #[should_panic(expected = "identical special cards")]
    fn comparing_two_dragons_for_less_than_should_panic() {
        // suppress panic logs
        let original_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        assert_eq!(
            Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }) < Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }),
            false
        );
        std::panic::set_hook(original_panic_hook);
    }

    #[test]
    #[should_panic(expected = "identical special cards")]
    fn comparing_two_dragons_for_greater_than_should_panic() {
        // suppress panic logs
        let original_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        assert_eq!(
            Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }) > Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }),
            false
        );
        std::panic::set_hook(original_panic_hook);
    }

    #[test]
    fn it_should_compare_phoenix_correctly() {
        // Phoenix to standard
        assert_eq!(
            Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }) < Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }) == Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }) > Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14)
            }),
            true
        );

        // standard to Phoenix
        assert_eq!(
            Single(Card {
                suit: CardSuit::Star,
                value: CardValue(2),
            }) < Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }),
            true
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(7),
            }) == Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(14),
            }) > Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }),
            false
        );
    }

    #[test]
    #[should_panic(expected = "identical special cards")]
    fn comparing_two_phoenixes_for_equality_should_panic() {
        // suppress panic logs
        let original_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        assert_eq!(
            Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }) == Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }),
            true
        );
        std::panic::set_hook(original_panic_hook);
    }

    #[test]
    #[should_panic(expected = "identical special cards")]
    fn comparing_two_phoenixes_for_less_than_should_panic() {
        // suppress panic logs
        let original_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        assert_eq!(
            Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }) < Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }),
            false
        );
        std::panic::set_hook(original_panic_hook);
    }

    #[test]
    #[should_panic(expected = "identical special cards")]
    fn comparing_two_phoenixes_for_greater_than_should_panic() {
        // suppress panic logs
        let original_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        assert_eq!(
            Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }) > Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }),
            false
        );
        std::panic::set_hook(original_panic_hook);
    }

    #[test]
    fn it_should_compare_dragon_to_phoenix_correctly() {
        assert_eq!(
            Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }) < Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }),
            true
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }) == Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            }) > Single(Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            }),
            false
        );
    }
    #[test]
    fn it_should_compare_mahjong_correctly() {
        // MahJong to standard
        assert_eq!(
            Single(Card {
                suit: CardSuit::MahJong,
                value: CardValue::noop(),
            }) < Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14)
            }),
            true
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::MahJong,
                value: CardValue::noop(),
            }) == Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::MahJong,
                value: CardValue::noop(),
            }) > Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14)
            }),
            false
        );

        // standard to MahJong
        assert_eq!(
            Single(Card {
                suit: CardSuit::Star,
                value: CardValue(2),
            }) < Single(Card {
                suit: CardSuit::MahJong,
                value: CardValue::noop(),
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(7),
            }) == Single(Card {
                suit: CardSuit::MahJong,
                value: CardValue::noop(),
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(13),
            }) > Single(Card {
                suit: CardSuit::MahJong,
                value: CardValue::noop(),
            }),
            true
        );
    }

    #[test]
    fn it_should_compare_the_dog_correctly() {
        // Dog to standard
        assert_eq!(
            Single(Card {
                suit: CardSuit::Dog,
                value: CardValue::noop(),
            }) < Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14)
            }),
            true
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Dog,
                value: CardValue::noop(),
            }) == Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14)
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Dog,
                value: CardValue::noop(),
            }) > Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14)
            }),
            false
        );

        // standard to Dog
        assert_eq!(
            Single(Card {
                suit: CardSuit::Star,
                value: CardValue(2),
            }) < Single(Card {
                suit: CardSuit::Dog,
                value: CardValue::noop(),
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Pagoda,
                value: CardValue(7),
            }) == Single(Card {
                suit: CardSuit::Dog,
                value: CardValue::noop(),
            }),
            false
        );
        assert_eq!(
            Single(Card {
                suit: CardSuit::Sword,
                value: CardValue(13),
            }) > Single(Card {
                suit: CardSuit::Dog,
                value: CardValue::noop(),
            }),
            true
        );
    }
}

/// a pair of cards of equal value
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Pair {
    pub value: CardValue,
    pub cards: Vec<Card>,
}

/// a sequence of pairs of adjacent value
/// u8 = number of pairs
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SequenceOfPairs {
    pub starting_value: CardValue,
    pub number_of_pairs: u8,
    pub cards: Vec<Card>,
}

/// a trio of cards of equal value
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Trio {
    pub value: CardValue,
    pub cards: Vec<Card>,
}

/// a bomb (4 of the same value)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct BombOf4 {
    pub value: CardValue,
    pub cards: Vec<Card>,
}

/// a bomb (sequence of 5+ of all the same suit)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct BombOfSequence {
    pub suit: Card,
    pub starting_value: CardValue,
    pub number_of_cards: u8,
    pub cards: Vec<Card>,
}

/// a full house (trio + pair)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FullHouse {
    pub trio_value: CardValue,
    pub pair_value: CardValue,
    pub lowest_value: CardValue,
    pub cards: Vec<Card>,
}

/// a sequence of length at least 5
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Sequence {
    pub starting_value: CardValue,
    pub number_of_cards: u8,
    pub cards: Vec<Card>,
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
