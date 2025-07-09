// webrust/examples/py_rangenumerate.rs
use webrust::prelude::*;

#[gui]
fn main() {
    println("@(blue, bold)🎯 webrust Range and Enumerate Examples");
    println("@(gray, italic)Demonstrating Python-like ranges and enumeration:");

    println("\n@(green, bold)=== Numeric Tests ===");
    print("@(cyan)20.to(0).by(-2): ");
    for i in 20.to(0).by(-2) {
        print("@(yellow){i}@(reset)   ");
    }
    println("");

    print("@(cyan)0.0.to(10.0).by(0.5): ");
    for x in 0.0.to(10.0).by(0.5) {
        print("@(yellow){x}@(reset)   ");
    }
    println("");

    print("@(cyan)0.to(5): ");
    for i in 0.to(5) {
        print("@(yellow){i}@(reset)   ");
    }
    println("");

    print("@(cyan)5.0.to(0.0): ");
    for i in 5.0.to(0.0) {
        print("@(yellow){i}@(reset)   ");
    }
    println("");

    println("\n@(green, bold)=== Character Tests ===");
    print("@(cyan)'a'.to('z'): ");
    for c in 'a'.to('z') {
        print("@(magenta){c}@(reset) ");
    }
    println("");

    print("@(cyan)'Z'.to('A').by(-2): ");
    for c in 'Z'.to('A').by(-2) {
        print("@(magenta){c}@(reset) ");
    }
    println("");

    print("@(cyan)'z'.to('a'): ");
    for c in 'z'.to('a') {
        print("@(magenta){c}@(reset) ");
    }
    println("");

    print("@(cyan)'0'.to('9'): ");
    for c in '0'.to('9') {
        print("@(magenta){c}@(reset) ");
    }
    println("");

    println("\n@(green, bold)=== Enumerate Tests (Python syntax) ===");
    let names = vec!["Bob", "Alice", "Guido"];

    // Standard enumeration
    print("@(cyan)enumerate(&names): ");
    for (index, value) in enumerate(&names) {
        print("@(white)(@(yellow){index}@(white): @(green){value}@(white)) ");
    }
    println("");

    // Enumeration with custom start (adjustment in display)
    let start = 10;
    print("@(cyan)enumerate(&names) + start: ");
    for (index, value) in enumerate(&names) {
        print("@(white)(@(yellow){index + start}@(white): @(green){value}@(white)) ");
    }
    println("");

    print("@(cyan)enumerate(0.to(5)): ");
    for (index, value) in enumerate(0.to(5)) {
        print("@(white)(@(yellow){index}@(white): @(blue){value}@(white)) ");
    }
    println("");

    // With custom start
    let start = 50;
    print("@(cyan)enumerate('a'.to('e')) + start: ");
    for (index, value) in enumerate('a'.to('e')) {
        print("@(white)(@(yellow){index + start}@(white): @(magenta){value}@(white)) ");
    }
    println("");

    println("\n@(purple, bold)=== Advanced Tests with Expressions ===");

    // Ranges with expressions
    print("@(cyan)Multiples of 3 up to 30: ");
    for i in 0.to(11) {
        print("@(orange){i * 3}@(reset) ");
    }
    println("");

    // Enumerate with complex expressions
    let fruits = vec!["apple", "banana", "cherry", "date"];
    print("@(cyan)Fruits with length: ");
    for (index, fruit) in enumerate(&fruits) {
        print("@(white)(@(yellow){index + 1}@(white): @(green){fruit}@(white)[@(blue){fruit.len()}@(white)]) ");
    }
    println("");

    // Characters with ASCII codes
    print("@(cyan)Letters with ASCII codes: ");
    for (index, letter) in enumerate('A'.to('F')) {
        print("@(white)(@(yellow){index}@(white): @(magenta){letter}@(white)=@(orange){letter as u8}@(white)) ");
    }
    println("");

    // Decimal ranges with formatting
    print("@(cyan)Square roots (0.0 to 2.0): ");
    for i in 0.to(5) {
        let x = i as f64 * 0.5;
        print("@(blue){x:.1}@(white)→@(green){x.sqrt():.3}@(reset) ");
    }
    println("");

    // Enumerate with conditions
    let ages = vec![15, 25, 35, 45, 55];
    print("@(cyan)Age categories: ");
    for (index, age) in enumerate(ages) {
        let category = if age < 18 { "minor" } else if age < 65 { "adult" } else { "senior" };
        print("@(white)(@(yellow){index + 1}@(white): @(orange){age}@(white)→@(green){category}@(white)) ");
    }
    println("");

    println("\n@(red, bold)=== Range + Enumerate Combinations ===");

    // Enumerate on a range with custom step
    print("@(cyan)Even numbers with index: ");
    for (index, number) in enumerate(0.to(20).by(2)) {
        print("@(white)[@(yellow){index}@(white)]@(blue){number}@(reset) ");
    }
    println("");

    // Enumerate on character range
    print("@(cyan)Vowels with position: ");
    for (index, vowel) in enumerate(['a', 'e', 'i', 'o', 'u']) {
        print("@(white)(@(yellow){index + 1}@(white):@(magenta){vowel}@(white)) ");
    }
    println("");

    // Range with enumerate and transformation
    print("@(cyan)Cubes of numbers 1-5: ");
    for (index, number) in enumerate(1.to(6)) {
        print("@(white){number}@(yellow)³@(white)=@(green){number * number * number}@(reset) ");
    }
    println("");

    println("\n@(bright_cyan, bold)🎉 Demonstration completed!");
}