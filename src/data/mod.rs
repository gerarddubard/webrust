// webrust/src/data/mod.rs
//! # Data Structures and Interactive Visualization
//!
//! Provides intelligent data visualization, interactive chart generation,
//! professional table formatting, and Python-like string manipulation.
//!
//! ## Modules
//!
//! - [`table`] - Smart table generation with pivot, merge, and LaTeX support
//! - [`chart`] - Interactive data visualization with 9+ chart types (ECharts)
//! - [`string`] - Python-like string methods with smart pattern recognition
//!
//! ## Examples
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! use std::collections::HashMap;
//! # #[gui] fn example() {
//! // Tables from any data
//! let data = vec![(1, 2, 3), (4, 5, 6)];
//! table(&data).header(["X", "Y", "Z"]);
//!
//! // Interactive charts
//! let sales = HashMap::from([("Q1", 100.0), ("Q2", 150.0)]);
//! chart(&sales, "bar").title("Sales");
//!
//! // String processing
//! let parts = "a,b,c".splitby(",");
//! let upper = "hello".upper();
//! # }
//! ```

pub mod table;
pub mod string;
pub mod chart;

pub use table::*;
pub use string::*;
pub use chart::*;