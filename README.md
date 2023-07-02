# FB2 Parser for Rust

This library deserializes books in FB2 format.
The library almost conforms to the [standard XSD schema](https://github.com/gribuser/fb2)
with a few exceptions:

- strings normalization is not verified nor performed, i.e. strings are deserialized as is
- binaries are not associated with the image references, e.g. Rc<Binary>
- xs:ID uniqueness is not verified
- minOccurs/maxOccurs are not verified, i.e. the parser doesn't verify whether there is at most 2 "output" elements
- xs:gYear timezone is ignored
- xs:date timezone is ignored

# Example

```rust
use std::fs::File;

fn main() {
    let file = File::open("examples/books/churchill_trial.fb2").unwrap();
    let book = fb2::parse(file).unwrap();
    println!("{:#?}", book);
}
```

Try with:
```shell
cargo run --example parse_sample
```

