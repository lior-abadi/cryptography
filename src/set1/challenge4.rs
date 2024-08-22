use super::challenge3::decrypt_single_byte_xor;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn run() {
    let all_inputs = read_all_inputs().unwrap();

    let mut utf8_matches: Vec<String> = vec![];
    for input in all_inputs.iter() {
        let (_, plaintext) = super::challenge3::run(input as &str);
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

fn read_all_inputs() -> io::Result<Vec<String>> {
    let current_dir = env::current_dir()?.join(Path::new("src/set1"));
    let text_input_dir = Path::new("challenge4_input.txt");

    let absolute_path = current_dir.join(text_input_dir);
    // Attempt to open the file and handle any errors
    let file = match File::open(&absolute_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Err(e);
        }
    };

    let lines: Vec<String> = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;
    Ok(lines)
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
