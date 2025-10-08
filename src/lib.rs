// webrust/src/lib.rs

//! # WebRust - Python-like Rust for Web Applications
//!
//! WebRust brings Python's ergonomic syntax to Rust, enabling rapid development of
//! web-based GUI applications with zero runtime overhead.
//!
//! ## Quick Example
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//!
//! #[gui]
//! fn main() {
//!     let name: String = input("What's your name?");
//!     println("Hello, {name}!");
//!     
//!     let squares: Vec<i32> = 0.to(10).then(|x| x * x);
//!     chart(&squares, "line").title("Squares");
//! }
//! ```
//!
//! ## Modules
//!
//! - [`io`] - Input/output operations: styled printing, user input, GUI server
//! - [`iter`] - Python-like iteration: ranges, enumerate, comprehensions  
//! - [`data`] - Data visualization: tables, charts, string methods
//! - [`graphic`] - Turtle graphics and object animations
//! - [`layout`] - Grid layouts and coordinate systems
//! - [`prelude`] - Re-exports commonly used items
//!
//! ## Core Features
//!
//! - **F-string interpolation**: `println("Value: {x}, Result: {x * 2}")`
//! - **Python-like ranges**: `0.to(10)`, `'a'.to('z')`, `0.to(100).by(5)`
//! - **Comprehensions**: `.when(predicate).then(mapper)` for filtering and mapping
//! - **Interactive charts**: Line, bar, pie, radar, and more with ECharts
//! - **Smart tables**: Automatic formatting from any serializable data
//! - **LaTeX rendering**: Mathematical expressions with MathJax
//! - **Turtle graphics**: Visual programming with animations
//! - **Styled output**: Colors, borders, alignment, positioning
//!
//! The `#[gui]` attribute macro transforms any function into a web application
//! with automatic server setup and browser launch.

#![allow(clippy::all)]
pub mod io;
pub mod iter;
pub mod data;
pub mod graphic;
pub mod layout;

pub use io::*;
pub use iter::*;
pub use data::*;
pub use serde;
pub use serde_json;

pub mod prelude {
    pub use crate::layout::coord::coord;
    pub use crate::layout::grid::*;
    pub use crate::io::print::{TH, TW};
    pub use crate::io::*;
    pub use crate::iter::*;
    pub use crate::data::*;
    pub use crate::graphic::turtle::*;
    pub use webrust_macros::gui;
    pub use crate::serde;
    pub use crate::serde_json;
}