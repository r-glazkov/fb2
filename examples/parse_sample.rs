use std::fs::File;

fn main() {
    let file = File::open("examples/books/churchill_trial.fb2").unwrap();
    let book = fb2::parse(file).unwrap();
    println!("{:#?}", book);
}

