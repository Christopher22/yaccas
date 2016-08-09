use std::iter::Iterator;
use std::default::Default;

use super::Token;

/// A trait for structs which represents scanner.
pub trait Scanner : Iterator<Item=Token> + Default {}