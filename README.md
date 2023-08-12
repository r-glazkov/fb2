# FB2 Parser for Rust

This library deserializes books in FB2 format.
The library almost conforms to the [standard XSD schema](https://github.com/gribuser/fb2)
with a few exceptions:

- strings normalization is not verified nor performed, i.e. strings are deserialized as is
- binaries are not associated with the image references, e.g. `Rc<Binary>`
- xs:ID uniqueness is not verified
- minOccurs/maxOccurs are not verified, i.e. the parser doesn't verify whether there is at most 2 "output" elements
- xs:gYear timezone is ignored
- xs:date timezone is ignored

# Example

```rust
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("examples/books/churchill_trial.fb2").unwrap();
    let reader = BufReader::new(file);
    let book = fb2::parse(reader).unwrap();
    println!("{:#?}", book);
}
```

Try with:
```shell
cargo run --example parse_sample
```

# TODO

Plan for the next releases:

- migrate to quick-xml because:
  - it supports more encodings
  - we can use `serde` support to get rid of the handwritten parser
- fix the current library when, for example, non-standard, occur in the FB2 files (like the `prose_contemporary` genre)
