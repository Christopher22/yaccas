use std::collections::HashMap;
use std::marker::PhantomData;
use std::boxed::Box;

use ::arguments::*;
use ::scanner::{Scanner, Token};

use super::callback::{Callback, Executable};
use super::{FreeArgumentSupport, Result};

/// The parser which parses the `Argument`s upon `Token`s provided by a `Scanner`.
pub struct Parser<'a> {
    /// Behavior on free arguments.
    pub free_arguments: FreeArgumentSupport,
    names: HashMap<&'a str, usize>,
    arguments: Vec<Arguments>,
    callbacks: Vec<Box<Executable + 'a>>,
}

impl<'a> Parser<'a> {
    /// Registers an `Argument` with specific name(s) and a callback which is called after successful parsing.
    /// # Example
    /// ```
    /// use yaccas::arguments::Flag;
    /// use yaccas::parser::{Parser, Result};
    /// use yaccas::scanner::Unix;
    ///
    /// let mut parser = Parser::default();
    /// let flag = Flag::default();
    ///
    /// parser.register(&["option", "o1", "o2"], flag, | _flag | {
    ///     // Do something with the argument here
    /// });
    ///
    /// assert_eq!(parser.parse(Unix::new(&["-option"])), Result::Success(Vec::new()));
    /// ```
    pub fn register<T: Argument + 'a, F: FnMut(&T) -> () + 'a>(&mut self,
                                                               names: &[&'a str],
                                                               argument: T,
                                                               handle: F) {
        self.arguments.push(argument.into());
        self.callbacks.push(Box::new(Callback::<T, F> {
            phantom: PhantomData,
            callback: handle,
        }));

        let index = self.arguments.len() - 1;

        for name in names {
            self.names.insert(name, index);
        }
    }

    /// Parses the `Tokens` provided by a `Scanner` and matches them with registered `Argument`s.
    pub fn parse<S: Scanner>(&mut self, scanner: S) -> Result {

        let mut current_argument: Option<usize> = None;
        let mut add_to_free_variables: bool = false;
        let mut free_variables: Vec<String> = Vec::new();

        for token in scanner {

            // Check if further arguments need to be parsed
            if add_to_free_variables {
                match token {
                    Token::Bound(free) |
                    Token::Free(free) => {
                        free_variables.push(free);
                    }
                }
                continue;
            }

            match (token, current_argument) {
                (Token::Bound(name), None) => {
                    // If an argument was found

                    if let Some(&index) = self.names.get::<str>(&name) {

                        // Find argument by ID
                        match self.arguments.get_mut(index) {
                            Some(&mut Arguments::Flag(ref mut flag)) => {
                                flag.activate();
                            }
                            Some(&mut Arguments::Command(ref mut command)) => {
                                if let Some(abort_reason) = command.execute() {
                                    return Result::Aborted(abort_reason);
                                }
                            }
                            Some(&mut Arguments::Value(_)) => {
                                current_argument = Some(index);
                            }
                            None => panic!("Invalid index!"),
                        }
                    } else if let FreeArgumentSupport::AtTheEnd = self.free_arguments {
                        add_to_free_variables = true;
                        free_variables.push(name);
                    } else if let FreeArgumentSupport::Everywhere = self.free_arguments {
                        free_variables.push(name);
                    } else {
                        return Result::InvalidArgument;
                    }
                }
                (Token::Bound(_), Some(_)) => {
                    // If an argument was found rather than a value
                    return Result::InvalidValue;
                }
                (Token::Free(value), Some(argument_index)) => {
                    // If a value was found
                    if let Arguments::Value(ref mut value_target) = self.arguments[argument_index] {
                        if !value_target.set_value(value) || !value_target.has_value() {
                            return Result::InvalidValue;
                        }
                        current_argument = None;
                    } else {
                        panic!("Argument index was invalid!");
                    }
                }
                (Token::Free(free_value), None) => {
                    // If a free variable was found
                    match self.free_arguments {
                        FreeArgumentSupport::None => {
                            return Result::InvalidArgument;
                        }
                        FreeArgumentSupport::AtTheEnd => {
                            if !add_to_free_variables {
                                add_to_free_variables = true;
                            }
                            free_variables.push(free_value);
                        }
                        FreeArgumentSupport::Everywhere => {
                            free_variables.push(free_value);
                        }
                    }
                }
            }
        }

        // Check if all arguments are sufficient
        for argument in self.arguments.iter() {
            if let &Arguments::Value(ref value) = argument {
                if !value.has_value() {
                    return Result::NotSufficient;
                }
            }
        }

        // Execute callbacks
        for (index, callback) in self.callbacks.iter_mut().enumerate() {
            callback.execute(&self.arguments[index])
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
            callbacks: Vec::new(),
        }
    }
}
