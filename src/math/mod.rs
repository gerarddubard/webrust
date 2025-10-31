// webrust/src/math/mod.rs
//! # Mathematical Functions
//!
//! Combinatorial and statistical functions for scientific computing.
//!
//! ## Modules
//!
//! - [`stat`] - Combinatorics (arrangements, combinations, permutations)
//!
//! ## Quick Reference
//!
//! ```rust
//! use webrust::prelude::*;
//! # fn example() {
//! let comb = C(5, 2);    // Combinations: 5 choose 2 = 10
//! let arr = A(5, 2);     // Arrangements: 5 arrange 2 = 20
//! let perm = P(5);       // Permutations: 5! = 120
//! # }
//! ```

pub mod stat;

pub use stat::*;
