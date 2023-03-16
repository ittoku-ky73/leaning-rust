//! # Crates IO
//!
//! `cratesio` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Art Module
pub mod art;

/// Adds one to the number given
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, cratesio::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
