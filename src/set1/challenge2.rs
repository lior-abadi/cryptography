// Objective: Write a function that takes two equal-length buffers and produces their XOR combination.

use hex;

pub fn run(hex_input1: &str, hex_input2: &str) -> String {
    let is_symmetric = check_symmetry(hex_input1, hex_input2);
    assert!(is_symmetric, "Inputs (buffers) should have equal length");

    // XOR truth table:
    // x  y  XOR
    // 0  0  0
    // 1  0  1
    // 0  1  1
    // 1  1  0

    let xor_result = xor_bytes(
        hex::decode(hex_input1).unwrap(),
        hex::decode(hex_input2).unwrap(),
    );
    return hex::encode(xor_result);
}

fn check_symmetry(hex_str1: &str, hex_str2: &str) -> bool {
    return hex_str1.len() == hex_str2.len();
}

fn xor_bytes(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();

    let min_length = a.len(); // since lengths are the same (checked before) we can use any vector's size

    for i in 0..min_length {
        result.push(a[i] ^ b[i]); // XOR the corresponding elements and push to the result
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

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
                hex::decode(hex_input).unwrap(),
                hex::decode(hex_input2).unwrap()
            ))
        );
    }

    #[test]
    fn test_check_symmetry() {
        let hex_input = "1c0111001f010100061a024b53535009181c";
        let hex_input2 = "686974207468652062756c6c277320657965";
        let hex_input3 = "1111";

        let check1 = check_symmetry(hex_input, hex_input2);
        assert!(check1);

        let check2 = check_symmetry(hex_input, hex_input3);
        assert!(!check2);
    }
}
