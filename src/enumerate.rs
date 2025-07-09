//! # Enumerate - Python-like enumeration function
//!
//! This module provides an `enumerate` function similar to Python's,
//! which enumerates iterator elements with their index.
//!
//! ## Usage
//!
//! use webrust::enumerate::enumerate;
//!
//! let data = vec!["Alice", "Bob", "Charlie"];
//!
//! // Standard enumeration (index starts at 0)
//! for (index, value) in enumerate(&data) {
//!     println!("{index}: {value}");
//! }
//!
//! // For custom start index, adjust in display
//! let start = 10;
//! for (index, value) in enumerate(&data) {
//!     println!("{}: {value}", index + start);
//! }
//!
//!
//! ## Compatibility
//!
//! This function works with all types implementing `IntoIterator`, 
//! including vectors, slices, ranges, and your custom types.

/// Enumerates iterator elements with their index, starting at 0
///
/// This function is equivalent to Python's `enumerate()` function
/// and produces tuples `(index, item)` for each element.
///
/// # Examples
///
/// use webrust::enumerate::enumerate;
///
/// let fruits = vec!["apple", "banana", "orange"];
///
/// for (i, fruit) in enumerate(&fruits) {
///     println!("{i}: {fruit}");
/// }
/// // Prints:
/// // 0: apple
/// // 1: banana
/// // 2: orange
///
///
/// For custom start index, adjust in your code:
///
/// use webrust::enumerate::enumerate;
///
/// let data = vec!["a", "b", "c"];
/// let start = 10;
///
/// for (i, item) in enumerate(&data) {
///     println!("{}: {item}", i + start);
/// }
/// // Prints:
/// // 10: a
/// // 11: b
/// // 12: c
///
#[inline]
pub fn enumerate<I: IntoIterator>(iter: I) -> impl Iterator<Item = (usize, I::Item)> {
    iter.into_iter().enumerate()
}