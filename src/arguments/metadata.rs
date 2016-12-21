/// The metadata of an argument.
pub struct Metadata<'a, T> {
    /// The callback of an argument.
    pub callback: Option<Box<FnMut(&T) -> () + 'a>>,
}

impl<'a, T> Metadata<'a, T> {
    /// Sets a callback.
    /// ```
    /// use yaccas::arguments::{Flag, Metadata};
    /// let mut meta : Metadata<Flag> = Metadata::default().set_callback(|_flag : &Flag| {
    ///     // Do something in the callback
    /// });
    /// ```
    pub fn set_callback<C: FnMut(&T) -> () + 'a>(mut self, callback: C) -> Self {
        self.callback = Some(Box::new(callback));
        self
    }
}

impl<'a, T> Default for Metadata<'a, T> {
    fn default() -> Self {
        Metadata { callback: None }
    }
}
