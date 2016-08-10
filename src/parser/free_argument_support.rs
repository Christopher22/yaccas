/// An enum which specifies the behavior of a parser on free (aka unknown) arguments.
/// # Example
/// ```
/// use yaccas::arguments::Value;
/// use yaccas::parser::{Parser, Result, FreeArgumentSupport};
/// use yaccas::scanner::Unix;
///
/// let mut parser = Parser::default();
/// parser.register(&["value"], Value::new::<u32>(), |_| {});
///
/// // Test: FreeArgumentSupport::None
/// parser.free_arguments = FreeArgumentSupport::None;
/// assert_eq!(parser.parse(Unix::new(&["-unknown"])), Result::InvalidArgument);
///
/// // Test: FreeArgumentSupport::AtTheEnd
/// parser.free_arguments = FreeArgumentSupport::AtTheEnd;
/// assert_eq!(parser.parse(Unix::new(&["-unknown", "-value", "2"])), Result::NotSufficient);
/// assert_eq!(parser.parse(Unix::new(&["-value", "2", "-unknown"])), Result::Success(vec!["unknown".to_owned()]));
///
/// // Test: FreeArgumentSupport::Everywhere
/// parser.free_arguments = FreeArgumentSupport::Everywhere;
/// assert_eq!(parser.parse(Unix::new(&["-unknown", "-value", "2"])), Result::Success(vec!["unknown".to_owned()]));
/// assert_eq!(parser.parse(Unix::new(&["-value", "2", "-unknown"])), Result::Success(vec!["unknown".to_owned()]));
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FreeArgumentSupport {
    /// No free arguments are supported.
    None,
    /// Free arguments are only supported at the end. After a free argument, following arguments are free, too.
    AtTheEnd,
    /// Free arguments are supported everywhere.
    Everywhere
}