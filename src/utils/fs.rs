use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_file(file_path: &Path) -> io::Result<File> {
    let absolute_path = env::current_dir()?.join(file_path);

    // Attempt to open the file and handle any errors
    let file = match File::open(&absolute_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Err(e);
        }
    };

    Ok(file)
}
pub fn read_all_inputs_return_lines(file_path: &Path) -> io::Result<Vec<String>> {
    match read_file(file_path) {
        Ok(file) => {
            let lines: Vec<String> = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;
            Ok(lines)
        }
        Err(e) => return Err(e),
    }
}

pub fn read_input_return_join_lines(file_path: &Path) -> io::Result<String> {
    match read_file(file_path) {
        Ok(file) => {
            // Read lines and join them into a single string
            let contents = io::BufReader::new(file)
                .lines()
                .collect::<Result<Vec<_>, _>>()? // Collect all lines into a Vec<String>
                .join(""); // Join all lines

            Ok(contents)
        }
        Err(e) => return Err(e),
    }
}
