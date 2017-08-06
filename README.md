![Tokesies](logo.png "Tokesies")

A string tokenizer library for Rust, where character filters are trait based and
can therefore carry state. Characters that separate tokens may also be
conditionally selected to be a token themselves.

There are filter implementations provided for a few basic use cases:

```rust
use tokesies::*;

let line = "hello!world, this is some_text";
let tokens = FilteredTokenizer::new(filters::DefaultFilter{}, line).collect::<Vec<Token>>();

// tokens: ["hello", "!", "world", ",", "this", "is", "some", "_", "text"]

assert_eq!(tokens.get(0).unwrap().term(), "hello");
```

You can alternatively provide a custom implementation:

```rust
use tokesies::*;

pub struct MyFilter;

impl filters::Filter for MyFilter {
    fn on_char(&self, c: &char) -> (bool, bool) {
        match *c {
            ' ' => (true, false),
            ',' => (true, true),
            _ => (false, false),
        }
    }
}

let line = "hello!world, this is some_text";
let tokens = FilteredTokenizer::new(MyFilter{}, line).collect::<Vec<Token>>();

// tokens: ["hello!world", ",", "this", "is", "some_text"]

assert_eq!(tokens.get(0).unwrap().term(), "hello!world");
```

Implementation is derived largely from [this blog][0] by [@daschl][1].

## Contributing and customizing

Contributions are very welcome, just fork and submit a pull request.

## Contact

Ashley Jeffs
* Web: [http://jeffs.eu](http://jeffs.eu)
* Twitter: [@Jeffail](https://twitter.com/Jeffail "@jeffail")
* Email: [ash@jeffs.eu](mailto:ash@jeffs.eu)

[0]: http://nitschinger.at/Text-Analysis-in-Rust-Tokenization/
[1]: https://github.com/daschl
