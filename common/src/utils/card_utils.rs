use crate::{Card, Pair, Single, ValidCardCombos};

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
    mod test_get_random_string_of_len {
        use super::super::get_card_combination;
        #[test]
        fn it_should_produce_intended_length() {
            // no cards
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // a single card
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // a pair of cards of equal rank
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // a sequence of pairs of adjacent value
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // a trio of cards of equal rank
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

            // a sequence of length at least 5
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
            assert_eq!(get_card_combination(&vec![]).is_some(), false);
        }
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
