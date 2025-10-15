# WebRust: A Unified Framework for Data Visualization and Interactive Computing

[![WebRust](https://img.shields.io/badge/WebRust-1.5.0-ff6b35?style=flat-square)](https://github.com/gerarddubard/webrust)
[![Rust](https://img.shields.io/badge/Rust-1.70+-000?style=flat-square&logo=rust)](https://rust-lang.org)
[![Documentation](https://img.shields.io/badge/docs-latest-blue?style=flat-square)](https://docs.rs/webrust)
[![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)

**Links:** [Documentation](https://docs.rs/webrust) | [Examples](https://github.com/gerarddubard/webrust/tree/main/examples) | [Crates.io](https://crates.io/crates/webrust)

---

## Abstract

WebRust is a Rust framework designed to bridge the ergonomics of Python with the performance characteristics of Rust, while providing integrated web-based visualization capabilities. The framework addresses the fragmentation in contemporary data analysis workflows by offering a unified interface for data manipulation, visualization, and interactive application development. Version 1.5.0 introduces an optional SQL analytics layer and optimized compilation times.

## Table of Contents

1. [Introduction](#introduction)
2. [Motivation](#motivation)
3. [Architecture](#architecture)
4. [Installation](#installation)
5. [Core Features](#core-features)
6. [Performance Characteristics](#performance-characteristics)
7. [Usage Examples](#usage-examples)
8. [Use Cases](#use-cases)
9. [Roadmap](#roadmap)
10. [Contributing](#contributing)
11. [License](#license)

---

## Introduction

### Overview

WebRust is a framework that combines Python-inspired syntax patterns with Rust's type safety and performance characteristics. The primary objective is to reduce the complexity of creating interactive, web-based data visualizations and applications while maintaining compile-time guarantees and native execution speeds.

### Key Characteristics

- **Ergonomic Syntax**: Python-like iterator patterns and comprehensions
- **Type Safety**: Full Rust type system integration
- **Web Integration**: Automatic browser-based UI generation
- **Zero Configuration**: No external dependencies for core functionality
- **Optional SQL**: DuckDB integration for analytical workloads (opt-in)

### Version 1.5.0 Highlights

- Modular SQL support (opt-in via feature flag)
- Reduced default compilation time: approximately 30 seconds (from 5-10 minutes)
- Performance optimizations: 40-60% improvement in rendering
- Enhanced memory efficiency: 60% reduction in allocation overhead

---

## Motivation

### Problem Statement

Contemporary data analysis and visualization workflows typically require multiple tools and languages:

1. **Data Retrieval**: SQL databases (PostgreSQL, MySQL)
2. **Data Processing**: Python with pandas/numpy
3. **Visualization**: matplotlib, plotly, or similar libraries
4. **Web Deployment**: Flask, Django, or JavaScript frameworks

This fragmentation results in:

- Multiple context switches between languages and tools
- Complex dependency management
- Data format conversion overhead
- Extended development cycles
- Infrastructure complexity for deployment

### Existing Approaches and Limitations

#### Terminal-Based Applications

Traditional command-line interfaces lack support for:

- Rich text formatting and colors
- Embedded visualizations
- Mathematical notation
- Interactive elements

#### Traditional Data Analysis Pipelines

Multi-tool workflows involving:

- SQL for queries
- Python for processing
- Separate visualization libraries
- Web frameworks for deployment

Result in high complexity and slow iteration cycles.

#### Web Framework Solutions

Frameworks like Rocket or Actix-web require:

- Multiple language expertise (HTML/CSS/JavaScript)
- Separate frontend/backend logic
- Complex state management
- Deployment infrastructure

#### Desktop GUI Frameworks

Native GUI frameworks (egui, iced) present challenges:

- Framework-specific API learning curve
- Platform-specific considerations
- Distribution complexity
- Update deployment overhead

### Design Philosophy

WebRust proposes a unified approach based on three principles:

1. **Syntax Evolution**: Adopting ergonomic patterns without sacrificing performance
2. **Ecosystem Integration**: Learning from Python, Rust, and SQL communities
3. **Modern Defaults**: Prioritizing visual, interactive, and zero-configuration solutions

---

## Architecture

### System Design

WebRust consists of three primary layers:

1. **Syntax Layer**: Macro-based transformations for Python-like constructs
2. **Runtime Layer**: HTTP server and browser communication
3. **Visualization Layer**: Integration with ECharts, MathJax, and Two.js

### Compilation Model

```text
Source Code → Macro Expansion → Type Checking → Native Compilation
     ↓              ↓                ↓                ↓
Python-like    Standard Rust    Type Safety    Native Performance
  Syntax        Iterators       Guaranteed      (no runtime cost)
```

### Optional SQL Integration

When enabled via `features = ["sql"]`:

- **Engine**: DuckDB (in-memory OLAP)
- **Data Format**: Apache Arrow (columnar)
- **Compilation**: 2-5 minutes (first build)
- **Capabilities**: Standard SQL, window functions, CTEs

---

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Basic Installation

For standard features (recommended):

```toml
[dependencies]
webrust = "1.5.0"
```

**Characteristics:**

- Compilation time: approximately 30 seconds
- Size: minimal
- Features: Python-like syntax, web GUI, charts, tables, LaTeX rendering, turtle graphics

### With SQL Analytics

For data-intensive applications:

```toml
[dependencies]
webrust = { version = "1.5.0", features = ["sql"] }
```

**Additional characteristics:**

- First compilation: 2-5 minutes (due to DuckDB)
- Subsequent builds: cached and faster
- Additional features: DuckDB integration, SQL queries, Arrow streaming

---

## Core Features

### 1. Iterator Extensions

Python-style range construction and iteration:

```rust,ignore
use webrust::prelude::*;

// Range iteration
for i in 0.to(10) {
    println!("{i}");
}

// Step specification
for i in 0.to(100).by(5) {
    println!("{i}");
}

// Character ranges
for c in 'a'.to('z') {
    println!("{c}");
}

// Floating-point and negative steps
for x in 4.0.to(0.0).by(-0.5) {
    println!("{x}");
}
```

### 2. Comprehension Patterns

```rust,ignore
use webrust::prelude::*;
use std::collections::HashMap;

// Map transformation
let squares: Vec<i32> = 0.to(10).then(|x| x * x);

// Filter and transform
let evens: Vec<i32> = 0.to(20)
    .when(|&x| x % 2 == 0)
    .then(|x| x);

// Dictionary construction
let dict: HashMap<i32, i32> = 0.to(5)
    .then(|x| (x, x * x));
```

**Implementation note**: All operations compile to standard Rust iterators with zero runtime overhead.

### 3. String Operations

```rust,ignore
use webrust::prelude::*;

// Splitting operations
let parts = "a,b,c".splitby(",");
let words = "hello  world".splitby("");  // Whitespace split
let lines = "L1\nL2\nL3".splitby("\n");

// Joining
let joined = parts.join(", ");

// Case transformations
let upper = "hello".upper();
let title = "hello world".title();
```

### 4. Formatted Output

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    let name = "Alice";
    let age = 30;
    let pi = std::f64::consts::PI;
    
    // Variable interpolation
    println!("Hello {name}, you are {age} years old");
    
    // Expressions
    println!("Next year: {age + 1}");
    
    // Format specifiers
    println!("PI approx {pi:.2}");
    
    // JSON serialization
    println!("Data: {my_struct:j}");
    
    // LaTeX rendering
    println!("$(E = mc^2)");
}
```

**Implementation**: Compile-time macro expansion with no runtime overhead.

### 5. Visualization Components

#### Charts

```rust,ignore
use webrust::prelude::*;
use std::collections::HashMap;

#[gui]
fn main() {
    // Bar chart
    let sales = HashMap::from([
        ("Q1", 120.0), ("Q2", 200.0),
        ("Q3", 150.0), ("Q4", 300.0)
    ]);
    chart(&sales, "bar")
        .title("Quarterly Sales")
        .color("#2ecc71");
    
    // Line chart
    let temps = vec![64.4, 67.1, 69.8, 72.5, 70.2];
    chart(&temps, "line")
        .title("Temperature Trend")
        .xlabels(vec!["Mon", "Tue", "Wed", "Thu", "Fri"]);
}
```

**Supported chart types**: line, bar, pie, doughnut, radar, area, scatter, gauge, funnel

#### Tables

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
    table(&matrix).header(["X", "Y", "Z"]);
    
    // LaTeX support in tables
    let physics = vec![
        ("Einstein", r"$(E = mc^2)"),
        ("Schrodinger", r"$(i\hbar\frac{\partial}{\partial t}\Psi = \hat{H}\Psi)"),
    ];
    table(&physics).header(["Scientist", "Equation"]);
}
```

#### Graphics and Animation

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    coord("cartesian");
    
    let turtle = object();
    turtle.color("blue").width(2.0);
    
    // Geometric drawing
    for _ in 0.to(4) {
        turtle.forward(100.0);
        turtle.right(90.0);
    }
    
    // Animation with easing
    turtle.rotate(360.0).ease("elasticOut");
    turtle.scale(1.5, 1.5).ease("sineInOut");
}
```

**Animation support**: 20+ easing functions (linear, sine, quad, elastic, bounce, back, expo)

### 6. SQL Integration (Optional)

When `features = ["sql"]` is enabled:

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    // Data loading
    query("CREATE TABLE sales AS SELECT * FROM read_csv_auto('sales.csv')");
    
    // Analytical queries
    query(r#"
        SELECT 
            product,
            SUM(amount) AS total_sales,
            COUNT(*) AS transactions
        FROM sales
        GROUP BY product
        ORDER BY total_sales DESC
        LIMIT 10
    "#);
    
    // Window functions
    query(r#"
        SELECT 
            product,
            quarter,
            revenue,
            SUM(revenue) OVER (PARTITION BY product) AS total,
            RANK() OVER (ORDER BY revenue DESC) AS rank
        FROM sales
        WHERE year = 2024
    "#);
}
```

**Capabilities:**

- Standard SQL support
- Built-in CSV/JSON readers
- Window functions and CTEs
- Schema introspection
- Arrow-based streaming for large datasets

---

## Performance Characteristics

### Compilation Time

| Configuration    | First Build       | Subsequent Builds  |
|------------------|-------------------|--------------------|
| Default (no SQL) | approx 30 seconds | approx 1-2 seconds |
| With SQL feature | 2-5 minutes       | approx 1-2 seconds |

### Runtime Performance

**Rendering optimizations (v1.5.0):**

- F-string transformation: approximately 0.85 microseconds per operation (43% improvement)
- Memory allocations: approximately 5 per transformation (67% reduction)
- Memory footprint: approximately 340 bytes per transformation (60% reduction)

**Techniques employed:**

- SIMD pattern matching via `memchr`
- Zero-copy optimization with `Cow<str>`
- Optimized number formatting (`itoa` for integers, `ryu` for floats)
- Direct buffer writing

**Result**: Maintains 60fps animation performance with instant feedback.

### Memory Efficiency

All Python-like syntax constructs compile to standard Rust iterators, resulting in:

- Zero runtime overhead
- Optimal memory usage
- Full compiler optimization applicability

---

## Usage Examples

### Basic Interactive Application

```rust,ignore
use webrust::prelude::*;

#[gui(bg="navy", fg="white", font="Courier New")]
fn main() {
    println!("@(cyan, bold, italic)Data Dashboard");
    
    let name: String = input("What's your name?");
    println!("Hello, {name}!");
    
    let data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    chart(&data, "line").title("Trend Analysis");
    
    let squares: Vec<i32> = 0.to(10).then(|x| x * x);
    table(&squares).header(["Index", "Square"]);
}
```

**Execution**: `cargo run` then browser opens automatically and UI renders

### Data Visualization

```rust,ignore
use webrust::prelude::*;
use std::collections::HashMap;

#[gui]
fn main() {
    let sales = HashMap::from([
        ("Q1", 120.0), ("Q2", 200.0),
        ("Q3", 150.0), ("Q4", 300.0)
    ]);
    
    chart(&sales, "bar").title("Quarterly Revenue");
    
    let growth = vec![100.0, 150.0, 180.0, 250.0];
    chart(&growth, "line").title("Growth Trend");
}
```

### Scientific Computing

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    coord("cartesian");
    
    // Projectile motion simulation
    let v0 = 50.0;
    let angle = 45.0_f64.to_radians();
    let g = 9.81;
    
    let trajectory: Vec<(f64, f64)> = (0..100)
        .then(|i| {
            let t = i as f64 * 0.1;
            let x = v0 * angle.cos() * t;
            let y = v0 * angle.sin() * t - 0.5 * g * t * t;
            (x, y.max(0.0))
        });
    
    let path = object();
    path.color("red").width(2.0);
    for (x, y) in trajectory {
        path.line(x - 1.0, y, x, y);
    }
    
    println!(r"$(y = v_0 \sin\theta \cdot t - \frac{1}{2}gt^2)");
}
```

### Data Analytics with SQL

Requires `features = ["sql"]`:

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    query(r#"
        CREATE TABLE logs AS 
        SELECT * FROM read_csv_auto('access_logs.csv');
        
        SELECT 
            DATE_TRUNC('hour', timestamp) AS hour,
            COUNT(*) AS requests,
            SUM(CASE WHEN status >= 400 THEN 1 ELSE 0 END) AS errors,
            AVG(response_time) AS avg_latency
        FROM logs
        WHERE timestamp >= NOW() - INTERVAL 24 HOURS
        GROUP BY hour
        ORDER BY hour DESC
    "#);
}
```

---

## Use Cases

### 1. Rapid Prototyping

**Target scenarios**: Hackathons, proof-of-concepts, client demonstrations

**Advantages**:

- Minimal boilerplate
- Instant visual feedback
- Single file applications
- Zero deployment complexity

### 2. Educational Tools

**Target scenarios**: Algorithm visualization, mathematical demonstrations, teaching materials

**Advantages**:

- LaTeX support for mathematical notation
- Interactive visualizations
- Clean, readable code for students
- Immediate execution feedback

### 3. Data Exploration

**Target scenarios**: Dataset analysis, report generation, dashboard creation

**Advantages**:

- Integrated visualization
- SQL support for complex queries (optional)
- Quick iteration cycles
- Web-based sharing

### 4. Scientific Computing

**Target scenarios**: Simulations, research notebooks, experimental visualizations

**Advantages**:

- Mathematical notation rendering
- Animation capabilities
- Numerical computation with Rust performance
- Publication-ready outputs

### 5. Business Intelligence

**Target scenarios**: Metrics dashboards, log analysis, operational monitoring

**Advantages** (with SQL feature):

- Complex aggregations
- Real-time data processing
- Interactive drill-down
- Professional visualizations

---

## Feature Selection Guidelines

### Use Default Configuration When

- Building prototypes or demos
- Working with small to medium datasets (less than 100K rows)
- Teaching programming concepts
- Creating interactive presentations
- Fast compilation is priority

### Enable SQL Feature When

- Processing large CSV/JSON files (more than 100K rows)
- Requiring complex joins and aggregations
- Building analytical dashboards
- Using window functions or Common Table Expressions
- OLAP-style queries are needed

---

## Roadmap

### Planned Features

- **Visualization**: Additional chart types (sankey, treemap, 3D plots)
- **Data Sources**: Native database connectors (PostgreSQL, MySQL)
- **Components**: Reusable widget system
- **Export**: Static HTML generation for deployment
- **Responsive Design**: Mobile-optimized interfaces

### Community Priorities

Feature prioritization is guided by:

- Ergonomic principles (readability, intuitiveness)
- Performance characteristics (safety, speed)
- Simplicity (zero-configuration approach)
- Modularity (optional features)

---

## Contributing

Contributions are welcome in the following areas:

- **Bug Reports**: [GitHub Issues](https://github.com/gerarddubard/webrust/issues)
- **Feature Requests**: [GitHub Discussions](https://github.com/gerarddubard/webrust/discussions)
- **Documentation**: Pull requests for documentation improvements
- **Examples**: Sharing use cases and applications

### Development Principles

1. Maintain Python-inspired ergonomics
2. Preserve Rust safety and performance guarantees
3. Keep zero-configuration philosophy
4. Ensure features remain optional when appropriate

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) file for details.

---

## Acknowledgments

WebRust builds upon several open-source projects:

- **DuckDB**: High-performance analytical database
- **Apache Arrow**: Columnar data format
- **tiny_http**: Lightweight HTTP server
- **serde**: Serialization framework
- **MathJax**: Mathematical notation rendering
- **ECharts**: Interactive charting library
- **Two.js**: 2D drawing library

Special thanks to the Python, Rust, and SQL communities for their contributions to programming language design and tooling.

---

## References

- Documentation: <https://docs.rs/webrust>
- Examples: <https://github.com/gerarddubard/webrust/tree/main/examples>
- Package: <https://crates.io/crates/webrust>
- Discussions: <https://github.com/gerarddubard/webrust/discussions>

---

**Version**: 1.5.0  
**Last Updated**: 2025  
**Maintainer**: See GitHub repository for current maintainer information