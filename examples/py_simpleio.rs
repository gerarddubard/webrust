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

    println("\n@(purple, bold)🎨 Text Alignment & Styling Showcase");
    println("@(gray, italic)Demonstrating all alignment modes with diverse styling options\n");

    // Basic alignment examples
    println("@(blue, bold)📍 Basic Alignment Examples");
    println("Centered Text").width(*CW).align("center").weight(1).color("blue");
    println("Left-aligned Text").width(*CW).align("left").weight(2).color("green").style("dashed");
    println("Right-aligned Text").width(*CW).align("right").weight(3).color("red").style("dotted");

    println("\n@(green, bold)📖 Justify with Professional Styling");
    let lorem_text = [
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit,",
        "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
        "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris",
        "nisi ut aliquip ex ea commodo consequat."
    ].join(" ");

    println(lorem_text).width(*CW).align("justify").weight(4).color("darkslategray").style("double").radius(12).background("linen");

    println("\n@(orange, bold)🎯 Width Variations with Alignment");

    // Quarter width
    println("Quarter Width Center").width(*CW / 4).align("center").background("lightblue");
    println("Quarter Width Left").width(*CW / 4).align("left").background("lightgreen");
    println("Quarter Width Right").width(*CW / 4).align("right").background("lightyellow");

    // Half width  
    println("\nHalf Width Centered Text").width(*CW / 2).align("center").background("lavender");
    println("Half Width Left-aligned Text").width(*CW / 2).align("left").background("lightcoral");
    println("Half Width Right-aligned Text").width(*CW / 2).align("right").background("lightsteelblue");

    println("\n@(teal, bold)⚖️ Weight & Style Variations");

    println("Ultra-thin border (1px)").width(*CW / 3).align("center").weight(1).color("lightgray").background("white");
    println("Thin border (2px)").width(*CW / 3).align("center").weight(2).color("gray").style("solid").background("whitesmoke");
    println("Medium border (3px)").width(*CW / 3).align("center").weight(3).color("darkgray").style("dashed").background("lightgray");
    println("Thick border (4px)").width(*CW / 3).align("center").weight(4).color("black").style("dotted").background("silver");
    println("Extra-thick border (5px)").width(*CW / 3).align("center").weight(5).color("navy").style("double").background("lightsteelblue");

    println("\n@(magenta, bold)🎭 Border Style Gallery");

    println("Solid Style").width(*CW / 4).align("center").weight(3).color("crimson").style("solid").background("mistyrose");
    println("Dashed Style").width(*CW / 4).align("center").weight(3).color("forestgreen").style("dashed").background("honeydew");
    println("Dotted Style").width(*CW / 4).align("center").weight(3).color("royalblue").style("dotted").background("aliceblue");
    println("Double Style").width(*CW / 4).align("center").weight(4).color("darkorange").style("double").background("papayawhip");

    println("\n@(cyan, bold)🌈 Radius Variations");

    println("Sharp corners (0px)").width(*CW / 5).align("center").weight(2).color("red").radius(0).background("lightcoral");
    println("Slight curve (5px)").width(*CW / 5).align("center").weight(2).color("orange").radius(5).background("peachpuff");
    println("Medium curve (10px)").width(*CW / 5).align("center").weight(2).color("gold").radius(10).background("lightyellow");
    println("Rounded (15px)").width(*CW / 5).align("center").weight(2).color("limegreen").radius(15).background("lightgreen");
    println("Very rounded (20px)").width(*CW / 5).align("center").weight(2).color("dodgerblue").radius(20).background("lightblue");

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

    println("\n@(indigo, bold)📜 Document Layout Example");

    // Header with thick style
    println("@(navy, bold)WEBRUST FRAMEWORK v0.7.0")
        .width(*CW)
        .align("center")
        .weight(4)
        .color("navy")
        .style("double")
        .radius(8)
        .background("lightcyan");

    // Justified body text with thin border
    println("Webrust revolutionizes Rust development by providing Python-like simplicity without sacrificing performance. Our framework enables rapid prototyping, elegant GUI creation, and seamless mathematical computing. The innovative styling system demonstrated here showcases just a fraction of webrust's capabilities.")
        .width(*CW)
        .align("justify")
        .weight(1)
        .color("darkslateblue")
        .style("solid")
        .radius(5)
        .background("ghostwhite");

    // Signature with distinctive style
    println("Created with ❤️ by the Webrust Team")
        .width(*CW)
        .align("right")
        .weight(2)
        .color("crimson")
        .style("dotted")
        .radius(15)
        .background("seashell");

    println("@(lime, bold)🔥 Advanced Styling Combinations");

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

    println("\n@(bright_green, bold)✨ Styling Showcase Complete!");
    println("@(gray, italic)Demonstrating the full power of webrust's styling system with weights 1-5, all border styles, various radius, and rich color combinations");
}