// webrust/src/text/mod.rs
//! # Text Manipulation Utilities
//!
//! Python-inspired string methods for intuitive text processing in Rust.
//!
//! ## Modules
//!
//! - [`string`] - Python-like string operations (`.splitby()`, `.upper()`, `.title()`, etc.)
//!
//! ## Examples
//!
//! ```rust
//! use webrust::prelude::*;
//! # fn example() {
//! // Smart splitting
//! let parts = "a,b,c".splitby(",");        // ["a", "b", "c"]
//! let words = "hello  world".splitby("");  // ["hello", "world"]
//!
//! // Case transformations
//! let title = "hello world".title();       // "Hello World"
//!
//! // Validation
//! let is_alpha = "hello".isalpha();        // true
//! # }
//! ```

pub mod string;

pub use string::*;
