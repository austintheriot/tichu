use crate::{
    BombOf4, Card, CardSuit, FullHouse, Pair, Sequence, SequenceBomb, SequenceOfPairs, Single,
    Trio, ValidCardCombos,
};

// TODO: account for single special cards and Phoenix wild card
pub fn get_card_combination(cards: &Vec<Card>) -> Option<ValidCardCombos> {
    let mut cards = cards.clone();
    let includes_phoenix = cards.iter().any(|card| card.suit == CardSuit::Phoenix);
    sort_cards_for_hand(&mut cards);

    // length 0: no cards
    if cards.is_empty() {
        return None;
    }

    // length 1: a single card
    if cards.len() == 1 {
        return Some(ValidCardCombos::Single(Single(
            cards.get(0).unwrap().clone(),
        )));
    }

    // length 2: a pair of cards of equal rank
    // OR 1 standard card and 1 Phoenix
    if cards.len() == 2 {
        // standard pair
        if let [card_0, card_1] = &cards[..cards.len()] {
            return if card_0.value == card_1.value {
                Some(ValidCardCombos::Pair(Pair {
                    cards: cards.clone(),
                    value: card_0.value.clone(),
                }))
            }
            // pair with 1 standard card and 1 Phoenix
            else if includes_phoenix {
                let std_card = cards.iter().find(|card| card.suit != CardSuit::Phoenix);
                Some(ValidCardCombos::Pair(Pair {
                    cards: cards.clone(),
                    value: std_card.unwrap().value.clone(),
                }))
            } else {
                None
            };
        }

        return None;
    }

    // length 3: a trio of cards of equal rank
    if cards.len() == 3 {
        if let [card_0, card_1, card_2] = &cards[..cards.len()] {
            return if card_0.value == card_1.value && card_1.value == card_2.value {
                Some(ValidCardCombos::Trio(Trio {
                    cards: cards.clone(),
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
                            cards: cards.clone(),
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

    // length 4:
    if cards.len() == 4 {
        // a bomb (4 of the same)
        if let [card_0, card_1, card_2, card_3] = &cards[..cards.len()] {
            if card_0.value == card_1.value
                && card_1.value == card_2.value
                && card_2.value == card_3.value
            {
                return Some(ValidCardCombos::BombOf4(BombOf4 {
                    cards: cards.clone(),
                    value: card_0.value.clone(),
                }));
            }

            // sequence of pairs is analyzed later
        }
    }

    // length 5:
    // a full house (trio + pair)
    // a sequence of length at least 5
    // a bomb (sequence of 5, all same suit)
    if cards.len() == 5 {
        // full house (first 3 are equal)
        if let [card_0, card_1, card_2, card_3, card_4] = &cards[..cards.len()] {
            if (card_0.value == card_1.value && card_0.value == card_2.value)
                && (card_3.value == card_4.value)
            {
                return Some(ValidCardCombos::FullHouse(FullHouse {
                    cards: cards.clone(),
                    trio_value: card_0.value.clone(),
                }));
            }
            // full house (last 3 are equal)
            else if (card_2.value == card_3.value && card_2.value == card_4.value)
                && (card_0.value == card_1.value)
            {
                return Some(ValidCardCombos::FullHouse(FullHouse {
                    cards: cards.clone(),
                    trio_value: card_2.value.clone(),
                }));
            }

            // plain sequences and sequence bombs are analyzed later
        }
    }

    // any length greater than or equal to 5:
    // sequence
    let mut is_sequence = true;
    let mut all_same_suit = true;
    for i in 1..cards.len() {
        let card = &cards[i];
        let prev_card = &cards[i - 1];
        if prev_card.suit != card.suit {
            all_same_suit = false;
        }
        if prev_card.value.add_one() != card.value {
            is_sequence = false;
            break;
        }
    }
    if is_sequence && cards.len() >= 5 {
        return if all_same_suit {
            // bomb sequence of length at least 5
            Some(ValidCardCombos::SequenceBomb(SequenceBomb {
                cards: cards.clone(),
                number_of_cards: 5,
                starting_value: cards[0].value.clone(),
                suit: cards[0].suit.clone(),
            }))
        } else {
            // plain sequence
            Some(ValidCardCombos::Sequence(Sequence {
                cards: cards.clone(),
                number_of_cards: cards.len() as u8,
                starting_value: cards[0].value.clone(),
            }))
        };
    }

    // any sequence of pairs of adjacent value
    if cards.len() % 2 == 0 && cards.len() > 3 {
        let mut is_sequence_of_pairs = true;

        // if a phoenix is present, swap out the phoenix for a normal card
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

            // only clone cards if necessary
            if let Some(i_of_lone_card) = i_of_lone_card {
                let cloned_card = (*cards_without_phoenix.get(i_of_lone_card).unwrap()).clone();
                let mut cards_without_phoenix: Vec<Card> = cards_without_phoenix
                    .iter()
                    .map(|card| (*card).clone())
                    .collect();
                cards_without_phoenix.insert(i_of_lone_card, cloned_card);
                cards_without_phoenix
            } else {
                cards
            }
        } else {
            cards
        };

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
            if is_even && prev_card.value.add_one() != card.value {
                is_sequence_of_pairs = false;
                break;
            }
        }
        if is_sequence_of_pairs {
            return Some(ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                cards: cards.clone(),
                starting_value: cards[0].value.clone(),
                number_of_pairs: cards.len() as u8 / 2,
            }));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::{get_card_combination, Card, CardSuit, CardValue};

    mod test_get_random_string_of_len {
        use super::super::get_card_combination;
        use crate::{
            BombOf4, Card, CardSuit, CardValue, FullHouse, Pair, Sequence, SequenceBomb,
            SequenceOfPairs, Single, Trio, ValidCardCombos,
        };

        #[test]
        fn it_should_return_some_for_correct_combos() {
            // a single card
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![Card {
                        suit: CardSuit::Sword,
                        value: CardValue(2),
                    }])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Single(Single(Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2),
                })))
            );

            // a pair of cards of equal rank (plain)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(7),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Pair(Pair {
                    value: CardValue(7),
                    cards: vec![ /* omitted */],
                }))
            );

            // a pair of cards of equal rank (with Phoenix)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Pair(Pair {
                    value: CardValue(7),
                    cards: vec![ /* omitted */],
                }))
            );

            // a trio of cards of equal rank (standard)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(3),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Trio(Trio {
                    value: CardValue(3),
                    cards: vec![ /* omitted */],
                }))
            );

            // a trio of cards of equal rank (with Phoenix)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Trio(Trio {
                    value: CardValue(3),
                    cards: vec![ /* omitted */],
                }))
            );

            // a sequence of pairs, length 4 (plain)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(15),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(15),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(14),
                    number_of_pairs: 2,
                    cards: vec![ /* omitted */],
                }))
            );

            // a sequence of pairs, length 4 (with phoenix)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(2),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(3),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(2),
                    number_of_pairs: 2,
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(2),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(3),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(2),
                    number_of_pairs: 2,
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(15),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(14),
                    number_of_pairs: 2,
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(15),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(14),
                    number_of_pairs: 2,
                    cards: vec![ /* omitted */],
                }))
            );

            // a bomb (4 of the same)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::BombOf4(BombOf4 {
                    value: CardValue(7),
                    cards: vec![ /* omitted */],
                }))
            );

            // a full house (trio + pair)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(15),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(15),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::FullHouse(FullHouse {
                    cards: vec![ /* omitted */],
                    trio_value: CardValue(14),
                }))
            );

            // a plain sequence of length 5
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                    cards: vec![ /* omitted */],
                    number_of_cards: 5,
                    starting_value: CardValue(3),
                }))
            );

            // sequence bomb
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(7),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceBomb(SequenceBomb {
                    cards: vec![ /* omitted */],
                    number_of_cards: 5,
                    starting_value: CardValue(3),
                    suit: CardSuit::Pagoda,
                }))
            );

            // plain sequence, length 6
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(8),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                    number_of_cards: 6,
                    starting_value: CardValue(3),
                    cards: vec![ /* omitted */],
                }))
            );

            // bomb sequence, length 6
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(8),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceBomb(SequenceBomb {
                    cards: vec![ /* omitted */],
                    number_of_cards: 6,
                    starting_value: CardValue(3),
                    suit: CardSuit::Pagoda,
                }))
            );

            // sequence of pairs, length 6 (plain)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(12),
                    number_of_pairs: 3,
                    cards: vec![ /* omitted */],
                }))
            );

            // sequence of pairs, length 6 (with phoenix)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(12),
                    number_of_pairs: 3,
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(12),
                    number_of_pairs: 3,
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(12),
                    number_of_pairs: 3,
                    cards: vec![ /* omitted */],
                }))
            );

            // plain sequence, length 7
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(9),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                    number_of_cards: 7,
                    starting_value: CardValue(3),
                    cards: vec![ /* omitted */],
                }))
            );

            // bomb sequence, length 7
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(9),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceBomb(SequenceBomb {
                    cards: vec![ /* omitted */],
                    number_of_cards: 7,
                    starting_value: CardValue(3),
                    suit: CardSuit::Pagoda,
                }))
            );

            // plain sequence, length 8
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                    number_of_cards: 8,
                    starting_value: CardValue(3),
                    cards: vec![ /* omitted */],
                }))
            );

            // bomb sequence, length 8
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(10),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceBomb(SequenceBomb {
                    cards: vec![ /* omitted */],
                    number_of_cards: 8,
                    starting_value: CardValue(3),
                    suit: CardSuit::Pagoda,
                }))
            );

            // sequence of pairs, length 8 (plain)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(11),
                    number_of_pairs: 4,
                    cards: vec![ /* omitted */],
                }))
            );

            // sequence of pairs, length 8 (with phoenix)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(11),
                    number_of_pairs: 4,
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(11),
                    number_of_pairs: 4,
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(11),
                    number_of_pairs: 4,
                    cards: vec![ /* omitted */],
                }))
            );

            // plain sequence, length 9
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                    number_of_cards: 9,
                    starting_value: CardValue(3),
                    cards: vec![ /* omitted */],
                }))
            );

            // bomb sequence, length 9
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceBomb(SequenceBomb {
                    cards: vec![ /* omitted */],
                    number_of_cards: 9,
                    starting_value: CardValue(3),
                    suit: CardSuit::Pagoda,
                }))
            );

            // plain sequence, length 10
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(12),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                    number_of_cards: 10,
                    starting_value: CardValue(3),
                    cards: vec![ /* omitted */],
                }))
            );

            // bomb sequence, length 10
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(12),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceBomb(SequenceBomb {
                    cards: vec![ /* omitted */],
                    number_of_cards: 10,
                    starting_value: CardValue(3),
                    suit: CardSuit::Pagoda,
                }))
            );

            // sequence of pairs, length 10 (plain)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(10),
                    number_of_pairs: 5,
                    cards: vec![ /* omitted */],
                }))
            );

            // sequence of pairs, length 10 (with phoenix)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(10),
                    number_of_pairs: 5,
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(10),
                    number_of_pairs: 5,
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(10),
                    number_of_pairs: 5,
                    cards: vec![ /* omitted */],
                }))
            );

            // plain sequence, length 11
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(13),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                    number_of_cards: 11,
                    starting_value: CardValue(3),
                    cards: vec![ /* omitted */],
                }))
            );

            // bomb sequence, length 11
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(13),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceBomb(SequenceBomb {
                    cards: vec![ /* omitted */],
                    number_of_cards: 11,
                    starting_value: CardValue(3),
                    suit: CardSuit::Pagoda,
                }))
            );

            // plain sequence, length 12
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                    number_of_cards: 12,
                    starting_value: CardValue(3),
                    cards: vec![ /* omitted */],
                }))
            );

            // bomb sequence, length 12
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceBomb(SequenceBomb {
                    cards: vec![ /* omitted */],
                    number_of_cards: 12,
                    starting_value: CardValue(3),
                    suit: CardSuit::Pagoda,
                }))
            );

            // sequence of pairs, length 12 (plain), (longest possible)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(9),
                    number_of_pairs: 12,
                    cards: vec![ /* omitted */],
                }))
            );

            // sequence of pairs, length 10 (with phoenix), (longest possible)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(9),
                    number_of_pairs: 6,
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(9),
                    number_of_pairs: 6,
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(9),
                    number_of_pairs: 6,
                    cards: vec![ /* omitted */],
                }))
            );

            // plain sequence, length 13 (longest possible)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(2),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                    number_of_cards: 13,
                    starting_value: CardValue(2),
                    cards: vec![ /* omitted */],
                }))
            );

            // bomb sequence, length 13 (longest possible)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(2),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceBomb(SequenceBomb {
                    cards: vec![ /* omitted */],
                    number_of_cards: 13,
                    starting_value: CardValue(3),
                    suit: CardSuit::Pagoda,
                }))
            );
        }
    }

    #[test]
    fn it_should_return_none_for_bogus_combos() {
        assert_eq!(get_card_combination(&vec![]), None);

        // two different cards
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(2),
                },
                Card {
                    suit: CardSuit::Star,
                    value: CardValue(3),
                }
            ]),
            None
        );

        // non-Phoenix special card and standard card:
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                },
                Card {
                    suit: CardSuit::Star,
                    value: CardValue(11),
                }
            ]),
            None
        );
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                },
                Card {
                    suit: CardSuit::Jade,
                    value: CardValue(6),
                }
            ]),
            None
        );
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14),
                }
            ]),
            None
        );

        // 3: 2 same value and 3rd non-matching
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(2),
                },
                Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2),
                },
                Card {
                    suit: CardSuit::Jade,
                    value: CardValue(5),
                }
            ]),
            None
        );
        // 3: 3 different values
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(2),
                },
                Card {
                    suit: CardSuit::Sword,
                    value: CardValue(3),
                },
                Card {
                    suit: CardSuit::Jade,
                    value: CardValue(4),
                }
            ]),
            None
        );
    }
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

#[cfg(test)]
mod test_sort_cards_for_hand {
    use crate::{sort_cards_for_hand, Card, CardSuit, CardValue, Deck};

    #[test]
    fn it_should_sort_for_hand_correctly() {
        let mut deck = Deck::new();
        deck.shuffle();
        sort_cards_for_hand(&mut deck.0);

        assert_eq!(
            deck.0.get(0),
            Some(&Card {
                suit: CardSuit::MahJong,
                value: CardValue::noop(),
            })
        );

        assert_eq!(
            deck.0.get(1),
            Some(&Card {
                suit: CardSuit::Dog,
                value: CardValue::noop(),
            })
        );

        assert_eq!(
            deck.0.get(2),
            Some(&Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            })
        );

        assert_eq!(
            deck.0.get(3),
            Some(&Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            })
        );

        assert_eq!(
            deck.0.get(4),
            Some(&Card {
                suit: CardSuit::Sword,
                value: CardValue(2),
            })
        );

        assert_eq!(
            deck.0.get(5),
            Some(&Card {
                suit: CardSuit::Jade,
                value: CardValue(2),
            })
        );

        assert_eq!(
            deck.0.get(6),
            Some(&Card {
                suit: CardSuit::Pagoda,
                value: CardValue(2),
            })
        );

        assert_eq!(
            deck.0.get(7),
            Some(&Card {
                suit: CardSuit::Star,
                value: CardValue(2),
            })
        );

        assert_eq!(
            deck.0.get(8),
            Some(&Card {
                suit: CardSuit::Sword,
                value: CardValue(3),
            })
        );

        assert_eq!(
            deck.0.get(9),
            Some(&Card {
                suit: CardSuit::Jade,
                value: CardValue(3),
            })
        );

        assert_eq!(
            deck.0.get(10),
            Some(&Card {
                suit: CardSuit::Pagoda,
                value: CardValue(3),
            })
        );

        assert_eq!(
            deck.0.get(11),
            Some(&Card {
                suit: CardSuit::Star,
                value: CardValue(3),
            })
        );

        assert_eq!(
            deck.0.get(55),
            Some(&Card {
                suit: CardSuit::Star,
                value: CardValue(14),
            })
        );
    }
}
