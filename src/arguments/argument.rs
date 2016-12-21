use super::{Parsable, Flag, Value, Command, Metadata};
use std::mem::transmute;

/// An enum of all arguments which may be processed by the parser.
pub enum Argument<'a> {
    /// A specific flag.
    Flag(Flag, Option<Metadata<'a, Flag>>),
    /// A specific value.
    Value(Value, Option<Metadata<'a, Value>>),
    /// A specific command.
    Command(Command, Option<Metadata<'a, Command>>),
}

impl<'a> Argument<'a> {
    /// Extends an Argument with a callback.
    /// # Example
    /// ```
    /// use yaccas::arguments::{Argument, Flag};
    /// use yaccas::parser::{Parser, Result};
    /// use yaccas::scanner::Unix;
    ///
    /// let mut parser = Parser::default();
    /// parser.register(&["option", "o1", "o2"], Argument::with_callback(
    ///     Flag::default(),
    ///     | _flag: &Flag | {
    ///         // Do something with the argument here
    /// }));
    /// ```
    pub fn with_callback<T, C>(argument: T, callback: C) -> Argument<'a>
        where T: Parsable,
              C: FnMut(&T) -> () + 'a
    {
        Argument::with_metadata(argument, Metadata::<T>::default().set_callback(callback))
    }

    /// Extends an Argument with additional Metadata.
    pub fn with_metadata<T: Parsable>(argument: T, metadata: Metadata<T>) -> Argument {
        match argument.into() {
            Argument::Flag(flag, _) => {
                Argument::Flag(flag,
                               Some(unsafe { transmute::<Metadata<T>, Metadata<Flag>>(metadata) }))
            }
            Argument::Value(value, _) => {
                Argument::Value(value,
                                Some(unsafe {
                                    transmute::<Metadata<T>, Metadata<Value>>(metadata)
                                }))
            }
            Argument::Command(command, _) => {
                Argument::Command(command,
                                  Some(unsafe {
                                      transmute::<Metadata<T>, Metadata<Command>>(metadata)
                                  }))
            }
        }
    }

    /// Executes the callback, if it is set in the metadata.
    pub fn execute_callback(&mut self) -> bool {
        match self {
            &mut Argument::Flag(ref flag, Some(ref mut meta)) => {
                if let Some(ref mut callback) = meta.callback {
                    callback(&flag);
                    true
                } else {
                    false
                }
            }
            &mut Argument::Value(ref value, Some(ref mut meta)) => {
                if let Some(ref mut callback) = meta.callback {
                    callback(&value);
                    true
                } else {
                    false
                }
            }
            &mut Argument::Command(ref command, Some(ref mut meta)) => {
                if let Some(ref mut callback) = meta.callback {
                    callback(&command);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
