// Repeating Key XOR

use hex;

mod tests {
    use crate::utils::xor::xor_utilities::repeating_key_xor;

    use super::*;

    #[test]
    fn test_run() {
        let input: &str =
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected_output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        let encrypted_input = repeating_key_xor(input.as_bytes(), "ICE".as_bytes());
        assert_eq!(hex::encode(encrypted_input), expected_output);
    }
}
