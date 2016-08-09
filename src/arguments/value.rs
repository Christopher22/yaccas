use std::any::{Any, TypeId};
use std::str::FromStr;

use super::{Argument, Arguments};

/// An argument which represents a value of a specific type.
/// # Example
/// ```
/// use yaccas::arguments::Value;
/// use yaccas::parser::{Parser, Result};
/// use yaccas::scanner::Unix;
///
/// // This option will be modified by the value
/// let mut will_be_46 = 0u32;
///
/// {
///     let mut parser = Parser::default();
///     let value = Value::new::<u32>();
///
///     parser.register(&["val"], value, | value | {
///         will_be_46 = value.get_value::<u32>().expect("This will only be executed if the parsing was successful!");
///     });
///
///     assert_eq!(parser.parse(Unix::new(&["-val", "46"])), Result::Success(Vec::new()));
/// }
///
/// assert_eq!(will_be_46, 46u32);
/// ```
pub struct Value {
    value : Option<String>,
    validator : fn(&str) -> bool,
    type_id : TypeId
}

impl Value {

    /// Creates a new value of a specific type `T` which implements `FromStr`.
    /// # Example
    /// ```
    /// use yaccas::arguments::Value;
    ///
    /// let value = Value::new::<u32>();
    /// ```
    pub fn new<T : FromStr + Any + 'static>() -> Value {
        Value {
            value : Option::None,
            validator : try_convert::<T>,
            type_id : TypeId::of::<T>()
        }
    }

    /// Creates a new value with a default value.
    /// # Example
    /// ```
    /// use yaccas::arguments::Value;
    ///
    /// assert!(Value::with_default::<u32, _>("46").is_some());
    /// assert!(Value::with_default::<u32, _>("Not a number!").is_none());
    /// ```
    pub fn with_default<T : FromStr + Any + 'static, S : AsRef<str>>(value : S) -> Option<Value> {
        if try_convert::<T>(value.as_ref()) {
            Some(Value {
                value : Option::Some(value.as_ref().to_owned()),
                validator : try_convert::<T>,
                type_id : TypeId::of::<T>()
            })
        }
        else {
            None
        }
    }

    /// Set the value if it is valid.
    /// # Example
    /// ```
    /// use yaccas::arguments::Value;
    ///
    /// let mut value = Value::new::<u32>();
    ///
    /// assert_eq!(value.set_value("Invalid value"), false);
    /// assert_eq!(value.get_value::<u32>(), None);
    ///
    /// assert_eq!(value.set_value("46"), true);
    /// assert_eq!(value.get_value::<u32>(), Some(46));
    /// ```
    pub fn set_value<S : AsRef<str>>(&mut self, value : S) -> bool {
        if (self.validator)(value.as_ref()) {
            self.value = Option::Some(value.as_ref().to_owned());
            true
        }
        else {
            false
        }
    }

    /// Returns the value from type `T` if possible.
    /// # Hint
    /// The type `T` is checked at runtime to be identical which that from `new`.
    /// # Example
    /// ```
    /// use yaccas::arguments::Value;
    ///
    /// let value = Value::with_default::<u32, _>("46").expect("Well, 46 is a number...");
    ///
    /// assert_eq!(value.get_value::<u32>(), Some(46));
    /// assert_eq!(value.get_value::<u8>(), None); // See hint
    /// ```
    pub fn get_value<T : FromStr + Any + 'static>(&self) -> Option<T> {
        if self.type_id != TypeId::of::<T>() {
            None
        }
        else if let Some(ref value) = self.value {
            T::from_str(value).ok()
        }
        else {
            None
        }
    }
}

impl Argument for Value {
    fn has_value(&self) -> bool {
        self.value.is_some()
    }
}

impl From<Value> for Arguments {
    fn from(value: Value) -> Arguments {
        Arguments::Value(value)
    }
}

fn try_convert<T : FromStr>(string : &str) -> bool {
    T::from_str(string).is_ok()
}