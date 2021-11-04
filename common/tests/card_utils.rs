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
        let combo = get_card_combination(&cards);
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
        let combo = get_card_combination(&cards);
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
        let combo = get_card_combination(&cards);
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
        let combo = get_card_combination(&cards);
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Pair(Pair {
            cards: returned_cards,
            value,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Pair(Pair {
            cards: returned_cards,
            value,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Trio(Trio {
            cards: returned_cards,
            value,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Trio(Trio {
            cards: returned_cards,
            value,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Trio(Trio {
            cards: returned_cards,
            value,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Trio(Trio {
            cards: returned_cards,
            value,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
            starting_value,
            number_of_pairs,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
            starting_value,
            number_of_pairs,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::BombOf4(BombOf4 {
            value,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::FullHouse(FullHouse {
            trio_value,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::FullHouse(FullHouse {
            trio_value,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::FullHouse(FullHouse {
            trio_value,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::FullHouse(FullHouse {
            trio_value,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::SequenceBomb(SequenceBomb {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            suit,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
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
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![ /* omitted */],
                number_of_cards: 6,
                starting_value: CardValue(3),
                suit: CardSuit::Pagoda,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
            starting_value,
            number_of_pairs,
            cards: returned_cards,
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(12),
                number_of_pairs: 3,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
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
                &get_card_combination(&vec![
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 7,
                starting_value: CardValue(4),
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::SequenceBomb(SequenceBomb {
            number_of_cards,
            starting_value,
            cards: returned_cards,
            suit,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
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
        let combo = get_card_combination(&cards);
        if let Some(ValidCardCombo::Sequence(Sequence {
            number_of_cards,
            starting_value,
            cards: returned_cards,
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
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(11),
                number_of_pairs: 4,
                cards: vec![ /* omitted */],
            }))
        );
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_9() {
        // non-bomb sequence, length 9
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
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                number_of_cards: 9,
                starting_value: CardValue(3),
                cards: vec![ /* omitted */],
            }))
        );

        // a non-bomb sequence of length 9 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(&vec![
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 9,
                starting_value: CardValue(4),
            }))
        );
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 9,
                starting_value: CardValue(3),
            }))
        );
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 9,
                starting_value: CardValue(3),
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
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![ /* omitted */],
                number_of_cards: 9,
                starting_value: CardValue(3),
                suit: CardSuit::Pagoda,
            }))
        );
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_10() {
        // non-bomb sequence, length 10
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
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                number_of_cards: 10,
                starting_value: CardValue(3),
                cards: vec![ /* omitted */],
            }))
        );

        // a non-bomb sequence of length 10 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(&vec![
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 10,
                starting_value: CardValue(4),
            }))
        );
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 10,
                starting_value: CardValue(3),
            }))
        );
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 10,
                starting_value: CardValue(3),
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
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(10),
                number_of_pairs: 5,
                cards: vec![ /* omitted */],
            }))
        );
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_11() {
        // non-bomb sequence, length 11
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
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                number_of_cards: 11,
                starting_value: CardValue(3),
                cards: vec![ /* omitted */],
            }))
        );

        // a non-bomb sequence of length 11 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(&vec![
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 11,
                starting_value: CardValue(4),
            }))
        );
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 11,
                starting_value: CardValue(3),
            }))
        );
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 11,
                starting_value: CardValue(3),
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
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![ /* omitted */],
                number_of_cards: 11,
                starting_value: CardValue(3),
                suit: CardSuit::Pagoda,
            }))
        );
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_12() {
        // non-bomb sequence, length 12
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
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                number_of_cards: 12,
                starting_value: CardValue(3),
                cards: vec![ /* omitted */],
            }))
        );

        // a non-bomb sequence of length 12 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(&vec![
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 12,
                starting_value: CardValue(4),
            }))
        );
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 12,
                starting_value: CardValue(3),
            }))
        );
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 12,
                starting_value: CardValue(3),
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
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(9),
                number_of_pairs: 12,
                cards: vec![ /* omitted */],
            }))
        );

        // sequence of pairs, length 12 (with phoenix), (longest possible)
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
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
            std::mem::discriminant(&ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(9),
                number_of_pairs: 6,
                cards: vec![ /* omitted */],
            }))
        );
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_13() {
        // non-bomb sequence, length 13 (longest possible)
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
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                number_of_cards: 13,
                starting_value: CardValue(2),
                cards: vec![ /* omitted */],
            }))
        );

        // a non-bomb sequence of length 13 (with phoenix)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(&vec![
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 13,
                starting_value: CardValue(3),
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 13,
                starting_value: CardValue(2),
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 13,
                starting_value: CardValue(2),
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
            std::mem::discriminant(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![ /* omitted */],
                number_of_cards: 13,
                starting_value: CardValue(3),
                suit: CardSuit::Pagoda,
            }))
        );
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_14() {
        // a non-bomb sequence of length 14 (with phoenix)--this is the longest possible combination
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(&vec![
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombo::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 14,
                starting_value: CardValue(2),
            }))
        );
    }

    #[test]
    fn it_should_return_none_for_bogus_combos_length_2() {
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

        // two special cards
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
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
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
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
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
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

        // Phoenix card and special card:
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }
            ]),
            None
        );
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }
            ]),
            None
        );
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }
            ]),
            None
        );
    }

    #[test]
    fn it_should_return_none_for_bogus_combos_length_3() {
        // 2 same value and 3rd non-matching
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

        // 3 different values
        assert_eq!(
            get_card_combination(&vec![
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
            ]),
            None
        );

        // run of 3
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

        // phoenix not useful
        assert_eq!(
            get_card_combination(&vec![
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
            ]),
            None
        );
    }

    #[test]
    fn it_should_return_none_for_bogus_combos_length_4() {
        // 4 different values
        assert_eq!(
            get_card_combination(&vec![
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
            ]),
            None
        );

        // 4 different values with Phoenix
        assert_eq!(
            get_card_combination(&vec![
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
            ]),
            None
        );

        // run of 4
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
                },
                Card {
                    suit: CardSuit::Star,
                    value: CardValue(5),
                }
            ]),
            None
        );

        // run of 4 with Phoenix
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
                },
                Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }
            ]),
            None
        );

        // non-sequential pairs
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
                    value: CardValue(4),
                },
                Card {
                    suit: CardSuit::Star,
                    value: CardValue(4),
                }
            ]),
            None
        );

        // non-sequential pairs with Phoenix
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
                    value: CardValue(4),
                },
                Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }
            ]),
            None
        );

        // 3 same and 1 out
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
                    value: CardValue(2),
                },
                Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3),
                },
            ]),
            None
        );

        // 3 same and 1 out with Phoenix
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
                    value: CardValue(2),
                },
                Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }
            ]),
            None
        );
    }

    #[test]
    fn it_should_return_none_for_bogus_combos_length_5() {
        // gap in run
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
                },
                Card {
                    suit: CardSuit::Star,
                    value: CardValue(5),
                },
                Card {
                    suit: CardSuit::Star,
                    value: CardValue(7),
                },
            ]),
            None
        );

        // gap in run with Phoenix
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
                },
                Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                },
                Card {
                    suit: CardSuit::Star,
                    value: CardValue(7),
                },
            ]),
            None
        );

        // 4 same and 1 out
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
            ]),
            None
        );

        // 4 same and 1 out with Phoenix
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
            ]),
            None
        );
    }

    #[test]
    fn it_should_return_none_for_bogus_combos_length_6() {
        // gap in run
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
            ]),
            None
        );

        // gap in run with Phoenix
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
            ]),
            None
        );

        // 5 same and 1 out (with Phoenix)
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
            ]),
            None
        );

        // bad pairs
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
            ]),
            None
        );
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
            ]),
            None
        );

        // bad pairs (with phoenix)
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
            ]),
            None
        );
    }

    #[test]
    fn it_should_return_none_for_invalid_suit_and_value_combos() {
        // card value below min
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(1),
                },
                Card {
                    suit: CardSuit::Star,
                    value: CardValue(1),
                },
            ]),
            None
        );

        // special card without noop
        assert_eq!(
            get_card_combination(&vec![Card {
                suit: CardSuit::Dragon,
                value: CardValue(2),
            },]),
            None
        );

        // regular card with noop
        assert_eq!(
            get_card_combination(&vec![Card {
                suit: CardSuit::Sword,
                value: CardValue::noop(),
            },]),
            None
        );

        // two of the same card
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2),
                },
                Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2),
                }
            ]),
            None
        );

        // two of the same card
        assert_eq!(
            get_card_combination(&vec![
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
            ]),
            None
        );

        // card value above max
        assert_eq!(
            get_card_combination(&vec![
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
            ]),
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
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(2)
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
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(2)
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
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(2)
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
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(2)
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
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(14)
            })
        ));
        // big combos
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Sequence(Sequence {
                cards: vec![/* omitted */],
                number_of_cards: 13,
                starting_value: CardValue(2),
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(14)
            })
        ));

        // lower bomb of 4
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(13)
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(14)
            })
        ));
    }

    #[test]
    fn it_should_not_allow_invalid_bombs_of_4() {
        // higher bomb of 4
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(13)
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(12)
            })
        ));

        // sequence bomb
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 5,
                starting_value: CardValue(2),
                suit: CardSuit::Jade,
            })),
            &ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(12)
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
            })),
            &sequence_bomb_example,
        ));
        // big combos
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::Sequence(Sequence {
                cards: vec![/* omitted */],
                number_of_cards: 13,
                starting_value: CardValue(2),
            })),
            &sequence_bomb_example,
        ));

        // bomb of 4
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::BombOf4(BombOf4 {
                cards: vec![/* omitted */],
                value: CardValue(14),
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
            })),
            &ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 6,
                starting_value: CardValue(2),
                suit: CardSuit::Jade,
            }),
        ));

        // sequence bomb of same number of cards but lower value
        assert!(next_combo_beats_prev(
            &Some(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 5,
                starting_value: CardValue(7),
                suit: CardSuit::Star,
            })),
            &ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 5,
                starting_value: CardValue(8),
                suit: CardSuit::Jade,
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
            })),
            &ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 5,
                starting_value: CardValue(8),
                suit: CardSuit::Jade,
            }),
        ));

        // sequence bomb with same number of higher value
        assert!(!next_combo_beats_prev(
            &Some(&ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 5,
                starting_value: CardValue(9),
                suit: CardSuit::Star,
            })),
            &ValidCardCombo::SequenceBomb(SequenceBomb {
                cards: vec![/* omitted */],
                number_of_cards: 5,
                starting_value: CardValue(8),
                suit: CardSuit::Jade,
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
                value: CardValue(14)
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
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
                ]
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop()
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
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
                ]
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(3)
                }],
                value: CardValue(3),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(12)
                }],
                value: CardValue(12),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(2),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(2),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(2),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(3),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(4)
                }],
                value: CardValue(4),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(7)
                }],
                value: CardValue(7),
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
            })),
            &ValidCardCombo::Single(Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
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
            })
        ));

        // full house
        assert!(next_combo_beats_prev(
            &None,
            &ValidCardCombo::FullHouse(FullHouse {
                cards: vec![/* omitted */],
                trio_value: CardValue(10),
            })
        ));

        // long sequence
        assert!(next_combo_beats_prev(
            &None,
            &ValidCardCombo::Sequence(Sequence {
                cards: vec![/* omitted */],
                number_of_cards: 10,
                starting_value: CardValue(2),
            })
        ));
    }
}
