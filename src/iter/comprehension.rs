// webrust/src/iter/comprehension.rs
//! # Python-like List and Dictionary Comprehensions
//!
//! Transforms Rust's iterator chains into intuitive Python-like syntax:
//! `.when()` for filtering and `.then()` for mapping with automatic collection.
//!
//! ## Speaking Syntax
//!
//! - `when(predicate)` = filter ("when this condition is true")
//! - `then(mapper)` = map + collect automatically
//!
//! ## List Comprehensions
//!
//! ```rust
//! use webrust::prelude::*;
//! # fn example() {
//! // [x**2 for x in range(10)]
//! let squares: Vec<i32> = 0.to(10).then(|x| x * x);
//!
//! // [x for x in range(20) if x % 2 == 0]
//! let evens: Vec<i32> = 0.to(20)
//!     .when(|&x| x % 2 == 0)
//!     .then(|x| x);
//! # }
//! ```
//!
//! ## Dictionary Comprehensions
//!
//! ```rust
//! use webrust::prelude::*;
//! use std::collections::HashMap;
//! # fn example() {
//! // {x: x**2 for x in range(5)}
//! let squares: HashMap<i32, i32> = 0.to(5).then(|x| (x, x * x));
//!
//! // {c: ord(c) for c in 'abc'}
//! let codes: HashMap<char, u8> = 'a'.to('d').then(|c| (c, c as u8));
//! # }
//! ```
//!
//! ## Chained Filters
//!
//! ```rust
//! use webrust::prelude::*;
//! # fn example() {
//! // Multiple conditions (webrust advantage)
//! let result: Vec<i32> = 0.to(100)
//!     .when(|&x| x % 2 == 0)    // Even numbers
//!     .when(|&x| x % 3 == 0)    // Divisible by 3
//!     .then(|x| x * x);         // Square them
//! # }
//! ```
//!
//! Type inference automatically detects `Vec<T>` vs `HashMap<K,V>`
//! based on the mapper's return type.

pub trait WhenThen<T>: IntoIterator<Item = T> + Sized {
    fn when<F>(self, predicate: F) -> std::iter::Filter<Self::IntoIter, F>
    where
        F: FnMut(&T) -> bool,
    {
        self.into_iter().filter(predicate)
    }

    fn then<U, F, C>(self, mapper: F) -> C
    where
        F: FnMut(T) -> U,
        C: FromIterator<U>,
    {
        self.into_iter().map(mapper).collect()
    }
}

impl<T, I> WhenThen<T> for I where I: IntoIterator<Item = T> {}