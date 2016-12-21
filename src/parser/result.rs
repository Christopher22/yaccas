/// An enum of all possible results after parsing.
/// # Example
/// ```
/// use yaccas::arguments::{Command, Flag, Value};
/// use yaccas::parser::{Parser, FreeArgumentSupport, Result};
/// use yaccas::scanner::Unix;
///
/// let mut parser = Parser::default();
///
/// let flag = Flag::default();
/// let value = Value::new::<u32>();
/// let command = Command::new(|| Some("A fancy name for abort"));
///
/// parser.register(&["option", "o1", "o2"], flag);
/// parser.register(&["abort"], command);
/// parser.register(&["value", "v"], value);
///
/// assert_eq!(parser.parse(Unix::new(&["-value", "not_a_number"])), Result::InvalidValue);
/// assert_eq!(parser.parse(Unix::new(&[""])), Result::NotSufficient);
/// assert_eq!(parser.parse(Unix::new(&["-value", "1", "free_variable"])), Result::Success(vec!["free_variable".to_owned()]));
/// assert_eq!(parser.parse(Unix::new(&["-abort"])), Result::Aborted("A fancy name for abort"));
///
/// parser.free_arguments = FreeArgumentSupport::None;
/// assert_eq!(parser.parse(Unix::new(&["-not_registered"])), Result::InvalidArgument);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum Result {
    /// Parsing was successful, a list of free arguments is returned.
    Success(Vec<String>),
    /// A registered `Command` aborted the parsing.
    Aborted(&'static str),
    /// A registered `Value` got neither a value from Scanner nor a default value.
    NotSufficient,
    /// An unknown name appears which could not be parsed as free argument.
    InvalidArgument,
    /// A registered `Value` got a value not matching its type and no default value was set.
    InvalidValue,
}
