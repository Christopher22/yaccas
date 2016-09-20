use std::vec::IntoIter;
use std::iter::Iterator;
use std::env::{Args, args};

use super::{Scanner, Token};

/// This scanner scans (case sensitive) according to the command line syntax of UNIX.
/// # Example
/// ```
/// use yaccas::scanner::{Token, Unix};
/// use std::env::Args;
///
/// // Case 1: Use the arguments from the command line.
/// let arguments_from_commandline = Unix::<Args>::default();
///
/// // Case 2: Use own input (i.e. for debugging).
/// let mut unix = Unix::new(&vec!["-flag", "--Flag", "-argument" , "value" , "command"]);
///
/// assert_eq!(unix.next(), Some(Token::Bound("flag".to_string())));
/// assert_eq!(unix.next(), Some(Token::Bound("Flag".to_string())));
/// assert_eq!(unix.next(), Some(Token::Bound("argument".to_string())));
/// assert_eq!(unix.next(), Some(Token::Free("value".to_string())));
/// assert_eq!(unix.next(), Some(Token::Free("command".to_string())));
/// assert_eq!(unix.next(), None);
/// ```
pub struct Unix<T: Iterator<Item = String> = Args> {
    args: T,
}

impl<T: Iterator<Item = String>> Iterator for Unix<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.args.next().map(|argument| {

            {
                let trimmed = argument.trim_left_matches('-');
                if trimmed.len() != argument.len() {
                    return Token::Bound(trimmed.to_string());
                }
            }

            Token::Free(argument)
        })
    }
}

impl Default for Unix<Args> {
    fn default() -> Unix<Args> {
        let mut args = args();
        args.next().expect("Args invalid!");
        Unix { args: args }
    }
}

impl Default for Unix<IntoIter<String>> {
    fn default() -> Unix<IntoIter<String>> {
        Unix { args: Vec::new().into_iter() }
    }
}

impl Unix<IntoIter<String>> {
    /// Creates a new scanner for debugging.
    /// # Hint
    /// To create a scanner processing the command line arguments instead use `Unix::<Args>::default()`.
    pub fn new<'a>(arguments: &[&'a str]) -> Unix<IntoIter<String>> {
        let strings: Vec<String> = arguments.iter().map(|x| x.to_string()).collect();
        Unix { args: strings.into_iter() }
    }
}

impl<T: Iterator<Item = String>> Scanner for Unix<T> where Unix<T>: Default {}
