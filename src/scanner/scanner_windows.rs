use std::vec::IntoIter;
use std::iter::Iterator;
use std::env::{Args, args};

use super::{Scanner, Token};

/// This scanner scans (case **in**sensitive) according to the command line syntax of Microsoft(R) Windows.
/// # Example
/// ```
/// use yaccas::scanner::{Token, Windows};
/// use std::env::Args;
///
/// // Case 1: Use the arguments from the command line.
/// let arguments_from_commandline = Windows::<Args>::default();
///
/// // Case 2: Use own input (i.e. for debugging).
/// let mut win = Windows::new(&vec!["/flag", "/Flag" , "value" , "command"]);
///
/// assert_eq!(win.next(), Some(Token::Bound("flag".to_string())));
/// assert_eq!(win.next(), Some(Token::Bound("flag".to_string()))); // Case insensitive!
/// assert_eq!(win.next(), Some(Token::Free("value".to_string())));
/// assert_eq!(win.next(), Some(Token::Free("command".to_string())));
/// assert_eq!(win.next(), None);
/// ```
pub struct Windows<T: Iterator<Item = String> = Args> {
    args: T,
}

impl<T: Iterator<Item = String>> Iterator for Windows<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.args.next().map(|argument| {

            {
                let trimmed = argument.trim_left_matches('/');
                if trimmed.len() != argument.len() {
                    return Token::Bound(trimmed.to_lowercase());
                }
            }

            Token::Free(argument.to_lowercase())
        })
    }
}

impl Default for Windows<Args> {
    fn default() -> Windows<Args> {
        let mut args = args();
        args.next().expect("Args invalid!");
        Windows { args: args }
    }
}

impl Default for Windows<IntoIter<String>> {
    fn default() -> Windows<IntoIter<String>> {
        Windows { args: Vec::new().into_iter() }
    }
}

impl Windows<IntoIter<String>> {
    /// Creates a new scanner for debugging.
    /// # Hint
    /// To create a scanner processing the command line arguments instead use `Windows::<Args>::default()`.
    pub fn new<'a>(arguments: &[&'a str]) -> Windows<IntoIter<String>> {
        let strings: Vec<String> = arguments.iter().map(|x| x.to_string()).collect();
        Windows { args: strings.into_iter() }
    }
}

impl<T: Iterator<Item = String>> Scanner for Windows<T> where Windows<T>: Default {}
