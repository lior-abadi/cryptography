// Decipher AES encrypted file
mod tests {
    use super::*;
    use crate::utils::{aes_128_ecb::ecb::ecb_decrypt, fs::read_input_return_join_lines};
    use std::path::Path;

    #[test]
    fn test_run() {
        let data_path = Path::new("src/set1/challenge7_input.txt");
        let ciphertext_string_b64 = read_input_return_join_lines(data_path).unwrap();
        let ciphertext = base64::decode(ciphertext_string_b64).unwrap();

        let key = "YELLOW SUBMARINE".as_bytes();

        let returns = ecb_decrypt(&ciphertext, key);
        println!("{:?}", String::from_utf8_lossy(&returns));
    }
}
