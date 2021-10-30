use crate::{BombOf4, Card, CardSuit, CardValue, FullHouse, MAX_CARDS_IN_HAND, Pair, Sequence, SequenceBomb, SequenceOfPairs, Single, Trio, ValidCardCombos};

// TODO: account for single special cards and Phoenix wild card
pub fn get_card_combination(cards: &Vec<Card>) -> Option<ValidCardCombos> {
    let original_cards = cards;
    let mut cards = cards.clone();
    let mut includes_phoenix = false;
    let mut includes_non_phoenix_special_card = false;
    let mut value_out_of_range = false;
    let mut identical_cards_found = false;

    sort_cards_for_hand(&mut cards);

    // rule out obvious invalid combinations and cards
    for (i, card) in cards.iter().enumerate() {
        // identical cards
        if i > 0 {
            let prev_card = &cards[i - 1];
            if prev_card.suit == card.suit && prev_card.value == card.value {
                identical_cards_found = true;
                break;
            }
        }

        // invalid combination of suit and value
        if card.value > CardValue::max() 
        || (card.suit.is_special() && card.value != CardValue::noop())
        || (!card.suit.is_special() && card.value < CardValue::min()) {
            value_out_of_range = true;
            break;
        }

        // find phoenix
        if card.suit == CardSuit::Phoenix {
            includes_phoenix = true;
        }

        // find non-phoenix special cards
        if card.suit == CardSuit::MahJong
            || card.suit == CardSuit::Dog
            || card.suit == CardSuit::Dragon
        {
            includes_non_phoenix_special_card = true;
        }
    }

    // cannot play an empty combination
    if cards.is_empty() 
    // cannot play more than is possible to hold in your hand
    || cards.len() > MAX_CARDS_IN_HAND 
    // Non-Phoenix special cards cannot be played in combination
    || includes_non_phoenix_special_card && cards.len() > 1  
    // invalid combination of suit and value
    || value_out_of_range
    // cannot have 2 of the EXACT same card
    || identical_cards_found
    {
        return None;
    }

    // length 1: a single card
    if cards.len() == 1 {
        return Some(ValidCardCombos::Single(Single(
            cards.get(0).unwrap().clone(),
        )));
    }

    if cards.len() == 2 {
        // standard pair
        if let [card_0, card_1] = &cards[..cards.len()] {
            return if card_0.value == card_1.value {
                Some(ValidCardCombos::Pair(Pair {
                    cards: original_cards.clone(),
                    value: card_0.value.clone(),
                }))
            }
            // pair with 1 standard card and 1 Phoenix
            else if includes_phoenix {
                let std_card = cards.iter().find(|card| card.suit != CardSuit::Phoenix);
                Some(ValidCardCombos::Pair(Pair {
                    cards: original_cards.clone(),
                    value: std_card.unwrap().value.clone(),
                }))
            } else {
                None
            };
        }

        return None;
    }

    if cards.len() == 3 {
        // trio of equal rank
        if let [card_0, card_1, card_2] = &cards[..cards.len()] {
            return if card_0.value == card_1.value && card_1.value == card_2.value {
                Some(ValidCardCombos::Trio(Trio {
                    cards: original_cards.clone(),
                    value: card_0.value.clone(),
                }))
            } else if includes_phoenix {
                let std_cards: Vec<&Card> = cards
                    .iter()
                    .filter(|card| card.suit != CardSuit::Phoenix)
                    .collect();
                if let [std_card_0, std_card_1] = &std_cards[0..std_cards.len()] {
                    if std_card_0.value == std_card_1.value {
                        Some(ValidCardCombos::Trio(Trio {
                            cards: original_cards.clone(),
                            value: std_card_0.value.clone(),
                        }))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
        }

        return None;
    }

    if cards.len() == 4 {
        // a bomb (4 of the same)
        if let [card_0, card_1, card_2, card_3] = &cards[..cards.len()] {
            if card_0.value == card_1.value
                && card_1.value == card_2.value
                && card_2.value == card_3.value
            {
                return Some(ValidCardCombos::BombOf4(BombOf4 {
                    cards: original_cards.clone(),
                    value: card_0.value.clone(),
                }));
            }

            // sequence of pairs is analyzed later
        }
    }

    if cards.len() == 5 {
        // manipulate locally
        let mut adjusted_cards = cards.clone();

        // replace Phoenix with a standard card
       if includes_phoenix {
            let mut cards_without_phoenix: Vec<Card> = cards
                .iter()
                .filter(|card| card.suit != CardSuit::Phoenix)
                .map(|card| (*card).clone())
                .collect();
            let highest_value_card = cards_without_phoenix[cards_without_phoenix.len() - 1].clone();
            cards_without_phoenix.push(highest_value_card.clone());
            adjusted_cards = cards_without_phoenix;
        }

        // full house (first 3 are equal)
        if let [card_0, card_1, card_2, card_3, card_4] = &adjusted_cards[..adjusted_cards.len()] {
            if (card_0.value == card_1.value && card_0.value == card_2.value)
                && (card_3.value == card_4.value)
                && (card_0.value != card_3.value)
            {
                return Some(ValidCardCombos::FullHouse(FullHouse {
                    cards: original_cards.clone(),
                    trio_value: card_0.value.clone(),
                }));
            }
            // full house (last 3 are equal)
            else if (card_2.value == card_3.value && card_2.value == card_4.value)
                && (card_0.value == card_1.value)
                && (card_0.value != card_2.value)
            {
                return Some(ValidCardCombos::FullHouse(FullHouse {
                    cards: original_cards.clone(),
                    trio_value: card_2.value.clone(),
                }));
            }

            // plain sequences and sequence bombs are analyzed later
        }
    }

    // sequence 
    if cards.len() >= 5 {
        let mut is_sequence = true;
        let mut all_same_suit = true;
        // manipulate cards locally to this function (hacky for now)
        let mut cards = cards.clone();

        // if a phoenix is present, optimistically swap out the phoenix for a normal card
        if includes_phoenix {
            let cards_without_phoenix: Vec<&Card> = cards
                .iter()
                .filter(|card| card.suit != CardSuit::Phoenix)
                .collect();

            let mut i_of_last_continuos_sequence_card = None;
            for i in 1..cards_without_phoenix.len() {
                let prev_card = &cards_without_phoenix[i - 1];
                let card = &cards_without_phoenix[i];

                if prev_card.value.add(1) != card.value {
                    i_of_last_continuos_sequence_card = Some(i - 1);
                    break;
                }
            }

            // create vector of cloned, owned cards
            let mut cards_without_phoenix: Vec<Card> = cards_without_phoenix
                .iter()
                .map(|card| (*card).clone())
                .collect();

            // gap in cards found--insert Phoenix in gap
            if let Some(i_of_last_continuos_sequence_card) = i_of_last_continuos_sequence_card {
                let mut new_card = cards_without_phoenix[i_of_last_continuos_sequence_card].clone();
                new_card.value = new_card.value.add(1);
                cards_without_phoenix.insert(i_of_last_continuos_sequence_card + 1, new_card);
            } else {
                // NO gap in cards found--put Phoenix at the end of the Sequence
                let mut new_card = cards_without_phoenix[cards_without_phoenix.len() - 1].clone();
                new_card.value = new_card.value.add(1);
                cards_without_phoenix.push(new_card);
            }
            cards = cards_without_phoenix;
        }

        // cards should no longer be mutable
        let cards = cards;

        // test if cards are sequence
        for i in 1..cards.len() {
            let card = &cards[i];
            let prev_card = &cards[i - 1];
            if prev_card.suit != card.suit {
                all_same_suit = false;
            }
            if prev_card.value.add(1) != card.value {
                is_sequence = false;
                break;
            }
        }
        if is_sequence {
            return if !includes_phoenix && all_same_suit {
                // bomb sequence of length at least 5
                Some(ValidCardCombos::SequenceBomb(SequenceBomb {
                    cards: original_cards.clone(),
                    number_of_cards: 5,
                    starting_value: cards[0].value.clone(),
                    suit: cards[0].suit.clone(),
                }))
            } else {
                // non-bomb sequence
                Some(ValidCardCombos::Sequence(Sequence {
                    cards: original_cards.clone(),
                    number_of_cards: cards.len() as u8,
                    starting_value: cards[0].value.clone(),
                }))
            };
        }
    }

    // any sequence of pairs of adjacent value
    let even_number_of_cards = cards.len() % 2 == 0;
    if even_number_of_cards && cards.len() > 3 {
        let mut is_sequence_of_pairs = true;

        // if a phoenix is present, optimistically swap out the phoenix for a normal card
        let cards = if includes_phoenix {
            let cards_without_phoenix: Vec<&Card> = cards
                .iter()
                .filter(|card| card.suit != CardSuit::Phoenix)
                .collect();

            let mut i_of_lone_card = None;
            for i in 1..cards_without_phoenix.len() {
                let prev_card = &cards_without_phoenix[i - 1];
                let card = &cards_without_phoenix[i];
                let next_card = cards_without_phoenix.get(i + 1);
                let is_second_card = i == 1;
                let is_last_card = i == cards_without_phoenix.len() - 1;

                if is_second_card {
                    if prev_card.value != card.value {
                        i_of_lone_card = Some(i - 1);
                        break;
                    }
                } else if is_last_card {
                    if prev_card.value != card.value {
                        i_of_lone_card = Some(i);
                        break;
                    }
                } else if prev_card.value != card.value
                    && card.value != next_card.expect("Card should be Some").value
                {
                    i_of_lone_card = Some(i);
                    break;
                }
            }

            if let Some(i_of_lone_card) = i_of_lone_card {
                let cloned_card = (*cards_without_phoenix.get(i_of_lone_card).unwrap()).clone();
                let mut cards_without_phoenix: Vec<Card> = cards_without_phoenix
                    .iter()
                    .map(|card| (*card).clone())
                    .collect();
                cards_without_phoenix.insert(i_of_lone_card, cloned_card);
                cards_without_phoenix
            } else {
                // no lone card found (probably not a valid combo)
                cards
            }
        } else {
            // no phoenix present, cards stay as they are
            cards
        };

        // test if is sequence of pairs
        for i in 1..cards.len() {
            let card = &cards[i];
            let prev_card = &cards[i - 1];
            let is_even = i % 2 == 0;
            // pairs have same value
            if !is_even && card.value != prev_card.value {
                is_sequence_of_pairs = false;
                break;
            }
            // adjacent pairs are only 1 value away
            if is_even && prev_card.value.add(1) != card.value {
                is_sequence_of_pairs = false;
                break;
            }
        }
        if is_sequence_of_pairs {
            return Some(ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                cards: original_cards.clone(),
                starting_value: cards[0].value.clone(),
                number_of_pairs: cards.len() as u8 / 2,
            }));
        }
    }

    None
}

pub fn sort_cards_for_hand(cards: &mut Vec<Card>) {
    cards.sort_by(|a, b| {
        if a.value == b.value {
            [&a.suit].cmp(&[&b.suit])
        } else {
            [&a.value].cmp(&[&b.value])
        }
    });
}
