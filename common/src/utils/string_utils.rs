use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

fn get_random_string_of_len(len: usize) -> String {
    let random_uuid = Uuid::new_v4().to_string();
    let mut random_uuid = random_uuid.graphemes(true).collect::<Vec<&str>>();
    random_uuid.retain(|s| *s != "-");
    random_uuid.truncate(len);
    let random_uuid = random_uuid.join("");
    random_uuid.to_uppercase()
}

/// Generates a new game code.
///
/// Default length is 3, but increases length if runs into game_code
/// name collisions more than 10 times at a given string length
/// (hacky implementation for now while prototyping)
///
/// @todo: increase initial length to prevent rogue game entries
/// @todo: use a more efficient generation algorithm
/// @todo: filter for swear words
/// @todo: implement proper error handling
pub fn get_new_game_code(game_codes: &HashMap<String, String>) -> String {
    let mut string_len: usize = 1;
    let mut count: u128 = 0;

    let mut random_name = get_random_string_of_len(string_len);
    while game_codes.contains_key(&random_name) {
        if string_len > GAME_CODE_MAX_LEN {
            panic!("Max iterations reached on get_new_game_code");
        }
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
    game_code.trim().to_uppercase()
}

pub fn clean_up_team_name(team_name: &str) -> String {
    team_name.trim().to_string()
}

pub const DISPLAY_NAME_MAX_LEN: usize = 25;

/// Returns Some(Errors) or None if no errors.
pub fn validate_display_name(display_name: &str) -> Option<String> {
    let mut error = String::from("");

    let cleaned_up_display_name = clean_up_display_name(display_name);

    if cleaned_up_display_name.is_empty() {
        error = String::from("Display name is required");
    } else if cleaned_up_display_name.len() > 25 {
        error = format!(
            "Display name exceeds maximum length of {}",
            DISPLAY_NAME_MAX_LEN
        );
    }

    if !error.is_empty() {
        Some(error)
    } else {
        None
    }
}

pub const GAME_CODE_MAX_LEN: usize = 8;

/// Returns Some(Errors) or None if no errors.
pub fn validate_game_code(game_code: &str) -> Option<String> {
    let mut error = String::from("");

    if clean_up_display_name(game_code).is_empty() {
        error = String::from("Game code is required");
    } else if game_code.to_uppercase() != game_code {
        error = String::from("Game code is not all uppercase");
    } else if game_code.len() > GAME_CODE_MAX_LEN {
        error = format!("Game code exceeds maximum length of {}", GAME_CODE_MAX_LEN);
    };

    if !error.is_empty() {
        Some(error)
    } else {
        None
    }
}

pub const TEAM_NAME_MAX_LEN: usize = 25;

pub fn validate_team_name(team_name: &str) -> Option<String> {
    let mut error = String::from("");

    let cleaned_up_team_name = clean_up_team_name(team_name);

    if cleaned_up_team_name.is_empty() {
        error = String::from("Team name must not be an empty string");
    } else if cleaned_up_team_name.len() > TEAM_NAME_MAX_LEN {
        error = format!("Team name exceeds maximum length of {}", TEAM_NAME_MAX_LEN);
    }

    if !error.is_empty() {
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

        #[test]
        fn it_should_not_include_invalid_characters() {
            assert!(!get_random_string_of_len(10).contains('-'));
        }
    }
}
