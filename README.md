# casual

Simple crate for parsing user input.

## Usage

```rust
use casual::{confirm, prompt};

// Rust type inference is used to know what to return.
let username: String = prompt("Please enter your name: ").get();

// `FromStr` is used to parse the input, so you can read any type that
// implements `FromStr`.
let age: u32 = prompt("Please enter your age: ").get();

// `check(..)` can be used to validate the input data.
let age: u32 = prompt("Please enter your age again: ").check(|x| *x < 120).get();

// A convenience function `confirm` is provided for getting a yes or no answer.
if !confirm("Are you sure you want to continue?") {
    panic!("Aborted!");
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
