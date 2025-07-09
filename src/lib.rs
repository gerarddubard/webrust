//! # webrust - Python-like Rust for Web Applications
//!
//! webrust is a revolutionary Rust crate that bridges the simplicity of Python with the power and safety of Rust.
//! It provides an elegant web-based interface for Rust applications, featuring Python-like syntax, advanced
//! f-string formatting, automatic type validation, and mathematical rendering capabilities.
//!
//! ## Core Philosophy
//!
//! webrust aims to make Rust more accessible while maintaining its core strengths:
//! - **Simplicity**: Python-like syntax for common operations
//! - **Safety**: Full Rust type safety and memory management
//! - **Innovation**: Web-based UI with automatic browser integration
//! - **Flexibility**: Advanced formatting and mathematical rendering
//!
//! ## Key Features
//!
//! - **F-String Support**: Advanced string formatting with Rust expressions
//! - **Web Interface**: Automatic web server with browser integration
//! - **Type Validation**: Real-time input validation with error feedback
//! - **Mathematical Rendering**: LaTeX support via MathJax integration
//! - **Styling System**: CSS-like styling directly in Rust code
//! - **Python-like Ranges**: Intuitive range generation with `start.to(end)` syntax
//! - **Enumerate Function**: Python-style enumeration with `enumerate(iterable)`
//!
//! ## Example Usage
//!
//! ```rust
//! use webrust::prelude::*;
//!
//! #[gui]
//! fn main() {
//!     let name: String = input("Enter your name:");
//!     let age: i32 = input("Enter your age:");
//!     
//!     println("Hello @(green, bold){name}@(reset), you are @(yellow){age}@(reset) years old!");
//!     
//!     // Python-like ranges
//!     println("Counting to 10:");
//!     for i in 1.to(11) {
//!         println("@(blue){i}@(reset)");
//!     }
//!     
//!     // Enumerate with styling
//!     let items = vec!["apple", "banana", "cherry"];
//!     for (index, item) in enumerate(&items) {
//!         println("@(yellow){index}@(reset): @(green){item}@(reset)");
//!     }
//!     
//!     latex("E = mc^2");
//! }
//! ```
//!
//! ## Module Organization
//!
//! - `gui`: Web server and browser integration
//! - `print`: Enhanced printing with styling support
//! - `input`: Type-safe input handling with validation
//! - `latex`: Mathematical rendering via MathJax
//! - `range`: Python-like range generation with fluent syntax
//! - `enumerate`: Python-style enumeration for iterables
//! - `prelude`: Common imports for ease of use

pub mod gui;
pub mod print;
pub mod input;
pub mod latex;
pub mod range;
pub mod enumerate;

pub mod prelude {
    pub use crate::gui::*;
    pub use crate::print::*;
    pub use crate::input::*;
    pub use crate::latex::*;
    pub use crate::range::*;
    pub use crate::enumerate::*;
    pub use webrust_macros::gui;
}