// webrust/src/iter/comprehension.rs
//! # Python-like List and Dictionary Comprehensions
//!
//! This module provides a revolutionary approach to collection processing in Rust,
//! bringing Python's beloved comprehension syntax to Rust with zero performance overhead.
//!
//! ## Overview
//!
//! The `WhenThen` trait transforms Rust's iterator chains into an intuitive, speaking syntax:
//! - **`when()`** = filter (equivalent to Python's `if` clause)  
//! - **`then()`** = map + collect (equivalent to Python's expression + automatic collection)
//!
//! ## Key Features
//!
//! - **🐍 Python-like comprehensions**: Familiar syntax with Rust performance
//! - **🚀 Automatic collection**: No manual `.collect()` calls needed
//! - **🎯 Smart type inference**: Automatically detects `Vec<T>` vs `HashMap<K,V>`
//! - **🔗 Chainable filters**: Multiple `.when()` calls for readable conditions
//! - **⚡ Zero overhead**: Compiles to identical iterator chains as manual code
//! - **🌍 Universal**: Works with any `IntoIterator` type
//!
//! ## Quick Start
//!
//! ```
//! use webrust::prelude::*;
//! use std::collections::HashMap;
//!
//! // List comprehensions
//! let squares: Vec<i32> = 0.to(10).then(|x| x * x);
//! // Python equivalent: [x**2 for x in range(10)]
//!
//! // Dictionary comprehensions (automatic inference!)
//! let squares_dict: HashMap<i32, i32> = 0.to(5).then(|x| (x, x * x));
//! // Python equivalent: {x: x**2 for x in range(5)}
//!
//! // Filtered comprehensions
//! let evens: Vec<i32> = 0.to(20).when(|&x| x % 2 == 0).then(|x| x);
//! // Python equivalent: [x for x in range(20) if x % 2 == 0]
//!
//! // Chained filters (webrust advantage!)
//! let special: Vec<i32> = 0.to(100)
//!     .when(|&x| x % 2 == 0)    // Only evens
//!     .when(|&x| x % 3 == 0)    // Only multiples of 3
//!     .then(|x| x * x);         // Square them
//! ```
//!
//! ## Python vs webrust Comparison
//!
//! ### List Comprehensions
//!
//! | Python | webrust | Result |
//! |--------|---------|--------|
//! | `[x**2 for x in range(10)]` | `0.to(10).then(\|x\| x * x)` | `[0, 1, 4, 9, ...]` |
//! | `[x for x in range(0, 20, 3)]` | `0.to(20).by(3).then(\|x\| x)` | `[0, 3, 6, 9, ...]` |
//! | `[x for x in items if x > 5]` | `items.when(\|&x\| x > 5).then(\|x\| x)` | Filtered items |
//!
//! ### Dictionary Comprehensions
//!
//! | Python | webrust | Type |
//! |--------|---------|------|
//! | `{x: x**2 for x in range(5)}` | `0.to(5).then(\|x\| (x, x * x))` | `HashMap<i32, i32>` |
//! | `{c: ord(c) for c in 'abc'}` | `'a'.to('d').then(\|c\| (c, c as u8))` | `HashMap<char, u8>` |
//!
//! ### Complex Filtering
//!
//! ```
//! use webrust::prelude::*;
//!
//! // Python: [x**2 for x in range(20) if x % 2 == 0 and x % 3 == 0]
//! let result: Vec<i32> = 0.to(20)
//!     .when(|&x| x % 2 == 0 && x % 3 == 0)
//!     .then(|x| x * x);
//!
//! // webrust advantage: chainable filters for readability
//! let result2: Vec<i32> = 0.to(20)
//!     .when(|&x| x % 2 == 0)    // First condition
//!     .when(|&x| x % 3 == 0)    // Second condition  
//!     .then(|x| x * x);         // Transform
//! ```
//!
//! ## Real-World Examples
//!
//! ### Data Processing
//!
//! ```
//! use webrust::prelude::*;
//!
//! let words = vec!["hello", "world", "python", "rust"];
//!
//! // Extract long words in uppercase
//! let long_words: Vec<String> = words
//!     .when(|word| word.len() > 4)
//!     .then(|word| word.upper());
//! // Result: ["HELLO", "WORLD", "PYTHON"]
//! ```
//!
//! ### Mathematical Computations
//!
//! ```
//! use webrust::prelude::*;
//!
//! // Generate Fibonacci sequence
//! let mut fib = vec![0, 1];
//! for i in 2..10 {
//!     let next = fib[i-1] + fib[i-2];
//!     fib.push(next);
//! }
//!
//! // Calculate ratios between consecutive Fibonacci numbers
//! let ratios: Vec<f64> = (1..fib.len())
//!     .when(|&i| fib[i-1] != 0)                    // Avoid division by zero
//!     .then(|i| fib[i] as f64 / fib[i-1] as f64);  // Golden ratio convergence
//! ```
//!
//! ### String Processing
//!
//! ```
//! use webrust::prelude::*;
//!
//! let emails = vec!["user@gmail.com", "test@yahoo.com", "admin@company.org"];
//!
//! // Extract domains from Gmail addresses
//! let gmail_domains: Vec<String> = emails
//!     .when(|email| email.contains("gmail"))
//!     .then(|email| email.split('@').nth(1).unwrap().to_string());
//! ```
//!
//! ## Type Inference Magic
//!
//! The `then()` method automatically infers the correct collection type:
//!
//! ```
//! use webrust::prelude::*;
//! use std::collections::{HashMap, HashSet};
//!
//! // Automatically creates Vec<T>
//! let numbers: Vec<i32> = 0.to(5).then(|x| x * 2);
//!
//! // Automatically creates HashMap<K, V> when returning tuples
//! let mapping: HashMap<char, i32> = 'a'.to('f').then(|c| (c, c as i32));
//!
//! // Works with any FromIterator implementation
//! let set: HashSet<i32> = 0.to(10).when(|&x| x % 2 == 0).then(|x| x);
//! ```
//!
//! ## Performance Notes
//!
//! - **Zero overhead**: Compiles to identical iterator chains as manual Rust code
//! - **Lazy evaluation**: Filters and maps are applied lazily, just like standard iterators
//! - **Single allocation**: `then()` pre-sizes collections when possible
//! - **No boxing**: All closures are compile-time monomorphized
//!
//! ## Compatibility
//!
//! Works with any type implementing `IntoIterator`:
//! - **webrust ranges**: `0.to(10)`, `'a'.to('z')`
//! - **Standard ranges**: `0..10`, `0..=10`
//! - **Vectors**: `vec![1, 2, 3]`
//! - **Arrays**: `[1, 2, 3]`
//! - **Custom iterators**: Any type with `IntoIterator`
//!
//! ## Advanced Patterns
//!
//! ### Nested Processing
//!
//! ```
//! use webrust::prelude::*;
//!
//! let matrix = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
//!
//! // Flatten and process
//! let processed: Vec<i32> = matrix
//!     .into_iter()
//!     .flatten()
//!     .when(|&x| x % 2 == 0)
//!     .then(|x| x * x);
//! ```
//!
//! ### Enumeration with Processing
//!
//! ```
//! use webrust::prelude::*;
//! use std::collections::HashMap;
//!
//! let items = vec!["apple", "banana", "cherry"];
//!
//! // Create indexed mapping
//! let indexed: HashMap<usize, String> = enumerate(&items)
//!     .when(|(_, item)| item.len() > 5)
//!     .then(|(i, item)| (i, item.upper()));
//! ```
//!
//! ## Trait Implementation
//!
//! The `WhenThen` trait is automatically implemented for all `IntoIterator` types:
//!
//! ```rust,ignore
//! pub trait WhenThen<T>: IntoIterator<Item = T> + Sized {
//!     fn when<F>(self, predicate: F) -> std::iter::Filter<Self::IntoIter, F>
//!     where F: FnMut(&T) -> bool;
//!     
//!     fn then<U, F, C>(self, mapper: F) -> C
//!     where F: FnMut(T) -> U, C: FromIterator<U>;
//! }
//! ```
//!
//! ## Speaking Syntax Philosophy
//!
//! webrust comprehensions use "speaking" method names that read naturally:
//! - **`when(condition)`** - "when this condition is true"
//! - **`then(transformation)`** - "then apply this transformation"
//!
//! This creates code that reads like natural language:
//!
//!
//! use webrust::prelude::*;
//!
//! # struct Person { age: i32 }
//! # let people = vec![Person { age: 25 }, Person { age: 17 }];
//! let adults: Vec<Person> = people
//!     .when(|person| person.age >= 18)  // "when person is adult"
//!     .then(|person| person.clone());   // "then keep the person"
//! ```

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