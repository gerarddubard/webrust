// examples/py_advancedio.rs
// Run with: cargo run --example advancedio
//
// This example demonstrates WebRust's advanced I/O capabilities:
// - Collection operations with turbofish operator
// - Complex type conversions and chaining
// - Number formatting (precision, scientific, hex, binary, octal)
// - Container formatting (compact :c and JSON :j)
// - Text alignment (left, center, right, justify)
// - Advanced border styling (weight, style, radius, background)
// - Dynamic table generation with print() loops
//
// Core features showcased:
// * `.collect::<Type>()` - Turbofish for explicit type collection
// * `{value:.2}` - Precision formatting
// * `{value:x}`, `{value:b}`, `{value:o}` - Hex, binary, octal
// * `{container:c}` - Compact debug format
// * `{container:j}` - Pretty-printed JSON format
// * `.width(w).align("mode")` - Text alignment (left/center/right/justify)
// * `.weight(1-5).style("type").radius(px)` - Border customization
// * `.space(px)` - Control spacing between elements
// * Pascal's triangle and multiplication table generation
//
// Tips:
// * Use :c for compact single-line output
// * Use :j for indented JSON with syntax highlighting
// * Combine alignment with width for precise layout control
// * Border styles: solid, dashed, dotted, double
// * Weights range from 1 (thin) to 5 (thick)

use std::collections::HashMap;
use webrust::prelude::*;

#[gui(bg = "navy", fg = "white", font = "Courier New", color = "black", size = "10px")]
fn main() {
    println("@(blue, bold, italic)🚀 WebRust Advanced I/O Features");

    // -------------------------------------------------------------------------
    // 1) Collections with turbofish operator
    // -------------------------------------------------------------------------
    println("\n@(green, bold)1. Collection operations:");

    let numbers: Vec<i32> = (1..=5).collect();
    let numbers_str = numbers.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println("Range to Vec: @(orange){numbers_str}");

    let sum_result = numbers.iter().sum::<i32>();
    println("Sum with turbofish: @(purple){sum_result}");

    let doubled_numbers: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .collect();
    let doubled_numbers_str = doubled_numbers.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println("Map and collect: @(bright_navy){doubled_numbers_str}");

    // -------------------------------------------------------------------------
    // 2) Complex type combinations
    // -------------------------------------------------------------------------
    println("\n@(bright_white, bold)2. Complex type conversions:");

    let age = 30;
    let complex_ternary = if age > 30 {
        "experienced"
    } else {
        "young"
    };
    println("Complex ternary: @(yellow){complex_ternary}");

    let name = "Alice";
    let reversed_name: String = name.chars().rev().collect();
    println("Chained operations: @(green){reversed_name}");

    let height = 1.75;
    let height_as_int = (height * 100.0) as i32;
    println("Type conversion chain: @(blue){height_as_int}");

    let married = true;
    let bool_parsed = married.to_string().parse::<bool>().unwrap_or(false);
    println("Boolean with turbofish: @(red){bool_parsed}");

    // Initials extraction
    let first_name = "Alice";
    let last_name = "Smith";
    let initials_result = first_name.chars().next()
        .zip(last_name.chars().next())
        .map(|(f, l)| {
            [f.to_ascii_uppercase(), l.to_ascii_uppercase()]
                .iter()
                .collect()
        })
        .unwrap_or_else(|| "N/A".to_string());
    println("Initials extraction: @(magenta){initials_result}");

    // -------------------------------------------------------------------------
    // 3) Number formatting showcase
    // -------------------------------------------------------------------------
    println("\n@(bright_white, bold)3. Number formatting:");

    let pi = std::f64::consts::PI;
    let age_fmt = 25i32;

    println("@(orange)PI with various formats:");
    println("@(dimgray)  - Standard: @(black){pi}");
    println("@(dimgray)  - With 2 decimals: @(black){pi:.2}");
    println("@(dimgray)  - With 6 decimals: @(black){pi:.6}");
    println("@(dimgray)  - Scientific notation: @(pink){pi:e}");

    println("@(darkcyan)Integer formats for @(darkorange){age_fmt}@(darkcyan):");
    println("@(dimgray)  - Padded with zeros: @(black){age_fmt:04}");
    println("@(dimgray)  - Hexadecimal lowercase: @(darkorange){age_fmt:x}");
    println("@(dimgray)  - Hexadecimal uppercase: @(darkorange){age_fmt:X}");
    println("@(dimgray)  - Binary: @(darkgreen){age_fmt:b}");
    println("@(dimgray)  - Octal: @(darkblue){age_fmt:o}");

    // -------------------------------------------------------------------------
    // 4) Container formatting (compact and JSON)
    // -------------------------------------------------------------------------
    println("\n@(bright_white, bold)4. Container formatting:");

    let vec_3d = vec![
        vec![
            vec![1, 2],
            vec![3, 4]
        ],
        vec![
            vec![5, 6],
            vec![7, 8]
        ]
    ];
    println("@(darkcyan)3D Array (:c - compact format): {vec_3d:c}");
    println("@(darkcyan)3D Array (:j - JSON format): \n{vec_3d:j}");

    // Complex nested HashMap structure
    let mut cities_data = HashMap::new();

    let mut france = HashMap::new();
    let mut paris = HashMap::new();
    paris.insert("population".to_string(), "2.2M".to_string());
    paris.insert("attractions".to_string(), "Eiffel Tower, Louvre".to_string());
    france.insert("Paris".to_string(), paris);

    let mut usa = HashMap::new();
    let mut new_york = HashMap::new();
    new_york.insert("population".to_string(), "8.4M".to_string());
    new_york.insert("attractions".to_string(), "Statue of Liberty, Times Square".to_string());
    usa.insert("New York".to_string(), new_york);

    cities_data.insert("France".to_string(), france);
    cities_data.insert("USA".to_string(), usa);

    println("@(darkblue)3-level nested structure (:c - compact): {cities_data:c}");
    println("@(darkblue)3-level nested structure (:j - JSON): \n{cities_data:j}");

    // -------------------------------------------------------------------------
    // 5) Dynamic table generation - Pascal's Triangle
    // -------------------------------------------------------------------------
    println("\n@(darkred, bold)5. Dynamic tables - Pascal's Triangle");

    println("Pascal's Triangle:\n").space(0);
    let rows = 10usize;
    for n in 0..rows {
        let mut c: u64 = 1;
        for k in 0..=n {
            print("{c}")
                .weight(1)
                .style("solid")
                .color("SteelBlue")
                .background("WhiteSmoke")
                .width(12);
            if k < n {
                c = c * (n - k) as u64 / (k + 1) as u64;
            }
        }
        println("").space(0);
    }

    // -------------------------------------------------------------------------
    // 6) Dynamic table generation - Multiplication Table
    // -------------------------------------------------------------------------
    println("\n@(darkred, bold)6. Multiplication Table");

    println("Multiplication Table:\n").space(0);
    let n = 9u32;

    // Header row
    print("@(red, bold)×@()")
        .weight(1)
        .style("solid")
        .color("SlateGray")
        .background("Ivory")
        .radius(8)
        .width(15)
        .space(0);

    for j in 1..=n {
        print("@(blue, bold){j}@()")
            .weight(1)
            .style("solid")
            .color("SlateGray")
            .background("Ivory")
            .radius(8)
            .width(15)
            .space(0);
    }
    println("").space(0);

    // Data rows
    for i in 1..=n {
        print("@(blue, bold){i}@()")
            .weight(1)
            .style("solid")
            .color("SlateGray")
            .background("Ivory")
            .radius(8)
            .width(15);

        for j in 1..=n {
            print("{i*j}")
                .weight(1)
                .style("solid")
                .color("LightSteelBlue")
                .background("White")
                .radius(8)
                .width(15);
        }
        println("").space(0);
    }

    // -------------------------------------------------------------------------
    // 7) Text alignment showcase
    // -------------------------------------------------------------------------
    println("\n@(purple, bold)🎨 Text Alignment & Styling Showcase");
    println("@(gray, italic)Demonstrating alignment modes with diverse styling\n");

    // Basic alignment
    println("@(blue, bold)📍 Basic Alignment Examples");
    println("Centered Text")
        .width(*CW)
        .align("center")
        .weight(1)
        .color("blue");
    println("Left-aligned Text")
        .width(*CW)
        .align("left")
        .weight(2)
        .color("green")
        .style("dashed");
    println("Right-aligned Text")
        .width(*CW)
        .align("right")
        .weight(3)
        .color("red")
        .style("dotted");

    // Justified text
    println("\n@(green, bold)📖 Justify with Professional Styling");
    let lorem_text = [
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit,",
        "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
        "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris",
        "nisi ut aliquip ex ea commodo consequat."
    ].join(" ");

    println(lorem_text)
        .width(*CW)
        .align("justify")
        .weight(4)
        .color("darkslategray")
        .style("double")
        .radius(12)
        .background("linen");

    // Width variations
    println("\n@(orange, bold)🎯 Width Variations with Alignment");

    println("Quarter Width Center")
        .width(*CW / 4)
        .align("center")
        .background("lightblue");
    println("Quarter Width Left")
        .width(*CW / 4)
        .align("left")
        .background("lightgreen");
    println("Quarter Width Right")
        .width(*CW / 4)
        .align("right")
        .background("lightyellow");

    println("\nHalf Width Centered Text")
        .width(*CW / 2)
        .align("center")
        .background("lavender");
    println("Half Width Left-aligned Text")
        .width(*CW / 2)
        .align("left")
        .background("lightcoral");
    println("Half Width Right-aligned Text")
        .width(*CW / 2)
        .align("right")
        .background("lightsteelblue");

    // Weight and style variations
    println("\n@(teal, bold)⚖️ Weight & Style Variations");

    println("Ultra-thin border (1px)")
        .width(*CW / 3)
        .align("center")
        .weight(1)
        .color("lightgray")
        .background("white");
    println("Thin border (2px)")
        .width(*CW / 3)
        .align("center")
        .weight(2)
        .color("gray")
        .style("solid")
        .background("whitesmoke");
    println("Medium border (3px)")
        .width(*CW / 3)
        .align("center")
        .weight(3)
        .color("darkgray")
        .style("dashed")
        .background("lightgray");
    println("Thick border (4px)")
        .width(*CW / 3)
        .align("center")
        .weight(4)
        .color("black")
        .style("dotted")
        .background("silver");
    println("Extra-thick border (5px)")
        .width(*CW / 3)
        .align("center")
        .weight(5)
        .color("navy")
        .style("double")
        .background("lightsteelblue");

    // Border style gallery
    println("\n@(magenta, bold)🎭 Border Style Gallery");

    println("Solid Style")
        .width(*CW / 4)
        .align("center")
        .weight(3)
        .color("crimson")
        .style("solid")
        .background("mistyrose");
    println("Dashed Style")
        .width(*CW / 4)
        .align("center")
        .weight(3)
        .color("forestgreen")
        .style("dashed")
        .background("honeydew");
    println("Dotted Style")
        .width(*CW / 4)
        .align("center")
        .weight(3)
        .color("royalblue")
        .style("dotted")
        .background("aliceblue");
    println("Double Style")
        .width(*CW / 4)
        .align("center")
        .weight(4)
        .color("darkorange")
        .style("double")
        .background("papayawhip");

    // Radius variations
    println("\n@(cyan, bold)🌈 Radius Variations");

    println("Sharp corners (0px)")
        .width(*CW / 5)
        .align("center")
        .weight(2)
        .color("red")
        .radius(0)
        .background("lightcoral");
    println("Slight curve (5px)")
        .width(*CW / 5)
        .align("center")
        .weight(2)
        .color("orange")
        .radius(5)
        .background("peachpuff");
    println("Medium curve (10px)")
        .width(*CW / 5)
        .align("center")
        .weight(2)
        .color("gold")
        .radius(10)
        .background("lightyellow");
    println("Rounded (15px)")
        .width(*CW / 5)
        .align("center")
        .weight(2)
        .color("limegreen")
        .radius(15)
        .background("lightgreen");
    println("Very rounded (20px)")
        .width(*CW / 5)
        .align("center")
        .weight(2)
        .color("dodgerblue")
        .radius(20)
        .background("lightblue");

    // Creative combinations
    println("\n@(red, bold)🎪 Creative Combinations");

    println("@(white, bold)EMERGENCY ALERT")
        .width(*CW)
        .align("center")
        .weight(5)
        .color("red")
        .style("double")
        .radius(0)
        .background("yellow");

    println("📈 Financial Report: Q4 Results Show 15% Growth")
        .width(*CW)
        .align("left")
        .weight(1)
        .color("darkgreen")
        .style("solid")
        .radius(3)
        .background("lightgreen");

    println("🎯 Call to Action: Visit our website today!")
        .width(*CW)
        .align("right")
        .weight(3)
        .color("purple")
        .style("dashed")
        .radius(25)
        .background("lavender");

    // Document layout example
    println("\n@(indigo, bold)📜 Document Layout Example");

    println("@(navy, bold)WEBRUST FRAMEWORK v0.7.0")
        .width(*CW)
        .align("center")
        .weight(4)
        .color("navy")
        .style("double")
        .radius(8)
        .background("lightcyan");

    println("Webrust revolutionizes Rust development by providing Python-like simplicity without sacrificing performance. Our framework enables rapid prototyping, elegant GUI creation, and seamless mathematical computing. The innovative styling system demonstrated here showcases just a fraction of webrust's capabilities.")
        .width(*CW)
        .align("justify")
        .weight(1)
        .color("darkslateblue")
        .style("solid")
        .radius(5)
        .background("ghostwhite");

    println("Created with ❤️ by the Webrust Team")
        .width(*CW)
        .align("right")
        .weight(2)
        .color("crimson")
        .style("dotted")
        .radius(15)
        .background("seashell");

    // Advanced combinations
    println("\n@(lime, bold)🔥 Advanced Styling Combinations");

    println("Thin Dashed Border + High Radius")
        .width(*CW / 2)
        .align("center")
        .weight(1)
        .color("teal")
        .style("dashed")
        .radius(25)
        .background("lightcyan");

    println("Thick Double Border + Sharp Corners")
        .width(*CW / 2)
        .align("center")
        .weight(5)
        .color("maroon")
        .style("double")
        .radius(0)
        .background("mistyrose");

    println("Medium Dotted Border + Moderate Radius")
        .width(*CW / 2)
        .align("center")
        .weight(3)
        .color("darkviolet")
        .style("dotted")
        .radius(12)
        .background("thistle");

    println("\n@(bright_green, bold)✨ Advanced I/O demonstration complete!");
}