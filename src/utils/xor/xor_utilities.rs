pub fn check_symmetry(hex_str1: &Vec<u8>, hex_str2: &Vec<u8>) -> bool {
    return hex_str1.len() == hex_str2.len();
}

pub fn xor_bytes(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let is_symmetric = check_symmetry(&a, &b);
    assert!(is_symmetric, "Inputs (buffers) should have equal length");

    let mut result = Vec::new();

    let min_length = a.len(); // since lengths are the same (checked before) we can use any vector's size

    for i in 0..min_length {
        result.push(a[i] ^ b[i]); // XOR the corresponding elements and push to the result
    }

    return result;
}

pub fn xor_with_single_byte_key(a: &Vec<u8>, key: u8) -> Vec<u8> {
    let mut result = Vec::new();

    for i in 0..a.len() {
        result.push(a[i] ^ key);
    }

    return result;
}

pub fn repeating_key_xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    let mut cyphertext = vec![];

    for (i, &byte) in input.iter().enumerate() {
        let key_byte = key[i % key.len()];
        cyphertext.push(byte ^ key_byte);
    }

    return cyphertext;
}
