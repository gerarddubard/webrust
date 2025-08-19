// webrust/src/iter/enumerate.rs
//! # Python-style Enumerate Function
//!
//! Provides the `enumerate` function that pairs each element
//! with its index, similar to Python's built-in `enumerate()`.
//!
//! ## Basic Usage
//!
//!
//! let fruits = vec!["apple", "banana", "cherry"];
//!
//! for (index, fruit) in enumerate(&fruits) {
//!     println!("{index}: {fruit}");
//! }
//! // Output:
//! // 0: apple
//! // 1: banana  
//! // 2: cherry
//!
//!
//! ## Custom Start Index
//!
//! Unlike Python, this enumerate always starts at 0, but you can
//! adjust the displayed index:
//!
//!
//! let start = 1;
//! for (index, item) in enumerate(&items) {
//!     println!("{}: {item}", index + start);
//! }
//!
//!
//! ## Works with Any Iterator
//!
//!
//! // With ranges
//! for (index, value) in enumerate(0.to(10).by(2)) {
//!     println!("Position {index}: Value {value}");
//! }
//!
//! // With character ranges
//! for (index, letter) in enumerate('a'.to('f')) {
//!     println!("{index}: {letter}");
//! }
//!
//! // With any collection
//! for (index, line) in enumerate(text.lines()) {
//!     println!("Line {index}: {line}");
//! }
//!
//!
//! ## Implementation
//!
//! This is a thin wrapper around Rust's built-in `.enumerate()`
//! method, providing a more familiar function-based syntax for
//! Python developers transitioning to Rust.

#[inline]
pub fn enumerate<I: IntoIterator>(iter: I) -> impl Iterator<Item = (usize, I::Item)> {
    iter.into_iter().enumerate()
}