// Break Repeating Key XOR Cypher
use base64;
use core::f64;
use std::io::{self, BufRead};
use std::{env, fs::File, path::Path};

use super::challenge3::decrypt_single_byte_xor;
use super::challenge5::repeating_key_xor;

fn break_xor_cypher(ciphertext: &[u8]) -> (String, String) {
    let mut normalized_distances: Vec<(usize, f64)> = Vec::new();

    for current_key_size in 2..40 {
        if ciphertext.len() < current_key_size * 4 {
            continue;
        }

        let chunk1 = &ciphertext[0..current_key_size];
        let chunk2 = &ciphertext[current_key_size..current_key_size * 2];
        let chunk3 = &ciphertext[current_key_size * 2..current_key_size * 3];
        let chunk4 = &ciphertext[current_key_size * 3..current_key_size * 4];

        let mut total_distance = 0.0;
        let mut count = 0;

        // Compute all pairwise Hamming distances between the 4 chunks.
        for (a, b) in [
            (chunk1, chunk2),
            (chunk2, chunk3),
            (chunk3, chunk4),
            (chunk1, chunk3),
            (chunk1, chunk4),
            (chunk2, chunk4),
        ] {
            if let Ok(dist) = calculate_hamming_distance(a, b) {
                total_distance += normalize_hamming_distance(current_key_size as f64, dist as f64);
                count += 1;
            }
        }

        // Count should be 6
        assert!(count == 6, "Count of pairs was not 6");
        if count > 0 {
            let normalized_distance = total_distance / count as f64;
            normalized_distances.push((current_key_size, normalized_distance));
        }
    }

    // Get the minimum key size based on normalized distances.
    let extracted_data = get_min_keysize_and_distance(&normalized_distances);
    let cyphertext_blocks = break_cypher_into_transposed_blocks(ciphertext, extracted_data.0);

    let mut key: Vec<u8> = Vec::new();
    for block in cyphertext_blocks {
        let (best_key_candidate, _) = decrypt_single_byte_xor(&block);
        key.push(best_key_candidate);
    }

    // Now that we know the key, we can decrypt the cyphertext

    return (
        String::from_utf8_lossy(&key).to_string(),
        String::from_utf8_lossy(&repeating_key_xor(ciphertext, &key)).to_string(),
    );
}

fn break_cypher_into_transposed_blocks(ciphertext: &[u8], keysize: usize) -> Vec<Vec<u8>> {
    let num_blocks = (ciphertext.len() + keysize - 1) / keysize;

    let mut blocks: Vec<Vec<u8>> = vec![Vec::new(); keysize];

    for i in 0..ciphertext.len() {
        blocks[i % keysize].push(ciphertext[i]); // transpose with %
    }

    blocks
}

fn get_min_keysize_and_distance(array: &Vec<(usize, f64)>) -> (usize, f64) {
    let mut min_keysize: usize = 0 as usize;
    let mut min_value = f64::INFINITY;

    for &(keysize, value) in array {
        if value < min_value {
            min_value = value;
            min_keysize = keysize
        }
    }

    return (min_keysize, min_value);
}

// Fixed Hamming distance calculation using XOR and loops.
fn calculate_hamming_distance(u: &[u8], v: &[u8]) -> Result<usize, &'static str> {
    if u.len() != v.len() {
        return Err("inputs need to have the same length");
    }

    let mut distance = 0;
    for (a, b) in u.iter().zip(v.iter()) {
        let xor_result = a ^ b;
        distance += xor_result.count_ones() as usize;
    }

    Ok(distance)
}

fn normalize_hamming_distance(keysize: f64, distance: f64) -> f64 {
    return distance / keysize;
}

fn bytes_to_bit_array(input: &[u8]) -> Vec<bool> {
    let mut bits = Vec::new();

    // Convert each byte in the string to its bit representation
    for byte in input {
        for i in (0..8).rev() {
            // Check each bit from the most significant to the least significant
            bits.push(byte & (1 << i) != 0);
        }
    }

    bits
}

fn read_input() -> io::Result<String> {
    let current_dir = env::current_dir()?.join(Path::new("src/set1"));
    let text_input_dir = Path::new("challenge6_input.txt");

    let absolute_path = current_dir.join(text_input_dir);
    // Attempt to open the file and handle any errors
    let file = match File::open(&absolute_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Err(e);
        }
    };

    // Read lines and join them into a single string
    let contents = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()? // Collect all lines into a Vec<String>
        .join(""); // Join all lines

    Ok(contents)
}

mod tests {
    use super::*;

    #[test]
    fn test_break_cypher() {
        let cyphertext_string_b64 = read_input().unwrap();
        let cyphertext_string = base64::decode(cyphertext_string_b64).unwrap();
        let returns = break_xor_cypher(&cyphertext_string);
        println!("{:?}", returns);
    }

    #[test]
    fn test_hamming_distance() {
        let a = "this is a test";
        let b = "wokka wokka!!!";

        if let Ok(distance) = calculate_hamming_distance(&(a.as_bytes()), &(b.as_bytes())) {
            assert_eq!(37, distance);
        }
    }

    #[test]
    fn test_read_input() {
        let cyphertext_string_b64 = read_input().unwrap();
        let input = base64::decode(cyphertext_string_b64).unwrap();
        println!("{:?}", input);
    }
}
