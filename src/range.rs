//! # Python-like Ranges in Rust
//!
//! This module provides structures and traits for creating Python-like ranges,
//! supporting numeric types and characters.
//!
//! ## Features
//!
//! - **Numeric ranges**: Support for all numeric types (integers, floats)
//! - **Character ranges**: Character iteration with customizable step
//! - **Fluent syntax**: `start.to(end)` and `start.to(end).by(step)`
//! - **Automatic direction**: Automatic detection of direction (ascending/descending)
//! - **Negative steps**: Support for descending iterations
//!
//! ## Usage Examples
//!
//! ```rust
//! use webrust::range::RangeExt;
//!
//! // Ascending range
//! for i in 0.to(10) {
//!     println!("{i}"); // 0, 1, 2, ..., 9
//! }
//!
//! // Range with custom step
//! for i in 0.to(10).by(2) {
//!     println!("{i}"); // 0, 2, 4, 6, 8
//! }
//!
//! // Descending range
//! for i in 10.to(0) {
//!     println!("{i}"); // 10, 9, 8, ..., 1
//! }
//!
//! // Range with negative step
//! for i in 20.to(0).by(-3) {
//!     println!("{i}"); // 20, 17, 14, 11, 8, 5, 2
//! }
//! ```

use std::ops::AddAssign;

/// Trait pour définir la valeur "un" pour un type
pub trait One {
    fn one() -> Self;
}

/// Implémentation de One pour les types numériques standards
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

/// Structure représentant un range numérique avec pas personnalisable
#[derive(Clone, Copy)]
pub struct Range<T> {
    current: T,
    end: T,
    step: T,
}

impl<T: Copy + PartialOrd + AddAssign + Default> Range<T> {
    /// Crée un nouveau range avec start, end et step spécifiés
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

/// Builder pour créer des ranges numériques
#[derive(Clone, Copy)]
pub struct Builder<T> {
    start: T,
    end: T,
}

impl<T: Copy + PartialOrd + AddAssign + Default + One + std::ops::Neg<Output = T>> Builder<T> {
    /// Crée un nouveau builder avec start et end
    #[inline]
    pub fn new(start: T, end: T) -> Self {
        Builder { start, end }
    }

    /// Spécifie le pas du range
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

/// Trait d'extension pour créer des ranges numériques
pub trait RangeExt: Sized {
    /// Crée un range de self vers end
    ///
    /// # Exemples
    ///
    /// // Range croissant
    /// for i in 0.to(5) {
    ///     println!("{}", i); // 0, 1, 2, 3, 4
    /// }
    ///
    /// // Range décroissant
    /// for i in 5.to(0) {
    ///     println!("{}", i); // 5, 4, 3, 2, 1
    /// }
    ///
    /// // Avec pas personnalisé
    /// for i in 0.to(10).by(2) {
    ///     println!("{}", i); // 0, 2, 4, 6, 8
    /// }
    ///
    fn to(self, end: Self) -> Builder<Self>;
}

impl<T: Copy + PartialOrd + AddAssign + Default + One + std::ops::Neg<Output = T>> RangeExt for T {
    #[inline]
    fn to(self, end: T) -> Builder<T> {
        Builder::new(self, end)
    }
}

/// Structure représentant un range de caractères
#[derive(Clone, Copy)]
pub struct CharRange {
    current: char,
    end: char,
    step: i32,
}

impl CharRange {
    /// Crée un nouveau range de caractères
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

/// Builder pour créer des ranges de caractères
#[derive(Clone, Copy)]
pub struct CharBuilder {
    start: char,
    end: char,
}

impl CharBuilder {
    /// Crée un nouveau builder de caractères
    #[inline]
    pub fn new(start: char, end: char) -> Self {
        CharBuilder { start, end }
    }

    /// Spécifie le pas du range de caractères
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

/// Trait d'extension pour créer des ranges de caractères
pub trait CharRangeExt {
    /// Crée un range de caractères de self vers end
    ///
    /// # Exemples
    ///
    /// // Range croissant
    /// for c in 'a'.to('z') {
    ///     print!("{} ", c); // a b c ... y
    /// }
    ///
    /// // Range décroissant
    /// for c in 'z'.to('a') {
    ///     print!("{} ", c); // z y x ... b
    /// }
    ///
    /// // Avec pas personnalisé
    /// for c in 'A'.to('Z').by(2) {
    ///     print!("{} ", c); // A C E G ... Y
    /// }
    ///
    fn to(self, end: Self) -> CharBuilder;
}

impl CharRangeExt for char {
    #[inline]
    fn to(self, end: char) -> CharBuilder {
        CharBuilder::new(self, end)
    }
}