// webrust/src/viz/mod.rs
//! # Data Structures and Interactive Visualization
//!
//! Provides intelligent viz visualization, interactive chart generation,
//! professional table formatting, and Python-like string manipulation.
//!
//! ## Modules
//!
//! - [`table`] - Smart table generation with pivot, merge, and LaTeX support
//! - [`chart`] - Interactive viz visualization with 9+ chart types (ECharts)
//!
//! ## Examples
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! use std::collections::HashMap;
//! # #[gui] fn example() {
//! // Tables from any viz
//! let viz = vec![(1, 2, 3), (4, 5, 6)];
//! table(&viz).header(["X", "Y", "Z"]);
//!
//! // Interactive charts
//! let sales = HashMap::from([("Q1", 100.0), ("Q2", 150.0)]);
//! chart(&sales, "bar").title("Sales");
//! # }
//! ```

pub mod table;
pub mod chart;

pub use table::*;
pub use chart::*;