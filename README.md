# Rust Regex Engine

## Introduction

This project is a custom regex engine implemented in Rust. It aims to provide fundamental regex functionalities with a focus on learning and understanding the inner workings of regex processing.

## Usage

To use this regex engine in your Rust project, include it as a module and use the `Regex` struct to compile and match regex patterns.

### Example

```rust
use rust_regex_engine::Regex;

fn main() {
    let regex = Regex::new("^[hH]ello,? [wW]orld ?!").unwrap();
    let res = regex.find("Hello, world!");
    println!("Does it match? {}", res.is_match());
}
```

## Features

- Basic regex pattern matching.
- Support for quantifiers like `*`, `+`, and `?`.
- Anchors for start (`^`) and end (`$`) of the string.
- Basic character classes.
- Non-capturing groups.
- (More features to be implemented)

## TODO

- [X] Implement `find()` function to return a `Match` structure, which includes:
  - `.matched` (bool): Indicates if the pattern was matched.
  - `.start` (usize): Starting index of the match.
  - `.end` (usize): Ending index of the match.
  - `range()`: Provides the range of indices for the matched substring.
- [X] Add support for character classes `[]`.
- [ ] Implement non-capturing groups `(?:abc)*`.
- [ ] Implement capturing groups `(abc)`.

## Tests

Basic tests are implemented to ensure the functionality of various regex patterns and quantifiers. Additional tests will be added along with the development of new features.
