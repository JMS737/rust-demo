// Source: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
use std::{fs::File, io::Read, io::Write};

pub fn read_file(path: &str) {
    let file_result = File::open(path);

    match file_result {
        Ok(mut result) => {
            let mut buffer = String::new();
            match result.read_to_string(&mut buffer) {
                Ok(_) => println!("{buffer}"),
                Err(error) => println!(
                    "Could not read contents of file '{}', message: {}",
                    path,
                    error.to_string()
                ),
            }
        }
        Err(error) => println!(
            "Could not open file '{}', message: {}",
            path,
            error.to_string()
        ),
    }
}

pub fn write_file(path: &str, content: &str) {
    let file_result = File::create(path);

    match file_result {
        Ok(mut result) => match result.write_all(content.as_bytes()) {
            Ok(_) => println!("Content saved to file '{}'.", path),
            Err(error) => println!(
                "Could not save file contents to '{}', message: {}",
                path,
                error.to_string()
            ),
        },
        Err(error) => println!(
            "Could not open file '{}', message: {}",
            path,
            error.to_string()
        ),
    };
}
