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
//! let months = vec!["Jan","Feb","Mar","Apr","May"];
//! let sales = vec![120.0, 200.0, 150.0, 300.0, 250.0];
//! line(&months, &sales);
//! # }
//! ```

pub mod chart;
pub mod table;

pub use chart::*;
pub use table::*;
