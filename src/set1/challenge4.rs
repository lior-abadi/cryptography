use crate::{set1::challenge3::decrypt_single_byte_xor, utils::fs::read_all_inputs_return_lines};

use std::path::Path;

fn run() {
    let current_dir = Path::new("src/set1/challenge4_input.txt");
    let all_inputs = read_all_inputs_return_lines(current_dir).unwrap();

    let mut utf8_matches: Vec<String> = vec![];
    for input in all_inputs.iter() {
        let inputs = hex::decode(input).unwrap();
        let (_, plaintext) = decrypt_single_byte_xor(&inputs);
        if let Ok(text) = String::from_utf8(plaintext) {
            utf8_matches.push(text);
        }
    }

    let mut best_match = "";
    let mut best_score = 0.0;

    for single_match in utf8_matches.iter() {
        let score = is_idiomatic_sentence(&single_match);
        if score > best_score {
            best_match = single_match;
            best_score = score;
        }
    }

    println!("The most sentence-like string is: '{}'", best_match);
}

fn is_idiomatic_sentence(sentence: &str) -> f64 {
    let alphabetic_count = sentence.chars().filter(|c| c.is_alphabetic()).count();
    let space_count = sentence.chars().filter(|c| c.is_whitespace()).count();
    let total_count = sentence.len();

    let alpha_space_ratio = (alphabetic_count + space_count) as f64 / total_count as f64;
    alpha_space_ratio
}

mod tests {
    use super::*;

    #[test]
    fn test_all_values() {
        run();
    }
}
