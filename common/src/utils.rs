use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

fn get_random_string_of_len(len: usize) -> String {
    let random_name = Uuid::new_v4().to_string();
    let mut random_name = random_name.graphemes(true).collect::<Vec<&str>>();
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
