use fb2::FictionBook;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("examples/books/churchill_trial.fb2").unwrap();
    let reader = BufReader::new(file);
    let book: FictionBook = quick_xml::de::from_reader(reader).unwrap();
    println!("{:#?}", book);
}
