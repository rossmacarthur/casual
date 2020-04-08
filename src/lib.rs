//! Easily get user input.
//!
//! Rust type inference is used to know what to return.
//! ```
//! let username: String = casual::prompt("Please enter your name: ").get();
//! ```
//!
//! [`FromStr`] is used to parse the input, so you can read any type that
//! implements [`FromStr`].
//!
//! ```
//! let age: u32 = casual::prompt("Please enter your age: ").get();
//! ```
//!
//! A convenience function `confirm` is provided for getting a yes or no answer.
//!
//! ```
//! if !casual::confirm("Are you sure you want to continue?") {
//!     panic!("Aborted!");
//! }
//! ```
//!
//! [`FromStr`]: http://doc.rust-lang.org/std/str/trait.FromStr.html

use std::{
    fmt::Display,
    io::{self, Write},
    str::FromStr,
};

fn read_line(prompt: &Option<String>) -> io::Result<String> {
    if let Some(prompt) = prompt {
        let mut stdout = io::stdout();
        stdout.write_all(prompt.as_bytes())?;
        stdout.flush()?;
    }
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    Ok(result)
}

/// An input builder.
#[derive(Debug)]
pub struct Input<T> {
    prompt: Option<String>,
    default: Option<T>,
}

impl<T> Default for Input<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Input<T> {
    /// Construct a new empty `Input`.
    pub fn new() -> Self {
        Self {
            prompt: None,
            default: None,
        }
    }

    /// Set the prompt to display before waiting for user input.
    pub fn prompt<S>(mut self, prompt: S) -> Self
    where
        S: Into<String>,
    {
        self.prompt = Some(prompt.into());
        self
    }

    /// Set the default value.
    ///
    /// If set, this will be returned in the event the user enters an empty
    /// input.
    pub fn default(mut self, default: T) -> Self {
        self.default = Some(default);
        self
    }
}

impl<T> Input<T>
where
    T: FromStr,
{
    fn try_get(self) -> io::Result<T>
    where
        <T as FromStr>::Err: Display,
    {
        Ok(loop {
            match read_line(&self.prompt)?.trim() {
                "" => match self.default {
                    Some(default) => break default,
                    None => continue,
                },
                raw => match raw.parse() {
                    Ok(result) => break result,
                    Err(err) => {
                        println!("Error: {}", err);
                        continue;
                    }
                },
            }
        })
    }

    /// Consumes the `Input` and reads the input from the user.
    ///
    /// This function uses [`FromStr`] to parse the input data.
    ///
    /// [`FromStr`]: http://doc.rust-lang.org/std/str/trait.FromStr.html
    pub fn get(self) -> T
    where
        <T as FromStr>::Err: Display,
    {
        self.try_get().unwrap()
    }
}

/// Returns a new empty `Input`.
///
/// # Examples
///
/// Read in something without any prompt.
///
/// ```
/// let data: String = input().get();
/// ```
pub fn input<T>() -> Input<T> {
    Input::new()
}

/// Returns an `Input` that prompts the user for input.
///
/// # Examples
///
/// Read in a simple string:
///
/// ```
/// let username: String = prompt("Please enter your name: ").get();
/// ```
///
/// Types that implement [`FromStr`] will be automatically parsed.
///
/// ```
/// let years = prompt("How many years have you been coding Rust: ").default(0).get();
/// ```
///
/// [`FromStr`]: http://doc.rust-lang.org/std/str/trait.FromStr.html
pub fn prompt<T, S>(prompt: S) -> Input<T>
where
    S: Into<String>,
{
    Input::new().prompt(prompt)
}

/// Prompts the user for confirmation (yes/no).
///
/// # Examples
///
/// ```
/// if confirm("Are you sure you want to continue?") {
///     // continue
/// } else {
///     panic!("Aborted!");
/// }
/// ```
pub fn confirm<S>(prompt: S) -> bool
where
    S: AsRef<str>,
{
    matches!(
        &*Input::new()
            .prompt(format!("{} [y/N] ", prompt.as_ref()))
            .default("no".to_string())
            .get()
            .to_lowercase(),
        "y" | "yes"
    )
}
