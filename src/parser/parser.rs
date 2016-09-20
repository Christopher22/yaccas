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

        let mut former_value: Option<usize> = None;
        let mut add_to_free_variables: bool = false;
        let mut free_variables: Vec<String> = Vec::new();

        // Walk throught scanner
        for token in scanner {

            let argument = match token {
                Token::Bound(bound) => bound,
                Token::Free(free) => free,
            };

            // If all arguments should be processed as free arguments ...
            if add_to_free_variables {
                free_variables.push(argument);
            }
            // If a argument waits for its value ...
            else if let Some(value_id) = former_value {

                // If the value is not free ...
                // if let Token::Bound(_) = token {
                //    return Result::InvalidValue;
                // }

                unsafe {
                    if let &mut Arguments::Value(ref mut value) = self.arguments
                        .get_unchecked_mut(value_id) {
                        if !value.set_value(argument) || !value.has_value() {
                            return Result::InvalidValue;
                        }

                        // Reset the value
                        former_value = None;
                    } else {
                        panic!("Former value is empty!");
                    }
                }
            }
            // If there is an argument with the name ...
            else if let Some(index) = self.names.get::<str>(&argument) {

                // ... process it.
                unsafe {
                    match self.arguments.get_unchecked_mut(*index) {
                        &mut Arguments::Flag(ref mut flag) => {
                            flag.activate();
                        }
                        &mut Arguments::Command(ref mut command) => {
                            if let Some(abort_reason) = command.execute() {
                                return Result::Aborted(abort_reason);
                            }
                        }
                        &mut Arguments::Value(_) => {
                            former_value = Some(*index);
                        }
                    };
                }
            }
            // Else if there is no matching argument ...
            else {
                // TODO: MULTIPE FLAGS
                match self.free_arguments {
                    FreeArgumentSupport::None => {
                        return Result::InvalidArgument;
                    }
                    FreeArgumentSupport::AtTheEnd => {
                        add_to_free_variables = true;
                        free_variables.push(argument);
                    }
                    FreeArgumentSupport::Everywhere => {
                        free_variables.push(argument);
                    }
                }
            }
        }

        for argument in self.arguments.iter() {
            if let &Arguments::Value(ref value) = argument {
                if !value.has_value() {
                    return Result::NotSufficient;
                }
            }
        }

        for (index, callback) in self.callbacks.iter_mut().enumerate() {
            callback.execute(unsafe { self.arguments.get_unchecked(index) })
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
