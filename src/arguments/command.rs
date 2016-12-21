use std::ops::{Fn, Deref};

use super::{Flag, Parsable, Argument};

/// The result of a `Command`. The parsing is aborted if a reason is set, else it continues.
/// # Example
/// ```
/// use yaccas::arguments::{Command, AbortReason};
/// use yaccas::parser::{Parser, Result};
/// use yaccas::scanner::Unix;
///
/// let mut parser = Parser::default();
/// let command_abort = Command::new(|| Some("Aborted"));
/// let command_no_abort = Command::new(|| None);
///
/// parser.register(&["a"], command_abort);
/// parser.register(&["n"], command_no_abort);
///
/// assert_eq!(parser.parse(Unix::new(&["-a"])), Result::Aborted("Aborted"));
/// assert_eq!(parser.parse(Unix::new(&["-n"])), Result::Success(Vec::new()));
/// ```
pub type AbortReason = Option<&'static str>;

/// An argument which may influence the parsing process.
/// # Example
/// ```
/// use yaccas::arguments::{Argument, Command, AbortReason};
/// use yaccas::parser::{Parser, Result};
/// use yaccas::scanner::Unix;
///
/// let mut parser = Parser::default();
/// let command = Command::new(|| Some("help"));
///
/// parser.register(&["help", "h"], Argument::with_callback(command, | command | {
///     // Fallback: Command was not executed OR command has not abort the execution.
/// }));
///
/// assert_eq!(parser.parse(Unix::new(&["-help"])), Result::Aborted("help"));
/// ```
pub struct Command {
    command: Box<Fn() -> AbortReason>,
    was_executed: Flag,
}

impl Command {
    /// Creates a new command.
    /// # Example
    /// ```
    /// use yaccas::arguments::Command;
    ///
    /// let command = Command::new(|| {
    ///     // This code gets executed if the command is parsed.
    ///     Some("This will abort the parsing process!")
    /// });
    /// ```
    pub fn new<F: Fn() -> AbortReason + 'static>(on_execution: F) -> Command {
        Command {
            command: Box::new(on_execution),
            was_executed: Flag::default(),
        }
    }

    /// Executes the command.
    pub fn execute(&mut self) -> AbortReason {
        self.was_executed.activate();
        (self.command)()
    }
}

impl Deref for Command {
    type Target = Flag;
    fn deref(&self) -> &Flag {
        &self.was_executed
    }
}

impl Parsable for Command {}

impl From<Command> for Argument<'static> {
    fn from(command: Command) -> Argument<'static> {
        Argument::Command(command, None)
    }
}
