//! # Data Structures and String Manipulation
//!
//! Provides intelligent data visualization, professional table generation,
//! and Python-like string manipulation with zero-cost abstractions.
//!
//! ## Key Features
//!
//! - **Smart table generation** - Automatic visualization from any data structure
//! - **Python-like strings** - Familiar methods with Rust performance
//! - **Intelligent splitting** - One `split_by()` method for all patterns
//! - **Rich string operations** - Case conversion, padding, validation
//! - **Fluent chaining** - Method chaining for elegant data processing
//! - **Type safety** - All operations maintain Rust's safety guarantees
//!
//! ## Modules
//!
//! - [`table`] - Intelligent table generation with pivot, merge, and formatting
//! - [`string`] - Python-like string methods with smart pattern recognition
//!
//! ## Quick Examples
//!
//! ### Automatic Table Generation
//!
//! use webrust::prelude::*;
//! use std::collections::HashMap;
//!
//! let mut scores = HashMap::new();
//! scores.insert("Alice", 95);
//! scores.insert("Bob", 87);
//! scores.insert("Charlie", 92);
//!
//! // Automatic table with intelligent formatting
//! table(&scores);
//!
//! // With custom headers and styling  
//! table(&scores).header(["Student", "Score"]).pivot();
//!
//!
//! ### Python-like String Processing
//!
//! use webrust::prelude::*;
//!
//! // Intelligent splitting - one method, multiple patterns
//! let langs = "python,rust,go".split_by(",");        // ["python", "rust", "go"]
//! let words = "hello  world\ttab".split_by("");       // ["hello", "world", "tab"] 
//! let lines = "L1\nL2\nL3".split_by("\n");           // ["L1", "L2", "L3"]
//!
//! // Fluent chaining with join
//! let result = "a,b,c".split_by(",").join(" → ");     // "a → b → c"
//!
//! // Rich string methods (Python-compatible)
//! let text = "hello world".title();                   // "Hello World"
//! let padded = "42".zfill(6);                         // "000042"
//! let centered = "rust".center(10, '*');              // "***rust***"
//!
//!
//! ## Advanced String Operations
//!
//! ### Case Transformations
//!
//! let text = "Hello WORLD";
//!
//! println!("Upper: {}", text.upper());           // "HELLO WORLD"
//! println!("Lower: {}", text.lower());           // "hello world"  
//! println!("Title: {}", text.title());           // "Hello World"
//! println!("Capitalized: {}", text.capitalize()); // "Hello world"
//! println!("Swapped: {}", text.swapcase());      // "hELLO world"
//!
//!
//! ### String Testing and Validation
//!
//! let email = "user@example.com";
//!
//! // Python-like testing methods
//! println!("Starts with user: {}", email.startswith("user"));    // true
//! println!("Ends with .com: {}", email.endswith(".com"));        // true
//! println!("Is alphabetic: {}", "hello".isalpha());              // true
//! println!("Is numeric: {}", "12345".isdigit());                 // true
//! println!("Count 'an': {}", "banana".count("an"));              // 2
//!
//!
//! ### Advanced Formatting and Padding
//!
//! // Zero-fill padding (perfect for IDs)
//! println!("ID: {}", "42".zfill(6));                    // "000042"
//!
//! // Text alignment and padding
//! println!("Left: '{}'", "text".ljust(10, '-'));       // "text------"
//! println!("Right: '{}'", "text".rjust(10, '-'));      // "------text"
//! println!("Center: '{}'", "text".center(10, '*'));    // "***text***"
//!
//!
//! ## Smart Pattern Recognition
//!
//! The `split_by()` method intelligently handles different splitting patterns:
//!
//!
//! // CSV data processing
//! let csv = "name,age,city\nAlice,25,Paris\nBob,30,Lyon";
//! let rows = csv.split_by("\n");                    // Line splitting
//! let headers = rows[0].split_by(",");              // Delimiter splitting
//!
//! // Text processing  
//! let sentence = "The quick brown fox";
//! let words = sentence.split_by("");                // Whitespace splitting
//!
//! // Chained processing
//! let clean_data = "  rust, python,  go  "
//!     .split_by(",")
//!     .iter()
//!     .map(|s| s.trim())
//!     .collect::<Vec<_>>()
//!     .join(" | ");                                 // "rust | python | go"
//!
//!
//! ## Data Visualization Intelligence
//!
//! The table module automatically detects data patterns and generates
//! appropriate visualizations:
//!
//!
//! // Nested structures
//! let mut nested_data = HashMap::new();
//! nested_data.insert("users", vec!["Alice", "Bob", "Charlie"]);
//! nested_data.insert("scores", vec!["95", "87", "92"]);
//!
//! table(&nested_data);  // Automatically formatted table
//!
//! // Complex data with relationships  
//! let sales_data = vec![
//!     ("Q1", "Product A", 1500),
//!     ("Q1", "Product B", 1200),
//!     ("Q2", "Product A", 1800),
//!     ("Q2", "Product B", 1600),
//! ];
//!
//! table(&sales_data).header(["Quarter", "Product", "Sales"]);
//!
//!
//! ## Performance and Safety
//!
//! All string operations are:
//! - **Zero-cost abstractions** - Compile to optimal Rust code
//! - **Memory efficient** - Pre-allocated capacity where possible  
//! - **UTF-8 safe** - Full Unicode support with proper handling
//! - **Type safe** - Compile-time guarantees prevent runtime errors
//! - **Inline optimized** - All methods marked `#[inline]` for performance
//!
//! ## Real-World Integration
//!
//! ### Log Processing
//!
//! let log_line = "2024-01-15 INFO user@example.com Successfully logged in";
//! let parts = log_line.split_by(" ");
//! let email = parts.iter().find(|s| s.contains("@")).unwrap_or(&"unknown");
//! let domain = email.split_by("@")[1];
//!
//! println!("User from domain: {}", domain);
//!
//!
//! ### Data Cleaning Pipeline
//!
//! let raw_data = "  Alice,  25,   Paris  \n  Bob,30,Lyon\n";
//! let clean_records: Vec<Vec<String>> = raw_data
//!     .split_by("\n")
//!     .iter()
//!     .filter(|line| !line.trim().is_empty())
//!     .map(|line| {
//!         line.split_by(",")
//!             .iter()
//!             .map(|field| field.trim().to_string())
//!             .collect()
//!     })
//!     .collect();
//!
//! table(&clean_records).header(["Name", "Age", "City"]);
//!
//!
//! ### Configuration Processing
//!
//! let config = "database.host=localhost\ndatabase.port=5432\napp.debug=true";
//! let settings: std::collections::HashMap<String, String> = config
//!     .split_by("\n")
//!     .iter()
//!     .filter_map(|line| {
//!         let parts = line.split_by("=");
//!         if parts.len() == 2 {
//!             Some((parts[0].to_string(), parts[1].to_string()))
//!         } else { None }
//!     })
//!     .collect();
//!
//! table(&settings).header(["Setting", "Value"]);
//!
//!
//! The data module bridges Python's string processing elegance with Rust's
//! performance and safety, making WebRust ideal for data manipulation,
//! text processing, and professional data visualization.

pub mod table;
pub mod string;

pub use table::*;
pub use string::*;