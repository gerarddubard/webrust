// webrust/examples/py_utils.rs
use webrust::prelude::*;

#[gui(bg = "blue", fg = "white", font = "Times", color = "black", size = "14px")]
fn main() {
    println("@(blue, bold)🎯 webrust Range and Enumerate Examples");
    println("@(gray, italic)Demonstrating Python-like ranges and enumeration:\n");

    println("\n@(green, bold)=== Numeric Tests (Horizontal Tables) ===");

    println("@(magenta, italic)0.to(5):\n");
    for i in 0.to(5) {
        print("@(white){i}").width(25).weight(1).color("white").background("purple").align("center").space(0);
    }
    println("");

    println("@(magenta, italic)4.5.to(0.0):\n");
    for i in 4.5.to(0.0) {
        print("@(white){i}").width(30).weight(1).color("white").background("darkred").align("center").space(0);
    }
    println("");

    println("@(magenta, italic)20.to(0).by(-2):\n");
    for i in 20.to(0).by(-2) {
        print("@(white){i}").width(20).weight(1).color("white").background("navy").align("center").space(0);
    }
    println("");

    println("@(magenta, italic)0.0.to(4.0).by(0.25):\n");
    for x in 0.0.to(4.0).by(0.25) {
        print("@(white){x}").width(25).weight(1).color("white").background("darkgreen").align("center").space(0);
    }
    println("");

    println("\n@(green, bold)=== Character Tests (Horizontal Tables) ===");

    println("@(magenta, italic)'a'.to('z'):\n");
    for c in 'a'.to('z') {
        print("@(white){c}").width(10).weight(1).color("white").background("darkorange").align("center").space(0);
    }
    println("");

    println("@(magenta, italic)'Z'.to('A').by(-2):\n");
    for c in 'Z'.to('A').by(-2) {
        print("@(white){c}").width(25).weight(1).color("white").background("darkmagenta").align("center").space(0);
    }
    println("");

    println("\n@(green, bold)=== Enumerate Tests (Real Tables) ===");

    println("@(magenta, italic)enumerate(0.to(5)):\n");
    print("Index").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    print("Value").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    println("").space(0);
    for (index, value) in enumerate(0.to(5)) {
        print("{index}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        print("{value}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        println("").space(0);
    }

    let names = vec!["Bob", "Alice", "Guido"];
    println("@(magenta, italic)enumerate(&names):\n");
    print("Index").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    print("Name").weight(1).color("SteelBlue").background("lightskyblue").width(100).align("center").space(0);
    println("").space(0);
    for (index, value) in enumerate(&names) {
        print("{index}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        print("{value}").weight(1).color("SteelBlue").background("White").width(100).align("center").space(0);
        println("").space(0);
    }

    println("@(magenta, italic)enumerate(&names) + start (10):\n");
    print("Index").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    print("Name").weight(1).color("SteelBlue").background("lightskyblue").width(100).align("center").space(0);
    println("").space(0);
    let start = 10;
    for (index, value) in enumerate(&names) {
        print("{index + start}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        print("{value}").weight(1).color("SteelBlue").background("White").width(100).align("center").space(0);
        println("").space(0);
    }

    println("@(magenta, italic)enumerate('a'.to('e')) + start (50):\n");
    print("Index").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    print("Character").weight(1).color("SteelBlue").background("lightskyblue").width(100).align("center").space(0);
    println("").space(0);
    let start = 50;
    for (index, value) in enumerate('a'.to('e')) {
        print("{index + start}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        print("{value}").weight(1).color("SteelBlue").background("White").width(100).align("center").space(0);
        println("").space(0);
    }

    println("\n@(purple, bold)=== Advanced Tests (Real Tables) ===");

    println("@(magenta, italic)Multiples of 3 up to 30:\n");
    print("Index").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    print("Multiple").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    println("").space(0);
    for (idx, i) in enumerate(0.to(11)) {
        print("{idx}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        print("{i * 3}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        println("").space(0);
    }

    let fruits = vec!["apple", "banana", "cherry", "date"];
    println("@(magenta, italic)Fruits with length:\n");
    print("Index").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    print("Fruit & Length").weight(1).color("SteelBlue").background("lightskyblue").width(150).align("center").space(0);
    println("").space(0);
    for (index, fruit) in enumerate(&fruits) {
        print("{index + 1}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        print("{fruit} (len: {fruit.len()})").weight(1).color("SteelBlue").background("White").width(150).align("center").space(0);
        println("").space(0);
    }

    println("@(magenta, italic)Letters with ASCII codes:\n");
    print("Index").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    print("Letter & ASCII").weight(1).color("SteelBlue").background("lightskyblue").width(120).align("center").space(0);
    println("").space(0);
    for (index, letter) in enumerate('A'.to('F')) {
        print("{index}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        print("{letter} = {letter as u8}").weight(1).color("SteelBlue").background("White").width(120).align("center").space(0);
        println("").space(0);
    }

    println("@(magenta, italic)Square roots (0.0 to 2.0):\n");
    print("Index").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    print("Square Root").weight(1).color("SteelBlue").background("lightskyblue").width(180).align("center").space(0);
    println("").space(0);
    for (idx, i) in enumerate(0.to(5)) {
        let sqrt_rounded = ((i as f64).sqrt() * 1000.0).round() / 1000.0;
        print("{idx}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        print("{sqrt_rounded:.3}").weight(1).color("SteelBlue").background("White").width(180).align("center").space(0);
        println("").space(0);
    }

    let ages = vec![15, 25, 35, 45, 55];
    println("@(magenta, italic)Age categories:\n");
    print("Index").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    print("Category & Age").weight(1).color("SteelBlue").background("lightskyblue").width(160).align("center").space(0);
    println("").space(0);
    for (index, age) in enumerate(ages) {
        let category = if age < 18 { "minor" } else if age < 65 { "adult" } else { "senior" };
        print("{index + 1}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        print("{category} ({age} years)").weight(1).color("SteelBlue").background("White").width(160).align("center").space(0);
        println("").space(0);
    }

    println("\n@(red, bold)=== Range + Enumerate Combinations (Tables) ===");

    println("@(magenta, italic)Even numbers with index:\n");
    print("Index").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    print("Even Number").weight(1).color("SteelBlue").background("lightskyblue").width(120).align("center").space(0);
    println("").space(0);
    for (index, number) in enumerate(0.to(20).by(2)) {
        print("{index}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        print("{number}").weight(1).color("SteelBlue").background("White").width(120).align("center").space(0);
        println("").space(0);
    }

    println("@(magenta, italic)Vowels with position:\n");
    print("Index").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    print("Vowel").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    println("").space(0);
    for (index, vowel) in enumerate(['a', 'e', 'i', 'o', 'u']) {
        print("{index + 1}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        print("{vowel}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        println("").space(0);
    }

    println("@(magenta, italic)Cubes of numbers 1-5:\n");
    print("Index").weight(1).color("SteelBlue").background("lightskyblue").width(80).align("center").space(0);
    print("Number & Cube").weight(1).color("SteelBlue").background("lightskyblue").width(140).align("center").space(0);
    println("").space(0);
    for (index, number) in enumerate(1.to(6)) {
        let cube = number * number * number;
        print("{index}").weight(1).color("SteelBlue").background("White").width(80).align("center").space(0);
        print("{number}$({number}^3) = {cube}").weight(1).color("SteelBlue").background("White").width(140).align("center").space(0);
        println("").space(0);
    }

    println("\n@(bright_magenta, italic, bold)🎉 Demonstration completed with real tables!");
}
