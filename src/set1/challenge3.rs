// Single-byte XOR cipher

use hex;

pub fn run(encrypted_xor_input: &str) -> (u8, Vec<u8>) {
    let encrypted_input_bytes = hex::decode(encrypted_xor_input).unwrap();
    return decrypt_single_byte_xor(&encrypted_input_bytes);
}

fn decrypt_single_byte_xor(input_as_bytes: &Vec<u8>) -> (u8, Vec<u8>) {
    let mut best_score = f64::MIN;
    let mut best_key = 0;
    let mut best_text = vec![];

    // Since it is a single-byte xor cypher, there are only 256 keys (0-255)
    for key in 0..=255 {
        let decrypted_text = xor_with_single_byte_key(input_as_bytes, key);
        let score = score_text(&decrypted_text);

        if score > best_score {
            best_score = score;
            best_key = key;
            best_text = decrypted_text;
        }
    }

    (best_key, best_text)
}

fn xor_with_single_byte_key(a: &Vec<u8>, key: u8) -> Vec<u8> {
    let mut result = Vec::new();

    for i in 0..a.len() {
        result.push(a[i] ^ key);
    }

    return result;
}

fn score_text(text: &Vec<u8>) -> f64 {
    let freq_map = vec![
        ('e', 12.70),
        ('t', 9.06),
        ('a', 8.17),
        ('o', 7.51),
        ('i', 6.97),
        ('n', 6.75),
        (' ', 13.00),
        ('s', 6.33),
        ('h', 6.09),
        ('r', 5.99),
        ('d', 4.25),
        ('l', 4.03),
        ('c', 2.78),
        ('u', 2.76),
        ('m', 2.41),
        ('w', 2.36),
        ('f', 2.23),
        ('g', 2.02),
        ('y', 1.97),
        ('p', 1.93),
        ('b', 1.29),
        ('v', 0.98),
        ('k', 0.77),
        ('x', 0.15),
        ('q', 0.10),
        ('j', 0.15),
        ('z', 0.07),
    ];

    let mut score = 0.0; // initialize score

    // Loop over each byte
    for &byte in text.iter() {
        // For each byte, compare it against the frequency map
        for &(ch, freq) in &freq_map {
            if ch as u8 == byte.to_ascii_lowercase() {
                score += freq;
                break; // break inner loop since we have already found a match
            }
        }
    }

    score
}

mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let encrypted_hex_string =
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let (key, plaintext) = run(&encrypted_hex_string);

        println!("Best Key {:?}", key);
        println!("Decrypted Text {:?}", String::from_utf8_lossy(&plaintext));
    }
}
