// Objective: Write a function that takes two equal-length buffers and produces their XOR combination.

use hex;

use crate::utils::xor::xor_utilities::xor_bytes;

pub fn run(hex_input1: &str, hex_input2: &str) -> String {
    // XOR truth table:
    // x  y  XOR
    // 0  0  0
    // 1  0  1
    // 0  1  1
    // 1  1  0

    let xor_result = xor_bytes(
        &hex::decode(hex_input1).unwrap(),
        &hex::decode(hex_input2).unwrap(),
    );
    return hex::encode(xor_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::xor::xor_utilities::check_symmetry;

    #[test]
    fn test_run() {
        let hex_input = "1c0111001f010100061a024b53535009181c";
        let hex_input2 = "686974207468652062756c6c277320657965";
        let expected_output = "746865206b696420646f6e277420706c6179".to_string();
        assert_eq!(run(hex_input, hex_input2), expected_output);
    }

    #[test]
    fn test_xor() {
        let hex_input = "1c0111001f010100061a024b53535009181c";
        let hex_input2 = "686974207468652062756c6c277320657965";

        println!(
            "{:?}",
            hex::encode(xor_bytes(
                &hex::decode(hex_input).unwrap(),
                &hex::decode(hex_input2).unwrap()
            ))
        );
    }

    #[test]
    fn test_check_symmetry() {
        let hex_input = "1c0111001f010100061a024b53535009181c";
        let hex_input2 = "686974207468652062756c6c277320657965";
        let hex_input3 = "1111";

        let check1 = check_symmetry(
            &hex::decode(hex_input).unwrap(),
            &hex::decode(hex_input2).unwrap(),
        );
        assert!(check1);

        let check2 = check_symmetry(
            &hex::decode(hex_input).unwrap(),
            &hex::decode(hex_input3).unwrap(),
        );
        assert!(!check2);
    }
}
