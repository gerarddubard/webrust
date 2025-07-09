//! # Input Module - Type-Safe User Input with Validation
//!
//! This module provides robust, type-safe input handling with real-time validation
//! and error feedback. It bridges the gap between Rust's strict type system and
//! user-friendly input collection.
//!
//! ## Core Philosophy
//!
//! Traditional console input in Rust requires manual parsing and error handling.
//! webrust simplifies this by:
//!
//! 1. **Automatic Type Detection**: Uses Rust's type system to determine expected input
//! 2. **Real-time Validation**: Validates input on the client side before submission
//! 3. **User-Friendly Errors**: Provides clear, actionable error messages
//! 4. **Retry Logic**: Automatically prompts for re-entry on validation failure
//!
//! ## Supported Types
//!
//! The input system supports all common Rust types:
//!
//! - **Integers**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
//! - **Unsigned Integers**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
//! - **Floats**: `f32`, `f64`
//! - **Boolean**: `bool` (accepts "true"/"false", case-insensitive)
//! - **Character**: `char` (single Unicode character)
//! - **String**: `String` (any text input)
//! - **Custom Types**: Any type implementing `FromStr`
//!
//! ## Functions
//!
//! ### `input_with_validation<T>(prompt: &str) -> T`
//!
//! The primary input function that:
//! 1. Displays the prompt to the user
//! 2. Creates a typed input request in the web interface
//! 3. Validates the input according to the target type
//! 4. Retries automatically if validation fails
//! 5. Returns the parsed value once valid input is received
//!
//! **Type inference makes this incredibly simple:**
//!
//! let age: i32 = input("Enter your age:");           // Validates as integer
//! let height: f64 = input("Enter your height:");     // Validates as float
//! let married: bool = input("Are you married?");     // Validates as boolean
//!
//!
//! ### `input_string(prompt: &str) -> String`
//!
//! Specialized function for string input:
//! - No validation required (all input is valid string)
//! - More explicit than using type annotation
//! - Useful when the generic version causes inference issues
//!
//! ### `try_input<T>(prompt: &str) -> Result<T, T::Err>`
//!
//! Non-blocking input function that:
//! - Returns `Result` instead of retrying on failure
//! - Allows custom error handling by the caller
//! - Useful for advanced validation scenarios
//!
//! ## Validation Process
//!
//! The validation happens in two stages:
//!
//! ### 1. Client-Side Validation (JavaScript)
//!
//! Before submission, the web interface:
//! 1. Sends input to `/api/validate` endpoint
//! 2. Server attempts to parse the input according to expected type
//! 3. Returns validation result with error message if invalid
//! 4. Client displays error and prevents submission if invalid
//!
//! ### 2. Server-Side Validation (Rust)
//!
//! The `validate_input_rust` function:
//! 1. Receives the type name and input value
//! 2. Attempts parsing using the appropriate `FromStr` implementation
//! 3. Returns detailed error messages for common parsing failures
//!
//! ## Error Messages
//!
//! The system provides helpful, type-specific error messages:
//!
//! - **Integer**: "invalid digit found in string"
//! - **Float**: "invalid float literal" 
//! - **Boolean**: "provided string was not 'true' or 'false'"
//! - **Character**: "input too long for character"
//!
//! ## Integration with F-Strings
//!
//! Input values work seamlessly with webrust f-string formatting:
//!
//!
//! let name: String = input("Name:");
//! let age: i32 = input("Age:");
//!
//! println("Hello @(green){name}@(reset), you are @(yellow){age}@(reset) years old!");
//!
//!
//! ## Advanced Usage
//!
//! ### Custom Types
//!
//! Any type implementing `FromStr` can be used:
//!
//!
//! use std::net::IpAddr;
//!
//! let ip: IpAddr = input("Enter IP address:");  // Validates IP format
//!
//!
//! ### Error Handling
//!
//!
//! match `try_input::<i32>`("Enter number:") {
//!     Ok(num) => println("You entered: {num}"),
//!     Err(e) => println("Invalid input: {e}"),
//! }
//!
//!
//! ## Implementation Notes
//!
//! - Uses `create_input_request_typed<T>` from the GUI module
//! - Type information is extracted using `std::any::type_name::<T>()`
//! - Validation occurs both client-side (for UX) and server-side (for safety)
//! - Thread-safe communication via channels between GUI and input threads

use crate::gui::create_input_request_typed;
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
