# FB2 for Rust

This library is a set of models that enable [quick-xml](https://crates.io/crates/quick-xml) to deserialize an FB2 file in a structure way.
The library almost conforms to the [standard XSD schema](https://github.com/gribuser/fb2)
with a few exceptions:

- unstructured date value is not required because it is sometimes absent in real books e.g. `<date></date>`
- validation of strings is not performed besides structured dates and language tags
- binaries are not associated with the image references, e.g. `Rc<Binary>`
- xs:ID uniqueness is not verified
- minOccurs/maxOccurs are not verified, i.e. the parser doesn't verify whether there is at most 2 "output" elements
- xs:gYear with a timezone fails deserialization
- xs:date with a timezone fails deserialization
- XML sequence is not always enforced

# Example

```rust
use fb2::FictionBook;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("examples/books/churchill_trial.fb2").unwrap();
    let reader = BufReader::new(file);
    let book: FictionBook = quick_xml::de::from_reader(reader).unwrap();
    println!("{:#?}", book);
}
```

Try with:

```shell
cargo run --example parse_sample
```

# How to deserialize windows-1251 (or other encoding)

Enable the `encoding` feature of quick-xml:

```toml
quick-xml = { version = "<version>", features = ["encoding", "serialize"] }
```

Then, deserialize as usual.
