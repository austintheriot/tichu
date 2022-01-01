#[cfg(feature = "client")]
use js_sys::Math::random;
use rand::{prelude::SliceRandom, rngs::SmallRng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

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

    pub fn minus(&self, number: u8) -> Self {
        let new_number = if self.0 == 0 { 0 } else { self.0 - number };
        CardValue(new_number)
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

// only runs on client: uses Math.random to seed random number
#[cfg(feature = "client")]
fn get_random_u64() -> u64 {
    (random() * 1_000_000 as f64) as u64
}

// only runs on server: uses SystemTime to seed random number
#[cfg(not(feature = "client"))]
fn get_random_u64() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

impl Deck {
    /// Creates a new, full, sorted Deck (i.e. it is NOT shuffled)
    pub fn new() -> Deck {
        Deck::default()
    }

    pub fn shuffle(&mut self) -> &mut Self {
        let rand_num = get_random_u64();
        self.0.shuffle(&mut SmallRng::seed_from_u64(rand_num));
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

    /// "None" + All Standard Cards
    pub fn wished_for_cards() -> Vec<Option<Card>> {
        let mut cards = Deck::new()
            .0
            .into_iter()
            .filter(|card| !card.suit.is_special())
            .map(Some)
            .collect::<Vec<Option<Card>>>();
        // first option is "None"
        cards.insert(0, None);
        cards
    }

    /// Find index of a card in the wished for possibilities
    pub fn i_of_wished_for_card(card: &Option<Card>) -> Option<usize> {
        Deck::wished_for_cards()
            .iter()
            .position(|wished_for_card| wished_for_card == card)
    }

    pub fn get_wished_for_card_from_i(i: usize) -> Option<Card> {
        let wished_for_cards = Deck::wished_for_cards();
        let card = wished_for_cards.get(i);
        if let Some(card) = card {
            card.clone()
        } else {
            None
        }
    }
}

const COMPARE_IDENTICAL_SPECIAL_CARDS: &str = "Can't compare identical special cards to each other";

/// a single card
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Single {
    pub cards: Vec<Card>,
    /// Phoenixes receive a value 0.5 higher than the previous card
    pub value: CardValue,
    pub user_id: String,
}
impl PartialOrd for Single {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // can't compare identical special cards
        if self.value.is_noop()
            && other.value.is_noop()
            && self.cards[0].suit == other.cards[0].suit
        {
            panic!("{COMPARE_IDENTICAL_SPECIAL_CARDS}");
        }

        // dragon beats everything
        if let CardSuit::Dragon = &self.cards[0].suit {
            return Some(Ordering::Greater);
        }
        if let CardSuit::Dragon = &other.cards[0].suit {
            return Some(Ordering::Less);
        }

        // Phoenix beats any standard card
        if let CardSuit::Phoenix = &self.cards[0].suit {
            return Some(Ordering::Greater);
        }
        if let CardSuit::Phoenix = &other.cards[0].suit {
            return Some(Ordering::Less);
        }

        Some([&self.value].cmp(&[&other.value]))
    }
}
impl Ord for Single {
    fn cmp(&self, other: &Self) -> Ordering {
        // can't compare identical special cards
        if self.value.is_noop()
            && other.value.is_noop()
            && self.cards[0].suit == other.cards[0].suit
        {
            panic!("{COMPARE_IDENTICAL_SPECIAL_CARDS}");
        }

        // dragon beats everything
        if let CardSuit::Dragon = &self.cards[0].suit {
            return Ordering::Greater;
        }
        if let CardSuit::Dragon = &other.cards[0].suit {
            return Ordering::Less;
        }

        // Phoenix beats any standard card
        if let CardSuit::Phoenix = &self.cards[0].suit {
            return Ordering::Greater;
        }
        if let CardSuit::Phoenix = &other.cards[0].suit {
            return Ordering::Less;
        }

        [&self.value].cmp(&[&other.value])
    }
}
impl PartialEq for Single {
    fn eq(&self, other: &Self) -> bool {
        if self.value.is_noop() && other.value.is_noop() {
            if self.cards[0].suit == other.cards[0].suit {
                // can't compare two identical special cards to each other
                panic!("{COMPARE_IDENTICAL_SPECIAL_CARDS}");
            } else {
                // only compare suits if they are special cards
                return self.cards[0].suit == other.cards[0].suit;
            }
        }

        self.value == other.value
    }
}
impl Eq for Single {}

const NOOP_PAIR_ERROR: &str = "A pair cannot consist of special card values";

/// a pair of cards of equal value
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pair {
    pub value: CardValue,
    pub cards: Vec<Card>,
    pub user_id: String,
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
    pub user_id: String,
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

const SPECIAL_CARD_TRIO_ERROR: &str = "Can't compare a Trio that contains special cards";

/// a trio of cards of equal value
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trio {
    pub value: CardValue,
    pub cards: Vec<Card>,
    pub user_id: String,
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

const SPECIAL_CARD_BOMB_OF_4_ERROR: &str = "Can't compare a Bomb of 4 that contains special cards";

/// a bomb (4 of the same value)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BombOf4 {
    pub value: CardValue,
    pub cards: Vec<Card>,
    pub user_id: String,
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

/// a bomb (sequence of 5+ of all the same suit)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SequenceBomb {
    pub suit: CardSuit,
    pub starting_value: CardValue,
    pub number_of_cards: u8,
    pub cards: Vec<Card>,
    pub user_id: String,
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

/// A full house (trio + pair)
///
/// The value of the Trio is what counts
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullHouse {
    pub trio_value: CardValue,
    pub cards: Vec<Card>,
    pub user_id: String,
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

/// a sequence of length at least 5
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sequence {
    pub starting_value: CardValue,
    pub number_of_cards: u8,
    pub cards: Vec<Card>,
    pub user_id: String,
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

    pub fn cards(&self) -> &Vec<Card> {
        match &self {
            ValidCardCombo::Single(single) => &single.cards,
            ValidCardCombo::Pair(pair) => &pair.cards,
            ValidCardCombo::Trio(trio) => &trio.cards,
            ValidCardCombo::BombOf4(bomb_of_4) => &bomb_of_4.cards,
            ValidCardCombo::FullHouse(full_house) => &full_house.cards,
            ValidCardCombo::Sequence(sequence) => &sequence.cards,
            ValidCardCombo::SequenceOfPairs(sequence_of_pairs) => &sequence_of_pairs.cards,
            ValidCardCombo::SequenceBomb(sequence_bomb) => &sequence_bomb.cards,
        }
    }

    pub fn user_id(&self) -> &String {
        match &self {
            ValidCardCombo::Single(single) => &single.user_id,
            ValidCardCombo::Pair(pair) => &pair.user_id,
            ValidCardCombo::Trio(trio) => &trio.user_id,
            ValidCardCombo::BombOf4(bomb_of_4) => &bomb_of_4.user_id,
            ValidCardCombo::FullHouse(full_house) => &full_house.user_id,
            ValidCardCombo::Sequence(sequence) => &sequence.user_id,
            ValidCardCombo::SequenceOfPairs(sequence_of_pairs) => &sequence_of_pairs.user_id,
            ValidCardCombo::SequenceBomb(sequence_bomb) => &sequence_bomb.user_id,
        }
    }
}
