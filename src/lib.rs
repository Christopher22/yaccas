//! # Yaccas
//! **Y**et **A**nother **C**allback-orientated **C**ommand line pArSer is ... well, yet another command line parser.

#![deny(missing_docs, trivial_casts, trivial_numeric_casts,
unstable_features, unused_import_braces, unused_qualifications)]

pub mod arguments;
pub mod parser;
pub mod scanner;

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
