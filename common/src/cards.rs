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
pub const MAX_CARDS_IN_HAND: usize = TOTAL_CARDS / NUM_PLAYERS;

pub const CARD_VALUE_NOOP: u8 = 0;
pub const CARD_VALUE_START_ITER: u8 = 1;
pub const CARD_VALUE_MIN: u8 = 2; // 2
pub const CARD_VALUE_MAX: u8 = 14; // Ace

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CardValue(pub u8);

impl CardValue {
    pub fn noop() -> Self {
        CardValue(CARD_VALUE_NOOP)
    }

    pub fn is_noop(&self) -> bool {
        self.0 == CARD_VALUE_NOOP
    }

    pub fn start_iter() -> Self {
        CardValue(CARD_VALUE_START_ITER)
    }

    pub fn add(&self, number: u8) -> Self {
        CardValue(self.0 + number)
    }

    pub fn min() -> Self {
        CardValue(CARD_VALUE_MIN)
    }

    pub fn max() -> Self {
        CardValue(CARD_VALUE_MAX)
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
    fn it_should_compare_values_correctly() {
        assert_eq!(CardValue::noop() < CardValue::min(), true);
        assert_eq!(CardValue(5) > CardValue(4), true);
        assert_eq!(CardValue(15) > CardValue::max(), true);
    }

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

impl CardSuit {
    pub fn is_special(&self) -> bool {
        self == &CardSuit::Phoenix
            || self == &CardSuit::Dragon
            || self == &CardSuit::MahJong
            || self == &CardSuit::Dog
    }
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
pub struct Single {
    pub card: Card,
    /// Phoenixes receive a value 0.5 higher than the previous card
    pub value: CardValue,
}
impl PartialOrd for Single {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // can't compare identical special cards
        if self.value.is_noop() && other.value.is_noop() && self.card.suit == other.card.suit {
            panic!("{COMPARE_IDENTICAL_SPECIAL_CARDS}");
        }

        // dragon beats everything
        if let CardSuit::Dragon = &self.card.suit {
            return Some(Ordering::Greater);
        }
        if let CardSuit::Dragon = &other.card.suit {
            return Some(Ordering::Less);
        }

        // Phoenix beats any standard card
        if let CardSuit::Phoenix = &self.card.suit {
            return Some(Ordering::Greater);
        }
        if let CardSuit::Phoenix = &other.card.suit {
            return Some(Ordering::Less);
        }

        Some([&self.value].cmp(&[&other.value]))
    }
}
impl Ord for Single {
    fn cmp(&self, other: &Self) -> Ordering {
        // can't compare identical special cards
        if self.value.is_noop() && other.value.is_noop() && self.card.suit == other.card.suit {
            panic!("{COMPARE_IDENTICAL_SPECIAL_CARDS}");
        }

        // dragon beats everything
        if let CardSuit::Dragon = &self.card.suit {
            return Ordering::Greater;
        }
        if let CardSuit::Dragon = &other.card.suit {
            return Ordering::Less;
        }

        // Phoenix beats any standard card
        if let CardSuit::Phoenix = &self.card.suit {
            return Ordering::Greater;
        }
        if let CardSuit::Phoenix = &other.card.suit {
            return Ordering::Less;
        }

        [&self.value].cmp(&[&other.value])
    }
}
impl PartialEq for Single {
    fn eq(&self, other: &Self) -> bool {
        if self.value.is_noop() && other.value.is_noop() {
            if self.card.suit == other.card.suit {
                // can't compare two identical special cards to each other
                panic!("{COMPARE_IDENTICAL_SPECIAL_CARDS}");
            } else {
                // only compare suits if they are special cards
                return self.card.suit == other.card.suit;
            }
        }

        self.value == other.value
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
            Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                },
                value: CardValue(2),
            } < Single {
                card: Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3)
                },
                value: CardValue(3),
            },
            true
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                },
                value: CardValue(2),
            } == Single {
                card: Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3)
                },
                value: CardValue(3),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                },
                value: CardValue(2)
            } > Single {
                card: Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3)
                },
                value: CardValue(3),
            },
            false
        );

        // different suit, is equal to
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                },
                value: CardValue(11),
            } < Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(11)
                },
                value: CardValue(11),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                },
                value: CardValue(11),
            } == Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(11)
                },
                value: CardValue(11),
            },
            true
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                },
                value: CardValue(11),
            } > Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(11)
                },
                value: CardValue(11),
            },
            false
        );

        // different suit, is greater than
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                },
                value: CardValue(14),
            } < Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                },
                value: CardValue(11),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                },
                value: CardValue(14),
            } == Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                },
                value: CardValue(11),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                },
                value: CardValue(14),
            } > Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                },
                value: CardValue(11),
            },
            true
        );
    }

    #[test]
    fn it_should_compare_std_cards_of_different_suits_correctly() {
        // different suit, is less than
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                },
                value: CardValue(2)
            } < Single {
                card: Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3)
                },
                value: CardValue(3),
            },
            true
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                },
                value: CardValue(2)
            } == Single {
                card: Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3)
                },
                value: CardValue(3),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                },
                value: CardValue(2)
            } > Single {
                card: Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3)
                },
                value: CardValue(3),
            },
            false
        );

        // different suit, is equal to
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                },
                value: CardValue(11),
            } < Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(11)
                },
                value: CardValue(11),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                },
                value: CardValue(11),
            } == Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(11)
                },
                value: CardValue(11),
            },
            true
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                },
                value: CardValue(11),
            } > Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(11)
                },
                value: CardValue(11),
            },
            false
        );

        // different suit, is greater than
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                },
                value: CardValue(14),
            } < Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                },
                value: CardValue(11),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                },
                value: CardValue(14),
            } == Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                },
                value: CardValue(11),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                },
                value: CardValue(14),
            } > Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                },
                value: CardValue(11),
            },
            true
        );
    }

    #[test]
    fn it_should_compare_dragon_correctly() {
        // Dragon to standard
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop()
            } < Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                },
                value: CardValue(14),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } == Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                },
                value: CardValue(14),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } > Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                },
                value: CardValue(14),
            },
            true
        );

        // standard to Dragon
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Star,
                    value: CardValue(2),
                },
                value: CardValue(2),
            } < Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            true
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(7),
                },
                value: CardValue(7)
            } == Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(13),
                },
                value: CardValue(13),
            } > Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
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
            Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } == Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
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
            Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } < Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
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
            Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } > Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            false
        );
        std::panic::set_hook(original_panic_hook);
    }

    #[test]
    fn it_should_compare_phoenix_correctly() {
        // Phoenix to standard
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } < Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                },
                value: CardValue(14),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } == Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                },
                value: CardValue(14),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } > Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                },
                value: CardValue(14),
            },
            true
        );

        // standard to Phoenix
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Star,
                    value: CardValue(2),
                },
                value: CardValue(2),
            } < Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            true
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(7),
                },
                value: CardValue(7)
            } == Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(14),
                },
                value: CardValue(14),
            } > Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
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
            Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } == Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
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
            Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } < Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
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
            Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } > Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            false
        );
        std::panic::set_hook(original_panic_hook);
    }

    #[test]
    fn it_should_compare_dragon_to_phoenix_correctly() {
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } < Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            true
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } == Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } > Single {
                card: Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            false
        );
    }
    #[test]
    fn it_should_compare_mahjong_correctly() {
        // MahJong to standard
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } < Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                },
                value: CardValue(14),
            },
            true
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } == Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                },
                value: CardValue(14),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } > Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                },
                value: CardValue(14),
            },
            false
        );

        // standard to MahJong
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Star,
                    value: CardValue(2),
                },
                value: CardValue(2),
            } < Single {
                card: Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                },
                value: CardValue(2),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(7),
                },
                value: CardValue(7),
            } == Single {
                card: Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(13),
                },
                value: CardValue(13),
            } > Single {
                card: Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            true
        );
    }

    #[test]
    fn it_should_compare_the_dog_correctly() {
        // Dog to standard
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } < Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                },
                value: CardValue(14),
            },
            true
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } == Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                },
                value: CardValue(14),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            } > Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                },
                value: CardValue(14),
            },
            false
        );

        // standard to Dog
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Star,
                    value: CardValue(2),
                },
                value: CardValue(2),
            } < Single {
                card: Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(7),
                },
                value: CardValue(7),
            } == Single {
                card: Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            false
        );
        assert_eq!(
            Single {
                card: Card {
                    suit: CardSuit::Sword,
                    value: CardValue(13),
                },
                value: CardValue(13),
            } > Single {
                card: Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                },
                value: CardValue::noop(),
            },
            true
        );
    }
}

const NOOP_PAIR_ERROR: &str = "A pair cannot consist of special card values";

/// a pair of cards of equal value
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pair {
    pub value: CardValue,
    pub cards: Vec<Card>,
}
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value.is_noop() || other.value.is_noop() {
            panic!("{NOOP_PAIR_ERROR}");
        }
        Some([&self.value].cmp(&[&other.value]))
    }
}
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value.is_noop() || other.value.is_noop() {
            panic!("{NOOP_PAIR_ERROR}");
        }
        [&self.value].cmp(&[&other.value])
    }
}
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        if self.value.is_noop() || other.value.is_noop() {
            panic!("{NOOP_PAIR_ERROR}");
        }
        self.value == other.value
    }
}
impl Eq for Pair {}

#[cfg(test)]
mod test_double {
    use crate::{Card, CardSuit, CardValue, Pair};

    #[test]
    fn it_should_compare_pairs_correctly() {
        let card_value_of_2 = CardValue(2);
        let pair_of_2_example_1 = Pair {
            value: card_value_of_2.clone(),
            cards: vec![ /* omitted */],
        };
        let pair_of_2_example_2 = Pair {
            value: card_value_of_2.clone(),
            cards: vec![ /* omitted */],
        };
        assert_eq!(pair_of_2_example_1 == pair_of_2_example_2, true);
        assert_eq!(pair_of_2_example_1 < pair_of_2_example_2, false);
        assert_eq!(pair_of_2_example_1 > pair_of_2_example_2, false);

        let card_value_of_13 = CardValue(13);
        let pair_of_2_example_3 = Pair {
            value: card_value_of_13.clone(),
            cards: vec![ /* omitted */],
        };
        let pair_of_2_example_4 = Pair {
            value: card_value_of_13.clone(),
            cards: vec![ /* omitted */],
        };
        assert_eq!(pair_of_2_example_3 == pair_of_2_example_4, true);
        assert_eq!(pair_of_2_example_3 < pair_of_2_example_4, false);
        assert_eq!(pair_of_2_example_3 > pair_of_2_example_4, false);
    }

    #[test]
    #[should_panic(expected = "special card")]
    fn it_should_throw_if_comparing_special_pairs_as_equal() {
        let original_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let card_value_of_2 = CardValue(0);
        let pair_of_2_example_1 = Pair {
            value: card_value_of_2.clone(),
            cards: vec![ /* omitted */],
        };
        let pair_of_2_example_2 = Pair {
            value: card_value_of_2.clone(),
            cards: vec![ /* omitted */],
        };
        assert_eq!(pair_of_2_example_1 == pair_of_2_example_2, true);
        std::panic::set_hook(original_panic_hook);
    }

    #[test]
    #[should_panic(expected = "special card")]
    fn it_should_throw_if_comparing_special_pairs_as_less_than() {
        let original_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let card_value_of_2 = CardValue(0);
        let pair_of_2_example_1 = Pair {
            value: card_value_of_2.clone(),
            cards: vec![ /* omitted */],
        };
        let pair_of_2_example_2 = Pair {
            value: card_value_of_2.clone(),
            cards: vec![ /* omitted */],
        };
        assert_eq!(pair_of_2_example_1 < pair_of_2_example_2, true);
        std::panic::set_hook(original_panic_hook);
    }

    #[test]
    #[should_panic(expected = "special card")]
    fn it_should_throw_if_comparing_special_pairs_as_greater_than() {
        let original_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let card_value_of_2 = CardValue(0);
        let pair_of_2_example_1 = Pair {
            value: card_value_of_2.clone(),
            cards: vec![ /* omitted */],
        };
        let pair_of_2_example_2 = Pair {
            value: card_value_of_2.clone(),
            cards: vec![
                Card {
                    suit: CardSuit::Dog,
                    value: card_value_of_2.clone(),
                },
                Card {
                    suit: CardSuit::Dog,
                    value: card_value_of_2.clone(),
                },
            ],
        };
        assert_eq!(pair_of_2_example_1 > pair_of_2_example_2, true);
        std::panic::set_hook(original_panic_hook);
    }
}

const UNEVEN_SEQUENCE_OF_PAIRS_ERROR: &str = "Can't compare unequal sequences of pairs";
const SPECIAL_CARD_SEQUENCE_OF_PAIRS_ERROR: &str =
    "Can't compare sequences of pairs that contain special cards";

/// a sequence of pairs of adjacent value
/// u8 = number of pairs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SequenceOfPairs {
    pub starting_value: CardValue,
    pub number_of_pairs: u8,
    pub cards: Vec<Card>,
}
impl PartialOrd for SequenceOfPairs {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.number_of_pairs != other.number_of_pairs || self.cards.len() != other.cards.len() {
            panic!("{UNEVEN_SEQUENCE_OF_PAIRS_ERROR}");
        }
        if self.starting_value.is_noop() || other.starting_value.is_noop() {
            panic!("{SPECIAL_CARD_SEQUENCE_OF_PAIRS_ERROR}");
        }
        Some([&self.starting_value].cmp(&[&other.starting_value]))
    }
}
impl Ord for SequenceOfPairs {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.number_of_pairs != other.number_of_pairs || self.cards.len() != other.cards.len() {
            panic!("{UNEVEN_SEQUENCE_OF_PAIRS_ERROR}");
        }
        if self.starting_value.is_noop() || other.starting_value.is_noop() {
            panic!("{SPECIAL_CARD_SEQUENCE_OF_PAIRS_ERROR}");
        }
        [&self.starting_value].cmp(&[&other.starting_value])
    }
}
impl PartialEq for SequenceOfPairs {
    fn eq(&self, other: &Self) -> bool {
        if self.number_of_pairs != other.number_of_pairs || self.cards.len() != other.cards.len() {
            panic!("{UNEVEN_SEQUENCE_OF_PAIRS_ERROR}");
        }
        if self.starting_value.is_noop() || other.starting_value.is_noop() {
            panic!("{SPECIAL_CARD_SEQUENCE_OF_PAIRS_ERROR}");
        }
        self.starting_value == other.starting_value
    }
}
impl Eq for SequenceOfPairs {}

#[cfg(test)]
mod test_sequence_of_pairs {
    use crate::{CardValue, SequenceOfPairs};

    #[test]
    fn it_should_compare_sequence_of_pairs_correctly() {
        let number_of_pairs = 2;
        let sequence_of_pairs_example_1 = SequenceOfPairs {
            starting_value: CardValue(2),
            number_of_pairs,
            cards: vec![ /* omitted */],
        };
        let sequence_of_pairs_example_2 = SequenceOfPairs {
            starting_value: CardValue(2),
            number_of_pairs,
            cards: vec![ /* omitted */],
        };

        let sequence_of_pairs_example_3 = SequenceOfPairs {
            starting_value: CardValue(11),
            number_of_pairs,
            cards: vec![ /* omitted */],
        };

        assert_eq!(
            sequence_of_pairs_example_1 == sequence_of_pairs_example_2,
            true
        );
        assert_eq!(
            sequence_of_pairs_example_1 < sequence_of_pairs_example_2,
            false
        );
        assert_eq!(
            sequence_of_pairs_example_1 > sequence_of_pairs_example_2,
            false
        );

        assert_eq!(
            sequence_of_pairs_example_1 == sequence_of_pairs_example_3,
            false
        );
        assert_eq!(
            sequence_of_pairs_example_1 < sequence_of_pairs_example_3,
            true
        );
        assert_eq!(
            sequence_of_pairs_example_1 > sequence_of_pairs_example_3,
            false
        );
    }
}

const SPECIAL_CARD_TRIO_ERROR: &str = "Can't compare a Trio that contains special cards";

/// a trio of cards of equal value
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trio {
    pub value: CardValue,
    pub cards: Vec<Card>,
}
impl PartialOrd for Trio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value.is_noop() || other.value.is_noop() {
            panic!("{SPECIAL_CARD_TRIO_ERROR}");
        }
        Some([&self.value].cmp(&[&other.value]))
    }
}
impl Ord for Trio {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value.is_noop() || other.value.is_noop() {
            panic!("{SPECIAL_CARD_TRIO_ERROR}");
        }
        [&self.value].cmp(&[&other.value])
    }
}
impl PartialEq for Trio {
    fn eq(&self, other: &Self) -> bool {
        if self.value.is_noop() || other.value.is_noop() {
            panic!("{SPECIAL_CARD_TRIO_ERROR}");
        }
        self.value == other.value
    }
}
impl Eq for Trio {}

#[cfg(test)]
mod test_trio {
    use crate::{CardValue, Trio};

    #[test]
    fn it_should_compare_trios_correctly() {
        let trio_example_1 = Trio {
            value: CardValue(3),
            cards: vec![ /* omitted */],
        };

        let trio_example_2 = Trio {
            value: CardValue(3),
            cards: vec![ /* omitted */],
        };

        let trio_example_3 = Trio {
            value: CardValue(7),
            cards: vec![ /* omitted */],
        };

        // example 1 : example 2
        assert_eq!(trio_example_1 == trio_example_2, true);
        assert_eq!(trio_example_1 < trio_example_2, false);
        assert_eq!(trio_example_1 > trio_example_2, false);

        // example 1 : example 3
        assert_eq!(trio_example_1 == trio_example_3, false);
        assert_eq!(trio_example_1 < trio_example_3, true);
        assert_eq!(trio_example_1 > trio_example_3, false);
    }
}

const SPECIAL_CARD_BOMB_OF_4_ERROR: &str = "Can't compare a Bomb of 4 that contains special cards";

/// a bomb (4 of the same value)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BombOf4 {
    pub value: CardValue,
    pub cards: Vec<Card>,
}
impl PartialOrd for BombOf4 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value.is_noop() || other.value.is_noop() {
            panic!("{SPECIAL_CARD_BOMB_OF_4_ERROR}");
        }

        Some([&self.value].cmp(&[&other.value]))
    }
}
impl Ord for BombOf4 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value.is_noop() || other.value.is_noop() {
            panic!("{SPECIAL_CARD_BOMB_OF_4_ERROR}");
        }
        [&self.value].cmp(&[&other.value])
    }
}
impl PartialEq for BombOf4 {
    fn eq(&self, other: &Self) -> bool {
        if self.value.is_noop() || other.value.is_noop() {
            panic!("{SPECIAL_CARD_BOMB_OF_4_ERROR}");
        }
        self.value == other.value
    }
}
impl Eq for BombOf4 {}

#[cfg(test)]
mod test_bomb_of_4 {
    use crate::{BombOf4, CardValue};

    #[test]
    fn it_should_compare_sequence_bombs_correctly() {
        let bomb_of_4_example_1 = BombOf4 {
            value: CardValue(3),
            cards: vec![ /* omitted */],
        };

        let bomb_of_4_example_2 = BombOf4 {
            value: CardValue(3),
            cards: vec![ /* omitted */],
        };

        let bomb_of_4_example_3 = BombOf4 {
            value: CardValue(7),
            cards: vec![ /* omitted */],
        };

        // example 1 : example 2
        assert_eq!(bomb_of_4_example_1 == bomb_of_4_example_2, true);
        assert_eq!(bomb_of_4_example_1 < bomb_of_4_example_2, false);
        assert_eq!(bomb_of_4_example_1 > bomb_of_4_example_2, false);

        // example 1 : example 3
        assert_eq!(bomb_of_4_example_1 == bomb_of_4_example_3, false);
        assert_eq!(bomb_of_4_example_1 < bomb_of_4_example_3, true);
        assert_eq!(bomb_of_4_example_1 > bomb_of_4_example_3, false);
    }
}

/// a bomb (sequence of 5+ of all the same suit)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SequenceBomb {
    pub suit: CardSuit,
    pub starting_value: CardValue,
    pub number_of_cards: u8,
    pub cards: Vec<Card>,
}

impl PartialOrd for SequenceBomb {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some([&self.starting_value].cmp(&[&other.starting_value]))
    }
}
impl Ord for SequenceBomb {
    fn cmp(&self, other: &Self) -> Ordering {
        [&self.starting_value].cmp(&[&other.starting_value])
    }
}
impl PartialEq for SequenceBomb {
    fn eq(&self, other: &Self) -> bool {
        self.starting_value == other.starting_value
    }
}
impl Eq for SequenceBomb {}

#[cfg(test)]
mod test_sequence_bomb {
    use crate::{CardSuit, CardValue, SequenceBomb};

    #[test]
    fn it_should_compare_sequence_bombs_correctly() {
        let sequence_bomb_example_1 = SequenceBomb {
            starting_value: CardValue(3),
            suit: CardSuit::Sword,
            number_of_cards: 5,
            cards: vec![ /* omitted */],
        };

        let sequence_bomb_example_2 = SequenceBomb {
            starting_value: CardValue(3),
            suit: CardSuit::Pagoda,
            number_of_cards: 5,
            cards: vec![ /* omitted */],
        };

        let sequence_bomb_example_3 = SequenceBomb {
            starting_value: CardValue(4),
            suit: CardSuit::Star,
            number_of_cards: 4,
            cards: vec![ /* omitted */],
        };

        // example 1 : example 2
        assert_eq!(sequence_bomb_example_1 == sequence_bomb_example_2, true);
        assert_eq!(sequence_bomb_example_1 < sequence_bomb_example_2, false);
        assert_eq!(sequence_bomb_example_1 > sequence_bomb_example_2, false);

        // example 1 : example 3
        assert_eq!(sequence_bomb_example_1 == sequence_bomb_example_3, false);
        assert_eq!(sequence_bomb_example_1 < sequence_bomb_example_3, true);
        assert_eq!(sequence_bomb_example_1 > sequence_bomb_example_3, false);
    }
}

/// A full house (trio + pair)
///
/// The value of the Trio is what counts
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullHouse {
    pub trio_value: CardValue,
    pub cards: Vec<Card>,
}
impl PartialOrd for FullHouse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some([&self.trio_value].cmp(&[&other.trio_value]))
    }
}
impl Ord for FullHouse {
    fn cmp(&self, other: &Self) -> Ordering {
        [&self.trio_value].cmp(&[&other.trio_value])
    }
}
impl PartialEq for FullHouse {
    fn eq(&self, other: &Self) -> bool {
        self.trio_value == other.trio_value
    }
}
impl Eq for FullHouse {}

#[cfg(test)]
mod test_full_house {
    use crate::{CardValue, FullHouse};

    #[test]
    fn it_should_compare_full_houses_correctly() {
        let full_house_example_1 = FullHouse {
            trio_value: CardValue(3),
            cards: vec![ /* omitted */],
        };

        let full_house_example_2 = FullHouse {
            trio_value: CardValue(3),
            cards: vec![ /* omitted */],
        };

        let full_house_example_3 = FullHouse {
            trio_value: CardValue(4),
            cards: vec![ /* omitted */],
        };

        // example 1 : example 2
        assert_eq!(full_house_example_1 == full_house_example_2, true);
        assert_eq!(full_house_example_1 < full_house_example_2, false);
        assert_eq!(full_house_example_1 > full_house_example_2, false);

        // example 1 : example 3
        assert_eq!(full_house_example_1 == full_house_example_3, false);
        assert_eq!(full_house_example_1 < full_house_example_3, true);
        assert_eq!(full_house_example_1 > full_house_example_3, false);
    }
}

/// a sequence of length at least 5
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sequence {
    pub starting_value: CardValue,
    pub number_of_cards: u8,
    pub cards: Vec<Card>,
}
impl PartialOrd for Sequence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some([&self.starting_value].cmp(&[&other.starting_value]))
    }
}
impl Ord for Sequence {
    fn cmp(&self, other: &Self) -> Ordering {
        [&self.starting_value].cmp(&[&other.starting_value])
    }
}
impl PartialEq for Sequence {
    fn eq(&self, other: &Self) -> bool {
        self.starting_value == other.starting_value
    }
}
impl Eq for Sequence {}

#[cfg(test)]
mod test_sequence {
    use crate::{CardValue, Sequence};

    #[test]
    fn it_should_compare_sequences_correctly() {
        let sequence_example_1 = Sequence {
            starting_value: CardValue(3),
            number_of_cards: 6,
            cards: vec![ /* omitted */],
        };

        let sequence_example_2 = Sequence {
            starting_value: CardValue(3),
            number_of_cards: 6,
            cards: vec![ /* omitted */],
        };

        let sequence_example_3 = Sequence {
            starting_value: CardValue(4),
            number_of_cards: 6,
            cards: vec![ /* omitted */],
        };

        // example 1 : example 2
        assert_eq!(sequence_example_1 == sequence_example_2, true);
        assert_eq!(sequence_example_1 < sequence_example_2, false);
        assert_eq!(sequence_example_1 > sequence_example_2, false);

        // example 1 : example 3
        assert_eq!(sequence_example_1 == sequence_example_3, false);
        assert_eq!(sequence_example_1 < sequence_example_3, true);
        assert_eq!(sequence_example_1 > sequence_example_3, false);
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub enum ValidCardCombo {
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
    SequenceBomb(SequenceBomb),

    /// a full house (trio + pair)
    FullHouse(FullHouse),

    /// a sequence of length at least 5
    Sequence(Sequence),
}

impl ValidCardCombo {
    pub fn is_bomb_of_4(&self) -> bool {
        matches!(self, ValidCardCombo::BombOf4(_))
    }

    pub fn is_sequence_bomb(&self) -> bool {
        matches!(self, ValidCardCombo::SequenceBomb(_))
    }

    pub fn is_bomb(&self) -> bool {
        self.is_bomb_of_4() || self.is_sequence_bomb()
    }
}
