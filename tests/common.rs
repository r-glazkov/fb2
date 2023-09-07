use std::fs::File;
use std::io::{BufReader, Read};

use fb2::FictionBook;

pub fn compare(path: &str, expected: FictionBook) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();

    let book: FictionBook = quick_xml::de::from_str(&content).unwrap();

    assert_eq!(book, expected);
}
