#[derive(Debug, PartialEq, Eq)]
/// An enum about the valid types of strings as input.
pub enum Token {
    /// A string which represents an argument.
    Bound(String),
    /// A string which represents a value or command.
    Free(String),
}
