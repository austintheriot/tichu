use common::{Card, ValidCardCombo};

/// Helper to get prettier logging on failed tests
fn fmt_panic(cards: Vec<Card>, combo: Option<ValidCardCombo>) -> String {
    panic!(
        "\n\nUnexpected combo received:\nOriginal Cards: {:?}\nCombo Received: {:?}\n\n",
        cards, combo
    )
}

#[cfg(test)]
mod test_get_card_combination {
    use crate::fmt_panic;
    use common::{
        get_card_combination, BombOf4, Card, CardSuit, CardValue, FullHouse, Pair, Sequence,
        SequenceBomb, SequenceOfPairs, Single, Trio, ValidCardCombo,
    };

    #[test]
    fn it_should_return_some_for_correct_single_cards() {
        // a single standard card
        let cards = vec![Card {
            suit: CardSuit::Sword,
            value: CardValue(2),
        }];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Single(Single {
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
        } else {
            fmt_panic(cards, combo);
        }

        // a single special card
        let cards = vec![Card {
            suit: CardSuit::Dragon,
            value: CardValue::noop(),
        }];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Single(Single {
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
        } else {
            fmt_panic(cards, combo);
        }

        let cards = vec![Card {
            suit: CardSuit::MahJong,
            value: CardValue::noop(),
        }];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Single(Single {
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
        } else {
            fmt_panic(cards, combo);
        }

        let cards = vec![Card {
            suit: CardSuit::Phoenix,
            value: CardValue::noop(),
        }];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Single(Single {
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
        } else {
            fmt_panic(cards, combo);
        }
    }

    #[test]
    fn it_should_return_some_for_single_phoenix() {
        // after None
        let cards = vec![Card {
            suit: CardSuit::Phoenix,
            value: CardValue::noop(),
        }];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Single(Single {
            cards: returned_cards,
            value,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
            assert_eq!(value, CardValue(2).minus(1));
        } else {
            fmt_panic(cards, combo);
        }

        // after standard card
        let cards = vec![Card {
            suit: CardSuit::Phoenix,
            value: CardValue::noop(),
        }];
        let combo = get_card_combination(
            Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Jade,
                    value: CardValue(2),
                }],
                value: CardValue(2),
                user_id: "1".to_string(),
            })),
            &cards,
            &"1".to_string(),
        );
        if let Some(ValidCardCombo::Single(Single {
            cards: returned_cards,
            value,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
            assert_eq!(value, CardValue(2));
        } else {
            fmt_panic(cards, combo);
        }

        let cards = vec![Card {
            suit: CardSuit::Phoenix,
            value: CardValue::noop(),
        }];
        let combo = get_card_combination(
            Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(10),
                }],
                value: CardValue(10),
                user_id: "1".to_string(),
            })),
            &cards,
            &"1".to_string(),
        );
        if let Some(ValidCardCombo::Single(Single {
            cards: returned_cards,
            value,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
            assert_eq!(value, CardValue(10));
        } else {
            fmt_panic(cards, combo);
        }

        let cards = vec![Card {
            suit: CardSuit::Phoenix,
            value: CardValue::noop(),
        }];
        let combo = get_card_combination(
            Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(14),
                }],
                value: CardValue(14),
                user_id: "1".to_string(),
            })),
            &cards,
            &"1".to_string(),
        );
        if let Some(ValidCardCombo::Single(Single {
            cards: returned_cards,
            value,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
            assert_eq!(value, CardValue(14));
        } else {
            fmt_panic(cards, combo);
        }

        // after MahJong
        let cards = vec![Card {
            suit: CardSuit::Phoenix,
            value: CardValue::noop(),
        }];
        let combo = get_card_combination(
            Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &cards,
            &"1".to_string(),
        );
        if let Some(ValidCardCombo::Single(Single {
            cards: returned_cards,
            value,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
            assert_eq!(value, CardValue(2).minus(1));
        } else {
            fmt_panic(cards, combo);
        }

        // after Dog
        let cards = vec![Card {
            suit: CardSuit::Phoenix,
            value: CardValue::noop(),
        }];
        let combo = get_card_combination(
            Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &cards,
            &"1".to_string(),
        );
        if let Some(ValidCardCombo::Single(Single {
            cards: returned_cards,
            value,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
            assert_eq!(value, CardValue(2).minus(1));
        } else {
            fmt_panic(cards, combo);
        }

        // after Dragon
        let cards = vec![Card {
            suit: CardSuit::Phoenix,
            value: CardValue::noop(),
        }];
        let combo = get_card_combination(
            Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &cards,
            &"1".to_string(),
        );
        assert_eq!(combo, None);
    }

    #[test]
    fn it_should_return_some_for_correct_pairs() {
        // a pair of cards of equal rank (plain)
        let cards = vec![
            Card {
                suit: CardSuit::Sword,
                value: CardValue(7),
            },
            Card {
                suit: CardSuit::Jade,
                value: CardValue(7),
            },
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Pair(Pair {
            cards: returned_cards,
            value,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
            assert_eq!(value, CardValue(7));
        } else {
            fmt_panic(cards, combo);
        }

        // a pair of cards of equal rank (with Phoenix)
        let cards = vec![
            Card {
                suit: CardSuit::Sword,
                value: CardValue(7),
            },
            Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            },
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Pair(Pair {
            cards: returned_cards,
            value,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
            assert_eq!(value, CardValue(7));
        } else {
            fmt_panic(cards, combo);
        }
    }

    #[test]
    fn it_should_return_some_for_correct_trios() {
        // a trio of cards of equal rank (standard)
        let cards = vec![
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Trio(Trio {
            cards: returned_cards,
            value,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
            assert_eq!(value, CardValue(3));
        } else {
            fmt_panic(cards, combo);
        }

        // a trio of cards of equal rank (with Phoenix)
        let cards = vec![
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Trio(Trio {
            cards: returned_cards,
            value,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
            assert_eq!(value, CardValue(3));
        } else {
            fmt_panic(cards, combo);
        }

        let cards = vec![
            Card {
                suit: CardSuit::Jade,
                value: CardValue(3),
            },
            Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            },
            Card {
                suit: CardSuit::Sword,
                value: CardValue(3),
            },
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Trio(Trio {
            cards: returned_cards,
            value,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
            assert_eq!(value, CardValue(3));
        } else {
            fmt_panic(cards, combo);
        }

        let cards = vec![
            Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            },
            Card {
                suit: CardSuit::Sword,
                value: CardValue(3),
            },
            Card {
                suit: CardSuit::Star,
                value: CardValue(3),
            },
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Trio(Trio {
            cards: returned_cards,
            value,
            ..
        })) = combo
        {
            assert_eq!(returned_cards, cards);
            assert_eq!(value, CardValue(3));
        } else {
            fmt_panic(cards, combo);
        }
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_4() {
        // a sequence of pairs, length 4 (plain)
        let cards = vec![
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(13),
            },
            Card {
                suit: CardSuit::Sword,
                value: CardValue(13),
            },
            Card {
                suit: CardSuit::Jade,
                value: CardValue(14),
            },
            Card {
                suit: CardSuit::Star,
                value: CardValue(14),
            },
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
            starting_value,
            number_of_pairs,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(starting_value, CardValue(13));
            assert_eq!(number_of_pairs, 2);
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // a sequence of pairs, length 4 (with phoenix)
        let cards = vec![
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
            starting_value,
            number_of_pairs,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(starting_value, CardValue(2));
            assert_eq!(number_of_pairs, 2);
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // a bomb (4 of the same)
        let cards = vec![
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::BombOf4(BombOf4 {
            value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(value, CardValue(7));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_5() {
        // a full house (trio + pair)
        let cards = vec![
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(13),
            },
            Card {
                suit: CardSuit::Star,
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::FullHouse(FullHouse {
            trio_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(trio_value, CardValue(13));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // a full house (pair + trio)
        let cards = vec![
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(13),
            },
            Card {
                suit: CardSuit::Star,
                value: CardValue(13),
            },
            Card {
                suit: CardSuit::Jade,
                value: CardValue(14),
            },
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14),
            },
            Card {
                suit: CardSuit::Sword,
                value: CardValue(14),
            },
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::FullHouse(FullHouse {
            trio_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(trio_value, CardValue(14));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // full house with phoenix as pair
        let cards = vec![
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(13),
            },
            Card {
                suit: CardSuit::Star,
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::FullHouse(FullHouse {
            trio_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(trio_value, CardValue(13));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // full house with phoenix as trio
        let cards = vec![
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(13),
            },
            Card {
                suit: CardSuit::Star,
                value: CardValue(13),
            },
            Card {
                suit: CardSuit::Jade,
                value: CardValue(14),
            },
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(14),
            },
            Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            },
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::FullHouse(FullHouse {
            trio_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(trio_value, CardValue(14));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // a non-bomb sequence of length 5
        let cards = vec![
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(number_of_cards, 5);
            assert_eq!(starting_value, CardValue(3));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // a non-bomb sequence of length 5 (with phoenix)
        let cards = vec![
            Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(number_of_cards, 5);
            assert_eq!(starting_value, CardValue(4));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        let cards = vec![
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(3),
            },
            Card {
                suit: CardSuit::Star,
                value: CardValue(4),
            },
            Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            },
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(6),
            },
            Card {
                suit: CardSuit::Sword,
                value: CardValue(7),
            },
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(number_of_cards, 5);
            assert_eq!(starting_value, CardValue(3));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        let cards = vec![
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
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            },
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(number_of_cards, 5);
            assert_eq!(starting_value, CardValue(3));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // sequence bomb length 5
        let cards = vec![
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::SequenceBomb(SequenceBomb {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            suit,
            ..
        })) = combo
        {
            assert_eq!(number_of_cards, 5);
            assert_eq!(starting_value, CardValue(3));
            assert_eq!(suit, CardSuit::Pagoda);
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_6() {
        // non-bomb sequence, length 6
        let cards = vec![
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(number_of_cards, 6);
            assert_eq!(starting_value, CardValue(3));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // a non-bomb sequence of length 6 (with phoenix)
        let cards = vec![
            Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(number_of_cards, 6);
            assert_eq!(starting_value, CardValue(4));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        let cards = vec![
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(3),
            },
            Card {
                suit: CardSuit::Star,
                value: CardValue(4),
            },
            Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(number_of_cards, 6);
            assert_eq!(starting_value, CardValue(3));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        let cards = vec![
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
                suit: CardSuit::Star,
                value: CardValue(7),
            },
            Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            },
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(number_of_cards, 6);
            assert_eq!(starting_value, CardValue(3));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // bomb sequence, length 6
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap(),
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![ /* omitted */],
                number_of_cards: 6,
                starting_value: CardValue(3),
                suit: CardSuit::Pagoda,
                user_id: "1".to_string(),
            }))
        );

        // sequence of pairs, length 6 (plain)
        let cards = vec![
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
            starting_value,
            number_of_pairs,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(starting_value, CardValue(12));
            assert_eq!(number_of_pairs, 3);
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // sequence of pairs, length 6 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(12),
                number_of_pairs: 3,
                cards: vec![ /* omitted */],
                user_id: "1".to_string(),
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(12),
                number_of_pairs: 3,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(12),
                number_of_pairs: 3,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_7() {
        // non-bomb sequence, length 7
        let cards = vec![
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(number_of_cards, 7);
            assert_eq!(starting_value, CardValue(3));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // a non-bomb sequence of length 7 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 7,
                starting_value: CardValue(4),
                user_id: "1".to_string(),
            }))
        );

        // bomb sequence, length 7
        let cards = vec![
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::SequenceBomb(SequenceBomb {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            suit,
            ..
        })) = combo
        {
            assert_eq!(number_of_cards, 7);
            assert_eq!(starting_value, CardValue(3));
            assert_eq!(suit, CardSuit::Pagoda);
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_8() {
        // non-bomb sequence, length 8
        let cards = vec![
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
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(number_of_cards, 8);
            assert_eq!(starting_value, CardValue(3));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // a non-bomb sequence of length 8 (with phoenix)
        let cards = vec![
            Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
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
                suit: CardSuit::Jade,
                value: CardValue(10),
            },
        ];
        let combo = get_card_combination(None, &cards, &"1".to_string());
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            ..
        })) = combo
        {
            assert_eq!(number_of_cards, 8);
            assert_eq!(starting_value, CardValue(4));
            assert_eq!(cards, returned_cards);
        } else {
            fmt_panic(cards, combo);
        }

        // bomb sequence, length 8
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![ /* omitted */],
                number_of_cards: 8,
                starting_value: CardValue(3),
                suit: CardSuit::Pagoda,
                user_id: "1".to_string(),
            }))
        );

        // sequence of pairs, length 8 (plain)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(11),
                number_of_pairs: 4,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );

        // sequence of pairs, length 8 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(11),
                number_of_pairs: 4,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(11),
                number_of_pairs: 4,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(11),
                number_of_pairs: 4,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_9() {
        // non-bomb sequence, length 9
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                number_of_cards: 9,
                starting_value: CardValue(3),
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );

        // a non-bomb sequence of length 9 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
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
                            suit: CardSuit::Jade,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(11),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 9,
                starting_value: CardValue(4),
                user_id: "1".to_string(),
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
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
                            suit: CardSuit::Star,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 9,
                starting_value: CardValue(3),
                user_id: "1".to_string(),
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                            suit: CardSuit::Star,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 9,
                starting_value: CardValue(3),
                user_id: "1".to_string(),
            }))
        );

        // bomb sequence, length 9
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![ /* omitted */],
                number_of_cards: 9,
                starting_value: CardValue(3),
                suit: CardSuit::Pagoda,
                user_id: "1".to_string(),
            }))
        );
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_10() {
        // non-bomb sequence, length 10
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                number_of_cards: 10,
                starting_value: CardValue(3),
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );

        // a non-bomb sequence of length 10 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
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
                            suit: CardSuit::Jade,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(12),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 10,
                starting_value: CardValue(4),
                user_id: "1".to_string(),
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(12),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 10,
                starting_value: CardValue(3),
                user_id: "1".to_string(),
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                            suit: CardSuit::Star,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 10,
                starting_value: CardValue(3),
                user_id: "1".to_string(),
            }))
        );

        // bomb sequence, length 10
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![ /* omitted */],
                number_of_cards: 10,
                starting_value: CardValue(3),
                suit: CardSuit::Pagoda,
                user_id: "1".to_string(),
            }))
        );

        // sequence of pairs, length 10 (plain)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(10),
                number_of_pairs: 5,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );

        // sequence of pairs, length 10 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(10),
                number_of_pairs: 5,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(10),
                number_of_pairs: 5,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(10),
                number_of_pairs: 5,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_11() {
        // non-bomb sequence, length 11
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                number_of_cards: 11,
                starting_value: CardValue(3),
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );

        // a non-bomb sequence of length 11 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
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
                            suit: CardSuit::Jade,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(13),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 11,
                starting_value: CardValue(4),
                user_id: "1".to_string(),
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(10),
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
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 11,
                starting_value: CardValue(3),
                user_id: "1".to_string(),
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                            suit: CardSuit::Star,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 11,
                starting_value: CardValue(3),
                user_id: "1".to_string(),
            }))
        );

        // bomb sequence, length 11
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![ /* omitted */],
                number_of_cards: 11,
                starting_value: CardValue(3),
                suit: CardSuit::Pagoda,
                user_id: "1".to_string(),
            }))
        );
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_12() {
        // non-bomb sequence, length 12
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                number_of_cards: 12,
                starting_value: CardValue(3),
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );

        // a non-bomb sequence of length 12 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
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
                            suit: CardSuit::Jade,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(14),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 12,
                starting_value: CardValue(4),
                user_id: "1".to_string(),
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(10),
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
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 12,
                starting_value: CardValue(3),
                user_id: "1".to_string(),
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                            suit: CardSuit::Star,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
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
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 12,
                starting_value: CardValue(3),
                user_id: "1".to_string(),
            }))
        );

        // bomb sequence, length 12
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![ /* omitted */],
                number_of_cards: 12,
                starting_value: CardValue(3),
                suit: CardSuit::Pagoda,
                user_id: "1".to_string(),
            }))
        );

        // sequence of pairs, length 12 (plain), (longest possible)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(9),
                number_of_pairs: 12,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );

        // sequence of pairs, length 12 (with phoenix), (longest possible)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(9),
                number_of_pairs: 6,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(9),
                number_of_pairs: 6,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(9),
                number_of_pairs: 6,
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_13() {
        // non-bomb sequence, length 13 (longest possible)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                number_of_cards: 13,
                starting_value: CardValue(2),
                user_id: "1".to_string(),
                cards: vec![ /* omitted */],
            }))
        );

        // a non-bomb sequence of length 13 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Star,
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
                            suit: CardSuit::Jade,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(14),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 13,
                starting_value: CardValue(3),
                user_id: "1".to_string(),
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
                        Card {
                            suit: CardSuit::Pagoda,
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
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(10),
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
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 13,
                starting_value: CardValue(2),
                user_id: "1".to_string(),
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
                        Card {
                            suit: CardSuit::Pagoda,
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
                            suit: CardSuit::Star,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(9),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(10),
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
                            suit: CardSuit::Sword,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 13,
                starting_value: CardValue(2),
                user_id: "1".to_string(),
            }))
        );

        // bomb sequence, length 13 (longest possible)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
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
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![ /* omitted */],
                number_of_cards: 13,
                starting_value: CardValue(3),
                suit: CardSuit::Pagoda,
                user_id: "1".to_string(),
            }))
        );
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_14() {
        // a non-bomb sequence of length 14 (with phoenix)--this is the longest possible combination
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(
                    None,
                    &vec![
                        Card {
                            suit: CardSuit::Phoenix,
                            value: CardValue::noop(),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(2),
                        },
                        Card {
                            suit: CardSuit::Star,
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
                            suit: CardSuit::Jade,
                            value: CardValue(10),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(12),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(14),
                        },
                    ],
                    &"1".to_string(),
                )
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 14,
                starting_value: CardValue(2),
                user_id: "1".to_string(),
            }))
        );
    }

    #[test]
    fn it_should_return_none_for_bogus_combos_length_2() {
        assert_eq!(get_card_combination(None, &vec![], &"1".to_string()), None);

        // two different cards
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Pagoda,
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(3),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );

        // two special cards
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Dragon,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::MahJong,
                        value: CardValue::noop(),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Dragon,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::Dog,
                        value: CardValue::noop(),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::MahJong,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::Dog,
                        value: CardValue::noop(),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );

        // non-Phoenix special card and standard card:
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Dog,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(11),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::MahJong,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::Jade,
                        value: CardValue(6),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Dragon,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::Pagoda,
                        value: CardValue(14),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );

        // Phoenix card and special card:
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::MahJong,
                        value: CardValue::noop(),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::Dragon,
                        value: CardValue::noop(),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::Dog,
                        value: CardValue::noop(),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );
    }

    #[test]
    fn it_should_return_none_for_bogus_combos_length_3() {
        // 2 same value and 3rd non-matching
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                ],
                &"1".to_string(),
            ),
            None
        );

        // 3 different values
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Pagoda,
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Sword,
                        value: CardValue(7),
                    },
                    Card {
                        suit: CardSuit::Jade,
                        value: CardValue(12),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );

        // run of 3
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                ],
                &"1".to_string(),
            ),
            None
        );

        // phoenix not useful
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Pagoda,
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::Sword,
                        value: CardValue(11),
                    },
                ],
                &"1".to_string(),
            ),
            None
        );
    }

    #[test]
    fn it_should_return_none_for_bogus_combos_length_4() {
        // 4 different values
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Pagoda,
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Sword,
                        value: CardValue(13),
                    },
                    Card {
                        suit: CardSuit::Jade,
                        value: CardValue(7),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(9),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );

        // 4 different values with Phoenix
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                        value: CardValue(7),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(9),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );

        // run of 4
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(5),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );

        // run of 4 with Phoenix
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                    },
                    Card {
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );

        // non-sequential pairs
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                        value: CardValue(4),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(4),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );

        // non-sequential pairs with Phoenix
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                        value: CardValue(4),
                    },
                    Card {
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );

        // 3 same and 1 out
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Jade,
                        value: CardValue(3),
                    },
                ],
                &"1".to_string(),
            ),
            None
        );

        // 3 same and 1 out with Phoenix
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );
    }

    #[test]
    fn it_should_return_none_for_bogus_combos_length_5() {
        // gap in run
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(5),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(7),
                    },
                ],
                &"1".to_string(),
            ),
            None
        );

        // gap in run with Phoenix
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                    },
                    Card {
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(7),
                    },
                ],
                &"1".to_string(),
            ),
            None
        );

        // 4 same and 1 out
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(3),
                    },
                ],
                &"1".to_string(),
            ),
            None
        );

        // 4 same and 1 out with Phoenix
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    },
                ],
                &"1".to_string(),
            ),
            None
        );
    }

    #[test]
    fn it_should_return_none_for_bogus_combos_length_6() {
        // gap in run
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(5),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(7),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(8),
                    },
                ],
                &"1".to_string(),
            ),
            None
        );

        // gap in run with Phoenix
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                    },
                    Card {
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(7),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(8),
                    },
                ],
                &"1".to_string(),
            ),
            None
        );

        // 5 same and 1 out (with Phoenix)
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(3),
                    },
                ],
                &"1".to_string(),
            ),
            None
        );

        // bad pairs
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                        value: CardValue(3),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(3),
                    },
                    Card {
                        suit: CardSuit::Jade,
                        value: CardValue(5),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(5),
                    },
                ],
                &"1".to_string(),
            ),
            None
        );
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                        suit: CardSuit::Star,
                        value: CardValue(5),
                    },
                ],
                &"1".to_string(),
            ),
            None
        );

        // bad pairs (with phoenix)
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                        value: CardValue(3),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(3),
                    },
                    Card {
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(5),
                    },
                ],
                &"1".to_string(),
            ),
            None
        );
    }

    #[test]
    fn it_should_return_none_for_invalid_suit_and_value_combos() {
        // card value below min
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Pagoda,
                        value: CardValue(1),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(1),
                    },
                ],
                &"1".to_string(),
            ),
            None
        );

        // special card without noop
        assert_eq!(
            get_card_combination(
                None,
                &vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue(2),
                },],
                &"1".to_string(),
            ),
            None
        );

        // regular card with noop
        assert_eq!(
            get_card_combination(
                None,
                &vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue::noop(),
                },],
                &"1".to_string(),
            ),
            None
        );

        // two of the same card
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Sword,
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Sword,
                        value: CardValue(2),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );

        // two of the same card
        assert_eq!(
            get_card_combination(
                None,
                &vec![
                    Card {
                        suit: CardSuit::Sword,
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Sword,
                        value: CardValue(2),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(2),
                    }
                ],
                &"1".to_string(),
            ),
            None
        );

        // card value above max
        assert_eq!(
            get_card_combination(
                None,
                &vec![
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
                ],
                &"1".to_string(),
            ),
            None
        );
    }
}

#[cfg(test)]
mod test_sort_cards_for_hand {
    use common::{sort_cards_for_hand, Card, CardSuit, CardValue, Deck};

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

#[cfg(test)]
mod test_next_combo_beats_prev {
    use common::{
        next_combo_beats_prev, BombOf4, Card, CardSuit, CardValue, FullHouse, Pair, Sequence,
        SequenceBomb, Single, ValidCardCombo,
    };
    use std::vec;

    #[test]
    fn it_should_allow_valid_bombs_of_4() {
        // any special card:
        // maj jong
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(2),
                user_id: "1".to_string(),
            })
        ));
        // dog
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(2),
                user_id: "1".to_string(),
            })
        ));
        // phoenix
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(2),
                user_id: "1".to_string(),
            })
        ));
        // dragon
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(2),
                user_id: "1".to_string(),
            })
        ));

        // any non-bomb combo:
        // single
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                }],
                value: CardValue(14),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(14),
                user_id: "1".to_string(),
            })
        ));
        // big combos
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Sequence(Sequence {
                cards: vec![/* omitted */],
                number_of_cards: 13,
                starting_value: CardValue(2),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(14),
                user_id: "1".to_string(),
            })
        ));

        // lower bomb of 4
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(13),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(14),
                user_id: "1".to_string(),
            })
        ));
    }

    #[test]
    fn it_should_not_allow_invalid_bombs_of_4() {
        // higher bomb of 4
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(13),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(12),
                user_id: "1".to_string(),
            })
        ));

        // sequence bomb
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 5,
                starting_value: CardValue(2),
                suit: CardSuit::Jade,
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(12),
                user_id: "1".to_string(),
            })
        ));
    }

    #[test]
    fn it_should_allow_valid_sequence_bombs() {
        let sequence_bomb_example = ValidCardCombo::SequenceBomb(SequenceBomb {
            cards: vec![/* omitted */],
            number_of_cards: 5,
            starting_value: CardValue(7),
            suit: CardSuit::Star,
            user_id: "1".to_string(),
        });

        // any special card:
        // maj jong
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &sequence_bomb_example,
        ));
        // dog
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &sequence_bomb_example,
        ));
        // phoenix
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &sequence_bomb_example,
        ));
        // dragon
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &sequence_bomb_example,
        ));

        // any non-bomb combo:
        // single
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                }],
                value: CardValue(14),
                user_id: "1".to_string(),
            })),
            &sequence_bomb_example,
        ));
        // big combos
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Sequence(Sequence {
                cards: vec![/* omitted */],
                number_of_cards: 13,
                starting_value: CardValue(2),
                user_id: "1".to_string(),
            })),
            &sequence_bomb_example,
        ));

        // bomb of 4
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(14),
                user_id: "1".to_string(),
            })),
            &sequence_bomb_example,
        ));

        // sequence bomb of fewer cards
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 5,
                starting_value: CardValue(7),
                suit: CardSuit::Star,
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 6,
                starting_value: CardValue(2),
                suit: CardSuit::Jade,
                user_id: "1".to_string(),
            }),
        ));

        // sequence bomb of same number of cards but lower value
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 5,
                starting_value: CardValue(7),
                suit: CardSuit::Star,
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 5,
                starting_value: CardValue(8),
                suit: CardSuit::Jade,
                user_id: "1".to_string(),
            }),
        ));
    }

    #[test]
    fn it_should_not_allow_invalid_sequence_bombs() {
        // longer sequence bomb
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 6,
                starting_value: CardValue(7),
                suit: CardSuit::Star,
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 5,
                starting_value: CardValue(8),
                suit: CardSuit::Jade,
                user_id: "1".to_string(),
            }),
        ));

        // sequence bomb with same number of higher value
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 5,
                starting_value: CardValue(9),
                suit: CardSuit::Star,
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 5,
                starting_value: CardValue(8),
                suit: CardSuit::Jade,
                user_id: "1".to_string(),
            }),
        ));
    }

    #[test]
    fn it_should_allow_valid_dragon_plays() {
        // any standard card:
        // standard card
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Jade,
                    value: CardValue(14)
                }],
                value: CardValue(14),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));
        // against phoenix
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));
        // against MahJong
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));
        // against Dog
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));
    }

    #[test]
    fn it_should_not_allow_invalid_dragon_plays() {
        // against non-single cards
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::Pair(Pair {
                value: CardValue(2),
                cards: vec![
                    Card {
                        suit: CardSuit::Jade,
                        value: CardValue(2)
                    },
                    Card {
                        suit: CardSuit::Pagoda,
                        value: CardValue(2)
                    }
                ],
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));
    }

    #[test]
    fn it_should_allow_valid_phoenix_plays() {
        // any single standard card
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Jade,
                    value: CardValue(14)
                }],
                value: CardValue(14),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));

        // dog
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));

        // mah jong
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));
    }

    #[test]
    fn it_should_not_allow_invalid_phoenix_plays() {
        // non-single cards
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::Pair(Pair {
                value: CardValue(2),
                cards: vec![
                    Card {
                        suit: CardSuit::Jade,
                        value: CardValue(2)
                    },
                    Card {
                        suit: CardSuit::Pagoda,
                        value: CardValue(2)
                    }
                ],
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));

        // dragon
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));
    }

    #[test]
    fn it_should_allow_valid_dog_plays() {
        // none
        assert!(next_combo_beats_prev(
            &None,
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));
    }

    #[test]
    fn it_should_not_allow_invalid_dog_plays() {
        // some
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));
    }

    #[test]
    fn it_should_allow_valid_mah_jong_plays() {
        // none
        assert!(next_combo_beats_prev(
            &None,
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));

        // dog
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));
    }

    #[test]
    fn it_should_not_allow_invalid_mah_jong_plays() {
        // some
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2),
                }],
                value: CardValue(2),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));
    }

    #[test]
    fn it_should_allow_valid_standard_singles() {
        // single
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Jade,
                    value: CardValue(2)
                }],
                value: CardValue(2),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(3)
                }],
                value: CardValue(3),
                user_id: "1".to_string(),
            })
        ));

        // on top of lower Phoenix
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue(11),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(12)
                }],
                value: CardValue(12),
                user_id: "1".to_string(),
            })
        ));

        // Mah Jong
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(2),
                user_id: "1".to_string(),
            })
        ));

        // Dog
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(2),
                user_id: "1".to_string(),
            })
        ));
    }

    #[test]
    fn it_should_not_allow_invalid_standard_singles() {
        // ==
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Jade,
                    value: CardValue(2)
                }],
                value: CardValue(3),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(2),
                user_id: "1".to_string(),
            })
        ));

        // <
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3)
                }],
                value: CardValue(3),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(3),
                user_id: "1".to_string(),
            })
        ));

        // against non-single cards
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::Pair(Pair {
                cards: vec![
                    Card {
                        suit: CardSuit::Jade,
                        value: CardValue(3)
                    },
                    Card {
                        suit: CardSuit::Sword,
                        value: CardValue(3)
                    }
                ],
                value: CardValue(3),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(4)
                }],
                value: CardValue(4),
                user_id: "1".to_string(),
            })
        ));

        // on top of higher Phoenix
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue(11),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(7)
                }],
                value: CardValue(7),
                user_id: "1".to_string(),
            })
        ));

        // on top of Dragon
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
                user_id: "1".to_string(),
            })
        ));
    }

    #[test]
    fn it_should_allow_any_card_when_none_has_been_played() {
        // single
        assert!(next_combo_beats_prev(
            &None,
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(2)
                }],
                value: CardValue(2),
                user_id: "1".to_string(),
            })
        ));
        assert!(next_combo_beats_prev(
            &None,
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(14)
                }],
                value: CardValue(14),
                user_id: "1".to_string(),
            })
        ));

        // Mah Jong
        assert!(next_combo_beats_prev(
            &None,
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));

        // Dog
        assert!(next_combo_beats_prev(
            &None,
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));

        // Phoenix
        assert!(next_combo_beats_prev(
            &None,
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue(2),
                user_id: "1".to_string(),
            })
        ));

        // Dragon
        assert!(next_combo_beats_prev(
            &None,
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
                user_id: "1".to_string(),
            })
        ));

        // full house
        assert!(next_combo_beats_prev(
            &None,
            &ValidCardCombo::FullHouse(FullHouse {
                cards: vec![/* omitted */],
                trio_value: CardValue(10),
                user_id: "1".to_string(),
            })
        ));

        // long sequence
        assert!(next_combo_beats_prev(
            &None,
            &ValidCardCombo::Sequence(Sequence {
                cards: vec![/* omitted */],
                number_of_cards: 10,
                starting_value: CardValue(2),
                user_id: "1".to_string(),
            })
        ));
    }
}

mod test_get_user_can_play_wished_for_card {
    use common::{
        get_card_combination, get_user_can_play_wished_for_card, Card, CardSuit, CardValue,
        FullHouse, Sequence, Single, ValidCardCombo,
    };

    #[test]
    fn it_should_return_correct_boolean() {
        // CAN PLAY - SINGLE ///////////////////////////////////////////////////////////////////
        let prev_combo = ValidCardCombo::Single(Single {
            cards: vec![Card {
                suit: CardSuit::Sword,
                value: CardValue(7),
            }],
            user_id: "omitted".to_string(),
            value: CardValue(7),
        });
        let users_hand = vec![
            Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            },
            Card {
                suit: CardSuit::Sword,
                value: CardValue(3),
            },
            Card {
                suit: CardSuit::Jade,
                value: CardValue(8),
            },
        ];
        let wished_for_card_value = CardValue(8);
        assert!(get_user_can_play_wished_for_card(
            Some(&prev_combo),
            &users_hand,
            &wished_for_card_value
        ));

        // CAN PLAY - COMBO ///////////////////////////////////////////////////////////////////
        let prev_combo = ValidCardCombo::FullHouse(FullHouse {
            cards: vec![
                Card {
                    value: CardValue(2),
                    suit: CardSuit::Sword,
                },
                Card {
                    value: CardValue(2),
                    suit: CardSuit::Jade,
                },
                Card {
                    value: CardValue(2),
                    suit: CardSuit::Pagoda,
                },
                Card {
                    value: CardValue(3),
                    suit: CardSuit::Star,
                },
                Card {
                    value: CardValue(3),
                    suit: CardSuit::Jade,
                },
            ],
            user_id: "omitted".to_string(),
            trio_value: CardValue(2),
        });
        let users_hand = vec![
            Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            },
            Card {
                suit: CardSuit::Sword,
                value: CardValue(12),
            },
            Card {
                suit: CardSuit::Jade,
                value: CardValue(8),
            },
            Card {
                suit: CardSuit::Sword,
                value: CardValue(8),
            },
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(8),
            },
        ];
        let wished_for_card_value = CardValue(12);
        assert!(get_user_can_play_wished_for_card(
            Some(&prev_combo),
            &users_hand,
            &wished_for_card_value
        ));

        // CAN PLAY - NO PREVIOUS COMBO ///////////////////////////////////////////////////////////////////
        let users_hand = vec![
            Card {
                suit: CardSuit::Jade,
                value: CardValue(2),
            },
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(4),
            },
        ];
        let wished_for_card_value = CardValue(4);
        assert!(get_user_can_play_wished_for_card(
            None,
            &users_hand,
            &wished_for_card_value
        ));

        // CAN'T PLAY - SINGLE ///////////////////////////////////////////////////////////////////
        let prev_combo = ValidCardCombo::Single(Single {
            cards: vec![Card {
                value: CardValue(7),
                suit: CardSuit::Pagoda,
            }],
            user_id: "omitted".to_string(),
            value: CardValue(7),
        });
        let users_hand = vec![
            Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            },
            Card {
                suit: CardSuit::Sword,
                value: CardValue(3),
            },
            Card {
                suit: CardSuit::Jade,
                value: CardValue(8),
            },
        ];
        let wished_for_card_value = CardValue(6);
        assert_eq!(
            get_user_can_play_wished_for_card(
                Some(&prev_combo),
                &users_hand,
                &wished_for_card_value
            ),
            false
        );

        // CAN'T PLAY - COMBO ///////////////////////////////////////////////////////////////////
        let prev_combo = ValidCardCombo::FullHouse(FullHouse {
            cards: vec![
                Card {
                    value: CardValue(4),
                    suit: CardSuit::Sword,
                },
                Card {
                    value: CardValue(4),
                    suit: CardSuit::Jade,
                },
                Card {
                    value: CardValue(4),
                    suit: CardSuit::Pagoda,
                },
                Card {
                    value: CardValue(3),
                    suit: CardSuit::Star,
                },
                Card {
                    value: CardValue(3),
                    suit: CardSuit::Jade,
                },
            ],
            user_id: "omitted".to_string(),
            trio_value: CardValue(4),
        });
        let users_hand = vec![
            Card {
                suit: CardSuit::Sword,
                value: CardValue(12),
            },
            Card {
                suit: CardSuit::Star,
                value: CardValue(12),
            },
            Card {
                suit: CardSuit::Jade,
                value: CardValue(8),
            },
            Card {
                suit: CardSuit::Sword,
                value: CardValue(8),
            },
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(8),
            },
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(9),
            },
        ];
        let wished_for_card_value = CardValue(9);
        assert_eq!(
            get_card_combination(Some(&prev_combo), &users_hand, &"".to_string()).is_some(),
            false
        );

        // CAN'T PLAY - NO PREVIOUS COMBO ///////////////////////////////////////////////////////////////////
        let users_hand = vec![
            Card {
                suit: CardSuit::Jade,
                value: CardValue(2),
            },
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(4),
            },
        ];
        let wished_for_card_value = CardValue(5);
        assert_eq!(
            get_user_can_play_wished_for_card(None, &users_hand, &wished_for_card_value),
            false
        );

        // CAN PLAY - NORMAL HAND LENGTH ///////////////////////////////////////////////////////////////////
        let prev_combo = ValidCardCombo::Sequence(Sequence {
            cards: vec![
                Card {
                    value: CardValue(2),
                    suit: CardSuit::Sword,
                },
                Card {
                    value: CardValue(3),
                    suit: CardSuit::Jade,
                },
                Card {
                    value: CardValue(4),
                    suit: CardSuit::Pagoda,
                },
                Card {
                    value: CardValue(5),
                    suit: CardSuit::Star,
                },
                Card {
                    value: CardValue(6),
                    suit: CardSuit::Jade,
                },
                Card {
                    value: CardValue(7),
                    suit: CardSuit::Sword,
                },
                Card {
                    value: CardValue(8),
                    suit: CardSuit::Star,
                },
                Card {
                    value: CardValue(9),
                    suit: CardSuit::Pagoda,
                },
            ],
            number_of_cards: 8,
            starting_value: CardValue(2),
            user_id: "omitted".to_string(),
        });
        let users_hand = vec![
            Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            },
            Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            },
            Card {
                suit: CardSuit::Jade,
                value: CardValue(2),
            },
            Card {
                suit: CardSuit::Sword,
                value: CardValue(3),
            },
            Card {
                suit: CardSuit::Jade,
                value: CardValue(4),
            },
            Card {
                suit: CardSuit::Sword,
                value: CardValue(5),
            },
            Card {
                suit: CardSuit::Pagoda,
                value: CardValue(6),
            },
            Card {
                suit: CardSuit::Jade,
                value: CardValue(7),
            },
            Card {
                suit: CardSuit::Sword,
                value: CardValue(8),
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
                suit: CardSuit::Star,
                value: CardValue(14),
            },
        ];
        let wished_for_card_value = CardValue(14);
        assert!(get_user_can_play_wished_for_card(
            Some(&prev_combo),
            &users_hand,
            &wished_for_card_value
        ));
    }
}
