// webrust/examples/py_format.rs
use webrust::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
}

#[gui]
fn main() {
    println("@(blue, bold)🎯 webrust Formatting Demo");
    println("@(gray)Testing :c (compact) and :j (JSON) formatters\n");

    /* SECTION 1 : NUMBER FORMATTING */
    println("@(purple, bold)2. Number formatting:");
    let pi = std::f64::consts::PI;
    let age = 25i32;
    println("@(cyan)PI with various formats:");
    println("@(gray)  - Standard: @(white){pi}");
    println("@(gray)  - With 2 decimals: @(white){pi:.2}");
    println("@(gray)  - With 6 decimals: @(white){pi:.6}");
    println("@(gray)  - Scientific notation: @(white){pi:e}");
    println("@(cyan)Integer formats for @(yellow){age}@(cyan):");
    println("@(gray)  - Padded with zeros: @(white){age:04}");
    println("@(gray)  - Hexadecimal lowercase: @(orange){age:x}");
    println("@(gray)  - Hexadecimal uppercase: @(orange){age:X}");
    println("@(gray)  - Binary: @(green){age:b}");
    println("@(gray)  - Octal: @(blue){age:o}");

    /* SECTION 2 : CONTAINER FORMATTING */
    // 3D array
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
    println("@(bright_cyan)3D Array (:c - compact format): {vec_3d:c}");
    println("@(bright_cyan)3D Array (:j - JSON format): \n{vec_3d:j}");

    // 3-level nested structure
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

    println("@(blue)3-level nested structure (:c - compact format): {cities_data:c}");
    println("@(blue)3-level nested structure (:j - JSON format): \n{cities_data:j}");
}
