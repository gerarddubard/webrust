// webrust/src/lib.rs
//! # webrust - Python-like Rust for Web Applications
//!
//! webrust bridges the elegance of Python with the power and safety of Rust,
//! providing a seamless web-based interface for interactive applications.
//!
//! ## Quick Start
//!
//! ```rust
//! use webrust::prelude::*;
//!
//! #[gui]
//! fn main() {
//!     println("@(blue, bold)Welcome to webrust!");
//!     
//!     let name: String = input("What's your name?");
//!     let age: i32 = input("How old are you?");
//!     
//!     println("Hello @(green){name}@(reset)! You are {age} years old.");
//!     
//!     // Mathematical expressions
//!     println("Einstein's equation: $(E = mc^2)");
//!     
//!     // Python-like ranges
//!     for i in 1.to(5) {
//!         println("Count: {i}");
//!     }
//!     
//!     // Data tables
//!     let data = vec![vec!["Alice", "25"], vec!["Bob", "30"]];
//!     table(&data).header(["Name", "Age"]);
//! }
//! ```
//!
//! ## Core Features
//!
//! - **Python-like syntax** with Rust's type safety
//! - **Advanced f-strings** with rich formatting
//! - **Real-time input validation** 
//! - **LaTeX mathematical rendering**
//! - **Intelligent data tables**
//! - **Rich text styling system**
//! - **Automatic web interface**
//!
//! The crate automatically starts a web server and opens your browser,
//! providing an interactive interface with no additional setup required.
pub mod io;
pub mod iter;
pub mod data;

pub use io::*;
pub use iter::*;
pub use data::*;
pub use serde::{Serialize, Deserialize};
pub use serde_json;

pub mod prelude {
    pub use crate::io::print::{print, println, PrintBox};
    pub use crate::io::*;
    pub use crate::iter::*;
    pub use crate::data::*;
    pub use webrust_macros::gui;
    pub use crate::iter::range::{RangeExt, CharRangeExt};
    pub use crate::iter::enumerate::enumerate;
    pub use crate::io::input::input_with_validation as input;
    pub use serde::{Serialize, Deserialize};
    pub use serde_json;
}