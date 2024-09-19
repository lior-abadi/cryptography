use crate::utils::{
    aes_128_ecb::ecb::{ecb_decrypt, ecb_encrypt},
    pkcs7::padding::{pkcs7_pad, pkcs7_unpad},
    xor::xor_utilities::xor_bytes,
};

pub fn cbc_encrypt(plaintext: &[u8], key: &[u8], iv: &[u8], block_size: usize) -> Vec<u8> {
    let mut previous_block = iv.to_vec();
    let mut ciphertext = Vec::new();
    let padded_plaintext = pkcs7_pad(plaintext, block_size);

    for block in padded_plaintext.chunks(block_size) {
        let xored_to_encrypt = xor_bytes(&block.to_vec(), &previous_block);
        let encrypted_block = ecb_encrypt(&xored_to_encrypt, key);
        ciphertext.extend(&encrypted_block);
        previous_block = encrypted_block;
    }

    ciphertext
}

pub fn cbc_decrypt(
    key: &[u8],
    iv: &[u8],
    ciphertext: &[u8],
    block_size: usize,
) -> Result<Vec<u8>, &'static str> {
    let mut previous_cipherblock = iv.to_vec();
    let mut plaintext = Vec::new();

    for block in ciphertext.chunks(block_size) {
        let decrypted = ecb_decrypt(block, key);
        let xored = xor_bytes(&decrypted, &previous_cipherblock);
        plaintext.extend(xored);
        previous_cipherblock = block.to_vec();
    }

    return pkcs7_unpad(&plaintext);
}
