use crate::Card;

pub fn is_valid_combination(cards: &Vec<Card>) -> bool {
    if cards.is_empty() {
        return false;
    }

    // a single card
    if cards.len() == 1 {
        return true;
    }

    // a pair of cards of equal rank
    if cards.len() == 2 {
        if let [card_0, card_1] = &cards[..cards.len()] {
            return card_0.value == card_1.value;
        }

        return false;
    }

    // a sequence of pairs of adjacent value

    // a trio of cards of equal rank

    // a bomb (4 of the same)

    // a full house (trio + pair)

    // a sequence of length at least 5

    unimplemented!()
}

#[cfg(test)]
mod tests {
    mod test_get_random_string_of_len {
        use super::super::is_valid_combination;
        #[test]
        fn it_should_produce_intended_length() {
            // no cards
            assert_eq!(is_valid_combination(&vec![]), false);

            // a single card
            assert_eq!(is_valid_combination(&vec![]), true);
            assert_eq!(is_valid_combination(&vec![]), true);
            assert_eq!(is_valid_combination(&vec![]), true);

            // a pair of cards of equal rank
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);

            // a sequence of pairs of adjacent value
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);

            // a trio of cards of equal rank
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);

            // a bomb (4 of the same)
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);

            // a full house (trio + pair)
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);

            // a sequence of length at least 5
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);

            // a sequence of length at least 5
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);
            assert_eq!(is_valid_combination(&vec![]), false);
        }
    }
}
