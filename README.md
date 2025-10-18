# WebRust: A Unified Framework for Data Visualization and Interactive Computing

[![WebRust](https://img.shields.io/badge/WebRust-1.6.0-ff6b35?style=flat-square)](https://github.com/gerarddubard/webrust)
[![Rust](https://img.shields.io/badge/Rust-1.70+-000?style=flat-square&logo=rust)](https://rust-lang.org)
[![Documentation](https://img.shields.io/badge/docs-latest-blue?style=flat-square)](https://docs.rs/webrust)
[![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)

**Links:** [Documentation](https://docs.rs/webrust) | [Examples](https://github.com/gerarddubard/webrust/tree/main/examples) | [Crates.io](https://crates.io/crates/webrust)

---

## Abstract

WebRust is a Rust framework designed to bridge the ergonomics of Python with the performance characteristics of Rust, while providing integrated web-based visualization capabilities. The framework addresses the fragmentation in contemporary data analysis workflows by offering a unified interface for data manipulation, visualization, and interactive application development. Version 1.6.0 introduces major SQL performance optimizations with zero-copy HTML escaping, intelligent batching strategies, and enhanced type formatting precision.

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
- **High-Performance SQL**: Optimized DuckDB integration with streaming results (opt-in)

### Version 1.6.0 Highlights

- **SQL Performance Breakthrough**: Zero-copy HTML escaping eliminates unnecessary allocations
- **Intelligent Batching**: Adaptive chunk sizing (200-800 rows) based on column count
- **Configurable Precision**: `ROUND_FLOATS` constant for flexible decimal formatting
- **Robust Streaming**: JavaScript tracking system prevents duplicate row rendering
- **Extended DuckDB Config**: Full extension support (httpfs, parquet, json) in file-backed mode
- **Type-Optimized Formatting**: Direct Arrow-to-string conversion with `itoa` and `ryu`

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
- **Data Format**: Apache Arrow (columnar, zero-copy)
- **Compilation**: 2-5 minutes (first build)
- **Performance**: Intelligent batching, optimized type conversion, streaming results
- **Capabilities**: Standard SQL, window functions, CTEs, JSON operations

### SQL Performance Architecture (v1.6.0)

```text
Arrow Batch → Type Detection → Zero-Copy Formatting → Adaptive Chunking → HTML Streaming
     ↓              ↓                   ↓                    ↓                ↓
Columnar      Primitive         itoa/ryu/Decimal      200-800 rows     JavaScript
 Data       Fast Path Opt      (no allocations)      (by col count)    Tracking
```

**Key optimizations:**

- **HTML Escape**: Direct allocation without thread-local buffer (eliminates clone)
- **Number Formatting**: Fast-path for integers (itoa) and floats (ryu)
- **Decimal Precision**: Configurable rounding via `ROUND_FLOATS` constant
- **Batch Sizing**: Dynamic adjustment (800 rows for ≤8 cols, 200 rows for ≥20 cols)
- **Deduplication**: JavaScript `__wr_rowsApplied` tracking prevents rendering errors

---

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Basic Installation

For standard features (recommended):

```toml, no run
[dependencies]
webrust = "1.6.0"
```

**Characteristics:**

- Compilation time: approximately 30 seconds
- Size: minimal
- Features: Python-like syntax, web GUI, charts, tables, LaTeX rendering, turtle graphics

### With SQL Analytics

For data-intensive applications:

```toml, no run
[dependencies]
webrust = { version = "1.6.0", features = ["sql"] }
```

**Additional characteristics:**

- First compilation: 2-5 minutes (due to DuckDB)
- Subsequent builds: cached and faster
- Additional features: DuckDB integration, SQL queries, Arrow streaming, analytical functions

---

## Core Features

### 1. Iterator Extensions

Python-style range construction and iteration:

```rust, no run
use webrust::prelude::*;

// Range iteration
for i in 0.to(10) {
    println("{i}");
}

// Step specification
for i in 0.to(100).by(5) {
    println("{i}");
}

// Character ranges
for c in 'a'.to('z') {
    println("{c}");
}

// Floating-point and negative steps
for x in 4.0.to(0.0).by(-0.5) {
    println("{x}");
}
```

### 2. Comprehension Patterns

```rust, no run
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

```rust, no run
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

```rust, no run
use webrust::prelude::*;

#[gui]
fn main() {
    let name = "Alice";
    let age = 30;
    let pi = std::f64::consts::PI;
    
    // Variable interpolation
    println("Hello {name}, you are {age} years old");
    
    // Expressions
    println("Next year: {age + 1}");
    
    // Format specifiers
    println("PI approx {pi:.2}");
    
    // JSON serialization
    println("Data: {my_struct:j}");
    
    // LaTeX rendering
    println("$(E = mc^2)");
}
```

**Implementation**: Compile-time macro expansion with no runtime overhead.

### 5. Visualization Components

#### Charts

```rust, no run
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

```rust, no run
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

```rust, no run
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

### 6. High-Performance SQL Integration (Optional)

When `features = ["sql"]` is enabled:

```rust, no run
use webrust::prelude::*;

#[gui]
fn main() {
    // Data loading with automatic streaming
    query("CREATE TABLE sales AS SELECT * FROM read_csv_auto('sales.csv')");
    
    // Analytical queries with intelligent batching
    query(r#"
        SELECT 
            product,
            SUM(amount) AS total_sales,
            COUNT(*) AS transactions,
            AVG(amount) AS avg_transaction
        FROM sales
        GROUP BY product
        ORDER BY total_sales DESC
        LIMIT 10
    "#);
    
    // Window functions with configurable precision
    query(r#"
        SELECT 
            product,
            quarter,
            revenue,
            SUM(revenue) OVER (PARTITION BY product ORDER BY quarter) AS cumulative,
            ROUND(100.0 * revenue / SUM(revenue) OVER (PARTITION BY product), 2) AS pct
        FROM sales
        WHERE year = 2024
    "#);
    
    // Schema introspection
    query("SCHEMA SELECT * FROM sales");
}
```

**Capabilities:**

- Standard SQL with DuckDB extensions
- Built-in CSV/JSON/Parquet readers
- Window functions and CTEs
- Schema introspection via `SCHEMA` command
- Arrow-based streaming for large datasets (millions of rows)
- Zero-copy data processing

#### Special SQL Commands (v1.6.0)

```rust, no run
use webrust::prelude::*;

#[gui]
fn main() {
    // Import data (auto-detects format)
    query("IMPORT 'data.csv' AS dataset");
    query("IMPORT 'metrics.parquet' AS metrics");
    query("IMPORT 'https://example.com/data.json' AS remote");
    
    // Export results
    query("EXPORT dataset TO 'output.csv'");
    query("EXPORT dataset TO 'output.parquet' FORMAT PARQUET");
    query("EXPORT (SELECT * FROM dataset WHERE x > 10) TO 'filtered.json' FORMAT JSON");
    
    // Switch to file-backed database
    query("OPEN 'persistent.duckdb'");
    
    // Load additional extensions
    query("LOAD spatial");  // GIS operations
    query("LOAD fts");      // Full-text search
    
    // Runtime configuration
    query("CONFIG SET memory_limit = '4GB'");
    query("CONFIG SET threads = 8");
}
```

---

## Performance Characteristics

### Compilation Time

| Configuration    | First Build       | Subsequent Builds  |
|------------------|-------------------|--------------------|
| Default (no SQL) | approx 30 seconds | approx 1-2 seconds |
| With SQL feature | 2-5 minutes       | approx 1-2 seconds |

### Runtime Performance

**SQL Streaming optimizations (v1.6.0):**

- **HTML Escape**: Zero-copy with direct allocation (~40% faster, eliminates clone overhead)
- **Number Formatting**:
    - Integers via `itoa`: ~10x faster than `format!`
    - Floats via `ryu`: ~2x faster with better accuracy
    - Decimals: Exact precision without floating-point errors
- **Batch Sizing**: Adaptive chunking based on column count
    - ≤8 columns: 800 rows/batch (optimized for wide tables)
    - 9-19 columns: 400 rows/batch (balanced performance)
    - ≥20 columns: 200 rows/batch (prevents JSON serialization overhead)
- **Deduplication**: JavaScript tracking prevents rendering errors in async contexts

**Rendering optimizations (core framework):**

- F-string transformation: approximately 0.85 microseconds per operation (43% improvement)
- Memory allocations: approximately 5 per transformation (67% reduction)
- Memory footprint: approximately 340 bytes per transformation (60% reduction)

**Techniques employed:**

- SIMD pattern matching via `memchr`
- Zero-copy optimization with `Cow<str>`
- Optimized number formatting (`itoa`, `ryu`)
- Direct buffer writing
- Thread-local buffer reuse (for CELL_BUF, SCRIPT_BUF, HTML_BUF)

**Result**: Maintains 60fps animation performance with instant feedback, handles millions of rows efficiently.

### Memory Efficiency

All Python-like syntax constructs compile to standard Rust iterators, resulting in:

- Zero runtime overhead
- Optimal memory usage
- Full compiler optimization applicability

SQL streaming uses Arrow's columnar format:

- Cache-friendly memory layout
- SIMD-optimized operations
- Minimal serialization overhead
- Efficient null handling

---

## Usage Examples

### Basic Interactive Application

```rust, no run
use webrust::prelude::*;

#[gui(bg="navy", fg="white", font="Courier New")]
fn main() {
    println("@(cyan, bold, italic)Data Dashboard");
    
    let name: String = input("What's your name?");
    println("Hello, {name}!");
    
    let data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    chart(&data, "line").title("Trend Analysis");
    
    let squares: Vec<i32> = 0.to(10).then(|x| x * x);
    table(&squares).header(["Index", "Square"]);
}
```

**Execution**: `cargo run` then browser opens automatically and UI renders

### Data Visualization

```rust, no run
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

```rust, no run
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
    
    println(r"$(y = v_0 \sin\theta \cdot t - \frac{1}{2}gt^2)");
}
```

### High-Performance Data Analytics with SQL

Requires `features = ["sql"]`:

```rust, no run
use webrust::prelude::*;

#[gui(bg="darkslategray", fg="lightcyan")]
fn main() {
    println("@(cyan, bold)📊 Real-Time Analytics Dashboard");
    
    // Load data with streaming
    query("IMPORT 'https://example.com/iris.csv' AS iris");
    
    println("@(yellow)→ Dataset Overview");
    query("SELECT COUNT(*) as rows, COUNT(DISTINCT species) as species FROM iris");
    
    println("@(yellow)→ Statistical Analysis");
    query(r#"
        SELECT
            species,
            COUNT(*) as count,
            ROUND(AVG(sepal_length), 2) as avg_sepal_length,
            ROUND(STDDEV(sepal_length), 2) as std_sepal_length,
            ROUND(MIN(sepal_length), 2) as min_sepal_length,
            ROUND(MAX(sepal_length), 2) as max_sepal_length
        FROM iris
        GROUP BY species
        ORDER BY species
    "#);
    
    println("@(yellow)→ Distribution Analysis with Window Functions");
    query(r#"
        SELECT
            species,
            sepal_length,
            ROUND(percentile_cont(0.5) WITHIN GROUP (ORDER BY sepal_length) 
                  OVER (PARTITION BY species), 2) as median,
            RANK() OVER (PARTITION BY species ORDER BY sepal_length DESC) as rank_in_species
        FROM iris
        ORDER BY species, rank_in_species
        LIMIT 15
    "#);
    
    // Export for further analysis
    query("EXPORT iris TO 'iris_processed.parquet'");
    
    println("@(green)✨ Analysis Complete");
}
```

### Complex Multi-Dataset Analysis

```rust, no run
use webrust::prelude::*;

#[gui]
fn main() {
    println("@(blue, bold)🚢 Multi-Dataset Analytics");
    
    // Load multiple datasets
    query("IMPORT 'https://example.com/titanic.csv' AS titanic");
    query("IMPORT 'https://example.com/iris.csv' AS iris");
    
    println("@(magenta)→ Cross-dataset comparison");
    query(r#"
        SELECT
            'Iris petal length' as metric,
            ROUND(AVG(petal_length), 2) as mean,
            ROUND(STDDEV(petal_length), 2) as std_dev,
            ROUND(MIN(petal_length), 2) as min_val,
            ROUND(MAX(petal_length), 2) as max_val
        FROM iris
        UNION ALL
        SELECT
            'Titanic age' as metric,
            ROUND(AVG(Age), 2) as mean,
            ROUND(STDDEV(Age), 2) as std_dev,
            ROUND(MIN(Age), 2) as min_val,
            ROUND(MAX(Age), 2) as max_val
        FROM titanic
        WHERE Age IS NOT NULL
    "#);
    
    println("@(yellow)→ Complex window analysis");
    query(r#"
        WITH survivors AS (
            SELECT
                Pclass,
                Sex,
                COUNT(*) as count
            FROM titanic
            WHERE Survived = 1
            GROUP BY Pclass, Sex
        )
        SELECT
            Pclass,
            Sex,
            count,
            SUM(count) OVER (
                PARTITION BY Pclass
                ORDER BY Sex
                ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
            ) as cumulative_count,
            ROUND(100.0 * count / SUM(count) OVER (PARTITION BY Pclass), 1) as pct_of_class
        FROM survivors
        ORDER BY Pclass, Sex
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

### 3. High-Performance Data Exploration

**Target scenarios**: Large dataset analysis, report generation, interactive dashboards

**Advantages** (with SQL feature):

- Handles millions of rows efficiently
- Integrated visualization with streaming
- SQL support for complex queries
- Quick iteration cycles
- Zero-copy Arrow processing
- Web-based sharing

### 4. Scientific Computing

**Target scenarios**: Simulations, research notebooks, experimental visualizations

**Advantages**:

- Mathematical notation rendering
- Animation capabilities
- Numerical computation with Rust performance
- Publication-ready outputs

### 5. Business Intelligence

**Target scenarios**: Metrics dashboards, log analysis, operational monitoring, KPI tracking

**Advantages** (with SQL feature):

- Complex aggregations with window functions
- Real-time data processing
- Interactive drill-down capabilities
- Professional visualizations
- Configurable precision for financial data
- Export to multiple formats (CSV, Parquet, JSON)

### 6. Real-Time Analytics

**Target scenarios**: Live data monitoring, streaming analytics, operational dashboards

**Advantages** (v1.6.0 SQL optimizations):

- Adaptive batching for responsive UIs
- Zero-copy processing for low latency
- Intelligent chunk sizing based on data shape
- Deduplication for reliable async updates

---

## Feature Selection Guidelines

### Use Default Configuration When

- Building prototypes or demos
- Working with small to medium datasets (less than 100K rows)
- Teaching programming concepts
- Creating interactive presentations
- Fast compilation is priority
- Visualization-focused applications

### Enable SQL Feature When

- Processing large CSV/JSON/Parquet files (100K+ rows)
- Requiring complex joins and aggregations
- Building analytical dashboards
- Using window functions or Common Table Expressions
- OLAP-style queries are needed
- Real-time data analysis with streaming results
- Need configurable numeric precision (ROUND_FLOATS)
- Working with multiple data sources

---

## Performance Best Practices (v1.6.0)

### SQL Query Optimization

1. **Use LIMIT for exploration**: Preview data before rendering full results
   ```rust, no run
   query("SELECT * FROM large_table LIMIT 100");
   ```

2. **Filter early**: Apply WHERE clauses before JOINs
   ```rust, no run
   query("SELECT * FROM orders o 
          JOIN users u ON o.user_id = u.id 
          WHERE o.created_at > '2024-01-01'");
   ```

3. **Export large results**: Don't render massive datasets in browser
   ```rust, no run
   query("EXPORT (SELECT * FROM big_query) TO 'output.parquet'");
   ```

4. **Use CTEs**: Break complex queries into readable parts
   ```rust, no run
   query(r#"
       WITH filtered AS (...),
            aggregated AS (...)
       SELECT * FROM aggregated
   "#);
   ```

5. **Leverage adaptive batching**: Let WebRust optimize chunk sizes
    - Tables with ≤8 columns render fastest
    - Very wide tables (20+ columns) automatically use smaller batches

### Precision Configuration

Modify `ROUND_FLOATS` constant in `sql.rs` for your use case:

- Financial data: `Some(2)` (2 decimal places)
- Scientific data: `Some(4)` or `Some(6)`
- Maximum precision: `None` (no rounding)

---

## Roadmap

### Version 1.7.0 (Planned)

- **Visualization**: Additional chart types (sankey, treemap, 3D plots)
- **SQL**: Connection pooling for concurrent queries
- **Performance**: SIMD-optimized string operations
- **Export**: Static HTML generation for deployment

### Future Considerations

- **Data Sources**: Native database connectors (PostgreSQL, MySQL)
- **Components**: Reusable widget system
- **Responsive Design**: Mobile-optimized interfaces
- **Real-time**: WebSocket support for live updates

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
- **Performance**: Benchmarks and optimization suggestions

### Development Principles

1. Maintain Python-inspired ergonomics
2. Preserve Rust safety and performance guarantees
3. Keep zero-configuration philosophy
4. Ensure features remain optional when appropriate
5. Optimize for common use cases without sacrificing flexibility

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
- **itoa**: Fast integer formatting
- **ryu**: Fast float formatting
- **memchr**: SIMD string search
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

**Version**: 1.6.0  
**Last Updated**: 2025  
**Maintainer**: See GitHub repository for current maintainer information
