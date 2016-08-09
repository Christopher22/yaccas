use std::marker::PhantomData;

use ::arguments::{Argument, Arguments};

pub struct Callback<T : Argument, F : FnMut(&T) -> ()> {
    pub phantom : PhantomData<T>,
    pub callback : F
}

pub trait Executable {
    fn execute(&mut self, arg : &Arguments);
}

impl<T : Argument, F : FnMut(&T) -> ()> Executable for Callback<T, F> {
    fn execute(&mut self, arg : &Arguments) {
        unsafe {
            (self.callback)(arg.unwrap::<T>());
        }
    }
}