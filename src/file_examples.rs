// Source: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
use std::{
    fs,
    fs::File,
    io,
    io::Write,
    io::{ErrorKind, Read},
};

pub fn read_file_basic(path: &str) -> String {
    let file_result = File::open(path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(created_file) => created_file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error)
            }
        },
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_) => buffer,
        Err(e) => panic!("Problem reading the file '{path}', message {e}"),
    }
}

pub fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("File '{path}' doesn't exist, attempting to create.");
            File::create(path).expect("Unable to create file")
        } else {
            panic!("Failed to read file.");
        }
    });
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)
        .expect("Failed to read data");

    buffer
}

pub fn read_file_improved(path: &str) -> Result<String, io::Error> {
    let mut buffer = String::new();

    // '?' operator will return the Err value early if Result::Err(), or return the value<T> if Result::Ok<T>()
    File::open(path)?.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub fn read_to_string(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

pub fn write_file(path: &str, content: &str) {
    let file_result = File::create(path);

    match file_result {
        Ok(mut result) => match result.write_all(content.as_bytes()) {
            Ok(_) => println!("Content saved to file '{}'.", path),
            Err(error) => panic!(
                "Could not save file contents to '{}', message: {}",
                path,
                error.to_string()
            ),
        },
        Err(error) => panic!(
            "Could not open file '{}', message: {}",
            path,
            error.to_string()
        ),
    };
}
