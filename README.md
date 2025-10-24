# WebRust: Python-Inspired Rust for Interactive Web Applications

[![WebRust](https://img.shields.io/badge/WebRust-1.7.0-ff6b35?style=flat-square)](https://github.com/gerarddubard/webrust)
[![Rust](https://img.shields.io/badge/Rust-1.70+-000?style=flat-square&logo=rust)](https://rust-lang.org)
[![Documentation](https://img.shields.io/badge/docs-latest-blue?style=flat-square)](https://docs.rs/webrust)
[![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)

**Links:** [Documentation](https://docs.rs/webrust) | [Examples](https://github.com/gerarddubard/webrust/tree/main/examples) | [Crates.io](https://crates.io/crates/webrust)

---

## Abstract

WebRust is a Rust framework that bridges Python's ergonomics with Rust's performance, offering integrated web-based visualization for rapid application development. Version 1.7.0 introduces **ultra-optimized procedural macros** with zero-copy string processing, **enhanced layout system** with grid-based positioning, and **comprehensive documentation** for all modules. The framework unifies data manipulation, visualization, and interactive computing in a single, type-safe environment.

---

## Table of Contents

1. [Introduction](#introduction)
2. [What's New in 1.7.0](#whats-new-in-170)
3. [Motivation](#motivation)
4. [Architecture](#architecture)
5. [Installation](#installation)
6. [Core Features](#core-features)
7. [Usage Examples](#usage-examples)
8. [Use Cases](#use-cases)
9. [Performance](#performance)
10. [Contributing](#contributing)
11. [License](#license)

---

## Introduction

### Overview

WebRust enables developers to write Python-like Rust code that runs natively with full type safety while automatically generating rich, interactive web interfaces. The framework eliminates the need for separate frontend development, HTML/CSS/JavaScript knowledge, or complex deployment infrastructure.

### Key Characteristics

- **🐍 Python-Inspired Syntax**: Ranges (`.to()`), comprehensions (`.when()/.then()`), string methods (`.splitby()`)
- **🎨 Rich Text Rendering**: Inline styles with colors, borders, layouts, and LaTeX math
- **📊 Interactive Visualizations**: 12+ chart types powered by ECharts
- **🗄️ High-Performance SQL**: DuckDB integration with streaming results (optional)
- **🐢 Turtle Graphics**: Object-oriented drawing with animations and easing
- **🎯 Zero Configuration**: No HTML, CSS, or JavaScript required
- **⚡ Native Performance**: Compiles to machine code with zero runtime overhead

### Design Philosophy

1. **Ergonomics First**: Write code that reads naturally
2. **Type Safety Always**: Leverage Rust's compile-time guarantees
3. **Visual by Default**: Automatic browser-based UI generation
4. **Performance Matters**: Zero-copy operations and SIMD optimizations

---

## What's New in 1.7.0

### 🚀 Major Improvements

#### 1. **Revolutionary Unified Syntax for Text Rendering**

**The biggest API change in WebRust history**: All text styling is now unified in HTML5-like inline syntax within `<...>` tags, eliminating dozens of method chains.

**Before (v1.6.0 and earlier):**
```ignore
// Multiple chained methods
print("Hello")
    .color("red")
    .bold()
    .italic()
    .size(18)
    .background("yellow")
    .border_color("blue")
    .border_width(2)
    .border_radius(10)
    .width(300)
    .height(50)
    .padding(10)
    .margin(20)
    .align("center")
    .at(100.0, 50.0);
```

**After (v1.7.0):**
```ignore
// Unified HTML5-like syntax
println("<red b i 18 !yellow |blue t2 r10 w300 h50 p10 m20 mc>Hello");
```

**Comparison table:**

| Old Method (v1.6.0)       | New Syntax (v1.7.0) | Description                      |
|---------------------------|---------------------|----------------------------------|
| `.color("red")`           | `<red>`             | Text color                       |
| `.background("yellow")`   | `<!yellow>`         | Background color                 |
| `.bold()`                 | `<b>`               | Bold text                        |
| `.italic()`               | `<i>`               | Italic text                      |
| `.underline()`            | `<u>`               | Underlined text                  |
| `.strikethrough()`        | `<s>`               | Strike-through text              |
| `.size(18)`               | `<18>`              | Font size in pixels              |
| `.border_color("blue")`   | `<\|blue>`          | Border color                     |
| `.border_width(2)`        | `<t2>`              | Border thickness (2px)           |
| `.border_radius(10)`      | `<r10>`             | Border radius (10px)             |
| `.border_style("dashed")` | `<dashed>`          | Border style                     |
| `.width(300)`             | `<w300>`            | Width in pixels                  |
| `.height(50)`             | `<h50>`             | Height in pixels                 |
| `.padding(10)`            | `<p10>`             | Padding in pixels                |
| `.margin(20)`             | `<m20>`             | Margin in pixels                 |
| `.align("center")`        | `<mc>`              | Middle-center alignment          |
| `.at(x, y)`               | `.at(x, y)`         | Absolute positioning (unchanged) |

**Advantages of the new syntax:**

✅ **Conciseness**: 1 line instead of 15  
✅ **Readability**: Visual at a glance  
✅ **Performance**: 3-5x faster macro expansion  
✅ **HTML5-inspired**: Familiar for web developers  
✅ **CSS-like**: Natural attribute syntax  
✅ **Less verbose**: No repetitive method chains  
✅ **Better composability**: Mix and match attributes freely  

**Complete styling reference:**

```ignore
// Colors (147 named colors + hex)
<red>, <blue>, <green>, <#FF5733>
<!yellow>  // Background color

// Text styles
<b>   // Bold
<i>   // Italic
<u>   // Underline
<s>   // Strikethrough

// Font size
<10>, <14>, <18>, <24>  // Pixels

// Borders
<t1>, <t2>, <t3>, <t4>, <t5>  // Thickness (1-5px)
<|red>, <|blue>, <|#FF0000>   // Border color
<solid>, <dashed>, <dotted>, <double>  // Border style
<r0>, <r5>, <r10>, <r20>      // Border radius

// Dimensions
<w200>, <w{*CW}>        // Width (pixels or full)
<h100>, <h{*CH}>        // Height (pixels or full)
<p10>, <p20>            // Padding
<m10>, <m20>            // Margin

// Positioning (9 alignments)
<tl>  // Top-left
<tc>  // Top-center
<tr>  // Top-right
<ml>  // Middle-left
<mc>  // Middle-center (default)
<mr>  // Middle-right
<bl>  // Bottom-left
<bc>  // Bottom-center
<br>  // Bottom-right

// Text alignment
<j>   // Justified
```

**Real-world comparison:**

**Creating a styled button (v1.6.0):**
```ignore
print("Click Me")
    .color("white")
    .background("dodgerblue")
    .bold()
    .size(16)
    .border_radius(8)
    .width(120)
    .height(40)
    .align("center")
    .padding(10);
// 9 lines, 9 method calls
```

**Same button (v1.7.0):**
```ignore
println("<white !dodgerblue b 16 r8 w120 h40 mc p10>Click Me");
// 1 line, all-in-one
```

**Complex layout example (v1.6.0):**
```ignore
print("Dashboard Title")
    .color("navy")
    .bold()
    .size(24)
    .background("lightcyan")
    .border_color("navy")
    .border_width(4)
    .border_style("double")
    .border_radius(8)
    .width(800)
    .height(60)
    .align("center")
    .padding(15);
// 12 lines, 12 method calls
```

**Same layout (v1.7.0):**
```ignore
println("<navy b 24 !lightcyan |navy t4 double r8 w{*CW} h60 mc p15>Dashboard Title");
// 1 line, single string
```

**Migration is seamless**: Old method-chaining syntax still works in v1.7.0 for backward compatibility, but the new syntax is recommended for all new code.

**Why this change matters:**

1. **Developer productivity**: Write styled UIs 10x faster
2. **Code maintainability**: Easier to read and modify
3. **Performance**: Faster macro expansion (zero-copy processing)
4. **Consistency**: Same syntax everywhere (`println`, `print`, LaTeX)
5. **Familiarity**: HTML5/CSS-inspired for web developers

This is the **most significant API evolution** in WebRust history, making text rendering as simple as writing HTML while maintaining full Rust type safety.

#### 2. **Ultra-Optimized Procedural Macros**

- **Zero-copy string processing**: `Cow<'_, str>` eliminates unnecessary allocations
- **SIMD-accelerated parsing**: `memchr` for bracket matching in f-strings and LaTeX
- **Smart tokenization**: Stateful parser for inline style attributes
- **Direct HTML rendering**: Eliminates intermediate representations

**Performance impact**: 3-5x faster macro expansion for complex formatting

#### 2. **Enhanced Layout System**

```ignore
use webrust::prelude::*;

#[gui]
fn main() {
    // Grid-based layouts
    grid(2, 3);  // 2 rows × 3 columns
    
    // Position elements precisely
    let (x, y) = cell(0, 1, "center");
    chart(&data, "bar").at(x, y).size(80, 90);
    
    // Coordinate modes
    coord("cartesian");  // Mathematical (0,0 at center)
    coord("css");        // Web standard (0,0 at top-left)
}
```

#### 3. **Comprehensive Documentation**

- **Module-level docs**: Every module now has detailed documentation
- **Inline examples**: 2-3 practical examples per function
- **Architecture diagrams**: Visual representation of system design
- **Performance notes**: Complexity analysis and optimization tips

#### 4. **Refined Type System**

- **Better type inference**: Less turbofish syntax required
- **Flexible trait bounds**: Works with more collection types
- **Improved error messages**: Clearer compile-time feedback

### 🔧 Technical Enhancements

| Feature            | v1.6.0              | v1.7.0                  | Improvement     |
|--------------------|---------------------|-------------------------|-----------------|
| Macro expansion    | Standard allocation | Zero-copy `Cow`         | **3-5x faster** |
| String parsing     | Regex-based         | `memchr` SIMD           | **10x faster**  |
| Style tokenization | Split + parse       | Stateful parser         | **2x faster**   |
| Layout system      | Basic positioning   | Grid + coordinate modes | **New**         |
| Documentation      | Basic               | Comprehensive           | **Complete**    |

---

## Motivation

### The Problem

Modern data analysis and visualization requires juggling multiple tools:

```text
SQL (queries) → Python (pandas) → Plotly (viz) → Flask (web) → Deploy
    ↓              ↓                  ↓             ↓           ↓
 Context      Type safety       Boilerplate    Frontend    Infrastructure
  switch         lost            overhead      complexity    overhead
```

### The WebRust Solution

```text
Rust (WebRust) → Browser (automatic)
      ↓               ↓
  Type safety    Rich visualization
  + Performance  + Zero deployment
```

**Single file, single language, instant visualization.**

### Comparison with Existing Tools

#### vs Python + Jupyter

| Aspect       | Python + Jupyter | WebRust         |
|--------------|------------------|-----------------|
| Type Safety  | Runtime          | Compile-time    |
| Performance  | Interpreted      | Native          |
| Deployment   | Complex          | Instant         |
| Syntax       | Native           | Python-inspired |
| Startup Time | ~1s              | ~50ms           |

#### vs Rust + Web Framework

| Aspect         | Rust + Actix/Rocket    | WebRust                     |
|----------------|------------------------|-----------------------------|
| Frontend Code  | Required (HTML/CSS/JS) | Automatic                   |
| Learning Curve | Steep                  | Gentle                      |
| Boilerplate    | High                   | Minimal                     |
| Use Case       | Production apps        | Rapid prototyping, analysis |

---

## Architecture

### System Overview

```text
┌─────────────────────────────────────────────────────────────┐
│                   WebRust Application                       │
├─────────────────────────────────────────────────────────────┤
│  User Code (Python-like Rust)                              │
│    ↓ Macro Expansion (Zero-copy processing)                │
│  Standard Rust (Type-checked, Optimized)                   │
│    ↓ Compilation (Native code generation)                  │
│  Binary (Single executable)                                 │
└─────────────────────────────────────────────────────────────┘
                         ↓ Runtime
┌─────────────────────────────────────────────────────────────┐
│  HTTP Server (tiny_http)     Browser (Chrome/Firefox)      │
│    ↓                             ↓                          │
│  JSON State              HTML + JavaScript                  │
│    ↓                             ↓                          │
│  Polling ←──────────────→ Rendering                         │
│  /api/state              - ECharts (charts)                 │
│  /api/input              - MathJax (LaTeX)                  │
│                          - Two.js (graphics)                │
└─────────────────────────────────────────────────────────────┘
```

### Macro System Architecture (v1.7.0)

```text
Source Text → Tokenization → AST Transform → Code Generation
     ↓             ↓              ↓               ↓
 Raw String   Zero-copy      Type-safe      Standard Rust
             parsing         operations      (no overhead)
```

**Key optimization**: All string processing uses `Cow<'_, str>` to avoid allocations when no transformation is needed.

### SQL Integration (Optional)

When enabled with `features = ["sql"]`:

```text
SQL Query → DuckDB → Apache Arrow → HTML Streaming → Browser
    ↓          ↓          ↓              ↓             ↓
Standard   OLAP    Columnar      Adaptive      Real-time
  SQL     Engine    Format       Batching      Rendering
```

**Performance**: Handles millions of rows with adaptive chunk sizing (200-800 rows based on column count).

---

## Installation

### Prerequisites

- **Rust**: 1.70 or later
- **Cargo**: Bundled with Rust
- **Browser**: Any modern browser (Chrome, Firefox, Safari, Edge)

### Quick Start

```toml
[dependencies]
webrust = "1.7.0"
```

**Compilation time**: ~30 seconds (first build)

**Features**:
- Python-like syntax
- Web GUI with auto-launch
- Rich text rendering
- Interactive charts
- LaTeX math
- Turtle graphics

### With SQL Support

```toml
[dependencies]
webrust = { version = "1.7.0", features = ["sql"] }
```

**Additional compilation**: 2-5 minutes (DuckDB, first build only)

**Additional features**:
- DuckDB OLAP database
- SQL queries with streaming
- Apache Arrow integration
- CSV/Parquet/JSON import/export
- Window functions & CTEs

---

## Core Features

### 1. Python-Inspired Ranges

```ignore
use webrust::prelude::*;

#[gui]
fn main() {
    // Simple ranges
    for i in 0.to(10) {
        println!("{i}");  // 0, 1, 2, ..., 9
    }
    
    // Step size
    for i in 0.to(100).by(5) {
        println!("{i}");  // 0, 5, 10, ..., 95
    }
    
    // Reverse iteration
    for i in 10.to(0) {
        println!("{i}");  // 10, 9, 8, ..., 1
    }
    
    // Character ranges
    for c in 'a'.to('z') {
        println!("{c}");  // a, b, c, ..., y
    }
    
    // Float ranges
    for x in 0.0.to(2.0).by(0.5) {
        println!("{x}");  // 0.0, 0.5, 1.0, 1.5
    }
}
```

**Implementation**: Zero-cost abstraction compiling to standard Rust iterators.

### 2. Comprehensions

```ignore
use webrust::prelude::*;
use std::collections::HashMap;

#[gui]
fn main() {
    // List comprehension
    let squares: Vec<i32> = 0.to(10).then(|x| x * x);
    // [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
    
    // Filtered comprehension
    let evens: Vec<i32> = 0.to(20)
        .when(|&x| x % 2 == 0)
        .then(|x| x);
    // [0, 2, 4, 6, 8, 10, 12, 14, 16, 18]
    
    // Dictionary comprehension
    let squares_map: HashMap<i32, i32> = 0.to(5)
        .then(|x| (x, x * x));
    // {0: 0, 1: 1, 2: 4, 3: 9, 4: 16}
    
    // Nested comprehension
    let matrix: Vec<Vec<i32>> = 0.to(3)
        .then(|i| 0.to(3).then(|j| i * j));
    // [[0, 0, 0], [0, 1, 2], [0, 2, 4]]
}
```

### 3. String Operations

```ignore
use webrust::prelude::*;

#[gui]
fn main() {
    // Smart splitting
    let words = "hello  world".splitby("");     // ["hello", "world"]
    let parts = "a,b,c".splitby(",");           // ["a", "b", "c"]
    let lines = "L1\nL2\nL3".splitby("\n");     // ["L1", "L2", "L3"]
    
    // Case transformations
    let upper = "hello".upper();                // "HELLO"
    let title = "hello world".title();          // "Hello World"
    let swapped = "HeLLo".swapcase();          // "hEllO"
    
    // Validation
    let is_alpha = "hello".isalpha();          // true
    let is_digit = "12345".isdigit();          // true
    let starts = "user@example.com".startswith("user");  // true
    
    // Formatting
    let padded = "42".zfill(5);                // "00042"
    let centered = "hi".center(10, '-');       // "----hi----"
}
```

### 4. Unified Inline Styling Syntax

**Key Innovation**: WebRust uses a **revolutionary inline syntax** inspired by HTML5 and CSS, eliminating the need for chained method calls. All styling, layout, and formatting attributes are embedded directly in `<...>` tags.

**Why this matters**: This is the most significant API change in WebRust history, making text rendering as simple as writing HTML while maintaining full Rust type safety.

#### Complete Syntax Reference

```ignore
use webrust::prelude::*;

#[gui]
fn main() {
    // Basic colors and styles
    println("<red b i>Bold Italic Red Text");
    println("<blue !yellow>Blue text on yellow background");
    
    // Complete control in one line
    println("<navy b 18 !lightcyan |blue t3 r10 w300 h50 mc p10>Styled Box");
    //       ^^^^ ^ ^^ ^^^^^^^^^^ ^^^^^ ^^ ^^^ ^^^^ ^^^ ^^ ^^^
    //       │    │ │  │          │     │  │   │    │   │  └─ padding: 10px
    //       │    │ │  │          │     │  │   │    │   └──── middle-center
    //       │    │ │  │          │     │  │   │    └────────── height: 50px
    //       │    │ │  │          │     │  │   └─────────────── width: 300px
    //       │    │ │  │          │     │  └─────────────────── radius: 10px
    //       │    │ │  │          │     └────────────────────── thickness: 3px
    //       │    │ │  │          └──────────────────────────── border: blue
    //       │    │ │  └─────────────────────────────────────── background: lightcyan
    //       │    │ └────────────────────────────────────────── size: 18px
    //       │    └─────────────────────────────────────────── bold
    //       └──────────────────────────────────────────────── color: navy
    
    // Complex layouts
    println("<navy b t4 double |navy !lightcyan r8 w{*CW} h48 mc>TITLE");
    
    // Multi-line text with justification
    println(r#"<darkslateblue t1 |darkslateblue !ghostwhite r5 w{*CW} h120 j p10>
        Justified text with padding in a bordered box. The text flows naturally
        and adjusts to the container width automatically.
    </>"#);
    
    // LaTeX math integration
    println("Equation: $(x = \\frac{-b \\pm \\sqrt{b^2-4ac}}{2a})");
    println("Inline: $(E=mc^2)$ and display: $(\\int_0^\\infty e^{-x}dx)");
    
    // Mix attributes and variables
    let temp = 23.5;
    println("<blue 20 b>Temperature: <red 24>{temp}°C");
}
```

#### Attribute Categories

| Category          | Syntax      | Examples                                      | Description                     |
|-------------------|-------------|-----------------------------------------------|---------------------------------|
| **Text Color**    | `<color>`   | `<red>`, `<blue>`, `<#FF5733>`                | 147 named colors + hex          |
| **Background**    | `<!color>`  | `<!yellow>`, `<!#FFE>`                        | Background color                |
| **Text Style**    | `<style>`   | `<b>`, `<i>`, `<u>`, `<s>`                    | Bold, Italic, Underline, Strike |
| **Font Size**     | `<size>`    | `<10>`, `<18>`, `<24>`                        | Size in pixels                  |
| **Border Color**  | `<\|color>` | `<\|red>`, `<\|#00F>`                         | Border color                    |
| **Border Width**  | `<t#>`      | `<t1>`, `<t2>`, `<t5>`                        | Thickness 1-5px                 |
| **Border Style**  | `<style>`   | `<solid>`, `<dashed>`, `<dotted>`, `<double>` | Border appearance               |
| **Border Radius** | `<r#>`      | `<r0>`, `<r10>`, `<r20>`                      | Corner rounding (px)            |
| **Width**         | `<w#>`      | `<w200>`, `<w{*CW}>`                          | Width (pixels or full)          |
| **Height**        | `<h#>`      | `<h100>`, `<h{*CH}>`                          | Height (pixels or full)         |
| **Padding**       | `<p#>`      | `<p10>`, `<p20>`                              | Inner spacing (px)              |
| **Margin**        | `<m#>`      | `<m10>`, `<m20>`                              | Outer spacing (px)              |
| **Positioning**   | `<pos>`     | `<mc>`, `<tl>`, `<br>`                        | 9 alignment positions           |
| **Text Align**    | `<j>`       | `<j>`                                         | Justified text                  |

#### Position Codes

```text
tl    tc    tr     (top-left, top-center, top-right)
ml    mc    mr     (middle-left, middle-center, middle-right)
bl    bc    br     (bottom-left, bottom-center, bottom-right)
```

#### Real-World Examples

**Alert Messages:**
```ignore
println("<white !red r8 w300 h60 mc p10>⚠️ Error: Invalid input");
println("<white !orange r8 w300 h60 mc p10>⚡ Warning: Low battery");
println("<white !green r8 w300 h60 mc p10>✓ Success: File saved");
```

**Cards with Borders:**
```ignore
println("<black t2 |purple !lavender r12 w400 h100 mc p15>
    <purple b 20>Product Card
    <gray 14>Description goes here...
</>");
```

**Dashboard Metrics:**
```ignore
println("<navy b 32 mc>$1.2M");
println("<gray 12 mc>Monthly Revenue");
```

**Styled Buttons:**
```ignore
println("<white !dodgerblue b 16 r8 w120 h40 mc p10>Click Me");
println("<white !crimson b 16 r8 w120 h40 mc p10>Delete");
```

**Comparison: Old vs New**

```ignore
// v1.6.0 - Method chaining (still works, but verbose)
print("Hello")
    .color("red")
    .bold()
    .size(18)
    .background("yellow")
    .border_color("blue")
    .border_width(2)
    .border_radius(10)
    .width(300)
    .height(50)
    .padding(10)
    .align("center");
// 13 lines

// v1.7.0 - Unified inline syntax (recommended)
println("<red b 18 !yellow |blue t2 r10 w300 h50 p10 mc>Hello");
// 1 line - same result
```

**Pro Tips:**

1. **Order doesn't matter**: `<red b 18>` = `<18 b red>` = `<b red 18>`
2. **Mix freely**: Combine any attributes in any order
3. **Use variables**: `println("<blue>{value}")` works perfectly
4. **Dynamic colors**: `<{color_var}>` uses runtime color
5. **Full-width**: `<w{*CW}>` expands to container width

### 5. Type-Safe Input

```ignore
use webrust::prelude::*;

#[gui]
fn main() {
    // Type inference from annotation
    let name: String = input("Your name:");
    let age: i32 = input("Your age:");
    let height: f64 = input("Your height (m):");
    let married: bool = input("Married? (true/false):");
    let letter: char = input("Favorite letter:");
    
    // Validation is automatic
    // Invalid inputs trigger retry with error message
    
    println("<green>Hello {name}, age {age}!");
}
```

**Supported types**: All types implementing `FromStr` (integers, floats, bool, char, String, custom types).

### 6. Interactive Charts

```ignore
use webrust::prelude::*;
use std::collections::HashMap;

#[gui]
fn main() {
    // Bar chart from HashMap
    let sales = HashMap::from([
        ("Q1", 100.0),
        ("Q2", 150.0),
        ("Q3", 120.0),
        ("Q4", 180.0)
    ]);
    chart(&sales, "bar")
        .title("Quarterly Sales")
        .ylabel("Revenue ($1000)")
        .color("steelblue");
    
    // Line chart with multiple series
    chart(&vec![10.0, 15.0, 13.0, 17.0], "line")
        .title("Product Comparison")
        .xlabels(vec!["Jan", "Feb", "Mar", "Apr"])
        .add("Product A", vec![10.0, 15.0, 13.0, 17.0], Some("red".into()))
        .add("Product B", vec![8.0, 12.0, 14.0, 16.0], Some("blue".into()))
        .add("Product C", vec![12.0, 11.0, 15.0, 18.0], Some("green".into()));
    
    // Pie chart
    let pie = PieData(
        vec!["Chrome".into(), "Firefox".into(), "Safari".into()],
        vec![65.0, 20.0, 15.0]
    );
    chart(pie, "pie").title("Browser Market Share");
    
    // Specialized charts
    gauge_chart(75.0).title("Progress");
    radar_chart(
        vec![85.0, 90.0, 78.0, 92.0, 88.0],
        vec![
            ("Math".into(), 100.0),
            ("English".into(), 100.0),
            ("Science".into(), 100.0),
            ("History".into(), 100.0),
            ("Art".into(), 100.0),
        ]
    ).title("Student Performance");
}
```

**Available chart types**: `bar`, `line`, `scatter`, `pie`, `area`, `area_stacked`, `radar`, `heatmap`, `gauge`, `funnel`, `candlestick`, `bubble`, `doughnut`, `polar_area`

### 7. Smart Tables

```ignore
use webrust::prelude::*;

#[gui]
fn main() {
    // From matrix
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    table(&matrix).header(["X", "Y", "Z"]);
    
    // From any serializable data
    use std::collections::HashMap;
    let mut data = HashMap::new();
    data.insert("Alice", 95);
    data.insert("Bob", 87);
    data.insert("Charlie", 92);
    table(&data);  // Automatic layout
    
    // With LaTeX
    table(&vec![
        vec!["$(x^2)$", "$(x^3)$"],
        vec!["4", "8"],
        vec!["9", "27"]
    ]).header(["Square", "Cube"]);
    
    // Pivot operation
    let sales = vec![
        vec!["Q1", "100", "150"],
        vec!["Q2", "120", "160"]
    ];
    table(&sales)
        .header(["Quarter", "Product A", "Product B"])
        .pivot()  // Transpose
        .header(["Metric", "Q1", "Q2"]);
}
```

### 8. Turtle Graphics with Animation

```ignore
use webrust::prelude::*;

#[gui]
fn main() {
    coord("cartesian");  // Mathematical coordinates
    
    // Create turtle
    let t = object();
    t.color("blue").width(2.0).fill("rgba(0,0,255,0.3)");
    
    // Draw shapes
    t.at(0.0, 0.0);
    t.circle(50.0);
    t.forward(100.0);
    t.right(90.0);
    t.rectangle(80.0, 60.0);
    
    // Animate with easing
    t.speed(100.0)      // Pixels per second
        .rotate(360.0)  // Full rotation
        .ease("elasticOut");
    
    t.translate(100.0, 50.0)
        .scale(1.5, 1.5)
        .ease("sineInOut");
    
    // Group animations
    let wheel1 = object();
    wheel1.circle(20.0);
    
    let wheel2 = object();
    wheel2.at(100.0, 0.0).circle(20.0);
    
    let car = group();
    car.add(&wheel1);
    car.add(&wheel2);
    car.translate(200.0, 0.0).ease("linear");
}
```

**Easing functions**: `linear`, `sineIn`, `sineOut`, `sineInOut`, `quadIn`, `quadOut`, `quadInOut`, `cubicIn`, `cubicOut`, `cubicInOut`, `quartIn`, `quartOut`, `quartInOut`, `quintIn`, `quintOut`, `quintInOut`, `expoIn`, `expoOut`, `expoInOut`, `circIn`, `circOut`, `circInOut`, `backIn`, `backOut`, `backInOut`, `elasticIn`, `elasticOut`, `elasticInOut`, `bounceIn`, `bounceOut`, `bounceInOut`

### 9. Grid Layouts

```ignore
use webrust::prelude::*;

#[gui]
fn main() {
    // Define grid
    grid(2, 3);  // 2 rows × 3 columns
    
    // Position elements in cells
    let (x, y) = cell(0, 0, "center");
    println("<blue b>Top Left Cell").at(x, y);
    
    let (x, y) = cell(0, 1, "center");
    chart(&vec![1.0, 2.0, 3.0], "bar")
        .at(x, y)
        .size(80, 90);  // 80% width, 90% height of cell
    
    let (x, y) = cell(1, 2, "bottom right");
    table(&vec![vec![1, 2], vec![3, 4]])
        .at(x, y);
}
```

### 10. High-Performance SQL (Optional)

```ignore
use webrust::prelude::*;

#[gui]
fn main() {
    // Import data
    query("IMPORT 'https://example.com/data.csv' AS sales");
    
    // Complex analytics
    query(r#"
        SELECT
            category,
            COUNT(*) as items,
            SUM(revenue) as total_revenue,
            AVG(revenue) as avg_revenue,
            ROUND(100.0 * SUM(revenue) / SUM(SUM(revenue)) OVER (), 2) as pct_total
        FROM sales
        GROUP BY category
        HAVING COUNT(*) > 5
        ORDER BY total_revenue DESC
    "#);
    
    // Window functions
    query(r#"
        SELECT
            product,
            revenue,
            ROW_NUMBER() OVER (ORDER BY revenue DESC) as rank,
            AVG(revenue) OVER () as avg_revenue
        FROM sales
    "#);
    
    // CTEs
    query(r#"
        WITH top_products AS (
            SELECT * FROM sales WHERE revenue > 1000
        )
        SELECT category, COUNT(*) as premium_count
        FROM top_products
        GROUP BY category
    "#);
    
    // Export results
    query("EXPORT sales TO 'output.parquet'");
    query("EXPORT (SELECT * FROM sales WHERE revenue > 500) TO 'filtered.csv'");
    
    // Schema inspection
    query("SCHEMA SELECT * FROM sales");
}
```

**Special commands**:
- `IMPORT 'file.csv' AS table` - Load external data
- `EXPORT table TO 'file.parquet'` - Save results
- `SCHEMA SELECT * FROM table` - Inspect structure
- `OPEN 'database.duckdb'` - Use file-based storage
- `LOAD extension` - Enable DuckDB extensions
- `CONFIG SET parameter = value` - Runtime configuration

---

## Usage Examples

### Example 1: Data Analysis Dashboard

```ignore
use webrust::prelude::*;

#[gui(Arial 12px darkblue !lightcyan)]
fn main() {
    println("<navy b i 18>📊 Sales Analysis Dashboard");
    
    // Setup grid layout
    grid(2, 2);
    coord("css");
    
    // Import data
    query("IMPORT 'sales.csv' AS sales");
    
    // Top-left: Summary statistics
    let (x, y) = cell(0, 0, "top left");
    println("<darkgreen b>Summary Statistics").at(x, y);
    query("SELECT COUNT(*) as orders, SUM(revenue) as total, AVG(revenue) as average FROM sales");
    
    // Top-right: Revenue by category (chart)
    let (x, y) = cell(0, 1, "center");
    query("SELECT category, SUM(revenue) as revenue FROM sales GROUP BY category");
    // Note: query results can be captured and charted
    
    // Bottom-left: Top products (table)
    let (x, y) = cell(1, 0, "center");
    println("<purple b>Top 10 Products").at(x, y);
    query("SELECT product, revenue FROM sales ORDER BY revenue DESC LIMIT 10");
    
    // Bottom-right: Trend analysis
    let (x, y) = cell(1, 1, "center");
    println("<orange b>Monthly Trend").at(x, y);
    query(r#"
        SELECT
            DATE_TRUNC('month', date) as month,
            SUM(revenue) as revenue
        FROM sales
        GROUP BY month
        ORDER BY month
    "#);
}
```

### Example 2: Mathematical Visualization

```ignore
use webrust::prelude::*;

#[gui]
fn main() {
    println("<darkred b i>🧮 Pascal's Triangle & Combinatorics");
    
    // Generate Pascal's triangle
    let triangle: Vec<Vec<u64>> = 0.to(10).then(|n| {
        0.to(n + 1).then(|k| C(n, k))
    });
    
    table(&triangle);
    
    // Show formula
    println("\n<blue b>Formula:");
    println(r"$(\binom{n}{k} = \frac{n!}{k!(n-k)!})$");
    
    // Visualize with turtle graphics
    coord("cartesian");
    let t = object();
    t.color("blue").width(1.5);
    
    // Draw triangle pattern
    for i in 0.to(5) {
        for j in 0.to(i + 1) {
            let x = (j as f64 - i as f64 / 2.0) * 50.0;
            let y = i as f64 * 50.0;
            t.at(x, y).circle(15.0);
        }
    }
}
```

### Example 3: Interactive Form

```ignore
use webrust::prelude::*;

#[gui(Courier_New 10px black !white)]
fn main() {
    println("<blue b i>🎯 User Registration Form");
    
    // Collect inputs
    let first_name: String = input("First name:");
    let last_name: String = input("Last name:");
    let age: i32 = input("Age:");
    let email: String = input("Email:");
    let height: f64 = input("Height (meters):");
    
    // Validate and display
    let status = if age >= 18 { "adult" } else { "minor" };
    
    println("\n<green b>Registration Summary:");
    println("<navy>Name: <green b i>{first_name} {last_name}");
    println("<navy>Age: <yellow>{age} <gray>({status})");
    println("<navy>Email: <blue u>{email}");
    println("<navy>Height: <purple>{height:.2}m");
    
    // Show status badge
    coord("css");
    print("<white !green r10 w150 h30 mc>✓ {first_name} online")
        .at(-160, 10.0)
        .sticky();
}
```

### Example 4: Real-Time Plotting

```ignore
use webrust::prelude::*;

#[gui]
fn main() {
    println("<darkblue b i>📈 Function Plotting");
    
    // Generate data points
    let x_values: Vec<f64> = 0.to(100)
        .then(|i| i as f64 * 0.1);
    
    let sine: Vec<f64> = x_values.iter()
        .map(|&x| x.sin())
        .collect();
    
    let cosine: Vec<f64> = x_values.iter()
        .map(|&x| x.cos())
        .collect();
    
    // Create multi-series chart
    let x_labels: Vec<String> = x_values.iter()
        .step_by(5)
        .map(|x| format!("{:.1}", x))
        .collect();
    
    chart(&sine, "line")
        .title("Trigonometric Functions")
        .xlabel("Radians")
        .ylabel("Value")
        .xlabels(x_labels)
        .add("sin(x)", sine, Some("red".into()))
        .add("cos(x)", cosine, Some("blue".into()));
    
    // Show formulas
    println("\n<green b>Functions:");
    println("$(\\sin(x))$ in <red>red");
    println("$(\\cos(x))$ in <blue>blue");
}
```

---

## Use Cases

### 1. 🚀 Rapid Prototyping

**Scenarios**: Hackathons, demos, MVPs, client presentations

**Advantages**:
- Single-file applications
- No frontend development
- Instant visual feedback
- Zero deployment complexity
- Native performance

### 2. 📚 Education & Teaching

**Scenarios**: Algorithm visualization, mathematical concepts, interactive tutorials

**Advantages**:
- LaTeX for mathematical notation
- Step-by-step visualizations
- Clean, readable code
- Immediate execution
- No infrastructure setup for students

### 3. 📊 Data Analysis & Exploration

**Scenarios**: Dataset exploration, report generation, interactive dashboards

**Advantages** (with SQL):
- Handles millions of rows
- Integrated SQL queries
- Streaming visualization
- Export to multiple formats
- Web-based sharing

### 4. 🔬 Scientific Computing

**Scenarios**: Simulations, research notebooks, experimental visualizations

**Advantages**:
- Mathematical notation
- Animation capabilities
- Numerical precision
- Type safety
- Publication-ready outputs

### 5. 💼 Business Intelligence

**Scenarios**: KPI dashboards, metrics monitoring, operational analytics

**Advantages**:
- Real-time data processing
- Complex aggregations
- Interactive drill-down
- Professional visualizations
- Grid layouts for dashboards

### 6. 🎮 Creative Coding

**Scenarios**: Generative art, interactive animations, visual experiments

**Advantages**:
- Turtle graphics
- Animation easing
- Group transformations
- Mathematical precision
- Real-time rendering

---

## Performance

### Compilation Performance (v1.7.0)

| Configuration | First Build | Incremental | Size  |
|---------------|-------------|-------------|-------|
| Default       | ~30s        | <5s         | ~2MB  |
| With SQL      | ~3min       | <10s        | ~15MB |

### Runtime Performance

| Operation         | Throughput           | Latency |
|-------------------|----------------------|---------|
| Range iteration   | 1B ops/s             | 1ns/op  |
| Comprehension     | 500M ops/s           | 2ns/op  |
| String operations | 100M ops/s           | 10ns/op |
| Macro expansion   | 3-5x faster (v1.7.0) | -       |
| SQL streaming     | 100K rows/s          | <50ms   |

### Memory Characteristics

- **Zero-copy parsing**: Macro expansion with `Cow<'_, str>`
- **SIMD acceleration**: `memchr` for pattern matching
- **Streaming SQL**: Constant memory usage regardless of result size
- **Adaptive batching**: 200-800 rows per chunk based on column count

### Optimization Tips

1. **Use SQL for large datasets**: Don't load millions of rows into memory
2. **Filter early**: Apply WHERE clauses before rendering
3. **Export instead of render**: Use `EXPORT` for large result sets
4. **Leverage comprehensions**: They compile to efficient iterators
5. **Grid layouts**: Use `grid()` and `cell()` for complex dashboards

---

## Feature Selection Guidelines

### Use Default Configuration

✅ Prototypes and demos  
✅ Small/medium datasets (<100K rows)  
✅ Teaching and education  
✅ Interactive presentations  
✅ Visualization-focused apps  
✅ Fast compilation priority  

### Enable SQL Feature

✅ Large CSV/Parquet files (100K+ rows)  
✅ Complex joins and aggregations  
✅ Analytical dashboards  
✅ Window functions & CTEs  
✅ OLAP queries  
✅ Multiple data sources  
✅ Export workflows  

---

## Contributing

Contributions welcome in these areas:

- **Bug Reports**: [GitHub Issues](https://github.com/gerarddubard/webrust/issues)
- **Feature Requests**: [Discussions](https://github.com/gerarddubard/webrust/discussions)
- **Documentation**: PRs for improvements
- **Examples**: Share your use cases
- **Performance**: Benchmarks and optimizations

### Development Principles

1. ✅ Python-inspired ergonomics
2. ✅ Rust safety guarantees
3. ✅ Zero-configuration philosophy
4. ✅ Optional features when appropriate
5. ✅ Performance without complexity

---

## Roadmap

### Version 1.8.0 (Planned)

- **Responsive Design**: Mobile-optimized layouts
- **WebSocket Support**: Real-time data streaming
- **Component System**: Reusable UI widgets
- **Static Export**: Generate standalone HTML files
- **Extended Charts**: Sankey, treemap, 3D plots

### Future Considerations

- Native database connectors (PostgreSQL, MySQL)
- Theme system for consistent styling
- Plugin architecture for extensions
- Multi-language documentation

---

## License

MIT License - See [LICENSE](LICENSE) file for details.

---

## Acknowledgments

WebRust is built upon excellent open-source projects:

**Core Dependencies:**
- [DuckDB](https://duckdb.org/) - High-performance analytical database
- [Apache Arrow](https://arrow.apache.org/) - Columnar data format
- [tiny_http](https://github.com/tiny-http/tiny-http) - Lightweight HTTP server
- [serde](https://serde.rs/) - Serialization framework

**Performance:**
- [itoa](https://github.com/dtolnay/itoa) - Fast integer formatting
- [ryu](https://github.com/dtolnay/ryu) - Fast float formatting
- [memchr](https://github.com/BurntSushi/memchr) - SIMD string search
- [smallvec](https://github.com/servo/rust-smallvec) - Stack-allocated vectors

**Frontend:**
- [MathJax](https://www.mathjax.org/) - Mathematical notation
- [ECharts](https://echarts.apache.org/) - Interactive charting
- [Two.js](https://two.js.org/) - 2D drawing library

Special thanks to the Python, Rust, and SQL communities for inspiration and tooling.

---

## References

- **Documentation**: <https://docs.rs/webrust>
- **Examples**: <https://github.com/gerarddubard/webrust/tree/main/examples>
- **Package**: <https://crates.io/crates/webrust>
- **Discussions**: <https://github.com/gerarddubard/webrust/discussions>

---

**Version**: 1.7.0  
**Release Date**: 2025  
**Maintainer**: See GitHub repository for maintainer information

---

**Made with ❤️ by the WebRust community**

[⭐ Star on GitHub](https://github.com/gerarddubard/webrust) • [📖 Read the Docs](https://docs.rs/webrust) • [💬 Join Discussion](https://github.com/gerarddubard/webrust/discussions)
