//! # WebRust - Python-like Rust for Web Applications
//!
//! WebRust brings Python's ergonomic syntax to Rust, enabling rapid development of
//! web-based GUI applications with zero runtime overhead.
//!
//! ## Quick Example
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//!
//! #[gui]
//! fn main() {
//!     let name: String = input("What's your name?");
//!     println("Hello, {name}!");
//!     
//!     let squares: Vec<i32> = 0.to(10).then(|x| x * x);
//!     chart(&squares, "line").title("Squares");
//! }
//! ```
//!
//! ## SQL Support (Optional)
//!
//! Enable SQL analytics with the `sql` feature:
//!
//! ```toml
//! [dependencies]
//! webrust = { version = "1.3.0", features = ["sql"] }
//! ```
//!
//! Then use SQL queries:
//!
//! ```rust,ignore
//! use webrust::prelude::*;
//!
//! #[gui]
//! fn main() {
//!     query("SELECT * FROM generate_series(1, 10) AS t(x)");
//! }
//! ```
//!
//! **Note**: The `sql` feature adds DuckDB (~5-10 min first compile).  
//! Without it, WebRust compiles in ~30 seconds.
//!
//! ## Modules
//!
//! - [`io`] - Input/output operations: styled printing, user input, GUI server
//! - [`iter`] - Python-like iteration: ranges, enumerate, comprehensions  
//! - [`viz`] - Data visualization: tables, charts, string methods
//! - [`graphic`] - Turtle graphics and object animations
//! - [`layout`] - Grid layouts and coordinate systems
//! - [`db`] - SQL queries with DuckDB (requires `sql` feature)
//! - [`text`] - String manipulation utilities
//! - [`prelude`] - Re-exports commonly used items
//!
//! ## Core Features
//!
//! - **F-string interpolation**: `println("Value: {x}, Result: {x * 2}")`
//! - **Python-like ranges**: `0.to(10)`, `'a'.to('z')`, `0.to(100).by(5)`
//! - **Comprehensions**: `.when(predicate).then(mapper)` for filtering and mapping
//! - **Interactive charts**: Line, bar, pie, radar, and more with ECharts
//! - **Smart tables**: Automatic formatting from any serializable data
//! - **SQL analytics** (optional): DuckDB integration with streaming HTML table output
//! - **LaTeX rendering**: Mathematical expressions with MathJax
//! - **Turtle graphics**: Visual programming with animations
//! - **Styled output**: Colors, borders, alignment, positioning
//!
//! The `#[gui]` attribute macro transforms any function into a web application
//! with automatic server setup and browser launch.

#![allow(clippy::all)]
pub mod io;
pub mod iter;
pub mod viz;
pub mod graphic;
pub mod layout;
pub mod text;

#[cfg(feature = "sql")]
pub mod db;

pub use io::*;
pub use iter::*;
pub use viz::*;
pub use layout::*;

#[cfg(feature = "sql")]
pub use db::*;

#[doc(hidden)]
pub use serde;
#[doc(hidden)]
pub use serde_json;
#[doc(hidden)]
pub use itoa;
#[doc(hidden)]
pub use ryu;


pub mod prelude {
    pub use crate::layout::*;
    pub use crate::io::print::{TH, TW};
    pub use crate::io::*;
    pub use crate::iter::*;
    pub use crate::text::*;
    pub use crate::viz::*;
    pub use crate::graphic::turtle::*;
    pub use webrust_macros::gui;

    #[cfg(feature = "sql")]
    pub use crate::db::*;
}