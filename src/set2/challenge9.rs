// Implement PKCS#7 padding

mod tests {
    use crate::utils::pkcs7::padding::{pkcs7_pad, pkcs7_unpad};

    use super::*;

    #[test]
    fn test_pad() {
        let input = "YELLOW SUBMARINE".as_bytes();
        let padded = pkcs7_pad(input, 20);
        println!("{:?}", String::from_utf8_lossy(&padded).to_string());
    }

    #[test]
    fn test_unpad() {
        let input = "YELLOW SUBMARINE".as_bytes();
        let padded = pkcs7_pad(input, 20);
        println!("Padded: {:?}", String::from_utf8_lossy(&padded).to_string());

        if let Ok(unpadded) = pkcs7_unpad(&padded) {
            println!(
                "Unpadded: {:?}",
                String::from_utf8_lossy(&unpadded).to_string()
            );
        }
    }
}
