# casual

[![Crates.io Version](https://img.shields.io/crates/v/casual.svg)](https://crates.io/crates/casual)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/casual)
[![Build Status](https://img.shields.io/github/workflow/status/rossmacarthur/casual/build/master)](https://github.com/rossmacarthur/casual/actions?query=workflow%3Abuild)

Simple crate for parsing user input.

## Getting started

Add the following dependency to your `Cargo.toml`.

```toml
[dependencies]
casual = "0.1"
```

## Usage

Rust type inference is used to know what to return.

```rust
let username: String = casual::prompt("Please enter your name: ").get();
```

[`FromStr`] is used to parse the input, so you can read any type that implements
[`FromStr`].

```rust
let age: u32 = casual::prompt("Please enter your age: ").get();
```

[`.matches()`] can be used to validate the input data.

```rust
let age: u32 = casual::prompt("Please enter your age again: ").matches(|x| *x < 120).get();
```

A convenience function [`confirm`] is provided for getting a yes or no answer.

```rust
if casual::confirm("Are you sure you want to continue?") {
    // continue
} else {
    panic!("Aborted!");
}
```

[`FromStr`]: http://doc.rust-lang.org/std/str/trait.FromStr.html
[`.matches()`]: https://docs.rs/casual/0.1/casual/struct.Input.html#method.matches
[`confirm`]: https://docs.rs/casual/0.1/casual/fn.confirm.html

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
