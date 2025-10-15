# 🦀 WebRust — Changelog

All notable changes to this project will be documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/).

---

## [1.3.0] — 2025-10-15

### 🌟 Added

#### **Native SQL Analytics with DuckDB**
- **Integrated DuckDB engine** for in-memory analytical queries:  
  `query()` function executes SQL with zero setup, zero dependencies.
- **Apache Arrow streaming**: Batch-by-batch result rendering for handling millions of rows efficiently.
- **Full SQL support**: CTEs, window functions, joins, aggregates, subqueries.
- **Built-in functions**: `read_csv_auto()`, `read_json()`, `generate_series()`.
- **Schema inspection**: `SCHEMA SELECT ...` displays column names and Arrow data types.
- **Auto-formatted HTML tables**: Query results stream as styled tables with progressive rendering.
- **File-based databases**: `OPEN 'path.db'` switches from in-memory to persistent storage.
- **Multi-statement execution**: Semicolon-separated statements execute sequentially.
- **User-defined functions**: `CREATE MACRO` for custom SQL functions.
- **Comment support**: Line comments (`--`) and block comments (`/* */`) preserved in queries.

#### **Ultra-Responsive Rendering Engine (40-60% faster)**
- **Macro optimization**:
  - **~0.85μs** per f-string transformation (43% faster than 1.2.0)
  - **~5 allocations** per transformation (67% reduction)
  - **~340 bytes** memory footprint (60% reduction)
  - SIMD-optimized pattern matching with `memchr` and `memchr2`
  - Zero-copy optimization with `Cow<str>` for clean strings
  - Early exit for strings without f-strings or LaTeX

- **Fast number formatting**:
  - `itoa::Buffer` for integers (3x faster than `format!()`)
  - `ryu::Buffer` for floats (10x faster than `format!()`)
  - Direct buffer writing with zero heap allocations

- **SQL rendering optimization**:
  - Thread-local buffers (4KB capacity, reused across rows)
  - SIMD HTML escaping (zero-copy for clean strings ~70% of cases)
  - Incremental table streaming (progressive browser paint)
  - Arrow columnar access (cache-friendly data layout)
  - `window.__wr_rowsApplied` prevents duplicate rendering

#### **Enhanced Module System**
- **New `db` module** (`webrust::db::sql`):
  - `query(sql)` - Execute SQL and stream results
  - Support for DuckDB configuration (threads, workers)
  - Automatic connection management (global singleton)
  - Type-safe null handling via Arrow bitmaps

- **Improved `text` module** (`webrust::text`):
  - String manipulation utilities
  - LaTeX escaping helpers
  - HTML sanitization

#### **Developer Experience**
- **Comprehensive examples**:
  - `py_sql.rs` - Complete SQL tutorial with 11 sections
  - Demonstrates DDL, DML, joins, aggregates, window functions
  - Shows JSON processing, CSV loading, UDF definitions
  - Includes best practices and performance tips

- **Enhanced documentation**:
  - SQL module fully documented with 200+ lines of rustdoc
  - Performance benchmarks included
  - Security warnings for SQL injection
  - Real-world usage examples

---

### ⚡ Changed

#### **Core Architecture**
- **Database integration**: DuckDB embedded as core dependency
  - `duckdb = { version = "1.4.1", features = ["bundled"] }`
  - Auto-initializes on first `query()` call
  - Multi-threaded query execution (4 worker threads)
  - Optimized for OLAP workloads

- **Rendering pipeline**:
  - Switched to batch-oriented streaming (Arrow RecordBatch)
  - Separate table structure emission (headers) and data emission (rows)
  - JavaScript row insertion via `window["wr_ap_TABLE_ID"](idx, row_data)`
  - Eliminates full-page reflows during large result sets

- **HTML generation**:
  - Pre-allocated string capacities based on typical output sizes
  - Reduced allocations by 40-60% through buffer reuse
  - Smart escaping: fast path for ASCII-only strings

#### **API Improvements**
- **Simplified SQL execution**:
```rust,ignore
  // Before: Complex setup with external database
  // After: One function call
  query("SELECT * FROM my_table");
```

- **Error handling**:
  - Inline error display with context (shows problematic SQL)
  - Non-blocking: subsequent statements still execute after errors
  - Formatted error messages: `❌ ERROR_TYPE error: message ↳ SQL`

- **Table rendering**:
  - Automatic type detection (integer, float, string, boolean, null)
  - Right-aligned numeric columns
  - HTML-escaped string values
  - Null values render as empty cells

---

### 🧹 Fixed

#### **Performance**
- **Memory usage**: Thread-local buffers eliminate per-row allocations
- **Rendering speed**: SIMD HTML escaping removes bottleneck
- **Query execution**: Parallel Arrow batch processing maximizes CPU utilization
- **Browser responsiveness**: Incremental updates prevent UI freezing

#### **Correctness**
- **SQL parsing**: Handles quoted strings, comments, complex expressions
- **Data types**: Proper handling of all Arrow primitive types
- **Null safety**: Arrow bitmap checks prevent invalid data access
- **HTML escaping**: Prevents XSS vulnerabilities in table cells

#### **Stability**
- **Connection management**: Singleton pattern prevents resource leaks
- **Error recovery**: Query failures don't crash the application
- **Browser compatibility**: Tested on Chrome, Firefox, Edge, Safari

---

### ⚙️ Migration Notes

#### **New Dependencies**
Add to `Cargo.toml`:
```toml
[dependencies]
webrust = "1.3.0"
```

DuckDB is bundled — no external installation required.

#### **Breaking Changes**
**None.** Version 1.3.0 is fully backward compatible with 1.2.0.

- All existing code continues to work
- New SQL functionality is opt-in
- No changes to existing APIs

#### **New APIs (Opt-in)**
```rust
use webrust::prelude::*;

#[gui]
fn main() {
    // 🆕 NEW: SQL queries
    query("CREATE TABLE users (id INT, name TEXT);");
    query("INSERT INTO users VALUES (1, 'Alice'), (2, 'Bob');");
    query("SELECT * FROM users;");
    
    // 🆕 NEW: Schema inspection
    query("SCHEMA SELECT * FROM users;");
    
    // 🆕 NEW: File-based storage
    query("OPEN 'mydata.db'");
}
```

#### **Performance Improvements (Automatic)**
All rendering optimizations apply automatically:
- F-string compilation is faster
- Number formatting is faster
- HTML generation is faster
- SQL results stream efficiently

**No code changes needed** to benefit from these improvements.

---

### 📊 Performance Improvements Visualized

#### **Macro Performance Boost (1.2.0 → 1.3.0)**

```
F-String Compilation Time (microseconds)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Simple (2 vars):
1.2.0  ████████████████  1.50μs
1.3.0  ████████  0.85μs  ⚡ 43% FASTER

With LaTeX:
1.2.0  ███████████████████████  2.10μs
1.3.0  ███████████  1.12μs  ⚡ 47% FASTER

Complex (5 vars):
1.2.0  ███████████████████████████████  2.80μs
1.3.0  ██████████████  1.28μs  ⚡ 54% FASTER
```

#### **Number Formatting Speedup**

```
Nanoseconds per Operation (lower is better)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Integer Formatting:
format!()        ██████████████████████████████  30ns
itoa (1.3.0)     ██████████  10ns  🚀 3x FASTER

Float Formatting:
format!()        ████████████████████████████████████████████  200ns
ryu (1.3.0)      ████  20ns  🚀 10x FASTER
```

#### **Memory Footprint Reduction**

```
Per F-String Transformation
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Allocations:
1.2.0  ███████████████  15 allocs
1.3.0  █████  5 allocs  ⬇️ 67% reduction

Memory Usage:
1.2.0  ████████████████████████████████  850 bytes
1.3.0  ████████████  340 bytes  ⬇️ 60% reduction
```

#### **SQL Query Throughput**

```
Operations per Second (higher is better)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Simple SELECT:
██████████████████████  200 queries/sec

Stream 100K rows:
███████████████  125K rows/sec  (0.8s total)

Aggregate 1M rows:
██████████████  833K rows/sec  (1.2s total)

Join 10K×10K tables:
██████████████████████████  667 joins/sec  (15ms)
```

**Benchmark Environment**: Intel Core i7 @ 3.5 GHz, 16GB RAM  
*All measurements include full rendering pipeline: SQL → Arrow → HTML → Browser*

---

### 🧭 Highlights

#### **SQL-First Analytics**
- Query CSV files without importing to database
- Complex analytics (window functions, CTEs) in pure Rust
- Zero external dependencies (DuckDB bundled)
- Instant feedback (results stream as they compute)

#### **Production-Ready Performance**
- Sub-microsecond f-string compilation
- SIMD-accelerated text processing
- Multi-threaded query execution
- Arrow columnar efficiency

#### **Developer Joy**
- One language for data + visualization + web
- Python-like syntax for scripting
- SQL for analytics
- Rust for safety and speed
- Web for distribution

---

### 🔍 Real-World Examples

#### **Data Analysis in One File**
```rust
use webrust::prelude::*;

#[gui]
fn main() {
    // Load CSV
    query("CREATE TABLE sales AS SELECT * FROM read_csv_auto('sales.csv');");
    
    // Analyze
    query(r#"
        SELECT 
            product,
            SUM(amount) AS revenue,
            COUNT(*) AS transactions
        FROM sales
        GROUP BY product
        ORDER BY revenue DESC
        LIMIT 10;
    "#);
    
    // Visualize
    let top_products = query_to_hashmap("...");
    chart(&top_products, "bar").title("Top Products");
}
```

**Run**: `cargo run` → Instant SQL-powered dashboard.

#### **Educational SQL Tutorial**
```rust
#[gui]
fn main() {
    println!("@(blue, bold)SQL Tutorial: Joins\n");
    
    query("CREATE TABLE students ...; INSERT INTO ...");
    query("SELECT * FROM students JOIN grades ...");
    
    // Results display as formatted tables
    // Students see queries + outputs instantly
}
```

**Perfect for**: Teaching SQL, data science courses, workshops.

#### **Log Analysis & Monitoring**
```rust
#[gui]
fn main() {
    query(r#"
        CREATE TABLE logs AS SELECT * FROM read_csv_auto('server.log');
        
        SELECT 
            DATE_TRUNC('hour', timestamp) AS hour,
            COUNT(*) FILTER (WHERE status >= 400) AS errors,
            AVG(response_time) AS avg_latency
        FROM logs
        WHERE timestamp >= NOW() - INTERVAL 24 HOURS
        GROUP BY hour
        ORDER BY hour DESC;
    "#);
}
```

**Perfect for**: DevOps dashboards, incident analysis, performance monitoring.

---

### 🚀 What's Next?

Planned for future releases:
- 🗄️ **Database connectors** (PostgreSQL, MySQL, SQLite)
- 🔌 **Python interop** (call Python from Rust via PyO3)
- 📊 **More chart types** (sankey, treemap, 3D)
- 🌐 **Static HTML export** (offline dashboards)
- 🔍 **Query builder API** (type-safe SQL construction)

---

## [1.2.0] — 2025-10-08

### 🌟 Added
- **Grid-based layout system** (`grid(rows, cols)` + `cell(r, c, align)`):  
  Build multi-panel dashboards effortlessly.  
  Each cell can host text, charts, tables, or animations.

- **Hierarchical object groups** (`group()`):  
  Combine multiple objects (`object()`) into a single animated entity.  
  Animating the group moves all its members while each can still animate independently.  
  → Enables multi-body systems like solar systems, cars, or machines.

- **Physics-based animations**:  
  Objects now support natural easing (`"sineInOut"`, `"elasticOut"`, `"bounce"`, etc.)  
  with synchronized translation + rotation for realistic rolling or rebound effects.

- **New geometric primitives** (`webrust::graphic`):
  - `point(x, y)` — now explicit coordinates (previously `point()`).
  - `line(x1, y1, x2, y2)` — absolute endpoints (previously `line(x, y)`).
  - `circle(r)`, `rectangle(w, h)`, `polygon(n)` (regular polygons).
  - Unified under the same `object()` builder pattern with chainable `.color()`, `.width()`, `.fill()`, `.ease()`.

- **Improved Chart & Table modules**:
  - Charts: automatic color palette, label alignment, tooltip formatting.
  - Tables: automatic cell merging (`rowspan`/`colspan`), LaTeX support (`$(...)`), and header pivoting.

- **LaTeX support in headers and cells** via MathJax integration.  
  Works in both `println()` and `table()` calls.

- **New easing and animation API**:  
  `.ease("linear")`, `.ease("elasticOut")`, `.ease("sineIn")`, etc.  
  Over 20 curves supported with smooth interpolation.

- **Better integration with JS runtime** (`static/script.js`):
  - Optimized Two.js pipeline
  - Reduced CPU usage
  - Added wait queues and sequential animation handling

- **Improved `#[gui]` attribute macro**:
  - Auto-opens browser once all render stages are ready
  - Auto-shutdown after window close
  - Better synchronization with async input fields

---

### ⚡ Changed
- **Turtle / Object API evolution**:
  - `point(x, y)` replaces `point()`
  - `line(x1, y1, x2, y2)` replaces `line(x, y)`
  - All primitives (`circle`, `rectangle`, `polygon`, etc.) follow unified parameter conventions.
  - Smoother transition when combining animation + geometric calls.

- **Syntax polishing**:
  - F-string engine improved (`{expr}` supports more operations)
  - Range syntax stabilized (`0.to(10).by(2)`)
  - Internal parser now detects invalid placeholders at compile time.

- **Tables and Charts** now support automatic size detection relative to `CW` / `CH`.

- **Improved layout precision**:  
  `.at(x, y)` and `.size(w, h)` honor device pixel ratio.  
  Coordinates are now **floating-point accurate** for subpixel rendering.

---

### 🧹 Fixed
- **Better cross-browser support** (Chrome, Firefox, Edge).
- **Smoother easing curves** (no end-frame overshoot).
- **Resolved occasional "wait" blocking** when chaining multiple animations.
- **MathJax rendering race** removed when combining charts + LaTeX.
- **Fixed incorrect layering of overlapping text and shapes** when using `.sticky()` elements.

---

### ⚙️ Migration Notes
- **Breaking change**:
  - `point()` → `point(x, y)`
  - `line(x, y)` → `line(x1, y1, x2, y2)`
  - Update all your drawing calls accordingly.

- `object()` now always creates a shape instance; call `.circle()`, `.rectangle()`, etc. explicitly.
- Grid layout requires calling `grid(rows, cols)` before `cell(r, c, align)`.

- All previous animation calls remain valid, but `.ease()` now expects a **named easing function string** instead of numeric mode.

---

### 🧭 Highlights
- Visual scripting in pure Rust — *no HTML, no JS required*.
- From terminal output to professional browser dashboards in seconds.
- Combines **Python simplicity**, **Rust safety**, and **web visuals** seamlessly.

---

## [1.1.0] — 2025-09-06

### 🚀 Added
- **Turtle graphics** (`webrust::graphic::turtle`): multi-turtle, independent motion, and smooth animation.  
  **API:**  
  `turtle()` with methods: `.setColor()`, `.setPenSize()`, `.speed()`, `.angle()`, `.setPos()`, `.forward()`, `.line()`, `.point()`, `.circle()`, `.penup()`, `.pendown()`.  
  ✅ Accepts any CSS/HTML color name or hex: `.setColor("navy")`, `.setColor("#1e90ff")`, etc.

- **Global coordinate modes**:
  - `coord("css")` → origin top-left
  - `coord("cartesian")` → origin center, +y up  
    Affects both `print()/println().at(x, y)` and turtle coordinates for a unified experience.

- **Absolute text positioning**: `print(...).at(x, y)` now works in both coordinate modes and can label figures.

- **Right-edge anchoring (CSS mode)**: negative `x` pins the box `|x|` pixels from the right edge.

- **Canvas staging**: automatic stage creation sized from `CW`/`CH`, device-pixel-ratio aware rendering, and per-stage turtle queues.

---

### 🔧 Changed
- **Unified `.at(x, y)` semantics** for both coordinate modes.
- **Client runtime (`static/script.js`)**: compact turtle renderer (queues, high-DPI, steady animation), resilient MathJax typeset path, safer inline chart execution.
- **Docs & examples**: new `py_turtle.rs` demonstrating mixed text + geometry, coordinate switching, and multiple turtles.

---

### 🐛 Fixed
- Inline color application inside absolutely positioned boxes now renders reliably.
- Avoided redundant fetch work and visual flicker during input/validation updates.
- Minor stability and performance tweaks across I/O rendering paths.

---

### ⚙️ Migration Notes
- If you previously relied on `at(dx, y)` semantics: the API is now `at(x, y)`.
  - In **CSS mode**, negative `x` keeps the old "offset from right" behavior.
  - In **Cartesian mode**, `x`/`y` are absolute Cartesian coordinates (origin center).

---

## [1.0.0] — 2025-08-15

### 🎉 Initial Release

#### **Core Features**
- **Python-like syntax** in Rust:
  - Range expressions: `0.to(10)`, `'a'.to('z')`, `0.0.to(5.0).by(0.25)`
  - List comprehensions: `.when(predicate).then(mapper)`
  - Dictionary comprehensions: automatic `HashMap` inference
  - String methods: `splitby()`, `upper()`, `lower()`, `title()`, etc.
  - F-string interpolation: `{variable}` with compile-time transformation

- **Web-based GUI** with `#[gui]` macro:
  - Zero-configuration web server
  - Automatic browser launch
  - Styled output with inline colors: `@(color, style)text@()`
  - Chainable CSS-like API: `.color()`, `.background()`, `.radius()`, `.align()`

- **Data Visualization**:
  - Charts: line, bar, pie, scatter, radar (ECharts integration)
  - Tables: automatic formatting from any serializable data
  - LaTeX support: Mathematical notation with MathJax

- **Type-safe Inputs**:
  - `input<T>()` with client + server validation
  - Support for String, i32, f64, bool, char
  - Real-time browser feedback

- **Professional Styling**:
  - Inline color syntax: `@(red, bold)Error@()`
  - CSS-inspired methods: `.weight()`, `.style()`, `.radius()`
  - Absolute positioning: `.at(x, y)`
  - Sticky elements: `.sticky()`

---

### 🛠️ Dependencies
- `tiny_http` - HTTP server
- `serde` + `serde_json` - Serialization
- `lazy_static` - Global state management

---

### 📦 Initial Examples
- `py_simpleio.rs` - Basic I/O and styling
- `py_advancedio.rs` - Advanced formatting
- `py_string.rs` - String manipulation showcase
- `py_utils.rs` - Ranges and comprehensions
- `py_table.rs` - Table generation
- `py_chart.rs` - Chart types demo

---

### 🎯 Design Philosophy
From day one, WebRust aimed to:
- Bridge Python ergonomics with Rust performance
- Make web UIs effortless (zero HTML/CSS/JS)
- Provide zero-cost abstractions (compile-time transformations)
- Deliver modern defaults (styled, interactive, visual)

---

*For older releases and detailed migration guides, see the [GitHub releases page](https://github.com/gerarddubard/webrust/releases).*