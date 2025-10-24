// webrust/src/layout/coord.rs
//! # Coordinate Mode Management
//!
//! Global coordinate system that affects all positioning operations:
//! text placement, turtle graphics, tables, and charts.
//!
//! ## Modes
//!
//! - **CSS** (default): Origin top-left (0,0), +x right, +y down
//! - **Cartesian**: Origin center (0,0), +x right, +y up
//!
//! ## Examples
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! # #[gui] fn example() {
//! // Default CSS mode
//! println("Top-left").at(0.0, 0.0);
//!
//! // Switch to mathematical coordinates
//! coord("cartesian");
//! println("Center screen").at(0.0, 0.0);
//! object().at(0.0, 100.0).circle(50.0);  // Above center
//!
//! // Back to CSS
//! coord("css");
//! # }
//! ```
//!
//! The mode persists throughout the application and applies to all
//! positioning operations, ensuring visual consistency.

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::LazyLock;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CoordMode {
    Css = 1,
    Cartesian = 2,
}

pub static COORD_MODE: LazyLock<AtomicU8> = LazyLock::new(|| AtomicU8::new(CoordMode::Css as u8));

pub fn coord(mode: &str) {
    let v = match mode.to_ascii_lowercase().as_str() {
        "cartesian" => CoordMode::Cartesian as u8,
        _ => CoordMode::Css as u8,
    };
    COORD_MODE.store(v, Ordering::Relaxed);
}

pub fn current() -> CoordMode {
    match COORD_MODE.load(Ordering::Relaxed) {
        2 => CoordMode::Cartesian,
        _ => CoordMode::Css,
    }
}

pub fn to_screen_coords(x: f64, y: f64) -> (f64, f64) {
    match current() {
        CoordMode::Cartesian => {
            let tw = *crate::io::print::TW as f64;
            let th = *crate::io::print::TH as f64;
            (x + tw / 2.0, th / 2.0 - y)
        }
        CoordMode::Css => (x, y),
    }
}
