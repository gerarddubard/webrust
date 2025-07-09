//! # Print Module - Enhanced Output with Styling
//!
//! This module provides enhanced printing capabilities with CSS-like styling support.
//! It extends Rust's standard printing with color, formatting, and HTML rendering
//! capabilities for the web interface.
//!
//! ## Styling System
//!
//! webrust uses a unique styling syntax inspired by CSS:
//!
//!
//! println("@(red, bold)Error:@(reset) Something went wrong!");
//! println("@(green)Success:@(reset) Operation completed @(blue, italic)successfully@(reset)!");
//!
//!
//! ### Supported Colors
//! - **Basic Colors**: black, red, green, yellow, blue, magenta, cyan, white
//! - **Extended Colors**: gray/grey, orange, purple, pink, bright_cyan
//! - **Background Colors**: bg-black, bg-red, bg-green, bg-yellow, etc.
//!
//! ### Supported Styles
//! - **bold**: Makes text bold
//! - **italic**: Makes text italic
//! - **underline**: Underlines text
//! - **strike**: Strikes through text
//!
//! ## Implementation Details
//!
//! ### `process_styles(text: &str) -> String`
//!
//! The core styling processor that:
//! 1. Uses regex to find styling markers: `@(style1, style2)content`
//! 2. Converts style names to CSS properties
//! 3. Wraps content in HTML `<span>` elements with inline styles
//! 4. Preserves formatting for code-like content
//!
//! ### `preserve_formatting(text: &str) -> String`
//!
//! Intelligently preserves formatting for structured content:
//! - Detects code-like content (contains `{`, `[`, newlines)
//! - Wraps in `<pre>` tags with monospace font
//! - Converts newlines to `<br>` tags for simple text
//!
//! ## Functions
//!
//! ### `print_str<T: std::fmt::Display>(text: T)`
//!
//! Prints text without a trailing newline:
//! - Processes styling markers
//! - Adds to web interface output buffer
//! - Supports any type implementing `Display`
//!
//! ### `println_str<T: std::fmt::Display>(text: T)`
//!
//! Prints text with a trailing newline (`<br>` in web interface):
//! - Same as `print_str` but adds line break
//! - Most commonly used printing function
//!
//! ## Color Mapping
//!
//! Colors are mapped to web-safe, dark-theme friendly values:
//! - `red` → `#ff6b6b` (soft red, easier on eyes)
//! - `green` → `#51cf66` (vibrant green)
//! - `blue` → `#339af0` (bright blue)
//! - `yellow` → `#ffd43b` (warm yellow)
//!
//! This ensures good readability on the dark terminal background.
//!
//! ## Usage Examples
//!
//!
//! // Basic colored output
//! println("@(red)Error: @(yellow)File not found");
//!
//! // Multiple styles
//! println("@(green, bold, underline)SUCCESS@(reset)");
//!
//! // Background colors
//! println("@(white, bg-red) CRITICAL @(reset)");
//!
//! // Mixed content
//! println("Status: @(green)OK@(reset), Code: @(blue)200@(reset)");
//!

use crate::gui::{add_output_same_line, add_output_new_line};
use regex::Regex;

fn process_styles(text: &str) -> String {
    lazy_static::lazy_static! { 
        static ref STYLE_REGEX: Regex = Regex::new(r"@\(([^)]*)\)([^@]*)").unwrap();
    }
    let mut result = text.to_string();
    result = STYLE_REGEX.replace_all(&result, |caps: &regex::Captures| {
        let styles = &caps[1];
        let content = &caps[2];
        if styles.is_empty() {
            return content.to_string();
        }

        let mut css_styles = Vec::new();
        for style in styles.split(',').map(|s| s.trim()) {
            match style {
                "black" => css_styles.push("color: #000000"),
                "red" => css_styles.push("color: #ff6b6b"),
                "green" => css_styles.push("color: #51cf66"),
                "yellow" => css_styles.push("color: #ffd43b"),
                "blue" => css_styles.push("color: #339af0"),
                "magenta" => css_styles.push("color: #e599f7"),
                "cyan" => css_styles.push("color: #22d3ee"),
                "white" => css_styles.push("color: #ffffff"),
                "gray" | "grey" => css_styles.push("color: #adb5bd"),
                "orange" => css_styles.push("color: #ff922b"),
                "purple" => css_styles.push("color: #9775fa"),
                "pink" => css_styles.push("color: #f783ac"),
                "bright_cyan" => css_styles.push("color: #00ffff"),

                "bold" => css_styles.push("font-weight: bold"),
                "italic" => css_styles.push("font-style: italic"),
                "underline" => css_styles.push("text-decoration: underline"),
                "strike" => css_styles.push("text-decoration: line-through"),

                "bg-black" => css_styles.push("background-color: #000000"),
                "bg-red" => css_styles.push("background-color: #ff6b6b"),
                "bg-green" => css_styles.push("background-color: #51cf66"),
                "bg-yellow" => css_styles.push("background-color: #ffd43b"),
                "bg-blue" => css_styles.push("background-color: #339af0"),
                "bg-magenta" => css_styles.push("background-color: #e599f7"),
                "bg-cyan" => css_styles.push("background-color: #22d3ee"),
                "bg-white" => css_styles.push("background-color: #ffffff"),
                "bg-gray" | "bg-grey" => css_styles.push("background-color: #adb5bd"),
                _ => {}
            }
        }
        if css_styles.is_empty() {
            content.to_string()
        } else {
            format!(r#"<span style="{}">{}</span>"#, css_styles.join("; "), content)
        }
    }).to_string();
    preserve_formatting(&result)
}

fn preserve_formatting(text: &str) -> String {
    if text.contains('\n') && (text.contains('{') || text.contains('[')) {
        format!("<pre style=\"font-family: 'Courier New', monospace; margin: 0; display: inline;\">{}</pre>", text)
    } else {
        text.to_string()
    }
}

pub fn print_str<T: std::fmt::Display>(text: T) {
    let processed = process_styles(&format!("{}", text));
    add_output_same_line(processed);
}

pub fn println_str<T: std::fmt::Display>(text: T) {
    let processed = process_styles(&format!("{}", text));
    add_output_new_line(format!("{}<br>", processed));
}

pub use print_str as print;
pub use println_str as println;