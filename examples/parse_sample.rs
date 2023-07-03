use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("examples/books/churchill_trial.fb2").unwrap();
    let reader = BufReader::new(file);
    let book = fb2::parse(reader).unwrap();
    println!("{:#?}", book);
}
