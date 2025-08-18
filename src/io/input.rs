// webrust/src/io/input.rs
//! # Type-Safe Input with Real-Time Validation
//!
//! Provides robust, type-safe input handling with client-side validation
//! and user-friendly error messages. Supports all common Rust types
//! with automatic parsing and retry logic.
//!
//! ## Supported Types
//!
//! - **Integers**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
//! - **Unsigned**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`  
//! - **Floats**: `f32`, `f64`
//! - **Other**: `bool`, `char`, `String`
//! - **Custom**: Any type implementing `FromStr`
//!
//! ## Basic Usage
//!
//!
//! let age: i32 = input("Enter your age:");           // Validates as integer
//! let height: f64 = input("Enter your height:");     // Validates as float  
//! let married: bool = input("Are you married?");     // Validates as boolean
//! let name: String = input("Enter your name:");      // No validation needed
//!
//!
//! ## Validation Process
//!
//! 1. **Client-side validation** - Immediate feedback in browser
//! 2. **Server-side parsing** - Type-safe conversion in Rust
//! 3. **Error handling** - Clear, actionable error messages
//! 4. **Automatic retry** - Re-prompt on validation failure
//!
//! ## Advanced Usage
//!
//!
//! // For custom error handling
//! match try_input::<i32>("Enter number:") {
//!     Ok(num) => println("Valid: {num}"),
//!     Err(e) => println("Invalid: {e}"),
//! }
//!
//! // Explicit string input
//! let text = input_string("Enter any text:");
//!
//!
//! ## Error Messages
//!
//! Provides helpful, type-specific validation errors:
//! - "invalid digit found in string" for integers
//! - "invalid float literal" for floating-point numbers
//! - "provided string was not 'true' or 'false'" for booleans

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
