// Repeating Key XOR

use hex;

fn repeating_key_xor(input: &str, key: &str) -> String {
    let mut cyphertext = vec![];
    let key_bytes = key.as_bytes();
    let plaintext_bytes = input.as_bytes();

    for (i, &byte) in plaintext_bytes.iter().enumerate() {
        let key_byte = key_bytes[i % key_bytes.len()];
        cyphertext.push(byte ^ key_byte);
    }

    return hex::encode(cyphertext);
}

mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input: &str =
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected_output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        let encrypted_input = repeating_key_xor(input, "ICE");
        assert_eq!(encrypted_input, expected_output);
    }
}
