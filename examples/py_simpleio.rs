// examples/py_simpleio.rs
// Run with: cargo run --example simpleio
//
// This example demonstrates WebRust's basic input/output capabilities:
// - Type-safe user input with automatic parsing
// - F-string interpolation with expressions
// - Inline color styling with @(color) syntax
// - Conditional expressions (ternary operator)
// - Type conversions and turbofish operator
// - Sticky positioned elements
//
// Core features showcased:
// * `input<T>()` - Type-safe input with automatic parsing for String, i32, f64, bool, char
// * `println("{var}")` - F-string interpolation with variables and expressions
// * `@(color, style)text@()` - Inline styling (color, bold, italic, etc.)
// * `.at(x, y).sticky()` - Absolute positioning with sticky elements
// * `coord("cartesian")` - Coordinate system switching
//
// Tips:
// * F-strings support any Rust expression: {age * 12}, {name.to_uppercase()}
// * Colors can be named (blue, red) or CSS values (rgb(255,0,0), #FF0000)
// * Styles: bold, italic, underline, strikethrough
// * Turbofish `::<Type>` required for ambiguous type conversions

use webrust::prelude::*;

#[gui(bg = "navy", fg = "white", font = "Courier New", color = "black", size = "10px")]
fn main() {
    println("@(blue, bold, italic)🎯 WebRust Personal Information Collector");
    println("@(gray, italic)Please fill in your details below:");

    // -------------------------------------------------------------------------
    // 1) Basic input and variable display
    // -------------------------------------------------------------------------
    println("@(green, bold, italic)1. Basic input and variable display:");

    let first_name: String = input("Your first name:");
    let last_name: String = input("Your last name:");
    let age: i32 = input("Your age:");
    let height: f64 = input("Your height (in meters):");
    let married: bool = input("Are you married (true/false):");
    let favorite_letter: char = input("What is your favorite letter?");

    let status = if married { "you are" } else { "you are not" };

    println("@(navy)Hello, @(green, bold){first_name} @(red, bold){last_name}@(navy), you are @(yellow){age}@(navy) years old, you are @(blue){height}@(navy) m tall, your favorite letter is @(magenta){favorite_letter}@(navy), and @(orange, bold){status}@(navy) married.");

    // Sticky positioned elements demonstration
    print("@(white)● {first_name} {last_name} online")
        .at(-20, 0.0)
        .background("green")
        .radius(10)
        .sticky();

    coord("cartesian");
    print("@(white)● In the middle")
        .at(0.0, 0.0)
        .background("red")
        .radius(5)
        .sticky();
    coord("css");

    // -------------------------------------------------------------------------
    // 2) Expressions in placeholders
    // -------------------------------------------------------------------------
    println("\n@(green, bold)2. Expressions in placeholders:");

    println("Age in months: @(yellow){age * 12}");
    println("Height in cm: @(blue){height * 100.0:.0}");
    println("Last name in uppercase: @(red, bold){last_name.to_uppercase()}");

    let first_letter = last_name.chars().next().unwrap_or('?');
    println("First letter of the last name: @(magenta){first_letter}");
    println("Is your favorite letter uppercase? @(navy){favorite_letter.is_uppercase()}");

    let letter_category = if favorite_letter.is_alphabetic() {
        if favorite_letter.is_ascii_lowercase() {
            "lowercase letter"
        } else {
            "uppercase letter"
        }
    } else if favorite_letter.is_numeric() {
        "digit"
    } else {
        "special character"
    };
    println("Letter category: @(bright_green){letter_category}");

    // -------------------------------------------------------------------------
    // 3) Ternary operator (conditional expressions)
    // -------------------------------------------------------------------------
    println("\n@(purple, bold)3. Ternary operator (conditional expressions):");

    let age_category = if age < 18 {
        "minor"
    } else if age < 65 {
        "adult"
    } else {
        "senior"
    };
    println("Age category: @(yellow){age_category}");

    let height_category = if height < 1.60 {
        "short"
    } else if height > 1.80 {
        "tall"
    } else {
        "average"
    };
    println("Height category: @(blue){height_category}");

    let name_length_status = if first_name.len() + last_name.len() > 10 {
        "long name"
    } else {
        "short name"
    };
    println("Name length status: @(green){name_length_status}");

    let marital_emoji = if married { "💍" } else { "🔓" };
    println("Marital emoji: @(pink){marital_emoji}");

    let letter_type_emoji = if favorite_letter.is_alphabetic() {
        "📝"
    } else if favorite_letter.is_numeric() {
        "🔢"
    } else {
        "🎯"
    };
    println("Letter type emoji: @(navy){letter_type_emoji}");

    let age_comparison = if age > 25 {
        "older than 25"
    } else {
        "25 or younger"
    };
    println("Age comparison: @(orange){age_comparison}");

    // -------------------------------------------------------------------------
    // 4) Turbofish operator (explicit type annotations)
    // -------------------------------------------------------------------------
    println("\n@(red, bold)4. Turbofish operator (explicit type annotations):");

    let number_str = age.to_string();
    let parsed_u32 = number_str.parse::<u32>().unwrap_or(0);
    println("String to u32: @(yellow){parsed_u32}");

    let parsed_f64 = number_str.parse::<f64>().unwrap_or(0.0);
    println("String to f64: @(blue){parsed_f64}");

    let height_u8 = height as u8;
    println("Height to u8 (truncated): @(green){height_u8}");

    let chars_vec = first_name.chars().collect::<Vec<char>>();
    println("Collect chars: @(magenta){chars_vec:c}");

    let parsed_height = height.to_string().parse::<f32>().unwrap_or(0.0);
    println("Parse height string: @(navy){parsed_height}");

    println("\n@(bright_green, bold)✨ Simple I/O demonstration complete!");
}