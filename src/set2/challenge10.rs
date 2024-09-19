mod tests {
    use crate::utils::{aes_cbc::cbc::cbc_decrypt, fs::read_input_return_join_lines};
    use std::path::Path;

    use super::*;

    #[test]
    fn test_decrypt() {
        let data_path = Path::new("src/set2/challenge10_input.txt");
        let ciphertext_string_b64 = read_input_return_join_lines(data_path).unwrap();
        let ciphertext = base64::decode(ciphertext_string_b64).unwrap();

        let key = "YELLOW SUBMARINE".as_bytes();
        let iv = vec![0u8; 16];
        let block_size = 16;
        match cbc_decrypt(key, &iv, &ciphertext, block_size) {
            Ok(plaintext) => println!("{:?}", String::from_utf8_lossy(&plaintext)),
            Err(e) => println!("{:?}", e),
        }
    }
}
