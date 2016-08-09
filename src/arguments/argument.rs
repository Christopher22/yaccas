use std::convert::Into;

use super::Arguments;

/// A trait for structs which could be interpreted as arguments.
pub trait Argument: Into<Arguments> {

    /// Returns if the argument has a valid "value".
    fn has_value(&self) -> bool;
}