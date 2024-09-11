mod tests {
    use super::*;
    use crate::utils::fs::read_all_inputs_return_lines;
    use std::path::Path;

    #[test]
    fn test_run() {
        let data_path = Path::new("src/set1/challenge8_input.txt");
        let ciphertext_string_b64_vector = read_all_inputs_return_lines(data_path).unwrap();
    }
}
