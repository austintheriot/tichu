use crate::{Card, Pair, Single, Trio, ValidCardCombos};

// TODO: account for single special cards and Phoenix wild card
pub fn get_card_combination(cards: &Vec<Card>) -> Option<ValidCardCombos> {
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
        if let [card_0, card_1] = &cards[..cards.len()] {
            return if card_0.value == card_1.value {
                Some(ValidCardCombos::Pair(Pair {
                    cards: cards.clone(),
                    value: card_0.value.clone(),
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
            } else {
                None
            };
        }

        return None;
    }

    // length 4:
    // a bomb (4 of the same)
    // a sequence of pairs of adjacent value

    // length 5:
    // a full house (trio + pair)
    // a sequence of length at least 5
    // a bomb (sequence of 5, all same suit)

    // any length greater than 5:
    // a sequence
    // a sequence of pairs of adjacent value

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::{get_card_combination, Card, CardSuit, CardValue};

    mod test_get_random_string_of_len {
        use super::super::get_card_combination;
        use crate::{Card, CardSuit, CardValue, Pair, Single, Trio, ValidCardCombos};

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
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![Card {
                        suit: CardSuit::Pagoda,
                        value: CardValue(14),
                    }])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Single(Single(Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14),
                })))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![Card {
                        suit: CardSuit::Dragon,
                        value: CardValue::noop(),
                    }])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Single(Single(Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                })))
            );

            // a pair of cards of equal rank
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
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(13),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Pair(Pair {
                    value: CardValue(13),
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Pair(Pair {
                    value: CardValue(14),
                    cards: vec![ /* omitted */],
                }))
            );

            // a trio of cards of equal rank
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
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(11),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Trio(Trio {
                    value: CardValue(11),
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
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Trio(Trio {
                    value: CardValue(14),
                    cards: vec![ /* omitted */],
                }))
            );

            // a sequence of pairs of adjacent value
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // a bomb (4 of the same)
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // a full house (trio + pair)
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // a sequence of length at least 5
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // sequence bomb
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // sequence of pairs of adjacent value (any length)
            // length 4
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // length 6
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // length 8
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // length 10
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // TODO: every combo as possible with a Phoenix:
            // length 2: a pair of cards of equal rank
            // OR 1 standard card and 1 Phoenix

            // length 3: a trio of cards of equal rank

            // length 4:
            // a bomb (4 of the same)
            // a sequence of pairs of adjacent value

            // length 5:
            // a full house (trio + pair)
            // a sequence of length at least 5
            // a bomb (sequence of 5, all same suit)

            // any length greater than 5:
            // a sequence
            // a sequence of pairs of adjacent value
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
