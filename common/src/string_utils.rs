use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

fn get_random_string_of_len(len: usize) -> String {
    let random_name = Uuid::new_v4().to_string();
    let mut random_name = random_name.graphemes(true).collect::<Vec<&str>>().split;
    random_name.truncate(len);
    let random_name = random_name.join("");
    random_name.to_uppercase()
}

/// Generates a new game code.
///
/// HACKY (for now while prototyping)
/// Default length is 3, but increases length if runs into game_code
/// name collisions more than 10 times at a given string length.
pub fn get_new_game_code(game_codes: &HashMap<String, String>) -> String {
    let mut string_len: usize = 1;
    let mut count: u128 = 0;

    let mut random_name = get_random_string_of_len(string_len);
    while game_codes.contains_key(&random_name) {
        if count > 10 {
            string_len += 1;
        }
        count += 1;
        random_name = get_random_string_of_len(string_len);
    }
    random_name
}

pub fn clean_up_display_name(display_name: &str) -> String {
    display_name.trim().to_string()
}

pub fn clean_up_game_code(game_code: &str) -> String {
    game_code.trim().to_uppercase().to_string()
}

/// Returns Some(Errors) or None if no errors.
pub fn validate_display_name(display_name: &str) -> Option<String> {
    let mut error = String::from("");

    if clean_up_display_name(display_name).len() == 0 {
        error = String::from("Display name is not long enough");
    }

    if error.len() != 0 {
        Some(error)
    } else {
        None
    }
}

/// Returns Some(Errors) or None if no errors.
pub fn validate_game_code(game_code: &str) -> Option<String> {
    let mut error = String::from("");

    if clean_up_display_name(game_code).len() == 0 {
        error = String::from("Game code is not long enough");
    }

    if error.len() != 0 {
        Some(error)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    mod test_get_random_string_of_len {
        use super::super::get_random_string_of_len;
        #[test]
        fn it_should_produce_intended_length() {
            assert_eq!(get_random_string_of_len(1).len(), 1);
            assert_eq!(get_random_string_of_len(2).len(), 2);
            assert_eq!(get_random_string_of_len(3).len(), 3);
            assert_eq!(get_random_string_of_len(4).len(), 4);
            assert_eq!(get_random_string_of_len(5).len(), 5);
            assert_eq!(get_random_string_of_len(6).len(), 6);
            assert_eq!(get_random_string_of_len(7).len(), 7);
        }
    }
}
