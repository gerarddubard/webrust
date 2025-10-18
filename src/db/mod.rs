// webrust/src/db/mod.rs
//! # SQL Database Integration with DuckDB
//!
//! High-performance analytical SQL with in-memory DuckDB, streaming HTML tables,
//! and zero-copy Arrow-based data processing.
//!
//! ## Overview
//!
//! This module provides a Python-like SQL interface with automatic result rendering,
//! perfect for data analysis, reporting, and exploratory data analysis in the browser.
//!
//! ## Key Features
//!
//! - **In-memory analytics**: DuckDB's columnar engine for OLAP workloads
//! - **Streaming results**: Batched HTML rendering via Apache Arrow
//! - **Built-in I/O**: Import/export CSV, Parquet, JSON with one command
//! - **Pre-loaded extensions**: httpfs (HTTP/S3), parquet, json
//! - **Type-safe**: Arrow schema detection with proper type formatting
//! - **Zero dependencies**: Pure Rust implementation with no external tools
//!
//! ## Quick Example
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//!
//! #[gui(bg = "navy", fg = "white")]
//! fn main() {
//!     // Load data from URL
//!     query("IMPORT 'https://example.com/iris.csv' AS iris");
//!
//!     // Compute statistics
//!     query(r#"
//!         SELECT
//!             species,
//!             AVG(sepal_length) as avg_sepal,
//!             COUNT(*) as count
//!         FROM iris
//!         GROUP BY species
//!     "#);
//!
//!     // Export results
//!     query("EXPORT iris TO 'iris.parquet'");
//! }
//! ```
//!
//! ## Module Structure
//!
//! - [`sql`] - Core SQL execution engine with streaming HTML rendering
//!
//! ## Supported SQL Features
//!
//! - **DDL**: CREATE, ALTER, DROP tables/views/sequences
//! - **DML**: INSERT, UPDATE, DELETE, MERGE
//! - **Queries**: SELECT with full DuckDB syntax
//! - **CTEs**: WITH recursive and non-recursive
//! - **Window functions**: RANK, ROW_NUMBER, LAG, LEAD, etc.
//! - **Aggregates**: GROUP BY, HAVING, ROLLUP, CUBE
//! - **JSON**: json_each, json_extract, json_array_length
//! - **Time series**: DATE_TRUNC, INTERVAL arithmetic
//! - **UDFs**: CREATE MACRO for custom functions
//!
//! ## Performance Characteristics
//!
//! | Operation | Complexity | Notes |
//! |-----------|-----------|-------|
//! | Sequential scan | O(n) | Vectorized SIMD |
//! | Index lookup | O(log n) | ART index |
//! | Hash join | O(n+m) | Parallel execution |
//! | Sort | O(n log n) | External if needed |
//! | Aggregation | O(n) | Hash-based |
//!
//! ## Memory Management
//!
//! - **Lazy evaluation**: Streaming pipeline without full materialization
//! - **Columnar storage**: Reduced memory footprint vs row-based
//! - **Buffer pooling**: Reused thread-local buffers for string formatting
//! - **Zero-copy**: Arrow arrays accessed without serialization
//!
//! ## Examples
//!
//! See the `examples/` directory:
//! - `py_simplesql.rs` - Basic DDL/DML and queries
//! - `py_advancedsql.rs` - Real datasets (Iris, Titanic) with analytics
//!
//! Run with: `cargo run --example simplesql --features sql`
//!
//! ## Extensions
//!
//! Pre-loaded extensions (no installation needed):
//! - **httpfs**: Read from HTTP/HTTPS/S3
//! - **parquet**: Native Parquet support
//! - **json**: JSON parsing and generation
//!
//! Additional extensions available via `LOAD`:
//! - spatial, fts, icu, inet, etc.
//!
//! ## Integration with WebRust
//!
//! SQL queries integrate seamlessly with other WebRust features:
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//!
//! #[gui]
//! fn main() {
//!     println("@(cyan, bold)Data Analysis Dashboard");
//!
//!     query("CREATE TABLE sales AS SELECT * FROM read_csv_auto('sales.csv')");
//!
//!     println("@(green)Top products:");
//!     query("SELECT product, SUM(revenue) FROM sales GROUP BY product LIMIT 5");
//!
//!     latex(r"\text{Total Revenue: } $\sum_{i=1}^{n} revenue_i$");
//! }
//! ```

pub mod sql;

pub use sql::*;