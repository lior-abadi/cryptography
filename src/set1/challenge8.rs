// Detect AES 128 ECB encryption

// Since in ECB the same 16byte block produces the same 16byte ciphertext in a deterministic way
// if repetition is found, there is a strong chance of AES encryption.

use std::collections::HashSet;

fn detect_aes_ecb(ciphertext: &[u8]) -> bool {
    let mut blocks = HashSet::new();

    for chunk in ciphertext.chunks(16) {
        if blocks.contains(chunk) {
            return true;
        }
        blocks.insert(chunk);
    }

    return false;
}

mod tests {
    use super::*;
    use crate::utils::fs::read_all_inputs_return_lines;
    use std::path::Path;

    #[test]
    fn test_run() {
        let data_path = Path::new("src/set1/challenge8_input.txt");
        let ciphertext_string_hex_vector = read_all_inputs_return_lines(data_path).unwrap();

        let mut set_of_aes_encrypted_text = Vec::new();

        for ciphertext_string_hex in ciphertext_string_hex_vector {
            let ciphertext = hex::decode(&ciphertext_string_hex).unwrap();

            if (detect_aes_ecb(&ciphertext)) {
                set_of_aes_encrypted_text.push(ciphertext_string_hex);
            }
        }

        for ciphertext in set_of_aes_encrypted_text {
            println!("{:?}\n", ciphertext);
        }
    }
}
