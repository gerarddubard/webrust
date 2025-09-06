// webrust/src/graphic/turtle.rs
//! # WebRust Graphics
//!
//! High-level, browser-backed drawing primitives for WebRust applications.
//! This module currently provides **turtle graphics** that run inside the
//! WebRust terminal via an HTML `<canvas>` element.
//!
//! ## Key Features
//! - **Multiple independent turtles** can move and draw simultaneously on the same stage
//! - **Fully integrated coordinate system** with WebRust's coordinate switcher:
//!   `coord("css")` or `coord("cartesian")` affects both graphics and text positioning
//! - **Consistent stage dimensions** following the global `CW` / `CH` constants 
//!   used throughout the I/O layer
//!
//! ## Quick Start
//! ```rust
//! use webrust::prelude::*;
//!
//! coord("cartesian");
//! let turtle = turtle();
//! turtle.setColor("tomato")
//!       .setPenSize(3.0)
//!       .angle(45.0)
//!       .setPos(0.0, 0.0)
//!       .forward(150.0);
//! ```
//!
//! ## Coordinate Systems
//! - `"css"` (default): Origin at top-left, +x → right, +y → down
//! - `"cartesian"`: Origin at center, +x → right, +y → **up**
//!
//! Switch coordinate systems anytime with `coord("css")` or `coord("cartesian")`. 
//! This setting is shared between graphics and text positioning, enabling precise 
//! alignment of visual elements.
//!
//! ## Module Structure
//! See [`turtle`] module for the complete turtle graphics API.

pub mod turtle;
pub use turtle::{turtle, Turtle};