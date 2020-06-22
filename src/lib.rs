//! Simple crate for parsing user input.
//!
//! # Examples
//!
//! Rust type inference is used to know what to return.
//!
//! ```ignore
//! let username: String = casual::prompt("Please enter your name: ").get();
//! ```
//!
//! [`FromStr`] is used to parse the input, so you can read any type that
//! implements [`FromStr`].
//!
//! ```ignore
//! let age: u32 = casual::prompt("Please enter your age: ").get();
//! ```
//!
//! [`.matches()`] can be used to validate the input data.
//!
//! ```ignore
//! let age: u32 = casual::prompt("Please enter your age again: ").matches(|x| *x < 120).get();
//! ```
//!
//! A convenience function [`confirm`] is provided for getting a yes or no
//! answer.
//!
//! ```ignore
//! if casual::confirm("Are you sure you want to continue?") {
//!     // continue
//! } else {
//!     panic!("Aborted!");
//! }
//! ```
//!
//! [`FromStr`]: http://doc.rust-lang.org/std/str/trait.FromStr.html
//! [`.matches()`]: struct.Input.html#method.matches
//! [`confirm`]: fn.confirm.html

use std::{
    fmt::{self, Debug, Display},
    io::{self, Write},
    str::FromStr,
};

/////////////////////////////////////////////////////////////////////////
// Definitions
/////////////////////////////////////////////////////////////////////////

/// A validator for user input.
struct Validator<T> {
    raw: Box<dyn Fn(&T) -> bool + 'static>,
}

/// An input builder.
pub struct Input<T> {
    prompt: Option<String>,
    default: Option<T>,
    validator: Option<Validator<T>>,
}

/////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////

impl<T> Validator<T> {
    /// Construct a new `Validator`.
    fn new<F>(raw: F) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        Self { raw: Box::new(raw) }
    }

    /// Run the validator on the given input.
    fn run(&self, input: &T) -> bool {
        (self.raw)(input)
    }
}

impl<T> Default for Input<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Debug for Input<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Input")
            .field("prompt", &self.prompt)
            .field("default", &self.default)
            .finish() // FIXME: use .finish_non_exhaustive() when it's
                      // stabilized
    }
}

impl<T> Input<T> {
    /// Construct a new empty `Input`.
    pub fn new() -> Self {
        Self {
            prompt: None,
            default: None,
            validator: None,
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

    /// Check input values.
    ///
    /// If set, this function will be called on the parsed user input and only
    /// if it passes will we return the value.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let num: u32 = Input::new().matches(|x| *x != 10).get()
    /// ```
    pub fn matches<F>(mut self, matches: F) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.validator = Some(Validator::new(matches));
        self
    }
}

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

impl<T> Input<T>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    fn try_get_with<F>(self, read_line: F) -> io::Result<T>
    where
        F: Fn(&Option<String>) -> io::Result<String>,
    {
        Ok(loop {
            match read_line(&self.prompt)?.trim() {
                "" => {
                    if let Some(default) = self.default {
                        break default;
                    } else {
                        continue;
                    }
                }
                raw => match raw.parse() {
                    Ok(result) => {
                        if let Some(validator) = &self.validator {
                            if !validator.run(&result) {
                                println!("Error: invalid input");
                                continue;
                            }
                        }
                        break result;
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                        continue;
                    }
                },
            }
        })
    }

    fn try_get(self) -> io::Result<T> {
        self.try_get_with(read_line)
    }

    /// Consumes the `Input` and reads the input from the user.
    ///
    /// This function uses [`FromStr`] to parse the input data.
    ///
    /// [`FromStr`]: http://doc.rust-lang.org/std/str/trait.FromStr.html
    pub fn get(self) -> T {
        self.try_get().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////////
// Shortcut functions
/////////////////////////////////////////////////////////////////////////

/// Returns a new empty `Input`.
///
/// # Examples
///
/// Read in something without any prompt.
///
/// ```ignore
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
/// ```ignore
/// let username: String = prompt("Please enter your name: ").get();
/// ```
///
/// Types that implement [`FromStr`] will be automatically parsed.
///
/// ```ignore
/// let years = prompt("How many years have you been coding Rust: ").default(0).get();
/// ```
///
/// [`FromStr`]: http://doc.rust-lang.org/std/str/trait.FromStr.html
pub fn prompt<S, T>(text: S) -> Input<T>
where
    S: Into<String>,
{
    Input::new().prompt(text)
}

/// Prompts the user for confirmation (yes/no).
///
/// # Examples
///
/// ```ignore
/// if confirm("Are you sure you want to continue?") {
///     // continue
/// } else {
///     panic!("Aborted!");
/// }
/// ```
pub fn confirm<S>(text: S) -> bool
where
    S: AsRef<str>,
{
    let result = prompt(format!("{} [y/N] ", text.as_ref()))
        .default("n".to_string())
        .matches(|s| matches!(&*s.trim().to_lowercase(), "n" | "no" | "y" | "yes"))
        .get();
    matches!(&*result.to_lowercase(), "y" | "yes")
}
