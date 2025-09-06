// webrust/src/coord.rs
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::LazyLock;

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum CoordMode{Css=1,Cartesian=2}

pub static COORD_MODE:LazyLock<AtomicU8>=LazyLock::new(||AtomicU8::new(CoordMode::Css as u8));

pub fn coord(mode:&str){
    let v=match mode.to_ascii_lowercase().as_str(){"cartesian"=>CoordMode::Cartesian as u8,_=>CoordMode::Css as u8};
    COORD_MODE.store(v,Ordering::Relaxed);
}
pub fn current()->CoordMode{
    match COORD_MODE.load(Ordering::Relaxed){2=>CoordMode::Cartesian,_=>CoordMode::Css}
}