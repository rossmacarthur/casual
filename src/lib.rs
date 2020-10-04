//! Simple crate for parsing user input.
//!
//! # Examples
//!
//! Rust type inference is used to know what to return.
//!
//! ```no_run
//! let username: String = casual::prompt("Please enter your name: ").get();
//! ```
//!
//! [`FromStr`] is used to parse the input, so you can read any type that
//! implements [`FromStr`].
//!
//! ```no_run
//! let age: u32 = casual::prompt("Please enter your age: ").get();
//! ```
//!
//! [`.matches()`] can be used to validate the input data.
//!
//! ```no_run
//! let age: u32 = casual::prompt("Please enter your age again: ")
//!     .matches(|x| *x < 120)
//!     .get();
//! ```
//!
//! A convenience function [`confirm`] is provided for getting a yes or no
//! answer.
//!
//! ```no_run
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

use std::fmt::{self, Debug, Display};
use std::io::{self, Write};
use std::str::FromStr;

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
    prefix: Option<String>,
    suffix: Option<String>,
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

impl<T: Debug> Debug for Input<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Input")
            .field("prefix", &self.prefix)
            .field("prompt", &self.prompt)
            .field("suffix", &self.suffix)
            .field("default", &self.default)
            .finish() // FIXME rust-lang/rust#67364:
                      // use .finish_non_exhaustive() when it's stabilized
    }
}

impl<T> Default for Input<T> {
    /// Construct a new empty `Input`.
    ///
    /// Identical to [`Input::new()`](struct.Input.html#method.new).
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Input<T> {
    /// Construct a new empty `Input`.
    ///
    /// Identical to [`Input::default()`](struct.Input.html#impl-Default).
    pub fn new() -> Self {
        Self {
            prefix: None,
            prompt: None,
            suffix: None,
            default: None,
            validator: None,
        }
    }

    /// Set the prompt to display before waiting for user input.
    pub fn prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    /// Set the prompt prefix.
    pub fn prefix<S: Into<String>>(mut self, prefix: S) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set the prompt suffix.
    pub fn suffix<S: Into<String>>(mut self, suffix: S) -> Self {
        self.suffix = Some(suffix.into());
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
    /// ```no_run
    /// # use casual::Input;
    /// let num: u32 = Input::new().matches(|x| *x != 10).get();
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
        let Self {
            prompt,
            prefix,
            suffix,
            default,
            validator,
        } = self;

        let prompt = prompt.map(move |prompt| {
            let mut p = String::new();
            if let Some(prefix) = prefix {
                p.push_str(&prefix);
            }
            p.push_str(&prompt);
            if let Some(suffix) = suffix {
                p.push_str(&suffix);
            }
            p
        });

        Ok(loop {
            match read_line(&prompt)?.trim() {
                "" => {
                    if let Some(default) = default {
                        break default;
                    } else {
                        continue;
                    }
                }
                raw => match raw.parse() {
                    Ok(result) => {
                        if let Some(validator) = &validator {
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
    /// ```no_run
    /// # use casual::Input;
    /// let num: u32 = Input::new().prompt("Enter a number: ").get();
    /// ```
    ///
    /// [`FromStr`]: http://doc.rust-lang.org/std/str/trait.FromStr.html
    pub fn get(self) -> T {
        self.try_get().unwrap()
    }

    /// Consumes the `Input` and checks the result.
    ///
    /// This function uses [`FromStr`] to parse the input data. The result is
    /// then fed to the given closure.
    ///
    /// ```no_run
    /// # use casual::Input;
    /// let is_confirmed = Input::new()
    ///     .prompt("Are you sure you want to continue? [yes/no] ")
    ///     .check(|s: String| s == "yes");
    /// ```
    ///
    /// [`FromStr`]: http://doc.rust-lang.org/std/str/trait.FromStr.html
    pub fn check<F>(self, check: F) -> bool
    where
        F: Fn(T) -> bool,
    {
        check(self.get())
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
/// ```no_run
/// # use casual::input;
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
/// ```no_run
/// # use casual::prompt;
/// let username: String = prompt("Please enter your name: ").get();
/// ```
///
/// Types that implement [`FromStr`] will be automatically parsed.
///
/// ```no_run
/// # use casual::prompt;
/// let years = prompt("How many years have you been coding Rust: ")
///     .default(0)
///     .get();
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
/// ```no_run
/// # use casual::confirm;
/// if confirm("Are you sure you want to continue?") {
///     // continue
/// } else {
///     panic!("Aborted!");
/// }
/// ```
pub fn confirm<S: Into<String>>(text: S) -> bool {
    prompt(text)
        .suffix(" [y/N] ")
        .default("n".to_string())
        .matches(|s| matches!(&*s.trim().to_lowercase(), "n" | "no" | "y" | "yes"))
        .check(|s| matches!(&*s.to_lowercase(), "y" | "yes"))
}
