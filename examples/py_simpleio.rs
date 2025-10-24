// webrust/examples/py_simpleio.rs

use webrust::prelude::*;

#[gui(Courier_New 10px black !white)]
fn main() {
    println("<blue b i>🎯 WebRust Personal Information Collector");
    println("<gray i>Please fill in your details below:");

    println("<green b i>1. Basic input and variable display:");
    let first_name: String = input("Your first name:");
    let last_name: String = input("Your last name:");
    let age: i32 = input("Your age:");
    let height: f64 = input("Your height (in meters):");
    let married: bool = input("Are you married (true/false):");
    let favorite_letter: char = input("What is your favorite letter?");

    let status = if married { "you are" } else { "you are not" };
    println("<navy>Hello, <green b i>{first_name} <red b>{last_name}<navy>, you are <yellow>{age}<navy> years old, you are <blue>{height}<navy> m tall, your favorite letter is <magenta>{favorite_letter}<navy>, and <orange b>{status}<navy> married.");

    print("<white !green r10 w120 h24 mc>● {first_name} {last_name} online")
        .at(-20, 0.0)
        .sticky();

    coord("cartesian");
    print("<white !red r6 w120 h22 mc>● In the middle")
        .at(0.0, 0.0)
        .sticky();

    coord("css");

    println("\n<green b>2. Expressions in placeholders:");
    println("Age in months: <yellow>{age * 12}");
    println("Height in cm: <blue>{height * 100.0:.0}");
    println("Last name in uppercase: <red b>{last_name.to_uppercase()}");
    let first_letter = last_name.chars().next().unwrap_or('?');
    println("First letter of the last name: <magenta>{first_letter}");
    println("Is your favorite letter uppercase? <navy>{favorite_letter.is_uppercase()}");

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
    println("Letter category: <bright_green>{letter_category}");

    println("\n<purple b>3. Ternary operator (conditional expressions):");
    let age_category = if age < 18 {
        "minor"
    } else if age < 65 {
        "adult"
    } else {
        "senior"
    };
    println("Age category: <yellow>{age_category}");
    let height_category = if height < 1.60 {
        "short"
    } else if height > 1.80 {
        "tall"
    } else {
        "average"
    };
    println("Height category: <blue>{height_category}");
    let name_length_status = if first_name.len() + last_name.len() > 10 {
        "long name"
    } else {
        "short name"
    };
    println("Name length status: <green>{name_length_status}");
    let marital_emoji = if married { "💍" } else { "🔓" };
    println("Marital emoji: <pink>{marital_emoji}");
    let letter_type_emoji = if favorite_letter.is_alphabetic() {
        "📝"
    } else if favorite_letter.is_numeric() {
        "🔢"
    } else {
        "🎯"
    };
    println("Letter type emoji: <navy>{letter_type_emoji}");
    let age_comparison = if age > 25 {
        "older than 25"
    } else {
        "25 or younger"
    };
    println("Age comparison: <orange>{age_comparison}");

    println("\n<bright_green b>✨ Simple I/O demonstration complete!");
}
