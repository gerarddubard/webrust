// webrust/src/iter/mod.rs
//! # Python-like Iteration Utilities
//!
//! Provides familiar Python-style iteration patterns in Rust with zero-cost
//! abstractions and compile-time optimizations.
//!
//! ## Modules
//!
//! - [`range`] - Python-like ranges with `.to()` and `.by()` syntax
//! - [`enumerate`] - Index-value iteration like Python's `enumerate()`
//! - [`comprehension`] - List/dict comprehensions with `.when()/.then()` syntax
//!
//! ## Examples
//!
//!
//! use webrust::prelude::*;
//! # fn example() {
//! // Ranges
//! for i in 0.to(10) { }              // range(10)
//! for i in 0.to(10).by(2) { }        // range(0, 10, 2)
//!
//! // Enumerate
//! for (index, item) in enumerate(&items) { }
//!
//! // Comprehensions
//! let squares: Vec<i32> = 0.to(10).then(|x| x * x);
//! let evens: Vec<i32> = 0.to(20).when(|&x| x % 2 == 0).then(|x| x);
//! # }
//!

pub mod range;
pub mod enumerate;
pub mod comprehension;

pub use range::*;
pub use enumerate::enumerate;
pub use comprehension::*;