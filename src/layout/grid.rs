// webrust/src/layout/grid.rs
//! # Grid-Based Layout System
//!
//! Divides the screen into a grid for consistent positioning of multiple
//! elements like dashboards, charts, and graphics.
//!
//! ## Grid Constants
//!
//! - `CW` - Cell width (computed from `TW` / columns)
//! - `CH` - Cell height (computed from `TH` / rows)
//!
//! ## Cell Positions
//!
//! - `"top left"` / `"tl"`, `"top center"` / `"tc"`, `"top right"` / `"tr"`
//! - `"middle left"` / `"ml"`, `"center"` / `"mc"`, `"middle right"` / `"mr"`
//! - `"bottom left"` / `"bl"`, `"bottom center"` / `"bc"`, `"bottom right"` / `"br"`
//!
//! ## Examples
//!
//!
//! use webrust::prelude::*;
//! # #[gui] fn example() {
//! grid(2, 3);  // 2 rows, 3 columns
//!
//! // Position in grid cells
//! let (x, y) = cell(0, 0, "center");
//! chart(&viz, "bar").at(x, y);
//!
//! let (x, y) = cell(1, 2, "top left");
//! table(&matrix).at(x, y);
//!
//! // Responsive sizing (percentage of cell)
//! let (w, h) = size_pct(80, 90);  // 80% width, 90% height
//! chart(&viz, "line").at(x, y).size(w, h);
//! # }
//!

use crate::layout::coord::{current, CoordMode};
use crate::io::print::{TW, TH};
use std::sync::LazyLock;

static mut GRID_ROWS: u32 = 1;
static mut GRID_COLS: u32 = 1;

pub static CW: LazyLock<u32> = LazyLock::new(|| {
    let (cell_w, _) = cell_size();
    cell_w as u32
});

pub static CH: LazyLock<u32> = LazyLock::new(|| {
    let (_, cell_h) = cell_size();
    cell_h as u32
});

pub fn grid(rows: u32, cols: u32) {unsafe {GRID_ROWS = rows;GRID_COLS = cols;}}
pub fn grid_size() -> (u32, u32) {unsafe { (GRID_ROWS, GRID_COLS) }}
pub fn cell_size() -> (f64, f64) {let tw = *TW as f64;let th = *TH as f64;let (rows, cols) = grid_size();(tw / cols as f64, th / rows as f64)}

pub fn size_pct(width_pct: u32, height_pct: u32) -> (u32, u32) {
    let w = (*CW as f64 * width_pct as f64 / 100.0) as u32;
    let h = (*CH as f64 * height_pct as f64 / 100.0) as u32;
    (w, h)
}

pub trait Sizable {
    fn set_size(&mut self, size: (u32, u32));

    fn size(mut self, width_pct: u32, height_pct: u32) -> Self
    where Self: Sized {
        let (w, h) = size_pct(width_pct, height_pct);
        self.set_size((w, h));
        self
    }
}

pub fn cell(r: u32, c: u32, pos: &str) -> (f64, f64) {
    let tw = *TW as f64; let th = *TH as f64;
    let rows = unsafe { GRID_ROWS.max(1) } as f64; let cols = unsafe { GRID_COLS.max(1) } as f64;
    let cell_w = tw / cols; let cell_h = th / rows;

    let (h_factor, v_factor) = match pos {
        "top left" | "tl" => (0.0, 0.0),
        "top center" | "top" | "tc" => (0.5, 0.0),
        "top right" | "tr" => (1.0, 0.0),
        "middle left" | "left" | "ml" => (0.0, 0.5),
        "middle center" | "center" | "mc" => (0.5, 0.5),
        "middle right" | "right" | "mr" => (1.0, 0.5),
        "bottom left" | "bl" => (0.0, 0.85),
        "bottom center" | "bottom" | "bc" => (0.5, 0.85),
        "bottom right" | "br" => (1.0, 0.85),
        _ => (0.5, 0.5),
    };

    match current() {
        CoordMode::Cartesian => (-tw/2.0 + cell_w*(c as f64+h_factor), th/2.0 - cell_h*(r as f64+v_factor)),
        CoordMode::Css => (cell_w*(c as f64+h_factor), cell_h*(r as f64+v_factor))
    }
}