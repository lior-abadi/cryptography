use aes::cipher::BlockDecrypt;
use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use aes::Aes128;

use crate::utils::pkcs7::padding::pkcs7_pad;

pub fn ecb_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    // Ensure the key length is 16 bytes for AES-128
    assert_eq!(key.len(), 16, "Key length must be 16 bytes for AES-128!");

    // Create a GenericArray for the key
    let key = GenericArray::from_slice(key);

    // Initialize AES-128 cipher
    let cipher = Aes128::new(&key);

    // Pad the data using PKCS7 padding
    let padded_data = pkcs7_pad(data, 16);

    // Encrypt block-by-block
    let mut ciphertext = Vec::new();
    for chunk in padded_data.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        ciphertext.extend_from_slice(&block);
    }

    ciphertext
}

pub fn ecb_decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let key_generic = GenericArray::clone_from_slice(key);
    let mut blocks = Vec::new();

    for x in (0..ciphertext.len()).step_by(16) {
        // Push the 16-byte block into the vector after cloning it from the slice
        blocks.push(GenericArray::clone_from_slice(&ciphertext[x..x + 16])); // blocks of 4x4
    }

    let cipher = Aes128::new(&key_generic); // initialize cipher
    cipher.decrypt_blocks(&mut blocks); // decrypt each ECB block

    // Flatten result
    let result: Vec<u8> = {
        let mut decrypted_bytes = Vec::new();
        for block in &blocks {
            for &byte in block.iter() {
                decrypted_bytes.push(byte);
            }
        }
        decrypted_bytes
    };

    result
}
