use std::fs::File;
use std::io::{BufReader, Read};

use fb2::*;

pub fn compare(path: &str, expected: FictionBook) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();

    let book = parse(content.as_bytes());

    match book {
        Ok(it) => assert_eq!(it, expected),
        Err(e) => assert_eq!(Some(e), None),
    }
}
