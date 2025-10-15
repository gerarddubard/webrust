// webrust/src/db/mod.rs
//! # SQL Database Integration with DuckDB
//!
//! High-performance SQL analytics with in-memory DuckDB database,
//! streaming result rendering, and Arrow-based data processing.
//!
//! ## Modules
//!
//! - [`sql`] - SQL query execution with streaming HTML tables
//!
//! ## Examples
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//!
//! #[gui]
//! fn main() {
//!     // Create table and insert data
//!     query("
//!         CREATE TABLE users (id INTEGER, name VARCHAR, age INTEGER);
//!         INSERT INTO users VALUES (1, 'Alice', 30), (2, 'Bob', 25);
//!     ");
//!     
//!     // Query with streaming results
//!     query("SELECT * FROM users WHERE age > 20");
//!     
//!     // Inspect schema
//!     query("SCHEMA SELECT * FROM users");
//!     
//!     // Use DuckDB built-ins
//!     query("SELECT * FROM generate_series(1, 100)");
//! }
//! ```

pub mod sql;

pub use sql::*;