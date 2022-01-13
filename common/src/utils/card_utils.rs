use crate::{BombOf4, Card, CardSuit, CardValue, FullHouse, MAX_CARDS_IN_HAND, Pair, Sequence, SequenceBomb, SequenceOfPairs, Single, Trio, ValidCardCombo};
use itertools::Itertools;


pub fn get_card_combination(prev_combo: Option<&ValidCardCombo>, cards: &Vec<Card>, user_id_who_played_cards: &str) -> Option<ValidCardCombo> {
    let original_cards = cards;
    let mut cards = cards.clone();
    let mut includes_phoenix = false;
    let mut includes_non_phoenix_special_card = false;
    let mut value_out_of_range = false;
    let mut identical_cards_found = false;
    let user_id =  user_id_who_played_cards.to_owned();

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
        let card = cards.get(0).unwrap();
        if card.suit == CardSuit::Phoenix {
            // since 1/2 values aren't possible with the Phoenix, new value should be equal
            // (which functions the same in the end)
            return if let Some(prev_combo) = prev_combo {
                if let ValidCardCombo::Single(prev_single) = prev_combo {
                    match prev_single.cards.first().expect("Every single card should have a Vec of cards").suit {
                        // lowest possible card
                        CardSuit::Dog | CardSuit::MahJong => Some(ValidCardCombo::Single(Single{
                            cards: vec![card.clone()],
                            // any card can beat the Phoenix when it's led
                            value: CardValue::min().minus(1),
                            user_id,
                        })),
                        // invalid play
                        CardSuit::Dragon | CardSuit::Phoenix=> None,
                        // copy the value of the previous Single
                        _ =>  Some(ValidCardCombo::Single(Single{
                            cards: vec![card.clone()],
                            value: prev_single.value.clone(),
                            user_id,
                        })),
                    }
                } else {
                    // invalid prev_combo
                    None
                }
            } else {
                // no previous value was played: play the least possible value card
                Some(ValidCardCombo::Single(Single{
                    cards: vec![card.clone()],
                    // any card can beat the Phoenix when it's led
                    value: CardValue::min().minus(1),
                    user_id,
                }))
            }
        }

        return Some(ValidCardCombo::Single(Single{
            cards: vec![card.clone()],
            value: card.value.clone(),
            user_id,
        }));
    }

    if cards.len() == 2 {
        // standard pair
        if let [card_0, card_1] = &cards[..cards.len()] {
            return if card_0.value == card_1.value {
                Some(ValidCardCombo::Pair(Pair {
                    cards: original_cards.clone(),
                    value: card_0.value.clone(),
                    user_id,
                }))
            }
            // pair with 1 standard card and 1 Phoenix
            else if includes_phoenix {
                let std_card = cards.iter().find(|card| card.suit != CardSuit::Phoenix);
                Some(ValidCardCombo::Pair(Pair {
                    cards: original_cards.clone(),
                    value: std_card.unwrap().value.clone(),
                    user_id,
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
                Some(ValidCardCombo::Trio(Trio {
                    cards: original_cards.clone(),
                    value: card_0.value.clone(),
                    user_id,
                }))
            } else if includes_phoenix {
                let std_cards: Vec<&Card> = cards
                    .iter()
                    .filter(|card| card.suit != CardSuit::Phoenix)
                    .collect();
                if let [std_card_0, std_card_1] = &std_cards[0..std_cards.len()] {
                    if std_card_0.value == std_card_1.value {
                        Some(ValidCardCombo::Trio(Trio {
                            cards: original_cards.clone(),
                            value: std_card_0.value.clone(),
                            user_id,
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
                return Some(ValidCardCombo::BombOf4(BombOf4 {
                    cards: original_cards.clone(),
                    value: card_0.value.clone(),
                    user_id,
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
            let highest_value_card = cards_without_phoenix.last().expect("Should have non-Phoenix cards").clone();
            cards_without_phoenix.push(highest_value_card);
            adjusted_cards = cards_without_phoenix;
        }

        // full house (first 3 are equal)
        if let [card_0, card_1, card_2, card_3, card_4] = &adjusted_cards[..adjusted_cards.len()] {
            if (card_0.value == card_1.value && card_0.value == card_2.value)
                && (card_3.value == card_4.value)
                && (card_0.value != card_3.value)
            {
                return Some(ValidCardCombo::FullHouse(FullHouse {
                    cards: original_cards.clone(),
                    trio_value: card_0.value.clone(),
                    user_id
                }));
            }
            // full house (last 3 are equal)
            else if (card_2.value == card_3.value && card_2.value == card_4.value)
                && (card_0.value == card_1.value)
                && (card_0.value != card_2.value)
            {
                return Some(ValidCardCombo::FullHouse(FullHouse {
                    cards: original_cards.clone(),
                    trio_value: card_2.value.clone(),
                    user_id
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
                let mut new_card = cards_without_phoenix.last().expect("There should be non-Phoenix cards").clone();
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
                Some(ValidCardCombo::SequenceBomb(SequenceBomb {
                    cards: original_cards.clone(),
                    number_of_cards: original_cards.len() as u8,
                    starting_value: cards[0].value.clone(),
                    suit: cards[0].suit.clone(),
                    user_id,
                }))
            } else {
                // non-bomb sequence
                Some(ValidCardCombo::Sequence(Sequence {
                    cards: original_cards.clone(),
                    number_of_cards: original_cards.len() as u8,
                    starting_value: cards[0].value.clone(),
                    user_id,
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
            return Some(ValidCardCombo::SequenceOfPairs(SequenceOfPairs {
                cards: original_cards.clone(),
                starting_value: cards[0].value.clone(),
                number_of_pairs: cards.len() as u8 / 2,
                user_id,
            }));
        }
    }

    None
}

pub fn next_combo_beats_prev(prev: &Option<&ValidCardCombo>, next: &ValidCardCombo) -> bool {
    if let Some(prev) = prev {
        // sequence bomb
        if let ValidCardCombo::SequenceBomb(next_sequence_bomb) = &next {
            // sequence bomb vs sequence bomb
            return if let ValidCardCombo::SequenceBomb(prev_sequence_bomb) = &prev {
               next_sequence_bomb.number_of_cards > prev_sequence_bomb.number_of_cards
                || (next_sequence_bomb.number_of_cards == prev_sequence_bomb.number_of_cards 
                    && next_sequence_bomb.starting_value > prev_sequence_bomb.starting_value)
            } else  {
                // sequence bomb beats any other combo
                true
            }
          }

        // bomb of 4
        if let ValidCardCombo::BombOf4(next_bomb_of_4) = &next {
            // bomb of 4 vs bomb of 4
            return if let ValidCardCombo::BombOf4(prev_bomb_4) = &prev {
                next_bomb_of_4.value > prev_bomb_4.value
            } else {
                // sequence bomb beats any other non sequence-bomb combo
                !prev.is_sequence_bomb()
            }
          }

          // Standard single on top of Phoenix single
          if let ValidCardCombo::Single(Single{ cards: prev_cards, value: prev_value, .. }) = &prev {
              if let Some(Card { suit: CardSuit::Phoenix, .. }) = prev_cards.first() {
                if let ValidCardCombo::Single(Single{ cards: next_cards, value: next_value, .. } ) = &next {
                    if let Some(Card { suit: next_suit, ..}) = next_cards.first() {
                        if !next_suit.is_special() {
                            return next_value > prev_value
                        }
                    }
                }
              }
          }

          // any standard card must be the same type and greater
          (std::mem::discriminant(*prev) == std::mem::discriminant(next)) && (next > prev)
    } else {
        // no card has yet been played, any combination can follow
        true
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

pub fn get_user_can_play_wished_for_card(prev_combo: Option<&ValidCardCombo>, users_hand: &Vec<Card>, wished_for_card_value: &CardValue) -> bool {
    // CardValue::noop() is equivalent to None
    if *wished_for_card_value == CardValue::noop() {
        return false;
    }

    // if user does not have the wished for card, return false
    if !users_hand.iter().any(|card| card.value == *wished_for_card_value) {
        return false
    }

    // if prev combo is none and the user has the wished for card, then they can play it,
    // and we already know that the user has the wished-for card value
    let prev_combo = if let Some(prev_combo) = prev_combo {
        prev_combo
    } else {
        return true;
    };

    // user doesn't have enough cards to match the combo
    if users_hand.len() < prev_combo.cards().len() {
        return false;
    }

    // create combinations of given length
    let mut all_combos_of_same_length: Vec<Vec<Card>> = users_hand.iter()
        .combinations(prev_combo.cards().len())
        .map(|cards| {
            cards.into_iter()
            .map(|card| card.to_owned())
            .collect()
        })
        .collect();
    
    // must have wished for card
    all_combos_of_same_length.retain(|cards| {
        cards.iter().any(|card| card.value == *wished_for_card_value)
    });

    // must be valid combo, and must beat previous combo
    return all_combos_of_same_length.iter().any(|cards| {
        let card_combo = get_card_combination(Some(prev_combo), cards, &String::from(""));
        if let Some(card_combo) = card_combo {
            let combo_beats_prev_combo =   next_combo_beats_prev(&Some(prev_combo), &card_combo);
            if combo_beats_prev_combo {
                // it's only necessary to find one combination that works to prove
                // that the user CAN play the wished for card
                return true;
            }
        }
        false
    });
}