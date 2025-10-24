// webrust/src/layout/mod.rs
//! # Layout System - Grids and Coordinate Modes
//!
//! Provides unified coordinate systems and grid-based layouts for precise
//! positioning of text, graphics, tables, and charts.
//!
//! ## Modules
//!
//! - [`coord`] - Coordinate mode management (CSS vs Cartesian)
//! - [`grid`] - Grid-based layouts with cell positioning and sizing
//!
//! ## Coordinate Modes
//!
//! - **CSS mode** (default): Origin at top-left, +y down
//! - **Cartesian mode**: Origin at center, +y up (mathematical/scientific)
//!
//! The mode affects all positioning: text `.at()`, graphics, tables, and charts.
//!
//! ## Examples
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! # #[gui] fn example() {
//! // Grid layout
//! grid(3, 4);  // 3 rows × 4 columns
//! let (x, y) = cell(0, 0, "center");
//! println("Top-left cell").at(x, y);
//!
//! // Cartesian coordinates
//! coord("cartesian");
//! println("Center").at(0.0, 0.0);  // Mathematical origin
//! # }
//! ```

pub mod coord;
pub mod grid;

pub use coord::*;

pub use grid::*;
