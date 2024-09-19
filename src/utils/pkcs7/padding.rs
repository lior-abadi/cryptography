pub fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let pad_len = block_size - (data.len() % block_size);
    let mut padded_data = Vec::from(data);
    padded_data.extend(vec![pad_len as u8; pad_len]);
    padded_data
}

pub fn pkcs7_unpad(data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.is_empty() {
        return Err("Data is empty");
    }
    let pad_len = *data.last().unwrap() as usize;
    if pad_len == 0 || pad_len > data.len() {
        return Err("Invalid padding");
    }
    for &byte in &data[data.len() - pad_len..] {
        if byte as usize != pad_len {
            return Err("Invalid padding");
        }
    }
    Ok(data[..data.len() - pad_len].to_vec())
}
