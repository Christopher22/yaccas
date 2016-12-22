//! This module contains `Scanner` which maps command line arguments to `Token`s dependently from OS.

mod token;
mod scanner;
mod scanner_unix;
mod scanner_windows;

pub use self::scanner::Scanner;
pub use self::token::Token;
pub use self::scanner_unix::Unix;
pub use self::scanner_windows::Windows;
