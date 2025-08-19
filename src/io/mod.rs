// webrust/src/io/mod.rs
//! # Input/Output Module
//!
//! Provides comprehensive I/O capabilities including web-based GUI,
//! advanced printing with styling, and type-safe input validation.
//!
//! ## Modules
//!
//! - [`gui`] - Web-based graphical interface with smart server management
//! - [`print`] - Advanced printing with CSS-like styling and LaTeX support  
//! - [`input`] - Type-safe input with real-time validation
//!
//! ## Example
//!
//! ```rust
//! use webrust::prelude::*;
//!
//! #[gui]
//! fn main() {
//!     // Styled output
//!     println("@(red, bold)Error:@(reset) Something went wrong!");
//!     
//!     // Type-safe input
//!     let age: i32 = input("Enter your age:");
//!     
//!     // Advanced styling
//!     println("Welcome!")
//!         .weight(2).color("blue").background("lightgray");
//! }
//! ```
pub mod gui;
pub mod print;
pub mod input;


pub use crate::io::gui::*;
pub use print::{print, println, process_webrust_styles_only, PrintBox};
pub use input::input;