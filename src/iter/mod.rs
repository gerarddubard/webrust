// webrust/src/iter/mod.rs
//! # Python-like Iteration Utilities
//!
//! Provides familiar Python-style iteration patterns in Rust,
//! including ranges with fluent syntax and enumeration functions.
//!
//! ## Modules
//!
//! - [`range`] - Python-like ranges with `start.to(end).by(step)` syntax
//! - [`enumerate`] - Index-value iteration like Python's `enumerate()`
//!
//! ## Quick Examples
//!
//! ```rust
//! use webrust::prelude::*;
//!
//! // Ranges
//! for i in 0.to(10) { }              // 0, 1, 2, ..., 9
//! for i in 0.to(10).by(2) { }        // 0, 2, 4, 6, 8
//! for i in 10.to(0) { }              // 10, 9, 8, ..., 1
//! for c in 'a'.to('z') { }           // a, b, c, ..., y
//!
//! // Enumeration
//! let items = vec!["apple", "banana", "cherry"];
//! for (index, item) in enumerate(&items) {
//!     println("{index}: {item}");
//! }
//! ```
//!
//! These utilities make Rust iteration as intuitive as Python
//! while maintaining Rust's performance and type safety.
pub mod range;
pub mod enumerate;
pub use range::*;
pub use enumerate::enumerate;