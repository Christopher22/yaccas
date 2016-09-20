//! This module contains `Scanner` which maps command line arguments to `Token`s dependently from OS.

mod token;
mod scanner;
mod scanner_unix;
mod scanner_windows;

pub use self::scanner::Scanner;
pub use self::token::Token;
pub use self::scanner_unix::Unix;
pub use self::scanner_windows::Windows;

/// This macro returns the appropriated `Scanner` for the current operation system.
/// # Example
/// ```
/// #[macro_use]
/// extern crate yaccas;
/// use yaccas::parser::{Result, Parser};
///
/// fn main() {
///     let mut parser = Parser::default();
///     assert_eq!(parser.parse(default_scanner!()), Result::Success(vec![]));
/// }
/// ```
#[macro_export]
macro_rules! default_scanner {
    () => ({
            use yaccas::scanner::{Unix, Windows};
            use std::env::Args;

            #[cfg(target_family = "windows")]
            #[inline(always)]
            fn default() -> Windows<Args> {
                Windows::<Args>::default()
            }

            #[cfg(not(target_family = "windows"))]
            #[inline(always)]
            fn default() -> Unix<Args> {
                Unix::<Args>::default()
            }

            default()
    })
}
