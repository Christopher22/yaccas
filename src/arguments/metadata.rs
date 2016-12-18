pub type Callback<T> = FnMut(&T) -> ();

/// The metadata of an argument.
pub struct Metadata<T> {
    callback : Option<Box<Callback<T>>>
}

impl<T> Default for Metadata<T> {
    fn default() -> Self {
        Metadata {
            callback : None
        }
    }
}