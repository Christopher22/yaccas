//! This module contains the parser and supporting enums.

mod parser;
mod free_argument_support;
mod result;

pub use self::parser::Parser;
pub use self::free_argument_support::FreeArgumentSupport;
pub use self::result::Result;
