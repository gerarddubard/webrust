// webrust/src/math/mod.rs
//! # Combinatorics Functions
//!
//! High-performance combinatorial calculations with overflow-safe algorithms.
//! Uses single-letter names matching mathematical notation.
//!
//! ## Functions
//!
//! - `C(n, k)` - **Combinations** (n choose k) : C(n,k) = n! / (k!(n-k)!)
//! - `A(n, k)` - **Arrangements** (n arrange k) : A(n,k) = n! / (n-k)!
//! - `P(n)` - **Permutations** (n factorial) : P(n) = n!
//!
//! All functions return `u64` and use optimized algorithms to minimize
//! intermediate overflow risks.
//!
//! ## Examples
//!
//! ### Combinations (C)
//! ```rust
//! use webrust::prelude::*;
//! # fn example() {
//! // How many ways to choose 2 items from 5?
//! let ways = C(5, 2);  // 10
//! assert_eq!(ways, 10);
//!
//! // Pascal's triangle
//! let row: Vec<u64> = 0.to(6).then(|k| C(5, k));
//! // [1, 5, 10, 10, 5, 1]
//! # }
//! ```
//!
//! ### Arrangements (A)
//! ```rust
//! use webrust::prelude::*;
//! # fn example() {
//! // How many ways to arrange 3 items from 5?
//! let ways = A(5, 3);  // 60
//! assert_eq!(ways, 60);
//!
//! // Podium positions from 8 runners
//! let podiums = A(8, 3);  // 336
//! # }
//! ```
//!
//! ### Permutations (P)
//! ```rust
//! use webrust::prelude::*;
//! # fn example() {
//! // How many ways to arrange 5 items?
//! let ways = P(5);  // 120
//! assert_eq!(ways, 120);
//!
//! // Permutations for larger sets
//! let large = P(10);  // 3,628,800
//! # }
//! ```
//!
//! ### Real-World Example: Pascal's Triangle
//! ```rust
//! use webrust::prelude::*;
//! # #[gui] fn example() {
//! println("<darkred b mt25>Pascal's Triangle");
//! let triangle: Vec<Vec<u64>> = 0.to(10).then(|n| {
//!     0.to(n + 1).then(|k| C(n, k))
//! });
//! table(&triangle);
//! # }
//! ```
//!
//! ## Algorithm Details
//!
//! - **C(n, k)**: Uses symmetry property C(n,k) = C(n,n-k) and
//!   incremental multiplication/division to avoid factorial overflow
//! - **A(n, k)**: Direct product avoiding full factorial computation
//! - **P(n)**: Simple factorial with sequential multiplication
//!
//! ## Performance
//!
//! All functions are `#[inline]` and compile to tight loops.
//! Time complexity: O(k) for C and A, O(n) for P.

#[inline]
#[allow(non_snake_case)]
pub fn C(n: u32, k: u32) -> u64 {
    let k = k.min(n - k);
    let mut res = 1u64;
    for i in 0..k {
        res = res * (n - i) as u64 / (i + 1) as u64;
    }
    res
}

#[inline]
#[allow(non_snake_case)]
pub fn A(n: u32, k: u32) -> u64 {
    let mut res = 1u64;
    for i in 0..k {
        res *= (n - i) as u64;
    }
    res
}

#[inline]
#[allow(non_snake_case)]
pub fn P(n: u32) -> u64 {
    let mut res = 1u64;
    for i in 2..=n {
        res *= i as u64;
    }
    res
}
