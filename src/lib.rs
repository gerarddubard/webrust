// webrust/src/lib.rs
#![allow(clippy::all)]
pub mod coord;
pub mod io;
pub mod iter;
pub mod data;
pub mod graphic;

pub use io::*;
pub use iter::*;
pub use data::*;
pub use serde;
pub use serde_json;

pub mod prelude {
    pub use crate::coord::coord;
    pub use crate::io::print::{CW, CH};
    pub use crate::io::*;
    pub use crate::iter::*;
    pub use crate::data::*;
    pub use crate::graphic::turtle::{turtle, Turtle};
    pub use webrust_macros::gui;
    pub use crate::serde;
    pub use crate::serde_json;
}
