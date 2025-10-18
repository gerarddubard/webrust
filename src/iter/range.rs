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

use std::ops::AddAssign;

pub trait One {
    fn one() -> Self;
}

macro_rules! impl_one {
    ($($t:ty => $v:expr),*) => {
        $(impl One for $t {
            #[inline]
            fn one() -> Self {
                $v
            }
        })*
    };
}

impl_one!(
    i8 => 1, i16 => 1, i32 => 1, i64 => 1, i128 => 1, isize => 1,
    u8 => 1, u16 => 1, u32 => 1, u64 => 1, u128 => 1, usize => 1,
    f32 => 1.0, f64 => 1.0
);

#[derive(Clone, Copy)]
pub struct Range<T> {
    current: T,
    end: T,
    step: T,
}

impl<T: Copy + PartialOrd + AddAssign + Default> Range<T> {
    #[inline]
    pub fn new(start: T, end: T, step: T) -> Self {
        Range {
            current: start,
            end,
            step,
        }
    }
}

impl<T: Copy + PartialOrd + AddAssign + Default> Iterator for Range<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let current = self.current;
        let zero = T::default();

        if (self.step > zero && current < self.end) || (self.step < zero && current > self.end) {
            self.current += self.step;
            Some(current)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Builder<T> {
    start: T,
    end: T,
}

impl<T: Copy + PartialOrd + AddAssign + Default + One + std::ops::Neg<Output = T>> Builder<T> {
    #[inline]
    pub fn new(start: T, end: T) -> Self {
        Builder { start, end }
    }
    #[inline]
    pub fn by(self, step: T) -> Range<T> {
        Range::new(self.start, self.end, step)
    }
}

impl<T: Copy + PartialOrd + AddAssign + Default + One + std::ops::Neg<Output = T>> IntoIterator for Builder<T> {
    type Item = T;
    type IntoIter = Range<T>;

    #[inline]
    fn into_iter(self) -> Range<T> {
        Range::new(
            self.start,
            self.end,
            if self.start <= self.end { T::one() } else { -T::one() }
        )
    }
}

pub trait RangeExt: Sized {
    fn to(self, end: Self) -> Builder<Self>;
}

impl<T: Copy + PartialOrd + AddAssign + Default + One + std::ops::Neg<Output = T>> RangeExt for T {
    #[inline]
    fn to(self, end: T) -> Builder<T> {
        Builder::new(self, end)
    }
}

#[derive(Clone, Copy)]
pub struct CharRange {
    current: char,
    end: char,
    step: i32,
}

impl CharRange {
    #[inline]
    pub fn new(start: char, end: char, step: i32) -> Self {
        CharRange {
            current: start,
            end,
            step,
        }
    }
}

impl Iterator for CharRange {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        let current = self.current;
        let (c, e) = (current as u32, self.end as u32);
        if (self.step > 0 && c < e) || (self.step < 0 && c > e) {
            if let Some(next) = char::from_u32((c as i32 + self.step) as u32) {
                self.current = next;
                Some(current)
            } else {
                None
            }
        } else {
            None
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
        CharBuilder { start, end }
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
        CharRange::new(
            self.start,
            self.end,
            if self.start <= self.end { 1 } else { -1 }
        )
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