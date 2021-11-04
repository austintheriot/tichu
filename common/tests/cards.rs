#[cfg(test)]
mod test_single_card {
    use common::{Card, CardSuit, CardValue, Single};

    #[test]
    fn it_should_compare_std_cards_of_same_suit_correctly() {
        // different suit, is less than
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(2),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3)
                }],
                value: CardValue(3),
            },
            true
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(2),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3)
                }],
                value: CardValue(3),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(2)
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3)
                }],
                value: CardValue(3),
            },
            false
        );

        // different suit, is equal to
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            },
            true
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            },
            false
        );

        // different suit, is greater than
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                }],
                value: CardValue(14),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                }],
                value: CardValue(14),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                }],
                value: CardValue(14),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                }],
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
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(2)
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3)
                }],
                value: CardValue(3),
            },
            true
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(2)
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3)
                }],
                value: CardValue(3),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2)
                }],
                value: CardValue(2)
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Jade,
                    value: CardValue(3)
                }],
                value: CardValue(3),
            },
            false
        );

        // different suit, is equal to
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            },
            true
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            },
            false
        );

        // different suit, is greater than
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                }],
                value: CardValue(14),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                }],
                value: CardValue(14),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                }],
                value: CardValue(11),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Star,
                    value: CardValue(14),
                }],
                value: CardValue(14),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(11)
                }],
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
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop()
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
            },
            true
        );

        // standard to Dragon
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Star,
                    value: CardValue(2),
                }],
                value: CardValue(2),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            },
            true
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(7),
                }],
                value: CardValue(7)
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(13),
                }],
                value: CardValue(13),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
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
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
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
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
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
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
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
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
            },
            true
        );

        // standard to Phoenix
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Star,
                    value: CardValue(2),
                }],
                value: CardValue(2),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            },
            true
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(7),
                }],
                value: CardValue(7)
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(14),
                }],
                value: CardValue(14),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
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
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
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
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
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
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
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
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            },
            true
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Phoenix,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                }],
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
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
            },
            true
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
            },
            false
        );

        // standard to MahJong
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Star,
                    value: CardValue(2),
                }],
                value: CardValue(2),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue(2),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(7),
                }],
                value: CardValue(7),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(13),
                }],
                value: CardValue(13),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                }],
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
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
            },
            true
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14)
                }],
                value: CardValue(14),
            },
            false
        );

        // standard to Dog
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Star,
                    value: CardValue(2),
                }],
                value: CardValue(2),
            } < Single {
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(7),
                }],
                value: CardValue(7),
            } == Single {
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            },
            false
        );
        assert_eq!(
            Single {
                cards: vec![Card {
                    suit: CardSuit::Sword,
                    value: CardValue(13),
                }],
                value: CardValue(13),
            } > Single {
                cards: vec![Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                }],
                value: CardValue::noop(),
            },
            true
        );
    }
}

#[cfg(test)]
mod test_double {
    use common::{Card, CardSuit, CardValue, Pair};

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

#[cfg(test)]
mod test_sequence_of_pairs {
    use common::{CardValue, SequenceOfPairs};

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

#[cfg(test)]
mod test_trio {
    use common::{CardValue, Trio};

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

#[cfg(test)]
mod test_bomb_of_4 {
    use common::{BombOf4, CardValue};

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

#[cfg(test)]
mod test_sequence_bomb {
    use common::{CardSuit, CardValue, SequenceBomb};

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

#[cfg(test)]
mod test_full_house {
    use common::{CardValue, FullHouse};

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

#[cfg(test)]
mod test_card_value {
    use common::CardValue;

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

#[cfg(test)]
mod test_deck {
    use common::{Card, CardSuit, CardValue, Deck};

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

#[cfg(test)]
mod test_sequence {
    use common::{CardValue, Sequence};

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
