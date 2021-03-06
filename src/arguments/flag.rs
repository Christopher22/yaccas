use std::convert::From;
use std::default::Default;

use super::{Parsable, Argument};

/// An argument which represents an option which may occur 0 - x times.
/// # Example
/// ```
/// use yaccas::arguments::{Argument, Flag};
/// use yaccas::parser::{Parser, Result};
/// use yaccas::scanner::Unix;
///
/// // This option will be modified by the flag
/// let mut will_be_true = false;
///
/// {
///     let mut parser = Parser::default();
///     let flag = Flag::default();
///
///     parser.register(&["option", "o"], Argument::with_callback(flag, | flag | {
///         // This will only be executed if the parsing was successful.
///         will_be_true = flag.is_activated();
///     }));
///
///     assert_eq!(parser.parse(Unix::new(&["-option"])), Result::Success(Vec::new()));
/// }
///
/// assert_eq!(will_be_true, true);
/// ```
pub struct Flag(u32);

impl Flag {
    /// Activates the flag and increments the counter of matches by 1.
    pub fn activate(&mut self) {
        self.0 += 1;
    }

    /// Checks if the flag is set.
    pub fn is_activated(&self) -> bool {
        self.0 > 0
    }

    /// Returns how many times the flag was set.
    /// # Example
    /// ```
    /// use yaccas::arguments::{Argument, Flag};
    /// use yaccas::parser::{Parser, Result};
    /// use yaccas::scanner::Unix;
    ///
    /// let mut parser = Parser::default();
    /// let flag = Flag::default();
    ///
    /// parser.register(&["option", "o"], Argument::with_callback(flag, | flag | {
    ///     assert_eq!(flag.get_matches(), 2u32);
    /// }));
    ///
    /// assert_eq!(parser.parse(Unix::new(&["-option", "-o"])), Result::Success(Vec::new()));
    /// ```
    pub fn get_matches(&self) -> u32 {
        self.0
    }
}

impl Default for Flag {
    fn default() -> Flag {
        Flag(0)
    }
}

impl Parsable for Flag {}

impl From<Flag> for Argument<'static> {
    fn from(value: Flag) -> Argument<'static> {
        Argument::Flag(value, None)
    }
}
