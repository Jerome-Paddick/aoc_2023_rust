use std::fs::{File, read};
use std::io::{BufReader, Lines};
use std::io::BufRead;
use std::path::Path;


pub(crate) fn get_file(filename: &str) -> BufReader<File> {
    // Open the file
    let path = Path::new(&filename);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            panic!("Error opening file: {}", e);
            // return;
        }
    };
    let reader = BufReader::new(file);
    reader
}

pub(crate) fn get_file_lines(filename: &str) -> Lines<BufReader<File>> {
    get_file(filename).lines()
}

pub(crate) fn get_file_buffer(filename: &str) -> BufReader<File> {
    // Open the file
    let path = Path::new(&filename);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            panic!("Error opening file: {}", e);
            // return;
        }
    };
    return BufReader::new(file);
}


pub(crate) fn get_file_len(filename: &str) {
    // Open the file
    let path = Path::new(&filename);
    match read(&path) {
        Ok(bytes) => println!("File has been read! Number of bytes: {}", bytes.len()),
        Err(e) => {
            panic!("Error opening file: {}", e);
            // return;
        }
    }
}