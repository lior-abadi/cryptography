// Decipher AES encrypted file
use base64;

fn read_input() -> io::Result<String> {
    let current_dir = env::current_dir()?.join(Path::new("src/set1"));
    let text_input_dir = Path::new("challenge6_input.txt");

    let absolute_path = current_dir.join(text_input_dir);
    // Attempt to open the file and handle any errors
    let file = match File::open(&absolute_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Err(e);
        }
    };

    // Read lines and join them into a single string
    let contents = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()? // Collect all lines into a Vec<String>
        .join(""); // Join all lines

    Ok(contents)
}
