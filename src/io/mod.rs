// webrust/src/io/mod.rs
//! # I/O & Layout - Unified Text and Graphics System
//!
//! Provides styled printing, user input, absolute positioning, and coordinate modes
//! that work seamlessly with both text and graphics.
//!
//! ## Modules
//!
//! - [`gui`] - Web server setup, browser launch, theme configuration
//! - [`print`] - Styled output with f-strings, colors, positioning, and screen metrics
//! - [`input`] - Type-safe user input with real-time validation
//!
//! ## Core Concept
//!
//! The coordinate mode set with `coord()` applies globally to both text placement
//! and turtle graphics, ensuring consistent positioning across your application.
//!
//! ## Examples
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! # #[gui] fn example() {
//! // Styled text with positioning
//! println("Dashboard")
//!     .background("navy")
//!     .color("white")
//!     .at(100.0, 50.0);
//!
//! // Type-safe input
//! let age: i32 = input("Enter age:");
//!
//! // Coordinate mode affects all positioning
//! coord("cartesian");  // Center origin, +y up
//! println("Centered").at(0.0, 0.0);
//! # }
//! ```
pub mod gui;
pub mod print;
pub mod input;

pub use crate::io::gui::*;
pub use print::{print, println, process_webrust_styles_only, PrintBox, TW, TH};
pub use input::input;