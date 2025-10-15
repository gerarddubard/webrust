// webrust/src/io/input.rs
//! # Type-Safe Input with Real-Time Validation
//!
//! Robust, type-safe input handling with client-side and server-side validation,
//! automatic type parsing, and user-friendly error messages.
//!
//! ## Supported Types
//!
//! All primitive types implementing `FromStr`:
//! - Integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
//! - Floats: `f32`, `f64`
//! - Other: `bool`, `char`, `String`
//! - Custom types implementing `FromStr`
//!
//! ## Examples
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! # #[gui] fn example() {
//! // Type inference from annotation
//! let age: i32 = input("Enter your age:");
//! let height: f64 = input("Enter height in meters:");
//! let married: bool = input("Married? (true/false):");
//!
//! // String input (no validation)
//! let name: String = input("Enter name:");
//! # }
//! ```
//!
//! Validation happens in two stages: JavaScript validates in the browser
//! for immediate feedback, then Rust validates on the server for type safety.
//! Invalid inputs trigger clear error messages and automatic retry.

use crate::io::gui::create_input_request_typed;
use std::str::FromStr;

pub fn input_with_validation<T>(prompt: &str) -> T
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    loop {
        let user_input = create_input_request_typed::<T>(prompt);
        match user_input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => { continue; }
        }
    }
}

pub fn input_string(prompt: &str) -> String { create_input_request_typed::<String>(prompt) }

pub fn try_input<T>(prompt: &str) -> Result<T, T::Err>
where
    T: FromStr,
{
    let user_input = create_input_request_typed::<T>(prompt);
    user_input.trim().parse()
}

pub use input_with_validation as input;
