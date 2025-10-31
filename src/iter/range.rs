// webrust/src/iter/range.rs
//! # Python-like Ranges in Rust
//!
//! Fluent range syntax supporting numeric types and characters with
//! automatic direction detection and custom step sizes.
//!
//! ## Numeric Ranges
//!
//! ```rust
//! use webrust::prelude::*;
//! # fn example() {
//! for i in 0.to(10) { }              // 0, 1, 2, ..., 9
//! for i in 0.to(10).by(2) { }        // 0, 2, 4, 6, 8
//! for i in 10.to(0) { }              // 10, 9, 8, ..., 1
//! for i in 20.to(0).by(-3) { }       // 20, 17, 14, 11, ...
//! # }
//! ```
//!
//! ## Character Ranges
//!
//! ```rust
//! use webrust::prelude::*;
//! # fn example() {
//! for c in 'a'.to('z') { }           // a, b, c, ..., y
//! for c in 'A'.to('Z').by(2) { }     // A, C, E, ..., Y
//! # }
//! ```
//!
//! ## Float Ranges
//!
//! ```rust
//! use webrust::prelude::*;
//! # fn example() {
//! for x in 0.0.to(2.0).by(0.5) { }   // 0.0, 0.5, 1.0, 1.5
//! # }
//! ```
//!
//! Direction is automatic: ascending if start < end, descending otherwise.

use core::ops::{AddAssign, SubAssign};

pub trait RangeNum: Copy + PartialOrd + AddAssign<Self> + SubAssign<Self> {
    fn zero() -> Self;
    fn one() -> Self;
    fn abs(v: Self) -> Self;
}

macro_rules! impl_range_num_int {
    ($($t:ty),*) => {$(
        impl RangeNum for $t {
            #[inline] fn zero() -> Self { 0 as $t }
            #[inline] fn one() -> Self { 1 as $t }
            #[inline] fn abs(v: Self) -> Self { v }
        }
    )*}
}
impl_range_num_int!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_range_num_sint {
    ($($t:ty),*) => {$(
        impl RangeNum for $t {
            #[inline] fn zero() -> Self { 0 as $t }
            #[inline] fn one() -> Self { 1 as $t }
            #[inline] fn abs(v: Self) -> Self { if v < 0 { -v } else { v } }
        }
    )*}
}
impl_range_num_sint!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_range_num_float {
    ($($t:ty),*) => {$(
        impl RangeNum for $t {
            #[inline] fn zero() -> Self { 0.0 as $t }
            #[inline] fn one() -> Self { 1.0 as $t }
            #[inline] fn abs(v: Self) -> Self { v.abs() }
        }
    )*}
}
impl_range_num_float!(f32, f64);

#[derive(Clone, Copy)]
pub struct Range<T: RangeNum> {
    current: T,
    end: T,
    step: T,
    descending: bool,
}

impl<T: RangeNum> Range<T> {
    #[inline]
    pub fn new(start: T, end: T, step: T) -> Self {
        let descending = start > end;
        let mut s = T::abs(step);
        if s == T::zero() {
            s = T::one();
        }
        Self {
            current: start,
            end,
            step: s,
            descending,
        }
    }
}

impl<T: RangeNum> Iterator for Range<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<T> {
        let cur = self.current;
        if !self.descending {
            if cur < self.end {
                self.current += self.step;
                Some(cur)
            } else {
                None
            }
        } else {
            if cur > self.end {
                self.current -= self.step;
                Some(cur)
            } else {
                None
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Builder<T: RangeNum> {
    start: T,
    end: T,
}

impl<T: RangeNum> Builder<T> {
    #[inline]
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
    #[inline]
    pub fn by(self, step: T) -> Range<T> {
        Range::new(self.start, self.end, step)
    }
}

impl<T: RangeNum> IntoIterator for Builder<T> {
    type Item = T;
    type IntoIter = Range<T>;
    #[inline]
    fn into_iter(self) -> Range<T> {
        Range::new(self.start, self.end, T::one())
    }
}

pub trait RangeExt: Sized + RangeNum {
    fn to(self, end: Self) -> Builder<Self>;
}

impl<T: RangeNum> RangeExt for T {
    #[inline]
    fn to(self, end: T) -> Builder<T> {
        Builder::new(self, end)
    }
}

#[derive(Clone, Copy)]
pub struct CharRange {
    current: char,
    end: char,
    step: u32,
    descending: bool,
}

impl CharRange {
    #[inline]
    pub fn new(start: char, end: char, step: i32) -> Self {
        let descending = start > end;
        let mut s = step.unsigned_abs() as u32;
        if s == 0 {
            s = 1;
        }
        Self {
            current: start,
            end,
            step: s,
            descending,
        }
    }
}

impl Iterator for CharRange {
    type Item = char;
    #[inline]
    fn next(&mut self) -> Option<char> {
        let cur = self.current as u32;
        let end = self.end as u32;
        if !self.descending {
            if cur < end {
                let out = self.current;
                self.current = core::char::from_u32(cur + self.step)?;
                Some(out)
            } else {
                None
            }
        } else {
            if cur > end {
                let out = self.current;
                self.current = core::char::from_u32(cur.saturating_sub(self.step))?;
                Some(out)
            } else {
                None
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct CharBuilder {
    start: char,
    end: char,
}

impl CharBuilder {
    #[inline]
    pub fn new(start: char, end: char) -> Self {
        Self { start, end }
    }
    #[inline]
    pub fn by(self, step: i32) -> CharRange {
        CharRange::new(self.start, self.end, step)
    }
}

impl IntoIterator for CharBuilder {
    type Item = char;
    type IntoIter = CharRange;
    #[inline]
    fn into_iter(self) -> CharRange {
        CharRange::new(self.start, self.end, 1)
    }
}

pub trait CharRangeExt {
    fn to(self, end: Self) -> CharBuilder;
}

impl CharRangeExt for char {
    #[inline]
    fn to(self, end: char) -> CharBuilder {
        CharBuilder::new(self, end)
    }
}
