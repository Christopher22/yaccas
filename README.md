# Yaccas
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
    let mut will_be_true = false;
    let mut will_be_42_as_everytime = 0u32;
    
    {
        let flag = Flag::default();
        let value = Value::new::<u32>();
        let command = Command::new(|| Some("A fancy name for abort"));
        
        let mut parser = Parser::default();

        parser.register(&["option", "o1", "o2"], flag, | flag | {
            // Check if the flag was set. All callbacks will only be executed if parsing was successfull!
            will_be_true = true;
        });
        
        parser.register(&["value", "v"], value, | value | {
            // Process the value or assign it somewhere
            will_be_42_as_everytime = value.get_value::<u32>().expect("The answer for everything is 42!");
        });

        parser.register(&["abort"], command, | _command | {
            // This will be executed if the command was not called or does not abort the execution.
        });
        
        match parser.parse(default_scanner!()) {
            Result::Success(free_arguments) => { /* ... */ },
            Result::Aborted("A fancy name for abort") => { /* ... */ },
            _ => { /* ... */ }
        }
    }
    
    // Do something with "will_be_true" or "will_be_42_as_everytime" here ...
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