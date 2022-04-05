//! Showcase crate to demonstrate some of my abilities with the Rust programing language.
//! See the README.md on the github project page for more details.

#![allow(unused_variables)]
#![allow(dead_code)]
#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;


/// Example json parser to showcase the library usage.
pub mod json;

/// Parser trait and parser combinators.
pub mod parsers;

/// Cached Regexes for the PEG
pub mod regexes;
