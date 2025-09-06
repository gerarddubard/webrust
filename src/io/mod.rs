//! # WebRust I/O & Layout
//!
//! A cohesive I/O layer for browser-hosted Rust programs: styled printing,
//! robust input, tables/charts, and a shared coordinate model used across
//! text and graphics.
//!
//! ## What you get
//! - **Styled output** with Python-like convenience (`println("...")` + chainable styles).
//! - **Absolute positioning** via `.at(x, y)` that honors the global coordinate mode.
//! - **Screen metrics** via `CW`/`CH` for responsive layouts.
//! - **Validated inputs** rendered inline inside the terminal.
//! - **Tables/charts** ready for dashboards.
//!
//! ## Coordinate mode (shared with graphics)
//! Switch globally with:
//!
//! coord("css");       // origin top-left, +y down (default)
//! coord("cartesian"); // origin center, +y up
//!
//! The mode applies to **both** text placement (`.at(x, y)`) and turtle graphics,
//! making it easy to align labels with drawings.
//!
//! ## Absolute placement
//!
//! use webrust::prelude::*;
//! # #[gui]
//! # fn main() {
//! coord("cartesian");
//! println("@(white, bold)Dashboard")
//!     .background("midnightblue")
//!     .radius(8)
//!     .at(0.0, (*CH as f64)/2.0 - 30.0); // top-center in cartesian mode
//! # }
//!
//!
//! In **CSS mode**, passing a **negative `x`** to `.at(x, y)` anchors from the
//! **right** edge (e.g. `.at(-20.0, 8.0)` → 20px from the right).
//!
//! ## Using `CW` / `CH`
//! `CW` and `CH` reflect the layout area (half the system screen on Windows by default)
//! and are exported for quick arithmetic like column widths or centering.
//!
//! See [`io::print`] for the printing API and styles, and `graphic::turtle` for canvas drawing.
pub mod gui;
pub mod print;
pub mod input;

pub use crate::io::gui::*;
pub use print::{print, println, process_webrust_styles_only, PrintBox, CW, CH};
pub use input::input;