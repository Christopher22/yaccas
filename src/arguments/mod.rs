//! This module contains the available types of arguments.

mod flag;
mod value;
mod command;
mod argument;
mod metadata;

pub use self::argument::Argument;
pub use self::flag::Flag;
pub use self::value::Value;
pub use self::command::{AbortReason, Command};
pub use self::metadata::Metadata;

/// A trait for structs which could be interpreted as arguments.
pub trait Parsable: Into<Argument<'static>> {}
