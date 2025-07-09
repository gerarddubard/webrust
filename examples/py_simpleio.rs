// webrust/examples/py_simpleio.rs
use webrust::prelude::*;

#[gui]
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
    println("@(cyan)Hello, @(green, bold){first_name} @(red, bold){last_name}@(cyan), you are @(yellow){age}@(cyan) years old, you are @(blue){height}@(cyan) m tall, your favorite letter is '@(magenta){favorite_letter}@(cyan)', and @(orange, bold){status}@(cyan) married.");

    /* SECTION 2: EXPRESSIONS IN PLACEHOLDERS */
    println("\n@(green, bold)2. Expressions in placeholders:");
    println("Age in months: @(yellow){age * 12}");
    println("Height in cm: @(blue){height * 100.0:.0}");
    println("Last name in uppercase: @(red, bold){last_name.to_uppercase()}");
    let first_letter = last_name.chars().next().unwrap_or('?');
    println("First letter of the last name: @(magenta){first_letter}");
    println("Is your favorite letter uppercase? @(cyan){favorite_letter.is_uppercase()}");
    let letter_category = if favorite_letter.is_alphabetic() {
        if favorite_letter.is_ascii_lowercase() { "lowercase letter" }
        else { "uppercase letter" }
    } else if favorite_letter.is_numeric() {
        "digit"
    } else {
        "special character"
    };
    println("Letter category: @(bright_green){letter_category}");

    /* SECTION 3: TERNARY OPERATOR (CONDITIONAL EXPRESSIONS) avec :c */
    println("\n@(purple, bold)3. Ternary operator (conditional expressions):");
    let age_category = if age < 18 { "minor" } else if age < 65 { "adult" } else { "senior" };
    println("Age category: @(yellow){age_category:c}");
    let height_category = if height < 1.60 { "short" } else if height > 1.80 { "tall" } else { "average" };
    println("Height category: @(blue){height_category:c}");
    let name_length_status = if first_name.len() + last_name.len() > 10 { "long name" } else { "short name" };
    println("Name length status: @(green){name_length_status:c}");
    let marital_emoji = if married { "💍" } else { "🔓" };
    println("Marital emoji: @(pink){marital_emoji:c}");
    let letter_type_emoji = if favorite_letter.is_alphabetic() { "📝" } else if favorite_letter.is_numeric() { "🔢" } else { "🎯" };
    println("Letter type emoji: @(cyan){letter_type_emoji:c}");
    let age_comparison = if age > 25 { "older than 25" } else { "25 or younger" };
    println("Age comparison: @(orange){age_comparison:c}");

    /* SECTION 4: TURBOFISH OPERATOR (EXPLICIT TYPE ANNOTATIONS) avec :c */
    println("\n@(red, bold)4. Turbofish operator (explicit type annotations):");
    let number_str = age.to_string();
    let parsed_u32 = number_str.parse::<u32>().unwrap_or(0);
    println("String to u32: @(yellow){parsed_u32:c}");
    let parsed_f64 = number_str.parse::<f64>().unwrap_or(0.0);
    println("String to f64: @(blue){parsed_f64:c}");
    let height_u8 = height as u8;
    println("Height to u8 (truncated): @(green){height_u8:c}");
    let chars_vec = first_name.chars().collect::<Vec<char>>();
    println("Collect chars: @(magenta){chars_vec:c}");
    let parsed_height = height.to_string().parse::<f32>().unwrap_or(0.0);
    println("Parse height string: @(cyan){parsed_height:c}");

    // Démonstration avec des collections
    let numbers: Vec<i32> = (1..=5).collect();
    println("Range to Vec: @(orange){numbers:c}");
    let sum_result = numbers.iter().sum::<i32>();
    println("Sum with turbofish: @(purple){sum_result:c}");
    let doubled_numbers = numbers.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println("Map and collect: @(bright_cyan){doubled_numbers:c}");

    /* SECTION 5: COMPLEX COMBINATIONS avec :c */
    println("\n@(bright_white, bold)5. Complex combinations:");
    let complex_ternary = if age.to_string().parse::<u32>().unwrap_or(0) > 30 { "experienced" } else { "young" };
    println("Complex ternary: @(yellow){complex_ternary:c}");
    let reversed_name = first_name.chars().rev().collect::<String>();
    println("Chained operations: @(green){reversed_name:c}");
    let height_as_int = (height * 100.0) as i32;
    println("Type conversion chain: @(blue){height_as_int:c}");
    let bool_parsed = married.to_string().parse::<bool>().unwrap_or(false);
    println("Boolean with turbofish: @(red){bool_parsed:c}");

    // Démonstration avec des opérations plus complexes
    let initials = format!("{}{}",
                           first_name.chars().next().unwrap_or('?'),
                           last_name.chars().next().unwrap_or('?')
    );
    let initials_result = if initials.len() == 2 { initials.to_uppercase() } else { "N/A".to_string() };
    println("Initials with complex ternary: @(magenta){initials_result:c}");
}
