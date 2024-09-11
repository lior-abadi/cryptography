// Decipher AES encrypted file
use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit},
    Aes128,
};

fn decrypt_aes(ciphertext: &[u8], key: &[u8]) -> String {
    let key_generic = GenericArray::clone_from_slice(key);
    let mut blocks = Vec::new();

    for x in (0..ciphertext.len()).step_by(16) {
        // Push the 16-byte block into the vector after cloning it from the slice
        blocks.push(GenericArray::clone_from_slice(&ciphertext[x..x + 16])); // blocks of 4x4
    }

    let cipher = Aes128::new(&key_generic); // initialize cipher
    cipher.decrypt_blocks(&mut blocks); // decrypt each ECB block

    // Flatten result
    let result: String = {
        let mut decrypted_chars = String::new();
        for block in &blocks {
            for &byte in block.iter() {
                decrypted_chars.push(byte as char);
            }
        }
        decrypted_chars
    };

    result
}

mod tests {
    use super::*;
    use crate::utils::fs::read_input_return_join_lines;
    use std::path::Path;

    #[test]
    fn test_run() {
        let data_path = Path::new("src/set1/challenge7_input.txt");
        let ciphertext_string_b64 = read_input_return_join_lines(data_path).unwrap();
        let ciphertext = base64::decode(ciphertext_string_b64).unwrap();

        let key = "YELLOW SUBMARINE".as_bytes();

        let returns = decrypt_aes(&ciphertext, key);
        println!("{:?}", returns);
    }
}
