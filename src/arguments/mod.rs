//! This module contains the available types of arguments.

mod argument;
mod flag;
mod value;
mod command;
mod arguments;
mod metadata;

pub use self::argument::Argument;
pub use self::arguments::Arguments;
pub use self::flag::Flag;
pub use self::value::Value;
pub use self::command::{AbortReason, Command};
pub use self::metadata::Metadata;