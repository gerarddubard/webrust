// webrust/src/iter/enumerate.rs
//! # Python-style Enumerate Function
//!
//! Pairs each element with its index, similar to Python's built-in `enumerate()`.
//! Works with any iterator, including webrust ranges.
//!
//! ## Examples
//!
//! ```rust
//! use webrust::prelude::*;
//! # fn example() {
//! let items = vec!["apple", "banana", "cherry"];
//! for (index, item) in enumerate(&items) {
//!     println!("{}: {}", index, item);
//! }
//! // Output: 0: apple, 1: banana, 2: cherry
//!
//! // With ranges
//! for (index, value) in enumerate(0.to(10).by(2)) {
//!     println!("Position {}: Value {}", index, value);
//! }
//!
//! // Custom start (manual offset)
//! let start = 1;
//! for (index, item) in enumerate(&items) {
//!     println!("{}: {}", index + start, item);
//! }
//! # }
//! ```
//!
//! This is a thin wrapper around Rust's `.enumerate()` with
//! function-based syntax familiar to Python developers.

#[inline]
pub fn enumerate<I: IntoIterator>(iter: I) -> impl Iterator<Item = (usize, I::Item)> {
    iter.into_iter().enumerate()
}