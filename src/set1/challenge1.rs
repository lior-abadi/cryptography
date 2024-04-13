// Objective: Convert hex to base64
// Rules: Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing.

use base64;
use hex;

#[allow(dead_code)]
pub fn run(hex_input: &str) -> String {
    return hex_to_base64(hex_input);
}

fn hex_to_base64(hex_str: &str) -> String {
    let hex_val = hex::decode(hex_str).unwrap();
    return base64::encode(hex_val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let hex_input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64_expected_output =
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
        assert_eq!(run(hex_input), base64_expected_output);
    }
}
