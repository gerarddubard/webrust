// webrust/src/data/mod.rs
//! # Data Structures and Visualization
//!
//! Provides intelligent data visualization and manipulation tools,
//! with automatic table generation from any serializable data structure.
//!
//! ## Modules
//!
//! - [`table`] - Intelligent table generation with pivot, merge, and formatting
//!
//! ## Quick Example
//!
//!
//! use webrust::prelude::*;
//! use std::collections::HashMap;
//!
//! let mut data = HashMap::new();
//! data.insert("Alice", 95);
//! data.insert("Bob", 87);
//!
//! // Automatic table generation
//! table(&data);
//!
//! // With customization
//! table(&data).header(["Name", "Score"]).pivot();
//!
//!
//! The module automatically detects data structure patterns and
//! generates appropriate visualizations with professional styling.

pub mod table;

pub use table::*;