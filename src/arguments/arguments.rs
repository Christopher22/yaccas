use std::mem::transmute;

use super::{Argument, Flag, Value, Command};

/// An enum of all arguments which may be processed by the parser.
pub enum Arguments {
    /// A specific flag.
    Flag(Flag),
    /// A specific value.
    Value(Value),
    /// A specific command.
    Command(Command)
}

impl Arguments {

    /// "Unwraps" the value of the enum without using "match".
    /// # Unsafe
    /// This function does not check the type of the enum at runtime.
    /// # Example
    /// ```
    /// use yaccas::arguments::{Value, Arguments};
    ///
    /// let argument : Value = Value::with_default::<u32, _>("42").expect("The answer for everything is 42!");
    /// let arguments : Arguments = argument.into();
    ///
    /// unsafe {
    ///     let unwrapped_pointer : &Value = arguments.unwrap::<Value>();
    ///     assert_eq!(unwrapped_pointer.get_value::<u32>(), Some(42));
    /// }
    /// ```
    pub unsafe fn unwrap<'a, T : Argument>(&'a self) -> &'a T {
        match self {
            &Arguments::Flag(ref flag) => transmute::<&'a Flag, &'a T>(flag),
            &Arguments::Value(ref value) => transmute::<&'a Value, &'a T>(value),
            &Arguments::Command(ref command) => transmute::<&'a Command, &'a T>(command)
        }
    }
}