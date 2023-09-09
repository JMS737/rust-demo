// Source: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
use std::{fs::File, io::Read};

pub fn read_file(path: &str) {
    let file_result = File::open(path);

    match file_result {
        Ok(mut result) => {
            let mut buffer = String::new();
            match result.read_to_string(&mut buffer) {
                Ok(_) => println!("{buffer}"),
                Err(error) => println!("Could not read contents of file '{}', message: {}", path, error.to_string()),
            }
        }
        Err(error) => println!("Could not open file '{}', message: {}", path, error.to_string()),
    }
}
