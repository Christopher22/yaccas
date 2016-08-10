# Yaccas [![Build Status](https://travis-ci.org/Christopher22/yaccas.svg?branch=master)](https://travis-ci.org/Christopher22/yaccas)
**Y**et **A**nother **C**allback-orientated **C**ommand line p**A**r**S**er is ... well, yet another command line parser.

## Features or "Yet another?! You are not that creative, are you?!"
Indeed, there are so many command line parser written in Rust out there... But would I have written this one if it is like all the other? 
Let me convince you: Why should you choose this one?

- Smoothly to integrate into existing projects
- Extreme lightweight, easy & fast
    - Zero dependencies: Only pure Rust!
- No handler, references or other bullshit: Just modern callbacks!
- Ready for every system: Accept the syntax of UNIX exactly like that one of WINDOWS.
- Completely documented, many (doc-)tests to check correctness
    
## Documentation
[Documentation on GitHub](https://christopher22.github.io/yaccas/yaccas/)

## Example
```Rust
#[macro_use]
extern crate yaccas;

use yaccas::arguments::{Command, Flag, Value};
use yaccas::parser::{Parser, FreeArgumentSupport, Result};

fn main() {
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
        
        parser.register(&["option", "o1", "o2"], flag, | flag | {
            // Flags are options which may occur 0 - x times.
            will_be_true_if_flag_is_set = flag.is_activated();
        });
        
        parser.register(&["value", "v"], value, | value | {
            // Values are command line argument-value pairs of a specific type.
            will_be_42_as_everytime = value.get_value::<u32>().expect("The answer for everything is 42!");
        });

        parser.register(&["abort"], command, | _command | {
            // Commands may or may not abort the execution of parsing, i.e. for "help".
            // This callback is a fallback: It is only called if the process was not aborted! 
        });
        
        match parser.parse(default_scanner!()) {
            Result::Success(free_arguments) => { /* ... */ },
            Result::Aborted("A fancy name for abort") => { /* ... */ },
            _ => { /* ... */ }
        }
    }
    
    // Do something with "will_be_true_if_flag_is_set" or "will_be_42_as_everytime" here ...
}
```

##Author
Christopher Gundler (<c.gundler@mail.de>)

##License
Licensed under either of
 * Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (http://opensource.org/licenses/MIT)
 
at your option.

##Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.