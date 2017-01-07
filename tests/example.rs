#[macro_use]
extern crate yaccas;

use yaccas::arguments::{Argument, Command, Flag, Value};
use yaccas::parser::{Parser, Result};
use yaccas::scanner::Unix;

#[test]
fn example_with_callback() {
    let mut will_be_true_if_flag_is_set = false;
    let mut will_be_42_as_everytime = 0u32;

    { // It's time for some magic ...

        // There a three types of arguments
        let flag = Flag::default();
        let value = Value::new::<u32>();
        let command = Command::new(|| Some("A fancy name for abort"));

        // Registers the arguments to a parser.
        // All callbacks will only be executed if parsing was successful!
        let mut parser = Parser::default();

        parser.register(&["option", "o1", "o2"], Argument::with_callback(flag, | flag | {
            // Flags are options which may occur 0 - x times.
            will_be_true_if_flag_is_set = flag.is_activated();
        }));

        parser.register(&["value", "v"], Argument::with_callback(value, | value | {
            // Values are command line argument-value pairs of a specific type.
            will_be_42_as_everytime = value.get_value::<u32>().expect("The answer for everything is 42!");
        }));

        parser.register(&["abort"], Argument::with_callback(command, | _command | {
            // Commands may or may not abort the execution of parsing, i.e. for "help".
            // This callback is a fallback: It is only called if the process was not aborted! 
        }));

        // You may use parser.parse(default_scanner!()) to get real arguments.
        match parser.parse(Unix::new(&vec!["-o1", "-value" , "42"])) {
            Result::Success(_) => { /* ... */ },
            Result::Aborted("A fancy name for abort") => { /* ... */ },
            _ => { panic!("This example will be successful!") }
        }
    }

    assert!(will_be_true_if_flag_is_set);
    assert_eq!(will_be_42_as_everytime, 42);
}