# webrust - Python-like Rust for Web Applications

[![Crates.io](https://img.shields.io/crates/v/webrust)](https://crates.io/crates/webrust)
[![Documentation](https://docs.rs/webrust/badge.svg)](https://docs.rs/webrust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/webrust/webrust/workflows/CI/badge.svg)](https://github.com/webrust/webrust/actions)

webrust is a revolutionary Rust crate that bridges the elegance of Python with the power and safety of Rust. It provides a seamless web-based interface for Rust applications, featuring Python-like syntax, advanced f-string formatting, real-time type validation, mathematical rendering capabilities, and intuitive iteration tools.

## 🚀 Philosophy

webrust serves as both a **bridge between Python and Rust** and a **catalyst for Rust's evolution toward simplicity**. We believe that Rust's power shouldn't come at the cost of developer experience. By introducing Python-like ergonomics while maintaining Rust's core strengths, webrust demonstrates how systems programming can be both safe and simple.

## ✨ Key Features

- **🐍 Python-like Syntax**: Familiar input/output patterns with Rust's type safety
- **🎨 Advanced F-Strings**: Rust expressions in string formatting with `:c` and `:j` specifiers
- **🌐 Web Interface**: Automatic browser integration with smart server management
- **🔒 Type Safety**: Real-time input validation with detailed error messages
- **📊 LaTeX Rendering**: Mathematical expressions via MathJax integration
- **🎭 Rich Styling**: CSS-like styling directly in Rust code
- **🔄 Python-like Ranges**: Intuitive range generation with `start.to(end)` syntax
- **📋 Enumerate Function**: Python-style enumeration with `enumerate(iterable)`
- **⚡ Zero Configuration**: Works out of the box with automatic setup
- **🔧 Cross-Platform**: Windows, macOS, and Linux support

## 📦 Installation

Add webrust to your `Cargo.toml`:

```toml
[dependencies]
webrust = "0.4.0"
```

Or use `cargo add`:

```bash
cargo add webrust
```

## 🏁 Quick Start

Create your first webrust application:

```rust
use webrust::prelude::*;

#[gui]
fn main() {
    println("@(blue, bold)🎯 Welcome to webrust!");
    
    let name: String = input("What's your name?");
    let age: i32 = input("How old are you?");
    
    println("Hello @(green, bold){name}@(reset)! You are @(yellow){age}@(reset) years old.");
    
    if age >= 18 {
        println("@(green)✅ You're an adult!");
    } else {
        println("@(orange)🧒 You're still young!");
    }
    
    // Python-like ranges
    println("@(cyan)Counting down from 10:@(reset)");
    for i in 10.to(0).by(-1) {
        println("@(red){i}@(reset)");
    }
    
    // Enumerate with styling
    let hobbies = vec!["coding", "reading", "music"];
    println("@(purple)Your hobbies:@(reset)");
    for (index, hobby) in enumerate(&hobbies) {
        let start = 1;
        println("@(yellow){index + start}@(reset). @(green){hobby}@(reset)");
    }
    
    // Mathematical expressions
    latex("\\text{The quadratic formula: } x = \\frac{-b \\pm \\sqrt{b^2-4ac}}{2a}");
}
```

Run with:
```bash
cargo run
```

## 🌐 How the Web Server Works

Unlike traditional console applications, webrust automatically:

1. **Starts a local web server** on `127.0.0.1:8080`
2. **Opens your default browser** to the application
3. **Provides a modern web interface** with real-time updates
4. **Handles all communication** between Rust and the browser
5. **Shuts down automatically** when you close the browser tab/window

### 🔄 Smart Server Management

**No need for Ctrl+C!** The server intelligently shuts down:
- **3 seconds** after closing the browser tab/window
- **5 seconds** after the last browser activity
- **30 seconds** maximum runtime (safety limit)

This creates a seamless experience where users can simply close the browser when done.

## 📚 Comprehensive Features Guide

### 🔢 Python-like Ranges

Create intuitive ranges with fluent syntax:

```rust
use webrust::prelude::*;

#[gui]
fn main() {
    // Basic counting
    println("Basic range 0 to 10:");
    for i in 0.to(10) {
        print("{i} ");
    }
    println("");
    
    // Custom step
    println("Even numbers 0 to 20:");
    for i in 0.to(21).by(2) {
        print("{i} ");
    }
    println("");
    
    // Reverse counting
    println("Countdown from 10:");
    for i in 10.to(0) {
        print("{i} ");
    }
    println("");
    
    // Negative steps
    println("Negative step from 20 to 0:");
    for i in 20.to(0).by(-3) {
        print("{i} ");
    }
    println("");
    
    // Decimal ranges
    println("Decimal range 0.0 to 2.0:");
    for x in 0.0.to(2.1).by(0.5) {
        print("{x} ");
    }
    println("");
    
    // Character ranges
    println("Character range 'a' to 'z':");
    for c in 'a'.to('z') {
        print("{c} ");
    }
    println("");
    
    // Character ranges with step
    println("Every other letter from 'A' to 'Z':");
    for c in 'A'.to('Z').by(2) {
        print("{c} ");
    }
    println("");
}
```

### 📋 Enumerate Function

Python-style enumeration with automatic indexing:

```rust
use webrust::prelude::*;

#[gui]
fn main() {
    let fruits = vec!["apple", "banana", "cherry", "date"];
    
    // Basic enumeration
    println("Basic enumeration:");
    for (index, fruit) in enumerate(&fruits) {
        println("{index}: {fruit}");
    }
    
    // Custom start index (adjust in display)
    println("\nCustom start index:");
    let start = 1;
    for (index, fruit) in enumerate(&fruits) {
        println("{index + start}: {fruit}");
    }
    
    // Enumerate with ranges
    println("\nEnumerate with ranges:");
    for (index, number) in enumerate(0.to(10).by(2)) {
        println("Index {index}: Number {number}");
    }
    
    // Enumerate characters
    println("\nEnumerate characters:");
    for (index, letter) in enumerate('a'.to('f')) {
        println("Position {index}: Letter {letter}");
    }
    
    // Complex enumeration with expressions
    println("\nComplex enumeration:");
    let ages = vec![15, 25, 35, 45, 55];
    for (index, age) in enumerate(&ages) {
        let category = if *age < 18 { "minor" } else if *age < 65 { "adult" } else { "senior" };
        println("Person {index + 1}: Age {*age} ({category})");
    }
}
```

### 🎨 F-String Formatting

webrust extends Rust with Python-like f-strings supporting complex expressions:

```rust
use webrust::prelude::*;

#[gui]
fn main() {
    let name = "Alice";
    let age = 30;
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Basic formatting
    println("Hello {name}, you are {age} years old!");
    
    // Mathematical expressions
    println("Sum of numbers: {numbers.iter().sum::<i32>()}");
    println("Square of age: {age * age}");
    
    // String operations
    println("Name in uppercase: {name.to_uppercase()}");
    println("Name length: {name.len()}");
    
    // Conditional expressions
    let status = if age >= 18 { "adult" } else { "minor" };
    println("Status: {status}");
    
    // Complex expressions
    let first_letter = name.chars().next().unwrap_or('?');
    println("First letter: {first_letter}");
    
    // Formatting specifiers
    println("Compact format: {numbers:c}");
    println("JSON-like format: {numbers:j}");
    
    // With ranges and enumerate
    println("Range elements:");
    for (i, val) in enumerate(1.to(6)) {
        println("  Item {i}: {val} (square: {val * val})");
    }
}
```

### 🎭 Rich Text Styling

CSS-like styling directly in Rust:

```rust
use webrust::prelude::*;

#[gui]
fn main() {
    // Basic colors
    println("@(red)Red text@(reset)");
    println("@(green)Green text@(reset)");
    println("@(blue)Blue text@(reset)");
    println("@(yellow)Yellow text@(reset)");
    
    // Text styles
    println("@(bold)Bold text@(reset)");
    println("@(italic)Italic text@(reset)");
    println("@(underline)Underlined text@(reset)");
    println("@(strike)Strikethrough text@(reset)");
    
    // Combined styles
    println("@(red, bold)Red and bold@(reset)");
    println("@(green, italic, underline)Green, italic, and underlined@(reset)");
    
    // Background colors
    println("@(white, bg-red) White text on red background @(reset)");
    println("@(black, bg-yellow) Black text on yellow background @(reset)");
    
    // Extended colors
    println("@(orange)Orange text@(reset)");
    println("@(purple)Purple text@(reset)");
    println("@(pink)Pink text@(reset)");
    println("@(gray)Gray text@(reset)");
    
    // Styled ranges and enumeration
    println("\nStyled output with ranges:");
    for i in 1.to(4) {
        println("@(cyan)Step {i}@(reset): Processing...");
    }
    
    println("\nStyled enumeration:");
    let items = vec!["todo", "in progress", "done"];
    for (index, status) in enumerate(&items) {
        let color = match status {
            "todo" => "red",
            "in progress" => "yellow", 
            "done" => "green",
            _ => "white"
        };
        println("@({color}){index + 1}: {status}@(reset)");
    }
}
```

### 🔒 Type-Safe Input

Automatic type validation with helpful error messages:

```rust
use webrust::prelude::*;

#[gui]
fn main() {
    // Integer input with validation
    let age: i32 = input("Enter your age:");
    println("You entered: {age}");
    
    // Float input with validation
    let height: f64 = input("Enter your height (in meters):");
    println("Your height: {height} meters");
    
    // Boolean input with validation
    let married: bool = input("Are you married? (true/false):");
    println("Married: {married}");
    
    // Character input with validation
    let initial: char = input("Enter your first initial:");
    println("Initial: {initial}");
    
    // String input (no validation needed)
    let name: String = input("Enter your name:");
    println("Name: {name}");
    
    // Using the input in expressions
    if age >= 18 {
        println("@(green)You are an adult!@(reset)");
    } else {
        println("@(orange)You are a minor.@(reset)");
    }
    
    // Complex validation example
    let grade: i32 = input("Enter your grade (0-100):");
    let letter_grade = match grade {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F"
    };
    println("Your letter grade: @(blue){letter_grade}@(reset)");
}
```

### 📊 Mathematical Rendering

High-quality LaTeX rendering for scientific applications:

```rust
use webrust::prelude::*;

#[gui]
fn main() {
    println("@(blue, bold)Mathematical Expressions with LaTeX@(reset)");
    
    // Basic equations
    latex("E = mc^2");
    latex("a^2 + b^2 = c^2");
    
    // Fractions and roots
    latex("x = \\frac{-b \\pm \\sqrt{b^2-4ac}}{2a}");
    latex("\\sqrt{\\frac{a}{b}} = \\frac{\\sqrt{a}}{\\sqrt{b}}");
    
    // Calculus
    latex("\\int_0^\\infty e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}");
    latex("\\frac{d}{dx} \\sin(x) = \\cos(x)");
    
    // Matrices
    latex("\\begin{pmatrix} a & b \\\\ c & d \\end{pmatrix}");
    latex("\\begin{bmatrix} 1 & 2 & 3 \\\\ 4 & 5 & 6 \\\\ 7 & 8 & 9 \\end{bmatrix}");
    
    // Summations and products
    latex("\\sum_{i=1}^{n} i = \\frac{n(n+1)}{2}");
    latex("\\prod_{i=1}^{n} i = n!");
    
    // Greek letters and symbols
    latex("\\alpha + \\beta = \\gamma");
    latex("\\nabla \\times \\vec{E} = -\\frac{\\partial \\vec{B}}{\\partial t}");
    
    // Complex expressions
    latex("\\lim_{x \\to 0} \\frac{\\sin(x)}{x} = 1");
    latex("\\oint_C \\vec{F} \\cdot d\\vec{r} = \\iint_S (\\nabla \\times \\vec{F}) \\cdot d\\vec{S}");
    
    // Data tables
    latex("\\begin{array}{|c|c|c|}
        \\hline
        \\text{Index} & \\text{Value} & \\text{Square} \\\\
        \\hline
        1 & 10 & 100 \\\\
        2 & 20 & 400 \\\\
        3 & 30 & 900 \\\\
        \\hline
        \\end{array}");
}
```

### 🔧 Advanced Combinations

Combining multiple features for powerful applications:

```rust
use webrust::prelude::*;

#[gui]
fn main() {
    println("@(blue, bold)Advanced webrust Combinations@(reset)");
    
    // Interactive mathematical sequence
    let start: i32 = input("Enter starting number:");
    let end: i32 = input("Enter ending number:");
    let step: i32 = input("Enter step size:");
    
    println("@(green)Generating sequence from {start} to {end} with step {step}:@(reset)");
    
    for (index, value) in enumerate(start.to(end).by(step)) {
        let square = value * value;
        let is_even = value % 2 == 0;
        let parity = if is_even { "even" } else { "odd" };
        let color = if is_even { "blue" } else { "red" };
        
        println("@({color})[{index}] {value} → {square} ({parity})@(reset)");
    }
    
    // Mathematical formula generation
    println("\n@(purple)LaTeX Formula for the sequence:@(reset)");
    latex("\\text{Sequence: } \\{a_n\\} = \\{a_1 + (n-1)d\\}");
    latex("\\text{where } a_1 = {start}, d = {step}");
    
    // Statistical analysis
    let values: Vec<i32> = start.to(end).by(step).collect();
    let sum: i32 = values.iter().sum();
    let count = values.len();
    let average = sum as f64 / count as f64;
    
    println("\n@(yellow)Statistical Analysis:@(reset)");
    println("Count: @(cyan){count}@(reset)");
    println("Sum: @(cyan){sum}@(reset)");
    println("Average: @(cyan){average:.2}@(reset)");
    
    // Character analysis
    println("\n@(magenta)Character Analysis:@(reset)");
    for (index, c) in enumerate('A'.to('Z').by(2)) {
        let ascii = c as u8;
        let position = index + 1;
        println("@(white)Position {position}: @(green){c}@(white) (ASCII: @(orange){ascii}@(white))");
    }
}
```

## 🛠 Examples

The crate includes comprehensive examples in the `examples/` directory:

### Running Examples

```bash
# Basic input/output with validation
cargo run --example py_simpleio

# Advanced f-string formatting demonstrations
cargo run --example py_format

# Mathematical and scientific rendering
cargo run --example py_latex

# Python-like range generation examples
cargo run --example py_ranges

# Range and enumeration patterns
cargo run --example py_rangenumerate

# Complex combinations and advanced features
cargo run --example py_advanced
```

### Example Structure

```
examples/
├── py_simpleio.rs          # Basic I/O and validation
├── py_format.rs            # F-string formatting
├── py_latex.rs             # LaTeX mathematical rendering
├── py_ranges.rs            # Range generation patterns
├── py_rangenumerate.rs     # Range and enumerate combinations
└── py_advanced.rs          # Advanced feature combinations
```

## 🚀 API Reference

### Core Functions

#### `input<T>(prompt: &str) -> T`
Prompts user for input with automatic type validation.

```rust
let age: i32 = input("Enter your age:");
let name: String = input("Enter your name:");
let height: f64 = input("Enter your height:");
```

#### `print(text)` and `println(text)`
Output functions with styling support.

```rust
print("Hello ");
print("World!");
println(""); // New line

println("@(red)Error:@(reset) Something went wrong!");
```

#### `latex(expression: &str)`
Renders LaTeX mathematical expressions.

```rust
latex("E = mc^2");
latex("\\frac{d}{dx} \\sin(x) = \\cos(x)");
```

### Range Functions

#### `start.to(end)` and `start.to(end).by(step)`
Creates Python-like ranges.

```rust
for i in 0.to(10) { /* ... */ }
for i in 0.to(10).by(2) { /* ... */ }
for c in 'a'.to('z') { /* ... */ }
```

#### `enumerate(iterable)`
Enumerates items with their indices.

```rust
for (index, value) in enumerate(&vec) { /* ... */ }
for (index, char) in enumerate('a'.to('z')) { /* ... */ }
```

### Styling System

#### Color Codes
- **Basic**: `red`, `green`, `blue`, `yellow`, `cyan`, `magenta`, `white`, `black`
- **Extended**: `orange`, `purple`, `pink`, `gray`, `bright_cyan`
- **Backgrounds**: `bg-red`, `bg-green`, `bg-blue`, etc.

#### Style Codes
- **Text**: `bold`, `italic`, `underline`, `strike`
- **Reset**: `reset` (clears all styling)

#### Usage
```rust
println("@(red, bold)Error:@(reset) @(blue)Info message@(reset)");
```

## 🔧 Configuration

### Project Setup

1. **Create a new Rust project:**
   ```bash
   cargo new my_webrust_app
   cd my_webrust_app
   ```

2. **Add webrust dependency:**
   ```toml
   [dependencies]
   webrust = "0.4.0"
   ```

3. **Create your main function:**
   ```rust
   use webrust::prelude::*;
   
   #[gui]
   fn main() {
       println("Hello, webrust!");
   }
   ```

### Advanced Configuration

#### Custom Port (Future Feature)
```rust
use webrust::prelude::*;

#[gui(port = 8081)]
fn main() {
    println("Running on port 8081");
}
```

#### Development Mode (Future Feature)
```rust
#[gui(debug = true)]
fn main() {
    println("Debug mode enabled");
}
```

## 🔮 Roadmap

### Recently Added (v0.4.0)
- ✅ **`start.to(end)` ranges** - Python-style range generation
- ✅ **`enumerate(iterable)`** - Index-value iteration
- ✅ **Character ranges** - `'a'.to('z')` with custom steps
- ✅ **Fluent syntax** - Chainable `.by(step)` operations
- ✅ **Expression evaluation** - Complex expressions in f-strings
- ✅ **Enhanced styling** - Extended color palette and combinations

### Planned Features (v0.5.0)
- **`zip(iter1, iter2)`** - Parallel iteration over multiple iterables
- **`len(container)`** - Universal length function for all containers
- **`sum(iterable)`** - Summation with automatic type inference
- **`min(iterable)` / `max(iterable)`** - Finding minimum/maximum values
- **`sorted(iterable)`** - Sorting with custom comparisons
- **`reversed(iterable)`** - Reverse iteration
- **List comprehensions** - Pythonic data transformation syntax

### Future Directions (v1.0.0+)
- **Performance optimizations** - Faster rendering and processing
- **Plugin system** - Extensible architecture for custom functionality
- **IDE integration** - Enhanced tooling and autocomplete support
- **Cross-platform mobile** - Progressive Web App capabilities
- **Real-time collaboration** - Multi-user applications support
- **Advanced mathematical** - 3D plotting and interactive visualizations
- **Database integration** - Built-in SQL and NoSQL support

## 🤝 Contributing

webrust welcomes contributions from the community! We're particularly interested in:

### Ways to Contribute

1. **New Python-like features** that enhance Rust's ergonomics
2. **Performance improvements** and optimizations
3. **Documentation** improvements and examples
4. **Testing** across different platforms and use cases
5. **Bug reports** and feature requests
6. **Example applications** showcasing webrust capabilities

### Development Setup

```bash
# Clone the repository
git clone https://github.com/webrust/webrust.git
cd webrust

# Run tests
cargo test

# Run examples
cargo run --example py_simpleio

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy
```

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Run formatting (`cargo fmt`)
7. Submit a pull request

### Code Style Guidelines

- Follow standard Rust conventions
- Use meaningful variable names
- Add documentation for public APIs
- Include examples in documentation
- Write comprehensive tests

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

```
MIT License

Copyright (c) 2024 webrust Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 🌟 Why Choose webrust?

### **For Python Developers**
- **Familiar syntax** with Rust's performance and safety
- **Gentle learning curve** into systems programming
- **Powerful type system** that catches errors at compile time
- **No runtime overhead** compared to Python's interpreter

### **For Rust Developers**
- **Simplified APIs** without sacrificing Rust's core benefits
- **Rapid prototyping** with immediate visual feedback
- **Educational tool** for teaching Rust concepts
- **Modern UI** without complex web development

### **For Educators**
- **Interactive examples** that students can modify and run
- **Visual feedback** makes learning more engaging
- **Safe environment** for experimenting with code
- **Mathematical rendering** for scientific applications

### **For Scientists and Researchers**
- **LaTeX support** for publication-quality mathematical expressions
- **Data visualization** capabilities (coming soon)
- **High performance** for computational tasks
- **Reproducible research** with compiled binaries

### **For Innovators and Startups**
- **Rapid prototyping** of ideas and concepts
- **Web-based demos** for presentations and testing
- **Cross-platform deployment** from a single codebase
- **Modern, professional appearance** out of the box

## 🎯 Use Cases

### Educational Applications
- **Programming courses** teaching Rust fundamentals
- **Algorithm visualization** with step-by-step execution
- **Mathematical modeling** with real-time rendering
- **Interactive tutorials** and coding exercises

### Scientific Computing
- **Data analysis** and statistical modeling
- **Mathematical simulations** with visual output
- **Research presentations** with executable code
- **Computational experiments** with parameter adjustment

### Rapid Prototyping
- **Proof of concepts** for new algorithms
- **User interface mockups** with real functionality
- **API testing** and validation
- **Algorithm comparison** and benchmarking

### Demonstrations and Presentations
- **Live coding** sessions with immediate feedback
- **Technical presentations** with interactive elements
- **Product demos** with professional appearance
- **Conference talks** with executable examples

## 🔗 Links and Resources

- **Website**: [webrust.org](https://webrust.org) (coming soon)
- **Documentation**: [docs.rs/webrust](https://docs.rs/webrust)
- **Repository**: [github.com/webrust/webrust](https://github.com/webrust/webrust)
- **Crates.io**: [crates.io/crates/webrust](https://crates.io/crates/webrust)
- **Examples**: [github.com/webrust/examples](https://github.com/webrust/examples)
- **Community**: [discord.gg/webrust](https://discord.gg/webrust) (coming soon)

## 🙏 Acknowledgments

webrust is built on the shoulders of giants. We thank:

- **The Rust Team** for creating an amazing language
- **The Python Community** for inspiration and design principles
- **MathJax** for mathematical rendering capabilities
- **All contributors** who help make webrust better

---

<div align="center">

**webrust: Making Rust as simple as Python, as powerful as ever.** 🦀✨

*Available now on [crates.io](https://crates.io/crates/webrust)!*

[Get Started](#-installation) • [Examples](#-examples) • [Documentation](https://docs.rs/webrust) • [Contributing](#-contributing)

</div>