// webrust/src/io/mod.rs
//! # Input/Output Module
//!
//! Provides comprehensive I/O capabilities including web-based GUI,
//! advanced printing with professional styling, dynamic layout system,
//! and type-safe input validation.
//!
//! ## Key Features
//!
//! - **Dynamic screen sizing** - Automatic CW/CH detection for responsive layouts
//! - **Professional styling** - CSS-like borders, colors, backgrounds, radius
//! - **Text alignment** - Left, center, right, and justify alignment
//! - **Rich text formatting** - Colors, fonts, LaTeX mathematical expressions
//! - **Type-safe input** - Real-time validation with error handling
//! - **Web-based GUI** - Smart server management with automatic browser launch
//!
//! ## Modules
//!
//! - [`gui`] - Web-based graphical interface with smart server management
//! - [`print`] - Advanced printing with CSS-like styling, LaTeX, and dynamic layouts   
//! - [`input`] - Type-safe input with real-time validation
//!
//! ## Quick Start
//!
//! ```ignore
//! use webrust::prelude::*;
//!
//! #[gui]
//! fn main() {
//!     // Rich styled output
//!     println("@(red, bold)Error:@(reset) Something went wrong!");
//!     
//!     // Professional layout with dynamic width
//!     println("@(navy, bold)WEBRUST FRAMEWORK v0.8.0")
//!         .width(*CW)                // Dynamic full width
//!         .align("center")           // Center alignment
//!         .weight(4)                 // Thick border
//!         .color("navy")             // Navy border
//!         .style("double")           // Double border style
//!         .radius(8)                 // Rounded corners
//!         .background("lightcyan");  // Background color
//!     
//!     // Type-safe input
//!     let age: i32 = input("Enter your age:");
//!     
//!     // Mathematical expressions
//!     println("Einstein's equation: $(E = mc^2)");
//! }
//! ```
//!
//! ## Dynamic Layout System
//!
//! WebRust v0.8.0 introduces automatic screen dimension detection:
//!
//! ```ignore
//! // CW (Content Width) - Half of screen width
//! // CH (Content Height) - Half of screen height
//!
//! // Responsive full-width header
//! println("Welcome to WebRust").width(*CW).align("center");
//!
//! // Two-column layout
//! println("Left Column").width(*CW / 2).align("left");
//! println("Right Column").width(*CW / 2).align("right");
//!
//! // Four-column grid
//! for i in 1..=4 {
//!     println(&format!("Column {}", i))
//!         .width(*CW / 4)
//!         .align("center")
//!         .background("lightblue");
//! }
//! ```
//!
//! ## Professional Styling
//!
//! Complete CSS-like styling system for professional output:
//!
//! ```ignore
//! // Alert system
//! println("@(white, bold)SUCCESS")
//!     .width(*CW)
//!     .align("center")
//!     .weight(3)                  // Border thickness
//!     .color("green")             // Border color
//!     .style("solid")             // Border style
//!     .radius(10)                 // Rounded corners
//!     .background("lightgreen");  // Background
//!
//! // Document layout
//! let content = "Long text content for justified alignment...";
//! println(content)
//!     .width(*CW)
//!     .align("justify")           // Justify text
//!     .weight(1)                  // Thin border
//!     .color("darkslateblue")
//!     .style("dashed")
//!     .radius(5)
//!     .background("ghostwhite");
//! ```
//!
//! ## Text Alignment Modes
//!
//! Four alignment modes provide complete layout control:
//!
//! - **`"left"`** - Left-aligned text within container
//! - **`"center"`** - Centered text within container  
//! - **`"right"`** - Right-aligned text within container
//! - **`"justify"`** - Justified text (full line width)
//!
//! ## Border and Styling Options
//!
//! - **Weight**: `1-5` pixels for border thickness
//! - **Style**: `"solid"`, `"dashed"`, `"dotted"`, `"double"`
//! - **Radius**: `0-25+` pixels for rounded corners
//! - **Colors**: CSS color names, hex codes, or RGB
//! - **Backgrounds**: Any CSS-compatible background color
//!
//! ## LaTeX Integration
//!
//! Seamless mathematical expression rendering:
//!
//! ```text
//! println("Quadratic formula: $(x = \\frac{-b \\pm \\sqrt{b^2-4ac}}{2a})");
//! println("Integral: $(\\int_0^\\infty e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2})");
//! ```
//!
//! ## Real-World Examples
//!
//! ### Professional Report Header
//!
//! ```ignore
//! println("@(navy, bold)QUARTERLY FINANCIAL REPORT")
//!     .width(*CW)
//!     .align("center")
//!     .weight(5)
//!     .color("navy")
//!     .style("double")
//!     .radius(0)
//!     .background("lightsteelblue");
//! ```
//!
//! ### Form Layout
//!
//! ```ignore
//! println("User Registration")
//!     .width(*CW / 2)
//!     .align("center")
//!     .weight(2)
//!     .background("aliceblue");
//!
//! let name: String = input("Full name:");
//! let email: String = input("Email address:");
//! ```
//!
//! ### Dashboard Cards
//!
//! ```ignore
//! let metrics = [
//!     ("Users", "1,234"),
//!     ("Revenue", "$45,678"),
//!     ("Growth", "+12.5%"),
//! ];
//!
//! for (label, value) in metrics {
//!     println(&format!("{}: {}", label, value))
//!         .width(*CW / 3)
//!         .align("center")
//!         .weight(1)
//!         .color("darkblue")
//!         .background("lightcyan")
//!         .radius(8);
//! }
//! ```
//!
//! The I/O module provides everything needed for professional, responsive,
//! and beautifully styled console applications with WebRust v0.8.0.

pub mod gui;
pub mod print;
pub mod input;

pub use crate::io::gui::*;
pub use print::{print, println, process_webrust_styles_only, PrintBox, CW, CH};
pub use input::input;