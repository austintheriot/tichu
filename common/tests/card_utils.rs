#[cfg(test)]
mod test_get_card_combination {
    use common::{
        get_card_combination, BombOf4, Card, CardSuit, CardValue, FullHouse, Pair, Sequence,
        SequenceBomb, SequenceOfPairs, Single, Trio, ValidCardCombos,
    };

    #[test]
    fn it_should_return_some_for_correct_single_cards() {
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
    }

    #[test]
    fn it_should_return_some_for_correct_pairs() {
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
    }

    #[test]
    fn it_should_return_some_for_correct_trios() {
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
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(&vec![
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::Trio(Trio {
                value: CardValue(3),
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
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_4() {
        // a sequence of pairs, length 4 (plain)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(&vec![
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
                        value: CardValue(13),
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
                        suit: CardSuit::Jade,
                        value: CardValue(14),
                    },
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(13),
                number_of_pairs: 2,
                cards: vec![ /* omitted */],
            }))
        );
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(&vec![
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
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    },
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                starting_value: CardValue(13),
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
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_5() {
        // a full house (trio + pair)
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(&vec![
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::FullHouse(FullHouse {
                cards: vec![ /* omitted */],
                trio_value: CardValue(13),
            }))
        );

        // full house with phoenix
        assert_eq!(
            std::mem::discriminant(
                &get_card_combination(&vec![
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::FullHouse(FullHouse {
                cards: vec![ /* omitted */],
                trio_value: CardValue(13),
            }))
        );

        // a non-bomb sequence of length 5
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

        // a non-bomb sequence of length 5 (with phoenix)
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 5,
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 5,
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
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
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

        // sequence bomb length 5
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
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_6() {
        // non-bomb sequence, length 6
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

        // a non-bomb sequence of length 6 (with phoenix)
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 6,
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 6,
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
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    },
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 6,
                starting_value: CardValue(3),
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
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_7() {
        // non-bomb sequence, length 7
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 7,
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
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(9),
                    },
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 7,
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
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    },
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 7,
                starting_value: CardValue(3),
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
    }

    #[test]
    fn it_should_return_some_for_correct_combinations_of_length_8() {
        // non-bomb sequence, length 8
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

        // a non-bomb sequence of length 8 (with phoenix)
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
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 8,
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
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(9),
                    },
                    Card {
                        suit: CardSuit::Star,
                        value: CardValue(10),
                    },
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 8,
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
                        suit: CardSuit::Phoenix,
                        value: CardValue::noop(),
                    },
                ])
                .unwrap()
            ),
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                cards: vec![ /* omitted */],
                number_of_cards: 8,
                starting_value: CardValue(3),
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::SequenceBomb(SequenceBomb {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::SequenceBomb(SequenceBomb {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
            std::mem::discriminant(&ValidCardCombos::SequenceBomb(SequenceBomb {
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
            std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
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
