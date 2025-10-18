// webrust/src/graphic/mod.rs
//! # Visual Programming with Turtle Graphics
//!
//! Provides turtle-style drawing with modern animations, transformations,
//! and group operations using Two.js rendering.
//!
//! ## Modules
//!
//! - [`turtle`] - Turtle graphics with pen control, shapes, animations, and grouping
//!
//! ## Examples
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! # #[gui] fn example() {
//! coord("cartesian");
//!
//! let turtle = object();
//! turtle.color("blue").width(2.0);
//! turtle.forward(100.0);
//! turtle.right(90.0);
//! turtle.circle(50.0);
//!
//! // Animate with easing
//! turtle.rotate(360.0).ease("sineInOut");
//! # }
//! ```

pub mod turtle;
pub use turtle::*;