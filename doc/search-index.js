var searchIndex = {};
searchIndex["yaccas"] = {"doc":"# Yaccas\n**Y**et **A**nother **C**allback-orientated **C**ommand line pArSer is ... well, yet another command line parser.","items":[[0,"arguments","yaccas","This module contains the available types of arguments.",null,null],[3,"Flag","yaccas::arguments","An argument which represents an option which may occur 0 - x times.\n# Example\n```\nuse yaccas::arguments::{Argument, Flag};\nuse yaccas::parser::{Parser, Result};\nuse yaccas::scanner::Unix;",null,null],[3,"Value","","An argument which represents a value of a specific type.\n# Example\n```\nuse yaccas::arguments::{Argument, Value};\nuse yaccas::parser::{Parser, Result};\nuse yaccas::scanner::Unix;",null,null],[3,"Command","","An argument which may influence the parsing process.\n# Example\n```\nuse yaccas::arguments::{Argument, Command, AbortReason};\nuse yaccas::parser::{Parser, Result};\nuse yaccas::scanner::Unix;",null,null],[3,"Metadata","","The metadata of an argument.",null,null],[12,"callback","","The callback of an argument.",0,null],[4,"Argument","","An enum of all arguments which may be processed by the parser.",null,null],[13,"Flag","","A specific flag.",1,null],[13,"Value","","A specific value.",1,null],[13,"Command","","A specific command.",1,null],[11,"activate","","Activates the flag and increments the counter of matches by 1.",2,null],[11,"is_activated","","Checks if the flag is set.",2,null],[11,"get_matches","","Returns how many times the flag was set.\n# Example\n```\nuse yaccas::arguments::{Argument, Flag};\nuse yaccas::parser::{Parser, Result};\nuse yaccas::scanner::Unix;",2,null],[11,"default","","",2,{"inputs":[],"output":{"name":"flag"}}],[11,"from","","",1,{"inputs":[{"name":"flag"}],"output":{"name":"argument"}}],[11,"new","","Creates a new value of a specific type `T` which implements `FromStr`.\n# Example\n```\nuse yaccas::arguments::Value;",3,{"inputs":[],"output":{"name":"value"}}],[11,"with_default","","Creates a new value with a default value.\n# Example\n```\nuse yaccas::arguments::Value;",3,{"inputs":[{"name":"s"}],"output":{"name":"option"}}],[11,"set_value","","Set the value if it is valid.\n# Example\n```\nuse yaccas::arguments::Value;",3,null],[11,"get_value","","Returns the value from type `T` if possible.\n# Hint\nThe type `T` is checked at runtime to be identical which that from `new`.\n# Example\n```\nuse yaccas::arguments::Value;",3,null],[11,"has_value","","Returns if the argument has currently a value.",3,null],[11,"from","","",1,{"inputs":[{"name":"value"}],"output":{"name":"argument"}}],[11,"new","","Creates a new command.\n# Example\n```\nuse yaccas::arguments::Command;",4,{"inputs":[{"name":"f"}],"output":{"name":"command"}}],[11,"execute","","Executes the command.",4,null],[11,"deref","","",4,null],[11,"from","","",1,{"inputs":[{"name":"command"}],"output":{"name":"argument"}}],[11,"with_callback","","Extends an Argument with a callback.\n# Hint\nThis is a shortcut for `Argument::with_metadata`.\n# Example\n```\nuse yaccas::arguments::{Argument, Flag};\nuse yaccas::parser::{Parser, Result};\nuse yaccas::scanner::Unix;",1,{"inputs":[{"name":"t"},{"name":"c"}],"output":{"name":"argument"}}],[11,"with_metadata","","Extends an `Argument` with additional metadata.\n# Example\n```\nuse yaccas::arguments::{Argument, Metadata, Flag};\nuse yaccas::parser::{Parser, Result};\nuse yaccas::scanner::Unix;",1,{"inputs":[{"name":"t"},{"name":"metadata"}],"output":{"name":"argument"}}],[11,"execute_callback","","Executes the callback iff it is set in the metadata.",1,null],[11,"set_callback","","Sets a callback.\n```\nuse yaccas::arguments::{Flag, Metadata};\nlet mut meta : Metadata&lt;Flag&gt; = Metadata::default().set_callback(|_flag : &amp;Flag| {\n    // Do something in the callback\n});\n```",0,null],[11,"default","","",0,{"inputs":[],"output":{"name":"self"}}],[6,"AbortReason","","The result of a `Command`. The parsing is aborted if a reason is set, else it continues.\n# Example\n```\nuse yaccas::arguments::{Command, AbortReason};\nuse yaccas::parser::{Parser, Result};\nuse yaccas::scanner::Unix;",null,null],[8,"Parsable","","A trait for structs which could be interpreted as arguments.",null,null],[0,"parser","yaccas","This module contains the parser and supporting enums.",null,null],[3,"Parser","yaccas::parser","The parser which parses the `Argument`s upon `Token`s provided by a `Scanner`.",null,null],[12,"free_arguments","","Behavior on free arguments.",5,null],[4,"FreeArgumentSupport","","An enum which specifies the behavior of a parser on free (aka unknown) arguments.\n# Example\n```\nuse yaccas::arguments::Value;\nuse yaccas::parser::{Parser, Result, FreeArgumentSupport};\nuse yaccas::scanner::Unix;",null,null],[13,"None","","No free arguments are supported.",6,null],[13,"AtTheEnd","","Free arguments are only supported at the end. After a free argument, following arguments are free, too.",6,null],[13,"Everywhere","","Free arguments are supported everywhere.",6,null],[4,"Result","","An enum of all possible results after parsing.\n# Example\n```\nuse yaccas::arguments::{Command, Flag, Value};\nuse yaccas::parser::{Parser, FreeArgumentSupport, Result};\nuse yaccas::scanner::Unix;",null,null],[13,"Success","","Parsing was successful, a list of free arguments is returned.",7,null],[13,"Aborted","","A registered `Command` aborted the parsing.",7,null],[13,"NotSufficient","","A registered `Value` got neither a value from Scanner nor a default value.",7,null],[13,"InvalidArgument","","An unknown name appears which could not be parsed as free argument.",7,null],[13,"InvalidValue","","A registered `Value` got a value not matching its type and no default value was set.",7,null],[11,"register","","Registers an `Argument` with specific name(s) and a callback which is called after successful parsing.\n# Example\n```\nuse yaccas::arguments::{Argument, Flag};\nuse yaccas::parser::{Parser, Result};\nuse yaccas::scanner::Unix;",5,null],[11,"parse","","Parses the `Tokens` provided by a `Scanner` and matches them with registered `Argument`s.",5,null],[11,"default","","",5,{"inputs":[],"output":{"name":"parser"}}],[11,"fmt","","",6,null],[11,"eq","","",6,null],[11,"clone","","",6,null],[11,"fmt","","",7,null],[11,"eq","","",7,null],[11,"ne","","",7,null],[0,"scanner","yaccas","This module contains `Scanner` which maps command line arguments to `Token`s dependently from OS.",null,null],[3,"Unix","yaccas::scanner","This scanner scans (case sensitive) according to the command line syntax of UNIX.\n# Example\n```\nuse yaccas::scanner::{Token, Unix};\nuse std::env::Args;",null,null],[3,"Windows","","This scanner scans (case **in**sensitive) according to the command line syntax of Microsoft(R) Windows.\n# Example\n```\nuse yaccas::scanner::{Token, Windows};\nuse std::env::Args;",null,null],[4,"Token","","An enum about the valid types of strings as input.",null,null],[13,"Bound","","A string which represents an argument.",8,null],[13,"Free","","A string which represents a value or command.",8,null],[11,"fmt","","",8,null],[11,"eq","","",8,null],[11,"ne","","",8,null],[11,"next","","",9,null],[11,"default","","",9,{"inputs":[],"output":{"name":"unix"}}],[11,"default","","",9,{"inputs":[],"output":{"name":"unix"}}],[11,"new","","Creates a new scanner for debugging.\n# Hint\nTo create a scanner processing the command line arguments instead use `Unix::&lt;Args&gt;::default()`.",9,null],[11,"next","","",10,null],[11,"default","","",10,{"inputs":[],"output":{"name":"windows"}}],[11,"default","","",10,{"inputs":[],"output":{"name":"windows"}}],[11,"new","","Creates a new scanner for debugging.\n# Hint\nTo create a scanner processing the command line arguments instead use `Windows::&lt;Args&gt;::default()`.",10,null],[8,"Scanner","","A trait for structs which represents scanner.",null,null],[14,"default_scanner","yaccas","This macro returns the appropriated `Scanner` for the current operation system.\n# Example\n```\n#[macro_use]\nextern crate yaccas;\nuse yaccas::parser::{Result, Parser};",null,null]],"paths":[[3,"Metadata"],[4,"Argument"],[3,"Flag"],[3,"Value"],[3,"Command"],[3,"Parser"],[4,"FreeArgumentSupport"],[4,"Result"],[4,"Token"],[3,"Unix"],[3,"Windows"]]};
initSearch(searchIndex);
