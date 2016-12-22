use std::collections::HashMap;
use std::convert::Into;

use ::arguments::*;
use ::scanner::{Scanner, Token};

use super::{FreeArgumentSupport, Result};

/// The parser which parses the `Argument`s upon `Token`s provided by a `Scanner`.
pub struct Parser<'a> {
    /// Behavior on free arguments.
    pub free_arguments: FreeArgumentSupport,
    names: HashMap<&'a str, usize>,
    arguments: Vec<Argument<'a>>,
}

impl<'a> Parser<'a> {
    /// Registers an `Argument` with specific name(s) and a callback which is called after successful parsing.
    /// # Example
    /// ```
    /// use yaccas::arguments::{Argument, Flag};
    /// use yaccas::parser::{Parser, Result};
    /// use yaccas::scanner::Unix;
    ///
    /// let mut parser = Parser::default();
    /// let flag = Flag::default();
    ///
    /// parser.register(&["option", "o1", "o2"], Argument::with_callback(flag, | _flag | {
    ///     // Do something with the argument here
    /// }));
    ///
    /// assert_eq!(parser.parse(Unix::new(&["-option"])), Result::Success(Vec::new()));
    /// ```
    pub fn register<T: Into<Argument<'a>>>(&mut self, names: &[&'a str], argument: T) {
        self.arguments.push(argument.into());

        let index = self.arguments.len() - 1;

        for name in names {
            self.names.insert(name, index);
        }
    }

    /// Parses the `Tokens` provided by a `Scanner` and matches them with registered `Argument`s.
    pub fn parse<S: Scanner>(&mut self, scanner: S) -> Result {
        let mut expecting_value = None;
        let mut add_to_free_variables = false;
        let mut free_variables = Vec::new();

        for token in scanner {
            // Check if further arguments need to be parsed
            if add_to_free_variables {
                free_variables.push(token.into());
                continue;
            }

            // If a `Value` is expecting a value...
            if let Some(index) = expecting_value {
                match token {
                    // Fail if the value is skipped
                    Token::Bound(_) => {
                        return Result::InvalidValue;
                    }
                    // Try to set the value
                    Token::Free(value) => {
                        if let Argument::Value(ref mut argument, _) = self.arguments[index] {
                            if !argument.set_value(value) || !argument.has_value() {
                                return Result::InvalidValue;
                            }
                            expecting_value = None;
                        } else {
                            panic!("Argument index was invalid!");
                        }
                    }
                }
            } else {
                // Try to parse the argument or return it as a free argument
                let free_argument = if let Token::Bound(name) = token {
                    // Try to find the argument by name
                    let arg_index = self.names.get::<str>(&name).cloned();
                    // Process the argument if possible
                    match arg_index.map(|index| {
                        (index, self.arguments.get_mut(index).expect("Invalid index"))
                    }) {
                        Some((_, &mut Argument::Flag(ref mut flag, _))) => {
                            flag.activate();
                            None
                        }
                        Some((_, &mut Argument::Command(ref mut command, _))) => {
                            if let Some(abort_reason) = command.execute() {
                                return Result::Aborted(abort_reason);
                            }
                            None
                        }
                        Some((index, &mut Argument::Value(_, _))) => {
                            expecting_value = Some(index);
                            None
                        }
                        _ => Some(name),
                    }
                } else {
                    Some(token.into())
                };

                // Process the free argument if set
                if let Some(free_argument) = free_argument {
                    match self.free_arguments {
                        FreeArgumentSupport::None => {
                            return Result::InvalidArgument;
                        }
                        FreeArgumentSupport::AtTheEnd => {
                            if !add_to_free_variables {
                                add_to_free_variables = true;
                            }
                            free_variables.push(free_argument);
                        }
                        FreeArgumentSupport::Everywhere => {
                            free_variables.push(free_argument);
                        }
                    }
                }
            }
        }

        // Check if all arguments are sufficient
        if self.arguments.iter().any(|argument| {
            match argument {
                &Argument::Value(ref value, _) if !value.has_value() => true,
                _ => false,
            }
        }) {
            return Result::NotSufficient;
        }

        // Parsing was successful: Execute the callbacks.
        for argument in self.arguments.iter_mut() {
            argument.execute_callback();
        }

        Result::Success(free_variables)
    }
}

impl<'a> Default for Parser<'a> {
    fn default() -> Parser<'a> {
        Parser {
            free_arguments: FreeArgumentSupport::AtTheEnd,
            names: HashMap::new(),
            arguments: Vec::new(),
        }
    }
}
