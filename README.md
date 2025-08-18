# webrust - Python-like Rust for Web Applications

[![Crates.io](https://img.shields.io/crates/v/webrust)](https://crates.io/crates/webrust)
[![Documentation](https://docs.rs/webrust/badge.svg)](https://docs.rs/webrust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/gerarddubard/webrust/workflows/CI/badge.svg)](https://github.com/gerarddubard/webrust/actions)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-windows%20%7C%20macos%20%7C%20linux-lightgrey)](https://github.com/gerarddubard/webrust)
[![Examples](https://img.shields.io/badge/examples-4-brightgreen)](https://github.com/gerarddubard/webrust/tree/main/examples)

webrust is a revolutionary Rust crate that bridges the elegance of Python with the power and safety of Rust. It provides a seamless web-based interface for Rust applications, featuring Python-like syntax, advanced f-string formatting, real-time type validation, mathematical rendering capabilities, intelligent data tables, and sophisticated styling systems.

## 📋 Table of Contents

- [🚀 Philosophy](#-philosophy)
- [✨ Key Features](#-key-features)
- [📦 Installation](#-installation)
- [🏁 Quick Start](#-quick-start)
- [🌐 Web Server](#-how-the-web-server-works)
- [📚 Comprehensive Features](#-comprehensive-features-guide)
    - [🎨 Advanced F-Strings](#-advanced-f-string-formatting)
    - [📊 Intelligent Tables](#-intelligent-table-system)
    - [🎯 PrintBox System](#-advanced-printbox-system)
    - [📐 LaTeX Math](#-latex-mathematical-rendering)
    - [🔄 Python-like Ranges](#-python-like-ranges-and-enumeration)
    - [🎭 Styling System](#-rich-styling-system)
    - [🔒 Type-Safe Input](#-advanced-type-safe-input)
    - [🔧 GUI Customization](#-gui-customization)
- [🛠 Examples](#-examples)
- [🚀 API Reference](#-api-reference)
- [🎯 Real-World Examples](#-real-world-examples)
- [🔮 Roadmap](#-roadmap)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)
- [🌟 Why Choose webrust?](#-why-choose-webrust)

## 🚀 Philosophy

webrust serves as both a **bridge between Python and Rust** and a **catalyst for Rust's evolution toward simplicity**. We believe that Rust's power shouldn't come at the cost of developer experience. By introducing Python-like ergonomics while maintaining Rust's core strengths, webrust demonstrates how systems programming can be both safe and elegant.

## ✨ Key Features

- **🐍 Python-like Syntax**: Familiar input/output patterns with Rust's type safety
- **🎨 Advanced F-Strings**: Complex Rust expressions with rich formatting (`:c`, `:j`, `:e`, `:.2`, etc.)
- **🌐 Smart Web Interface**: Automatic browser integration with intelligent server management
- **🔒 Real-time Validation**: Client-side and server-side input validation with detailed error messages
- **📊 LaTeX Integration**: Mathematical expressions via MathJax with `$(...)` syntax
- **🎭 Rich Styling System**: CSS-like styling with `@(color, style)` syntax
- **📋 Intelligent Tables**: Automatic data visualization with pivot, merge, and header capabilities
- **🎯 Advanced PrintBox**: Sophisticated output formatting with borders, colors, alignment, and spacing
- **🔄 Python-like Ranges**: Intuitive range generation with `start.to(end).by(step)` syntax
- **📋 Enumerate Function**: Python-style enumeration with `enumerate(iterable)`
- **⚡ Zero Configuration**: Works out of the box with customizable themes
- **🔧 Cross-Platform**: Windows, macOS, and Linux support

## 📦 Installation

Add webrust to your `Cargo.toml`:

toml
[dependencies]
webrust = "0.5.0"


Or use `cargo add`:

bash
cargo add webrust


## 🏁 Quick Start

Create your first professional webrust application:


use webrust::prelude::*;
use std::collections::HashMap;

#[gui(bg = "navy", fg = "white", font = "Arial", color = "lightcyan", size = "14px")]
fn main() {
    println("@(cyan, bold)🚀 webrust v0.5.0 - Production Demo");
    
    // Interactive data collection with real-time validation
    let company: String = input("Company name:");
    let year: i32 = input("Analysis year:");
    let employees: i32 = input("Number of employees:");
    
    // Generate realistic business data
    let mut quarterly_revenue = HashMap::new();
    quarterly_revenue.insert("Q1", employees * 50000);
    quarterly_revenue.insert("Q2", employees * 55000);  
    quarterly_revenue.insert("Q3", employees * 52000);
    quarterly_revenue.insert("Q4", employees * 62000);
    
    // Professional dashboard output
    println("\n@(yellow, bold)📊 {company} - {year} Business Dashboard");
    
    // Company overview table
    let overview = vec![
        vec!["Company", &company],
        vec!["Year", &year.to_string()],
        vec!["Employees", &employees.to_string()],
        vec!["Revenue/Employee", &format!("${:.0}K", 
            quarterly_revenue.values().sum::<i32>() as f64 / employees as f64 / 1000.0)],
    ];
    table(&overview).header(["Metric", "Value"]);
    
    // Quarterly performance with pivot analysis
    println("@(green, bold)📈 Quarterly Revenue Analysis:");
    table(&quarterly_revenue);
    table(&quarterly_revenue).pivot().header(["Revenue"]);
    
    // Mathematical projections with LaTeX
    let total_revenue: i32 = quarterly_revenue.values().sum();
    let growth_rate = 15.5;
    let projected_next_year = total_revenue as f64 * (1.0 + growth_rate / 100.0);
    
    println("\n@(purple, bold)📐 Financial Projections:");
    println("Current revenue: @(green)${:.0}M", total_revenue as f64 / 1_000_000.0);
    println("Growth formula: $(R_{{new}} = R_{{current}} \\times (1 + r))");
    println("Projected {}: $(R = {} \\times 1.{:.0}) = @(cyan)${:.1}M", 
        year + 1, total_revenue, growth_rate, projected_next_year / 1_000_000.0);
    
    // Styled performance indicators
    for quarter in ["Q1", "Q2", "Q3", "Q4"] {
        let revenue = quarterly_revenue[quarter];
        let color = match quarter {
            "Q1" => "blue", "Q2" => "green", 
            "Q3" => "orange", "Q4" => "purple"
        };
        println("@({color}, bold){quarter}@(): ${:.1}M", revenue as f64 / 1_000_000.0)
            .weight(1).color(color).background("lightgray")
            .radius(5).width(120).align("center");
    }
    
    println("\n@(bright_cyan, bold)✨ Professional dashboard complete!");
    println("@(gray, italic)This is what 5 minutes of webrust development produces.");
}


## 🌐 How the Web Server Works

webrust automatically creates a sophisticated web interface:

1. **Starts a local web server** on `127.0.0.1:8080`
2. **Opens your default browser** to the application
3. **Provides real-time updates** with modern UI
4. **Handles client-server communication** seamlessly
5. **Shuts down intelligently** when you're done

### 🔄 Smart Server Management

The server intelligently manages its lifecycle:
- **Immediate shutdown** when browser tab closes
- **3-second grace period** after last user interaction
- **30-second maximum** runtime for safety
- **No manual intervention** required (no Ctrl+C needed!)

### ⚡ Performance
- **Server startup**: <500ms
- **Page load**: <200ms
- **Real-time validation**: <50ms
- **Table rendering**: 1000+ rows in <100ms

## 📚 Comprehensive Features Guide

### 🎨 Advanced F-String Formatting

webrust provides the most sophisticated f-string system in Rust:


use webrust::prelude::*;

#[gui]
fn main() {
    let name = "Alice";
    let age = 30;
    let pi = std::f64::consts::PI;
    let numbers = vec![1, 2, 3, 4, 5];
    let data = vec![vec![1, 2], vec![3, 4]];
    
    // Basic variable insertion
    println("Hello {name}, you are {age} years old!");
    
    // Complex expressions with method chaining
    println("Sum: {numbers.iter().sum::<i32>()}");
    println("Uppercase: {name.to_uppercase()}");
    println("Age squared: {age * age}");
    
    // Advanced formatting specifiers
    println("PI standard: {pi}");
    println("PI with 2 decimals: {pi:.2}");
    println("PI with 6 decimals: {pi:.6}");
    println("PI scientific: {pi:e}");
    println("Age padded: {age:04}");
    println("Age hex: {age:x}");
    println("Age binary: {age:b}");
    
    // Container formatting (unique to webrust)
    println("Numbers compact: {numbers:c}");
    println("Numbers JSON-like: {numbers:j}");
    println("2D array compact: {data:c}");
    println("2D array JSON: {data:j}");
    
    // Conditional expressions in f-strings
    let status = if age >= 18 { "adult" } else { "minor" };
    println("You are an {status}");
    
    // Complex turbofish expressions
    let reversed: String = name.chars().rev().collect();
    println("Reversed name: {reversed}");
    let parsed = age.to_string().parse::<f64>().unwrap_or(0.0);
    println("Age as float: {parsed}");
}


### 📊 Intelligent Table System

webrust automatically generates beautiful, professional tables from any data structure:


use webrust::prelude::*;
use std::collections::HashMap;

#[gui]
fn main() {
    // Simple vector operations
    let quarterly_sales = vec![125000, 145000, 138000, 162000];
    println("@(blue, bold)Quarterly Sales Performance:");
    table(&quarterly_sales).header(["Q1", "Q2", "Q3", "Q4"]);
    
    // Pivot for different analytical view
    println("Pivoted view:");
    table(&quarterly_sales).header(["Q1", "Q2", "Q3", "Q4"]).pivot().header(["Sales"]);
    
    // Employee performance matrix
    let mut performance = HashMap::new();
    performance.insert("Alice Johnson", vec![92, 88, 95, 91]);
    performance.insert("Bob Smith", vec![87, 92, 89, 94]);
    performance.insert("Carol Davis", vec![94, 96, 93, 97]);
    performance.insert("David Wilson", vec![89, 85, 91, 88]);
    
    println("@(green, bold)Employee Performance Scores:");
    table(&performance).header(["Q1", "Q2", "Q3", "Q4"]);
    
    // Pivot for quarterly comparison
    println("Performance by quarter:");
    table(&performance).pivot();
    
    // Complex nested business data (auto-flattening)
    let mut cities_data = HashMap::new();
    let mut france = HashMap::new();
    let mut paris = HashMap::new();
    paris.insert("population".to_string(), "2.2M".to_string());
    paris.insert("attractions".to_string(), "Eiffel Tower, Louvre".to_string());
    france.insert("Paris".to_string(), paris);
    
    let mut marseille = HashMap::new();
    marseille.insert("population".to_string(), "870K".to_string());
    marseille.insert("attractions".to_string(), "Old Port, Calanques".to_string());
    france.insert("Marseille".to_string(), marseille);
    
    let mut usa = HashMap::new();
    let mut new_york = HashMap::new();
    new_york.insert("population".to_string(), "8.4M".to_string());
    new_york.insert("attractions".to_string(), "Statue of Liberty, Times Square".to_string());
    usa.insert("New York".to_string(), new_york);
    cities_data.insert("France".to_string(), france);
    cities_data.insert("USA".to_string(), usa);
    
    println("@(orange, bold)3-level nested structure (auto-flattened):");
    table(&cities_data);
    
    println("Pivoted city data:");
    table(&cities_data).pivot();
    
    // Survey data with visual grouping (.merge())
    let survey_data = vec![
        vec!["Excellent", "Customer Service"],
        vec!["Excellent", "Product Quality"],
        vec!["Excellent", "Website Design"],
        vec!["Good", "Delivery Speed"],
        vec!["Good", "Ordering Process"],
        vec!["Average", "Price"],
        vec!["Average", "Support Hours"],
    ];
    
    println("@(cyan, bold)Survey Results (with visual grouping):");
    table(&survey_data).header(["Rating", "Aspect"]).merge();
    // .merge() visually groups identical adjacent cells
    
    // Color matrix demonstration
    let color_matrix = vec![
        vec!["Red", "Red", "Blue"],
        vec!["Red", "Red", "Blue"],
        vec!["Green", "Green", "Blue"],
    ];
    
    println("Color pattern visualization:");
    table(&color_matrix).header(["1", "2", "3"]).merge();
}


#### Advanced Table Features
- **🔄 Pivot Operations** - Transform rows to columns for different analytical views
- **🎨 Visual Grouping** - `.merge()` combines identical adjacent cells for clean presentation
- **🏗️ Auto-Flattening** - Handles complex nested HashMaps automatically
- **📐 LaTeX Support** - Mathematical notation in headers and data values
- **📊 Smart Formatting** - Numbers right-aligned, text left-aligned, automatic sizing
- **🎯 Professional Styling** - Publication-quality HTML output with CSS

### 🎯 Advanced PrintBox System

Create professional layouts with sophisticated styling and borders:


use webrust::prelude::*;

#[gui]
fn main() {
    // Basic styling with colors and borders
    println("@(red, bold)Important Message")
        .weight(2).color("red").background("lightgray").radius(8);
    
    // Custom alignment and spacing
    print("Left").align("left").width(100).weight(1).color("blue");
    print("Center").align("center").width(100).weight(1).color("green");
    print("Right").align("right").width(100).weight(1).color("red");
    println("");
    
    // Selective borders (top, right, bottom, left)
    println("Partial borders only")
        .border(true, false, true, false)
        .weight(3).color("purple").radius(10);
    
    // Professional table layout with PrintBox
    println("@(cyan, bold)Multiplication Table:").space(0);
    
    // Header row
    print("×").weight(1).color("white").background("darkblue").width(25).align("center");
    for j in 1.to(6) {
        print("{j}").weight(1).color("white").background("darkblue").width(25).align("center");
    }
    println("").space(0);
    
    // Data rows with styled cells
    for i in 1.to(6) {
        print("{i}").weight(1).color("white").background("darkblue").width(25).align("center");
        for j in 1.to(6) {
            print("{i * j}").weight(1).color("darkblue").background("lightblue").width(25).align("center");
        }
        println("").space(0);
    }
    
    // Advanced styling combinations
    println("Professional Card")
        .weight(2).color("darkgreen").background("lightgreen")
        .radius(15).width(200).align("center").space(5);
    
    // Different border styles
    println("Dashed outline").weight(2).style("dashed").color("orange");
    println("Dotted frame").weight(1).style("dotted").color("purple");
}


### 📐 LaTeX Mathematical Rendering

High-quality mathematical expressions with intelligent string handling:


use webrust::prelude::*;

#[gui]
fn main() {
    println("@(blue, bold)Mathematical Expressions Guide");
    
    // Simple formulas (regular strings work fine)
    println("@(green)Basic Physics:");
    println("Energy: $(E = mc^2)");
    println("Force: $(F = ma)");
    println("Momentum: $(p = mv)");
    println("Power: $(P = Fv)");
    
    // Mixed variables and LaTeX
    let a = 3.0;
    let b = 4.0;
    let c = (a*a + b*b).sqrt();
    println("@(purple)Pythagorean Theorem:");
    println("Given a = {a} and b = {b}");
    println("Then $(c = \\sqrt{a^2 + b^2}) = {c:.2}");
    
    // Trigonometric table with LaTeX headers
    let trig = vec![
        vec!["$(0)",      "$(0)",           "$(1)",           "$(0)"],
        vec!["$(\\pi/4)",  "$(\\sqrt{2}/2)", "$(\\sqrt{2}/2)", "$(1)"],
        vec!["$(\\pi/2)",  "$(1)",           "$(0)",           "not defined"],
    ];
    println("@(orange, bold)Trigonometric Values:");
    table(&trig).header([
        "$(\\theta)",
        "$(\\sin\\theta)",
        "$(\\cos\\theta)",
        "$(\\tan\\theta)",
    ]);
    
    // Complex equations (raw strings recommended for clarity)
    println("@(red, bold)Advanced Equations (use raw strings):");
    let equations = vec![
        vec!["Maxwell 1", r"$(\nabla \cdot \mathbf{E} = \frac{\rho}{\epsilon_0})"],
        vec!["Maxwell 2", r"$(\nabla \cdot \mathbf{B} = 0)"],
        vec!["Maxwell 3", r"$(\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t})"],
        vec!["Schrödinger", r"$(i\hbar\frac{\partial}{\partial t}\Psi = \hat{H}\Psi)"],
        vec!["Fourier Transform", r"$(\mathcal{F}\{f(t)\} = \int_{-\infty}^{\infty} f(t) e^{-2\pi i \xi t} dt)"],
    ];
    table(&equations).header(["Equation", "Mathematical Form"]);
    
    // Matrix representations (raw strings essential for readability)
    println("@(cyan, bold)Transformation Matrices:");
    let transforms_2d = vec![
        vec!["2D Rotation", r"$(\begin{pmatrix} \cos\theta & -\sin\theta \\ \sin\theta & \cos\theta \end{pmatrix})"],
        vec!["2D Scaling", r"$(\begin{pmatrix} s_x & 0 \\ 0 & s_y \end{pmatrix})"],
        vec!["Reflection X", r"$(\begin{pmatrix} 1 & 0 \\ 0 & -1 \end{pmatrix})"],
    ];
    table(&transforms_2d).header(["Transform", "2D Matrix"]);
    
    let transforms_3d = vec![
        vec!["3D Translation", r"$(\begin{pmatrix} 1 & 0 & 0 & t_x \\ 0 & 1 & 0 & t_y \\ 0 & 0 & 1 & t_z \\ 0 & 0 & 0 & 1 \end{pmatrix})"],
        vec!["3D Scaling", r"$(\begin{pmatrix} s_x & 0 & 0 & 0 \\ 0 & s_y & 0 & 0 \\ 0 & 0 & s_z & 0 \\ 0 & 0 & 0 & 1 \end{pmatrix})"],
    ];
    table(&transforms_3d).header(["Transform", "3D Homogeneous Matrix"]);
    
    // Scientific data with LaTeX notation
    let experiments = vec![
        vec!["Experiment A", r"$(2.45 \times 10^{-3})", r"$(±0.02)", "m/s²", "✅"],
        vec!["Experiment B", r"$(1.87 \times 10^{-2})", r"$(±0.05)", "m/s²", "✅"],
        vec!["Experiment C", r"$(3.12 \times 10^{-3})", r"$(±0.01)", "m/s²", "❌"],
    ];
    table(&experiments).header(["Test", "Acceleration", "Error", "Unit", "Valid"]);
    
    println("\n@(bright_green, bold)💡 LaTeX Pro Tips:");
    println("@(gray, italic)Simple: $(E = mc^2) - regular strings work fine");
    println("@(gray, italic)Complex: r\"$(\\\\frac{...}{...})\" - raw strings for clarity");
    println("@(gray, italic)Matrices: r\"$(\\\\begin{pmatrix}...)\" - raw strings essential");
}


### 🔄 Python-like Ranges and Enumeration

Intuitive iteration patterns that feel natural:


use webrust::prelude::*;

#[gui]
fn main() {
    // Basic numeric ranges
    println("@(green, bold)Basic Ranges:");
    print("0 to 10: ");
    for i in 0.to(10) {
        print("{i} ");
    }
    println("");
    
    // Custom steps
    println("@(blue, bold)Even numbers (step 2):");
    for i in 0.to(21).by(2) {
        print("{i} ");
    }
    println("");
    
    // Reverse ranges (automatic direction detection)
    println("@(red, bold)Countdown:");
    for i in 10.to(0) {
        print("{i} ");
    }
    println("");
    
    // Negative steps
    println("@(purple, bold)Negative step (-3):");
    for i in 20.to(0).by(-3) {
        print("{i} ");
    }
    println("");
    
    // Float ranges
    println("@(orange, bold)Float range (0.5 step):");
    for x in 0.0.to(3.1).by(0.5) {
        print("{x} ");
    }
    println("");
    
    // Character ranges
    println("@(cyan, bold)Character ranges:");
    print("a-z: ");
    for c in 'a'.to('z') {
        print("{c} ");
    }
    println("");
    
    print("Every 2nd letter: ");
    for c in 'A'.to('Z').by(2) {
        print("{c} ");
    }
    println("");
    
    // Enumeration (Python-style)
    println("@(yellow, bold)Enumeration Examples:");
    let fruits = vec!["apple", "banana", "cherry", "date"];
    for (index, fruit) in enumerate(&fruits) {
        println("  {index}: {fruit}");
    }
    
    // Enumerate with ranges
    println("Enumerate range values:");
    for (index, value) in enumerate(5.to(10)) {
        println("  Position {index}: Value {value}");
    }
    
    // Complex enumeration with business logic
    let sales_data = vec![125000, 145000, 138000, 162000];
    println("@(magenta, bold)Quarterly Analysis:");
    for (quarter, sales) in enumerate(&sales_data) {
        let quarter_name = format!("Q{}", quarter + 1);
        let performance = if *sales > 140000 { "Excellent" } else { "Good" };
        println("  {quarter_name}: ${sales} - {performance}");
    }
}


### 🎭 Rich Styling System

Comprehensive text styling with colors and formatting:


use webrust::prelude::*;

#[gui]
fn main() {
    // Basic color palette
    println("@(red)Red text");
    println("@(green)Green text");
    println("@(blue)Blue text");
    println("@(yellow)Yellow text");
    println("@(cyan)Cyan text");
    println("@(magenta)Magenta text");
    println("@(white)White text");
    println("@(black)Black text");
    
    // Extended color palette
    println("@(orange)Orange@() @(purple)Purple@() @(pink)Pink");
    println("@(gray)Gray@() @(darkred)Dark Red@() @(darkgreen)Dark Green");
    println("@(darkblue)Dark Blue@() @(bright_cyan)Bright Cyan");
    
    // Text formatting styles
    println("@(bold)Bold text");
    println("@(italic)Italic text");
    println("@(underline)Underlined text");
    println("@(strike)Strikethrough text");
    
    // Combined styles (comma-separated)
    println("@(red, bold)Red and bold");
    println("@(green, italic, underline)Green, italic, and underlined");
    println("@(blue, bold, italic)Blue, bold, and italic");
    
    // Dynamic styling with variables
    let status = "success";
    let color = match status {
        "success" => "green",
        "warning" => "yellow",
        "error" => "red",
        _ => "white"
    };
    println("@({color}, bold)Status: {status}");
    
    // Conditional styling in loops
    println("@(purple, bold)Progress Indicators:");
    for i in 1.to(5) {
        let (color, symbol) = match i {
            1 => ("red", "●"),
            2 => ("orange", "●"), 
            3 => ("yellow", "●"),
            4 => ("green", "●"),
            _ => ("blue", "★")
        };
        println("@({color})Step {i}: {symbol} Processing...");
    }
    
    // Complex mixed styling
    println("@(purple, bold)Analysis:@() The @(green, bold)revenue@() increased by @(blue, italic)15.5%@() this @(red, underline)quarter@().");
}


### 🔒 Advanced Type-Safe Input

Real-time validation with comprehensive error handling:


use webrust::prelude::*;

#[gui]
fn main() {
    println("@(blue, bold)🔒 Type-Safe Input Demonstration");
    
    // Basic types with automatic validation
    let age: i32 = input("Enter your age (integer):");
    let height: f64 = input("Enter your height in meters (float):");
    let married: bool = input("Are you married? (true/false):");
    let initial: char = input("Enter your first initial (single character):");
    let name: String = input("Enter your full name:");
    
    // Display with rich formatting
    println("\n@(green, bold)📋 Personal Information Summary:");
    println("Name: @(cyan, bold){name}");
    println("Age: @(yellow){age}@() years");
    println("Height: @(blue){height:.2}@() meters");
    println("Initial: @(magenta)'{initial}'");
    println("Married: @(purple){married}");
    
    // Advanced calculations with validated input
    let age_months = age * 12;
    let height_cm = (height * 100.0) as i32;
    let bmi_weight: f64 = input("Enter your weight in kg for BMI calculation:");
    let bmi = bmi_weight / (height * height);
    
    println("\n@(orange, bold)📊 Calculated Metrics:");
    println("Age in months: @(cyan){age_months}");
    println("Height in cm: @(cyan){height_cm}");
    println("BMI: @(cyan){bmi:.1}");
    
    // Conditional logic based on validated input
    let age_category = match age {
        0..=12 => ("child", "blue"),
        13..=19 => ("teenager", "green"), 
        20..=64 => ("adult", "orange"),
        _ => ("senior", "purple")
    };
    
    let bmi_category = match bmi {
        bmi if bmi < 18.5 => ("underweight", "blue"),
        bmi if bmi < 25.0 => ("normal", "green"),
        bmi if bmi < 30.0 => ("overweight", "orange"),
        _ => ("obese", "red")
    };
    
    println("\n@(purple, bold)📈 Health Categories:");
    println("Age category: @({}, bold){}", age_category.1, age_category.0);
    println("BMI category: @({}, bold){}", bmi_category.1, bmi_category.0);
    
    // Generate comprehensive summary table
    let summary = vec![
        vec!["Name", &name],
        vec!["Age", &format!("{age} ({}) ", age_category.0)],
        vec!["Height", &format!("{height:.2}m ({height_cm}cm)")],
        vec!["Weight", &format!("{bmi_weight}kg")],
        vec!["BMI", &format!("{bmi:.1} ({})", bmi_category.0)],
        vec!["Married", &married.to_string()],
        vec!["Initial", &initial.to_string()]
    ];
    
    println("\n@(cyan, bold)📋 Complete Profile:");
    table(&summary).header(["Field", "Value"]);
    
    // Health recommendations based on data
    println("\n@(green, bold)💡 Recommendations:");
    if bmi < 18.5 {
        println("@(blue)Consider consulting a nutritionist for healthy weight gain.");
    } else if bmi > 25.0 {
        println("@(orange)Consider a balanced diet and regular exercise.");
    } else {
        println("@(green)Excellent! Maintain your healthy lifestyle.");
    }
}


### 🔧 GUI Customization

Customize the appearance with comprehensive theme options:


use webrust::prelude::*;

// Dark professional theme
#[gui(bg = "navy", fg = "white", font = "Courier New", color = "lightblue", size = "14px")]
fn dark_theme() {
    println("@(cyan, bold)🌙 Dark Professional Theme");
    println("Perfect for development and data analysis.");
    println("Navy background with light blue accents.");
}

// Light clean theme  
#[gui(bg = "white", fg = "lightgray", font = "Arial", color = "black", size = "16px")]
fn light_theme() {
    println("@(blue, bold)☀️ Light Clean Theme");
    println("Ideal for presentations and documentation.");
    println("Clean white background with excellent readability.");
}

// Scientific theme
#[gui(bg = "darkslategray", fg = "lightsteelblue", font = "Georgia", color = "white", size = "13px")]
fn scientific_theme() {
    println("@(yellow, bold)🔬 Scientific Theme");
    println("Optimized for mathematical and scientific work.");
    println("Reduces eye strain during long analysis sessions.");
}

// Business theme
#[gui(bg = "midnightblue", fg = "aliceblue", font = "Segoe UI", color = "lightcyan", size = "15px")]
fn business_theme() {
    println("@(gold, bold)💼 Business Professional Theme");
    println("Corporate-friendly colors and typography.");
    println("Perfect for client presentations and reports.");
}

// Default (minimal configuration)
#[gui]
fn main() {
    println("@(green, bold)🎨 Theme Selection");
    println("Choose your preferred theme:");
    println("• @(blue)dark_theme()@() - Dark professional");
    println("• @(orange)light_theme()@() - Light clean"); 
    println("• @(purple)scientific_theme()@() - Scientific work");
    println("• @(cyan)business_theme()@() - Business professional");
    println("\nReplace the function name in your code and restart!");
}


## 🛠 Examples

The crate includes comprehensive examples showcasing all features:

### Running Examples

bash
# Basic I/O with f-strings and validation
cargo run --example py_simpleio

# Advanced I/O with LaTeX and styling  
cargo run --example py_advancedio

# Range and enumeration patterns
cargo run --example py_utils

# Comprehensive table demonstrations
cargo run --example py_table


### Example Structure


examples/
├── py_simpleio.rs          # F-strings, input validation, expressions
├── py_advancedio.rs        # LaTeX rendering, PrintBox styling
├── py_utils.rs             # Ranges, enumerate, combinations
└── py_table.rs             # Advanced tables, pivot, merge, LaTeX


Each example is a complete, runnable application demonstrating different aspects of webrust.

## 🚀 API Reference

### Core Functions

#### Input Functions

let value: T = input("prompt");          // Type-safe input with validation
let text = input_string("prompt");       // String input (explicit)
let result = try_input::<T>("prompt");   // Returns Result<T, ParseError>


#### Output Functions

print("text");     // Inline output
println("text");   // Line output with PrintBox chaining


#### Table Functions

table(&data);                           // Auto-generated table
table(&data).header(["A", "B", "C"]);   // With column headers
table(&data).pivot();                   // Transpose rows/columns
table(&data).merge();                   // Merge identical adjacent cells


### Range and Enumeration

// Numeric ranges
for i in start.to(end) { }              // Basic range
for i in start.to(end).by(step) { }     // Custom step
for x in 0.0.to(5.0).by(0.1) { }       // Float range

// Character ranges
for c in 'a'.to('z') { }               // Character range
for c in 'A'.to('Z').by(2) { }         // Custom step

// Enumeration
for (index, item) in enumerate(iter) { } // With automatic indexing


### Styling and PrintBox

// Text styling
println("@(color, style)text");

// PrintBox methods (chainable)
.weight(px)                    // Border thickness
.color("color")                // Border color  
.style("solid/dashed/dotted")  // Border style
.radius(px)                    // Rounded corners
.width(px)                     // Fixed width
.align("left/center/right")    // Text alignment
.space(px)                     // Line spacing
.background("color")           // Background color
.border(t, r, b, l)           // Individual border sides (bool)


### F-String Formatting

// Basic: {variable}
// Expressions: {variable.method().chain()}
// Formatting: {variable:specifier}

// Format specifiers:
// :c     - Compact container format
// :j     - JSON-like container format  
// :.2    - 2 decimal places
// :.6    - 6 decimal places
// :e     - Scientific notation
// :.0    - No decimal places
// :04    - Zero-padded to 4 digits
// :x/:X  - Hexadecimal (lower/upper)
// :b     - Binary
// :o     - Octal


### LaTeX Math

// Inline math: $(expression)
println("Einstein: $(E = mc^2)");
println("Variable with math: value = {var}, formula: $(ax^2 + bx + c)");

// Complex expressions (use raw strings)
println("Complex: ", r"$(\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t})");


### GUI Themes

#[gui]                                    // Default theme
#[gui(bg = "color")]                      // Custom background only
#[gui(bg = "color", fg = "color")]        // Background + foreground
#[gui(bg = "navy", fg = "white", font = "Arial", color = "lightblue", size = "14px")]


## 🎯 Real-World Examples

### Scientific Data Analysis


use webrust::prelude::*;
use std::collections::HashMap;

#[gui(bg = "darkslategray", fg = "lightsteelblue", font = "Georgia", color = "white")]
fn main() {
    println("@(cyan, bold)🔬 Scientific Data Analysis Platform");
    
    // Collect experimental data
    let experiment_count: usize = input("Number of experiments:");
    let mut experiments = Vec::new();
    
    for i in 1.to(experiment_count + 1) {
        println("@(yellow)Experiment {i}:");
        let temperature: f64 = input("  Temperature (°C):");
        let pressure: f64 = input("  Pressure (atm):");
        let reaction_rate: f64 = input("  Reaction rate (mol/s):");
        
        experiments.push(vec![
            format!("Exp {i}"),
            format!("{temperature:.1}"),
            format!("{pressure:.2}"),
            format!("{reaction_rate:.3}")
        ]);
    }
    
    // Display data table
    println("\n@(green, bold)📊 Experimental Results:");
    table(&experiments).header(["ID", "Temp (°C)", "Pressure (atm)", "Rate (mol/s)"]);
    
    // Statistical analysis
    let temperatures: Vec<f64> = experiments.iter()
        .map(|row| row[1].parse().unwrap())
        .collect();
    let rates: Vec<f64> = experiments.iter()
        .map(|row| row[3].parse().unwrap())
        .collect();
    
    let avg_temp: f64 = temperatures.iter().sum::<f64>() / temperatures.len() as f64;
    let avg_rate: f64 = rates.iter().sum::<f64>() / rates.len() as f64;
    
    // Correlation analysis
    let correlation = calculate_correlation(&temperatures, &rates);
    
    println("\n@(purple, bold)📈 Statistical Analysis:");
    println("Average temperature: @(cyan){avg_temp:.1}°C");
    println("Average reaction rate: @(cyan){avg_rate:.3} mol/s");
    println("Temperature-rate correlation: @(yellow){correlation:.3}");
    
    // Mathematical relationships
    println("\n@(orange, bold)📐 Mathematical Relationships:");
    println("Arrhenius equation: $(k = A e^{{-E_a/(RT)}})");
    println("Where:");
    println("  $(k)$ = reaction rate constant");
    println("  $(A)$ = pre-exponential factor");  
    println("  $(E_a)$ = activation energy");
    println("  $(R)$ = gas constant");
    println("  $(T)$ = temperature (K)");
    
    // Results interpretation
    let interpretation = if correlation.abs() > 0.7 {
        ("strong", "green")
    } else if correlation.abs() > 0.3 {
        ("moderate", "orange") 
    } else {
        ("weak", "red")
    };
    
    println("\n@(blue, bold)🔍 Interpretation:");
    println("The correlation between temperature and reaction rate is");
    println("@({}, bold){}@() (r = {correlation:.3})", interpretation.1, interpretation.0);
}

fn calculate_correlation(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y).map(|(a, b)| a * b).sum();
    let sum_x2: f64 = x.iter().map(|a| a * a).sum();
    let sum_y2: f64 = y.iter().map(|b| b * b).sum();
    
    (n * sum_xy - sum_x * sum_y) / 
    ((n * sum_x2 - sum_x * sum_x) * (n * sum_y2 - sum_y * sum_y)).sqrt()
}


### Financial Portfolio Dashboard


use webrust::prelude::*;
use std::collections::HashMap;

#[gui(bg = "midnightblue", fg = "aliceblue", font = "Segoe UI", color = "lightcyan")]
fn main() {
    println("@(gold, bold)💼 Portfolio Management Dashboard");
    
    // Portfolio data collection
    let portfolio_name: String = input("Portfolio name:");
    let initial_investment: f64 = input("Initial investment ($):");
    
    let mut holdings = HashMap::new();
    let assets = vec!["AAPL", "GOOGL", "MSFT", "TSLA", "SPY"];
    
    for asset in &assets {
        let shares: f64 = input(&format!("Shares of {asset}:"));
        let price: f64 = input(&format!("Current price of {asset} ($):"));
        holdings.insert(*asset, (shares, price, shares * price));
    }
    
    // Calculate portfolio metrics
    let total_value: f64 = holdings.values().map(|(_, _, value)| value).sum();
    let total_return = total_value - initial_investment;
    let return_percentage = (total_return / initial_investment) * 100.0;
    
    // Display portfolio overview
    println("\n@(yellow, bold)📊 Portfolio: {portfolio_name}");
    
    let overview = vec![
        vec!["Initial Investment", &format!("${initial_investment:,.2}")],
        vec!["Current Value", &format!("${total_value:,.2}")],
        vec!["Total Return", &format!("${total_return:,.2}")],
        vec!["Return %", &format!("{return_percentage:+.2}%")],
    ];
    table(&overview).header(["Metric", "Value"]);
    
    // Detailed holdings table
    println("\n@(green, bold)📈 Holdings Breakdown:");
    let mut holdings_table = Vec::new();
    for (asset, (shares, price, value)) in &holdings {
        let weight = (value / total_value) * 100.0;
        holdings_table.push(vec![
            asset.to_string(),
            format!("{shares:.2}"),
            format!("${price:.2}"),
            format!("${value:,.2}"),
            format!("{weight:.1}%")
        ]);
    }
    table(&holdings_table).header(["Asset", "Shares", "Price", "Value", "Weight"]);
    
    // Portfolio analysis with LaTeX
    println("\n@(purple, bold)📐 Portfolio Mathematics:");
    println("Portfolio value: $(V_p = \\sum_{{i=1}}^n S_i \\times P_i)");
    println("Where:");
    println("  $(S_i)$ = shares of asset $i$");
    println("  $(P_i)$ = price of asset $i$");
    println("  $(V_p = {total_value:.2})$ = total portfolio value");
    
    println("\nReturn calculation: $(R = \\frac{{V_{{current}} - V_{{initial}}}}{{V_{{initial}}}}) \\times 100\\%)");
    println("$(R = \\frac{{{total_value:.0} - {initial_investment:.0}}}{{{initial_investment:.0}}} \\times 100\\% = {return_percentage:+.2}\\%)");
    
    // Risk analysis
    println("\n@(orange, bold)⚖️ Risk Analysis:");
    let largest_holding = holdings_table.iter()
        .max_by(|a, b| a[3].parse::<f64>().unwrap_or(0.0).partial_cmp(&b[3].parse::<f64>().unwrap_or(0.0)).unwrap())
        .unwrap();
    
    println("Largest holding: @(cyan){}@() ({})", largest_holding[0], largest_holding[4]);
    
    let diversification_score = if holdings.len() >= 5 { "Good" } else { "Limited" };
    let risk_level = match return_percentage {
        x if x > 20.0 => ("High Growth", "green"),
        x if x > 5.0 => ("Moderate", "yellow"),
        x if x > 0.0 => ("Conservative", "blue"),
        _ => ("Loss", "red")
    };
    
    println("Diversification: @(blue){}", diversification_score);
    println("Risk Profile: @({}, bold){}", risk_level.1, risk_level.0);
    
    // Performance indicators
    println("\n@(cyan, bold)🎯 Performance Indicators:");
    for (asset, (shares, price, value)) in holdings {
        let weight = (value / total_value) * 100.0;
        let color = if weight > 25.0 { "red" } else if weight > 15.0 { "orange" } else { "green" };
        
        println("@({color}){asset}@(): {weight:.1}% allocation")
            .weight(1).color(color).background("lightgray")
            .radius(3).width(150).align("left");
    }
}


### Educational Math Tutor


use webrust::prelude::*;

#[gui(bg = "navy", fg = "white", font = "Georgia", color = "lightcyan")]
fn main() {
    println("@(yellow, bold)🎓 Interactive Mathematics Tutor");
    
    let topics = vec![
        "Algebra", "Trigonometry", "Calculus", "Statistics", "Linear Algebra"
    ];
    
    println("\n@(cyan)Available topics:");
    for (i, topic) in enumerate(&topics) {
        println("  @(white){i + 1}. {topic}");
    }
    
    let choice: usize = input("Choose a topic (1-5):");
    if choice < 1 || choice > topics.len() {
        println("@(red)Invalid choice!");
        return;
    }
    
    let topic = &topics[choice - 1];
    println("\n@(green, bold)📚 {topic} Interactive Module");
    
    match choice {
        1 => algebra_module(),
        2 => trigonometry_module(), 
        3 => calculus_module(),
        4 => statistics_module(),
        5 => linear_algebra_module(),
        _ => {}
    }
}

fn algebra_module() {
    println("@(blue, bold)🔢 Quadratic Equation Solver");
    println("Solve equations of the form: $(ax^2 + bx + c = 0)");
    
    let a: f64 = input("Enter coefficient a:");
    let b: f64 = input("Enter coefficient b:");
    let c: f64 = input("Enter coefficient c:");
    
    if a == 0.0 {
        println("@(red)Error: 'a' cannot be zero for quadratic equations!");
        return;
    }
    
    let discriminant = b*b - 4.0*a*c;
    
    println("\n@(purple, bold)📋 Your Equation:");
    println("$({a}x^2 + {b}x + {c} = 0)");
    
    println("\n@(orange, bold)📐 Solution Process:");
    println("Discriminant: $(\\Delta = b^2 - 4ac = {b}^2 - 4({a})({c}) = {discriminant:.2})");
    
    if discriminant > 0.0 {
        let x1 = (-b + discriminant.sqrt()) / (2.0*a);
        let x2 = (-b - discriminant.sqrt()) / (2.0*a);
        
        println("\n@(green, bold)✅ Two Real Solutions:");
        println("$(x_1 = \\frac{{-b + \\sqrt{{\\Delta}}}}{{2a}} = \\frac{{-{b} + \\sqrt{{{discriminant:.2}}}}}{{2({a})}} = {x1:.3})");
        println("$(x_2 = \\frac{{-b - \\sqrt{{\\Delta}}}}{{2a}} = \\frac{{-{b} - \\sqrt{{{discriminant:.2}}}}}{{2({a})}} = {x2:.3})");
        
        // Verification
        let verify1 = a*x1*x1 + b*x1 + c;
        let verify2 = a*x2*x2 + b*x2 + c;
        
        println("\n@(cyan, bold)🔍 Verification:");
        println("$({a}({x1:.3})^2 + {b}({x1:.3}) + {c} = {verify1:.6})");
        println("$({a}({x2:.3})^2 + {b}({x2:.3}) + {c} = {verify2:.6})");
        
    } else if discriminant == 0.0 {
        let x = -b / (2.0*a);
        println("\n@(yellow, bold)⚡ One Real Solution (Repeated Root):");
        println("$(x = \\frac{{-b}}{{2a}} = \\frac{{-{b}}}{{2({a})}} = {x:.3})");
        
    } else {
        let real_part = -b / (2.0*a);
        let imaginary_part = (-discriminant).sqrt() / (2.0*a);
        
        println("\n@(blue, bold)📊 Complex Solutions:");
        println("$(x_1 = {real_part:.3} + {imaginary_part:.3}i)");
        println("$(x_2 = {real_part:.3} - {imaginary_part:.3}i)");
    }
    
    // Graphical insights
    println("\n@(purple, bold)📈 Graphical Analysis:");
    let vertex_x = -b / (2.0*a);
    let vertex_y = a*vertex_x*vertex_x + b*vertex_x + c;
    println("Vertex: $({vertex_x:.2}, {vertex_y:.2})");
    println("Opens: {}", if a > 0.0 { "Upward ⬆️" } else { "Downward ⬇️" });
}

fn trigonometry_module() {
    println("@(blue, bold)📐 Trigonometric Function Calculator");
    
    let angle: f64 = input("Enter angle in radians:");
    let degrees = angle * 180.0 / std::f64::consts::PI;
    
    let sin_val = angle.sin();
    let cos_val = angle.cos();
    let tan_val = angle.tan();
    
    println("\n@(green, bold)📊 Results for $(\\theta = {angle:.3})$ radians $({degrees:.1}°)$:");
    
    let trig_table = vec![
        vec!["$(\\sin(\\theta))", &format!("{sin_val:.6}")],
        vec!["$(\\cos(\\theta))", &format!("{cos_val:.6}")],
        vec!["$(\\tan(\\theta))", &format!("{tan_val:.6}")],
        vec!["$(\\sec(\\theta))", &format!("{:.6}", 1.0/cos_val)],
        vec!["$(\\csc(\\theta))", &format!("{:.6}", 1.0/sin_val)],
        vec!["$(\\cot(\\theta))", &format!("{:.6}", 1.0/tan_val)],
    ];
    table(&trig_table).header(["Function", "Value"]);
    
    // Verify Pythagorean identity
    let identity = sin_val*sin_val + cos_val*cos_val;
    println("\n@(purple, bold)🔍 Identity Verification:");
    println("Pythagorean identity: $(\\sin^2(\\theta) + \\cos^2(\\theta) = 1)");
    println("Calculated: $({sin_val:.6})^2 + ({cos_val:.6})^2 = {identity:.10}");
    
    let error = (identity - 1.0).abs();
    let accuracy = if error < 1e-10 { "Excellent" } else if error < 1e-6 { "Good" } else { "Fair" };
    println("Numerical accuracy: @(green){accuracy}@() (error: {error:.2e})");
}

fn linear_algebra_module() {
    println("@(blue, bold)🧮 Matrix Operations");
    
    println("Creating a 2×2 matrix transformation:");
    let a11: f64 = input("Enter element (1,1):");
    let a12: f64 = input("Enter element (1,2):");
    let a21: f64 = input("Enter element (2,1):");
    let a22: f64 = input("Enter element (2,2):");
    
    println("\n@(green, bold)🔢 Your Matrix:");
    println("$(A = \\begin{{pmatrix}} {a11} & {a12} \\\\ {a21} & {a22} \\end{{pmatrix}})");
    
    // Calculate determinant
    let det = a11 * a22 - a12 * a21;
    println("\n@(purple, bold)📐 Matrix Properties:");
    println("Determinant: $(\\det(A) = {a11} \\cdot {a22} - {a12} \\cdot {a21} = {det:.3})");
    
    // Matrix interpretation
    if det.abs() < 1e-10 {
        println("@(red)⚠️ Matrix is singular (non-invertible)");
    } else {
        println("@(green)✅ Matrix is invertible");
        
        // Calculate inverse
        let inv_det = 1.0 / det;
        println("\nInverse matrix: $(A^{{-1}} = \\frac{{1}}{{{det:.3}}} \\begin{{pmatrix}} {a22} & {a12_neg} \\\\ {a21_neg} & {a11} \\end{{pmatrix}})");
        
        let properties = vec![
            vec!["Determinant", &format!("{det:.3}")],
            vec!["Trace", &format!("{:.3}", a11 + a22)],
            vec!["Condition", if det != 0.0 { "Invertible" } else { "Singular" }],
        ];
        table(&properties).header(["Property", "Value"]);
    }
}

// Additional helper functions for statistics and calculus modules...
fn statistics_module() {
    println("@(blue, bold)📊 Statistical Analysis");
    // Implementation similar to above modules...
}

fn calculus_module() {
    println("@(blue, bold)∫ Calculus Explorer");
    // Implementation for derivatives and integrals...
}


## 🔮 Roadmap

### Current Version (v0.5.0)
- ✅ **Advanced F-Strings** - Complex expressions with rich formatting specifiers
- ✅ **Intelligent Tables** - Auto-generation with pivot, merge, and header support
- ✅ **PrintBox System** - Professional styling with borders, colors, and alignment
- ✅ **LaTeX Integration** - Mathematical expressions with `$(...)` syntax
- ✅ **Python-like Ranges** - `start.to(end).by(step)` with character support
- ✅ **Enumerate Function** - Index-value iteration patterns
- ✅ **Real-time Validation** - Client and server-side input checking
- ✅ **Theme Customization** - GUI appearance control
- ✅ **Smart Server Management** - Automatic lifecycle handling

### Planned Features (v0.6.0)
- **Enhanced Tables** - Sorting, filtering, and interactive features
- **Chart Generation** - Built-in plotting and data visualization
- **File I/O Support** - CSV, JSON, and Excel import/export
- **Template System** - Reusable UI components and layouts
- **Multi-page Applications** - Navigation and routing support
- **Real-time Updates** - Live data streaming and updates

### Future Directions (v1.0.0+)
- **3D Visualizations** - Three.js integration for scientific plotting
- **Database Integration** - Direct SQL and NoSQL database support
- **Plugin Architecture** - Extensible system for custom functionality
- **Mobile Support** - Progressive Web App capabilities
- **Collaboration Features** - Multi-user real-time applications
- **IDE Integration** - Enhanced tooling and development experience
- **Performance Optimization** - WebAssembly compilation for speed
- **Advanced Math** - Computer algebra system integration

## 🔧 Troubleshooting

### Common Issues

**🔴 Port 8080 is busy**
- Server automatically tries ports 8081, 8082, etc.
- Check console output for actual port used

**🔴 Browser doesn't open automatically**
- Navigate manually to `http://127.0.0.1:8080`
- Check firewall settings
- Ensure no other applications are blocking the port

**🔴 LaTeX not rendering**
- Check browser console for MathJax loading errors
- Ensure internet connection for CDN access
- Try refreshing the page

**🔴 Input validation errors**
- Check that input matches expected type
- For numbers, ensure no extra characters
- For booleans, use exactly "true" or "false"

**🔴 Tables not displaying correctly**
- Verify data structure is serializable
- Check for special characters in data
- Ensure balanced HashMap structures

**🔴 Styling not working**
- Verify correct syntax: `@(color, style)text@()`
- Check color names are valid CSS colors
- Ensure proper nesting of style tags

### Getting Help

- **📖 Documentation**: [docs.rs/webrust](https://docs.rs/webrust)
- **🐛 Bug Reports**: [GitHub Issues](https://github.com/gerarddubard/webrust/issues)
- **💬 Discussions**: [GitHub Discussions](https://github.com/gerarddubard/webrust/discussions)
- **📧 Contact**: For urgent issues, check the repository

## 🤝 Contributing

webrust welcomes contributions from the community! We're building the future of Rust development.

### Priority Areas

1. **New Python-like Features** - Functions and syntax that enhance Rust ergonomics
2. **Performance Optimizations** - Faster rendering, smaller bundles, better memory usage
3. **Documentation** - Examples, tutorials, and API documentation
4. **Testing** - Platform compatibility, edge cases, and integration tests
5. **Educational Content** - Learning materials and interactive tutorials

### Development Setup

bash
# Clone and setup
git clone https://github.com/gerarddubard/webrust.git
cd webrust

# Run tests
cargo test

# Test examples
cargo run --example py_simpleio
cargo run --example py_advancedio
cargo run --example py_utils
cargo run --example py_table

# Check formatting and linting
cargo fmt --check
cargo clippy

# Build documentation
cargo doc --open


### Contributing Guidelines

1. **Fork** the repository and create a feature branch
2. **Write tests** for new functionality
3. **Follow** Rust conventions and idioms
4. **Document** public APIs with examples
5. **Test** across platforms (Windows, macOS, Linux)
6. **Submit** a pull request with clear description

### Code Style

- Use meaningful variable and function names
- Include comprehensive documentation comments
- Write integration tests for user-facing features
- Follow the existing code formatting patterns
- Add examples to the `examples/` directory for new features

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🌟 Why Choose webrust?

### **For Python Developers**
- **Familiar syntax** with compile-time safety and zero-cost performance
- **Smooth transition** to systems programming without losing productivity
- **Rich ecosystem** of Rust libraries with Python-like convenience
- **Modern tooling** with instant feedback and professional results

### **For Rust Developers**
- **Simplified development** without sacrificing Rust's core strengths
- **Rapid prototyping** with immediate visual results
- **Educational tool** for demonstrating Rust concepts interactively
- **Professional presentation** layer for algorithms and data structures

### **For Educators and Students**
- **Interactive learning** with immediate feedback and visualization
- **Mathematical rendering** for scientific and engineering courses
- **Engaging examples** that students can modify and experiment with
- **Professional output** suitable for assignments and presentations

### **For Scientists and Researchers**
- **Publication-quality** mathematical expressions and data visualization
- **High performance** computing with user-friendly interfaces
- **Reproducible research** with compiled, self-contained applications
- **Data analysis** capabilities with statistical functions

### **For Businesses and Teams**
- **Rapid prototyping** of ideas and proof-of-concepts
- **Professional demos** for clients and stakeholders
- **Cross-platform deployment** from a single codebase
- **Maintainable applications** with Rust's reliability and safety

---

<div align="center">

**webrust: Where Python Simplicity Meets Rust Power** 🐍⚡🦀

*The future of interactive programming is here. Build it, run it, share it.*

[**📦 Install Now**](https://crates.io/crates/webrust) • [**📚 Read the Docs**](https://docs.rs/webrust) • [**🐙 View Source**](https://github.com/gerarddubard/webrust) • [**🤝 Contribute**](https://github.com/gerarddubard/webrust/blob/main/CONTRIBUTING.md)

**Ready to revolutionize your Rust development experience?**
bash
cargo add webrust


</div>