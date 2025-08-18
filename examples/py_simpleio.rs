// webrust/examples/py_simpleio.rs
use std::collections::HashMap;
use webrust::prelude::*;
#[gui(bg = "navy", fg = "white", font = "Courier New", color = "black", size = "10px")]
fn main() {
    println("@(blue, bold, italic)🎯 webrust Personal Information Collector");
    println("@(gray, italic)Please fill in your details below:");
    println("@(green, bold, italic)1. Basic input and variable display:");

    let first_name: String = input("Your first name:");
    let last_name: String = input("Your last name:");
    let age: i32 = input("Your age:");
    let height: f64 = input("Your height (in meters):");
    let married: bool = input("Are you married (true/false):");
    let favorite_letter: char = input("What is your favorite letter?");
    let status = if married { "you are" } else { "you are not" };

    println("@(navy)Hello, @(green, bold){first_name} @(red, bold){last_name}@(navy), you are @(yellow){age}@(navy) years old, you are @(blue){height}@(navy) m tall, your favorite letter is @(magenta){favorite_letter}@(navy), and @(orange, bold){status}@(navy) married.");

    // Section 2: Expressions in placeholders
    println("\n@(green, bold)2. Expressions in placeholders:");
    println("Age in months: @(yellow){age * 12}");
    println("Height in cm: @(blue){height * 100.0:.0}");
    println("Last name in uppercase: @(red, bold){last_name.to_uppercase()}");
    let first_letter = last_name.chars().next().unwrap_or('?');
    println("First letter of the last name: @(magenta){first_letter}");
    println("Is your favorite letter uppercase? @(navy){favorite_letter.is_uppercase()}");

    let letter_category = if favorite_letter.is_alphabetic() {
        if favorite_letter.is_ascii_lowercase() { "lowercase letter" } else { "uppercase letter" }
    } else if favorite_letter.is_numeric() {
        "digit"
    } else {
        "special character"
    };
    println("Letter category: @(bright_green){letter_category}");

    // Section 3: Ternary operator (conditional expressions)
    println("\n@(purple, bold)3. Ternary operator (conditional expressions):");
    let age_category = if age < 18 { "minor" } else if age < 65 { "adult" } else { "senior" };
    println("Age category: @(yellow){age_category}");
    let height_category = if height < 1.60 { "short" } else if height > 1.80 { "tall" } else { "average" };
    println("Height category: @(blue){height_category}");
    let name_length_status = if first_name.len() + last_name.len() > 10 { "long name" } else { "short name" };
    println("Name length status: @(green){name_length_status}");
    let marital_emoji = if married { "💍" } else { "🔓" };
    println("Marital emoji: @(pink){marital_emoji}");
    let letter_type_emoji = if favorite_letter.is_alphabetic() { "📝" } else if favorite_letter.is_numeric() { "🔢" } else { "🎯" };
    println("Letter type emoji: @(navy){letter_type_emoji}");
    let age_comparison = if age > 25 { "older than 25" } else { "25 or younger" };
    println("Age comparison: @(orange){age_comparison}");

    // Section 4: Turbofish operator (explicit type annotations)
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

    // Collections demonstration
    let numbers: Vec<i32> = (1..=5).collect();
    let numbers_str = numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
    println("Range to Vec: @(orange){numbers_str}");
    let sum_result = numbers.iter().sum::<i32>();
    println("Sum with turbofish: @(purple){sum_result}");
    let doubled_numbers: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    let doubled_numbers_str = doubled_numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
    println("Map and collect: @(bright_navy){doubled_numbers_str}");

    // Section 5: Complex combinations
    println("\n@(bright_white, bold)5. Complex combinations:");
    let complex_ternary = if age > 30 { "experienced" } else { "young" };
    println("Complex ternary: @(yellow){complex_ternary}");
    let reversed_name: String = first_name.chars().rev().collect();
    println("Chained operations: @(green){reversed_name}");
    let height_as_int = (height * 100.0) as i32;
    println("Type conversion chain: @(blue){height_as_int}");
    let bool_parsed = married.to_string().parse::<bool>().unwrap_or(false);
    println("Boolean with turbofish: @(red){bool_parsed}");

    // Initials
    let initials_result = first_name.chars().next()
        .zip(last_name.chars().next())
        .map(|(f, l)| [f.to_ascii_uppercase(), l.to_ascii_uppercase()].iter().collect())
        .unwrap_or_else(|| "N/A".to_string());
    println("Initials with complex ternary: @(magenta){initials_result}");

    println("\n@(bright_white, bold)6. Number formatting:");
    let pi = std::f64::consts::PI;
    let age = 25i32;
    println("@(orange)PI with various formats:");
    println("@(dimgray)  - Standard: @(black){pi}");
    println("@(dimgray)  - With 2 decimals: @(black){pi:.2}");
    println("@(dimgray)  - With 6 decimals: @(black){pi:.6}");
    println("@(dimgray)  - Scientific notation: @(pink){pi:e}");
    println("@(darkcyan)Integer formats for @(darkorange){age}@(darkcyan):");
    println("@(dimgray)  - Padded with zeros: @(black){age:04}");
    println("@(dimgray)  - Hexadecimal lowercase: @(darkorange){age:x}");
    println("@(dimgray)  - Hexadecimal uppercase: @(darkorange){age:X}");
    println("@(dimgray)  - Binary: @(darkgreen){age:b}");
    println("@(dimgray)  - Octal: @(darkblue){age:o}");

    println("\n@(bright_white, bold)7. Container formatting:");
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

    println("@(darkblue)3-level nested structure (:c - compact format): {cities_data:c}");
    println("@(darkblue)3-level nested structure (:j - JSON format): \n{cities_data:j}");
}