use super::{Argument, Flag, Value, Command, Metadata};
use std::mem::transmute;

/// An enum of all arguments which may be processed by the parser.
pub enum Arguments<'a> {
    /// A specific flag.
    Flag(Flag, Option<Metadata<'a, Flag>>),
    /// A specific value.
    Value(Value, Option<Metadata<'a, Value>>),
    /// A specific command.
    Command(Command, Option<Metadata<'a, Command>>),
}

impl<'a> Arguments<'a> {
    /// Extends an Argument with a callback.
    /// # Example
    /// ```
    /// use yaccas::arguments::{Arguments, Flag};
    /// use yaccas::parser::{Parser, Result};
    /// use yaccas::scanner::Unix;
    ///
    /// let mut parser = Parser::default();
    /// parser.register(&["option", "o1", "o2"], Arguments::with_callback(
    ///     Flag::default(),
    ///     | _flag: &Flag | {
    ///         // Do something with the argument here
    /// }));
    /// ```
    pub fn with_callback<T, C>(argument: T, callback: C) -> Arguments<'a>
        where T: Argument,
              C: FnMut(&T) -> () + 'a
    {
        Arguments::with_metadata(argument, Metadata::<T>::default().set_callback(callback))
    }

    /// Extends an Argument with additional Metadata.
    pub fn with_metadata<T: Argument>(argument: T, metadata: Metadata<T>) -> Arguments {
        match argument.into() {
            Arguments::Flag(flag, _) => {
                Arguments::Flag(flag,
                                Some(unsafe { transmute::<Metadata<T>, Metadata<Flag>>(metadata) }))
            }
            Arguments::Value(value, _) => {
                Arguments::Value(value,
                                 Some(unsafe {
                                     transmute::<Metadata<T>, Metadata<Value>>(metadata)
                                 }))
            }
            Arguments::Command(command, _) => {
                Arguments::Command(command,
                                   Some(unsafe {
                                       transmute::<Metadata<T>, Metadata<Command>>(metadata)
                                   }))
            }
        }
    }

    /// Executes the callback, if it is set in the metadata.
    pub fn execute_callback(&mut self) -> bool {
        match self {
            &mut Arguments::Flag(ref flag, Some(ref mut meta)) => {
                if let Some(ref mut callback) = meta.callback {
                    callback(&flag);
                    true
                } else {
                    false
                }
            }
            &mut Arguments::Value(ref value, Some(ref mut meta)) => {
                if let Some(ref mut callback) = meta.callback {
                    callback(&value);
                    true
                } else {
                    false
                }
            }
            &mut Arguments::Command(ref command, Some(ref mut meta)) => {
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
