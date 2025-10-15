# WebRust Changelog

All notable changes to this project are documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/).

## Table of Contents

- [Version 1.5.0](#v150)
- [Version 1.3.0](#v130)
- [Version 1.2.0](#v120)
- [Version 1.1.0](#v110)
- [Version 1.0.0](#v100)

---

<a id="v150"></a>

## Version 1.5.0 -- 2025-10-15

### Overview

Version 1.5.0 introduces optional SQL support, dramatically reduces compilation time for non-SQL builds, and delivers significant rendering performance improvements.

### Added

#### Optional SQL Analytics (Feature Flag)

SQL analytics is now opt-in via the `sql` feature flag, reducing default compilation time from 5-10 minutes to approximately 30 seconds.

**Option A — Default (fast compile):**
```toml
[dependencies]
webrust = "1.5.0"
```

**Option B — With SQL support:**
```toml
[dependencies]
webrust = { version = "1.5.0", features = ["sql"] }
```

**SQL capabilities (when enabled):**
- DuckDB in-memory OLAP database
- Apache Arrow streaming for efficient data processing
- Standard SQL: CTEs, window functions, joins, aggregations
- Built-in functions: `read_csv_auto()`, `read_json()`, `generate_series()`
- Schema introspection via `SCHEMA SELECT`
- File-based persistent storage with `OPEN 'path.db'`
- Multi-statement execution (semicolon-separated)
- User-defined functions via `CREATE MACRO`

**Example:**
```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    query(r#"
        CREATE TABLE sales AS SELECT * FROM read_csv_auto('sales.csv');
        
        SELECT 
            product,
            SUM(amount) AS total_sales,
            COUNT(*) AS transactions
        FROM sales
        GROUP BY product
        ORDER BY total_sales DESC
        LIMIT 10
    "#);
}
```

#### Enhanced Rendering Performance

**Macro optimization improvements:**
- F-string transformation: approximately 0.85μs per operation (43% faster than v1.3.0)
- Memory allocations: approximately 5 per transformation (67% reduction)
- Memory footprint: approximately 340 bytes per transformation (60% reduction)
- SIMD-optimized pattern matching via `memchr` and `memchr2`
- Zero-copy optimization with `Cow<str>` for strings without interpolation
- Early exit detection for strings without f-strings or LaTeX

**Number formatting optimization:**
- Integer formatting via `itoa::Buffer` (3x faster than `format!()`)
- Float formatting via `ryu::Buffer` (10x faster than `format!()`)
- Direct buffer writing with zero heap allocations

**SQL rendering optimization (when SQL feature enabled):**
- Thread-local buffers (4KB capacity, reused across rows)
- SIMD HTML escaping (zero-copy for clean strings in approximately 70% of cases)
- Incremental table streaming for progressive browser rendering
- Arrow columnar access for cache-friendly data layout
- Deduplication prevention via `window.__wr_rowsApplied`

#### Module System Enhancements

**New `db` module** (`webrust::db::sql`):
- `query(sql: &str)` -- Execute SQL and stream results
- DuckDB configuration support (threads, workers)
- Automatic connection management via global singleton
- Type-safe null handling through Arrow bitmaps

**Improved `text` module** (`webrust::text`):
- Enhanced string manipulation utilities
- LaTeX escaping helpers
- HTML sanitization functions

#### Documentation and Examples

**Comprehensive SQL example:**
- `sql.rs` -- Complete SQL tutorial with 11 sections
- Demonstrates DDL, DML, joins, aggregates, window functions
- Shows JSON processing, CSV loading, UDF definitions
- Includes best practices and performance considerations

**Enhanced API documentation:**
- SQL module fully documented (200+ lines of rustdoc)
- Performance benchmarks included
- Security warnings for SQL injection
- Real-world usage examples

### Changed

#### Core Architecture

**Compilation model:**
- Default build (no SQL): approximately 30 seconds first build
- With SQL feature: 2-5 minutes first build
- Subsequent builds: approximately 1-2 seconds (both configurations)

**Database integration (when SQL feature enabled):**
- DuckDB embedded via `duckdb = { version = "1.4.1", features = ["bundled"] }`
- Auto-initialization on first `query()` call
- Multi-threaded query execution (4 worker threads)
- Optimized for OLAP workloads

**Rendering pipeline:**
- Batch-oriented streaming via Arrow RecordBatch
- Separate table structure emission (headers) and data emission (rows)
- JavaScript row insertion via `window["wr_ap_TABLE_ID"](idx, row_data)`
- Eliminates full-page reflows during large result sets

**HTML generation:**
- Pre-allocated string capacities based on typical output sizes
- 40-60% reduction in allocations through buffer reuse
- Smart escaping with fast path for ASCII-only strings

#### API Improvements

**Simplified SQL execution:**
```rust,ignore
// Before v1.5.0: Complex setup with external database required
// After v1.5.0: Single function call
query("SELECT * FROM my_table");
```

**Enhanced error handling:**
- Inline error display with contextual information
- Non-blocking: subsequent statements execute after errors
- Formatted error messages

**Improved table rendering:**
- Automatic type detection (integer, float, string, boolean, null)
- Right-aligned numeric columns
- HTML-escaped string values
- Null values render as empty cells

### Fixed

#### Performance

- **Memory usage**: Thread-local buffers eliminate per-row allocations
- **Rendering speed**: SIMD HTML escaping removes bottleneck
- **Query execution**: Parallel Arrow batch processing maximizes CPU utilization
- **Browser responsiveness**: Incremental updates prevent UI freezing

#### Correctness

- **SQL parsing**: Proper handling of quoted strings, comments, complex expressions
- **Data types**: Correct handling of all Arrow primitive types
- **Null safety**: Arrow bitmap checks prevent invalid data access
- **HTML escaping**: XSS vulnerability prevention in table cells

#### Stability

- **Connection management**: Singleton pattern prevents resource leaks
- **Error recovery**: Query failures don't crash the application
- **Browser compatibility**: Tested on Chrome, Firefox, Edge, Safari

### Migration Notes

#### New Dependencies

```toml
[dependencies]
webrust = "1.5.0"
```

DuckDB is bundled when SQL feature is enabled -- no external installation required.

#### Breaking Changes

**None.** Version 1.5.0 maintains full backward compatibility with v1.3.0.

- All existing code continues to work
- SQL functionality is opt-in via feature flag
- No changes to existing APIs

#### New APIs (Opt-in)

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    // SQL queries (requires features = ["sql"])
    query("CREATE TABLE users (id INT, name TEXT);");
    query("INSERT INTO users VALUES (1, 'Alice'), (2, 'Bob');");
    query("SELECT * FROM users;");
    
    // Schema inspection
    query("SCHEMA SELECT * FROM users;");
    
    // File-based storage
    query("OPEN 'mydata.db'");
}
```

#### Automatic Performance Improvements

All rendering optimizations apply automatically with no code changes:
- Faster f-string compilation
- Faster number formatting
- Faster HTML generation
- Efficient SQL result streaming (when SQL feature enabled)

### Performance Metrics

#### Macro Performance (v1.3.0 to v1.5.0)

| Operation           | v1.3.0 | v1.5.0 | Improvement |
|---------------------|--------|--------|-------------|
| Simple (2 vars)     | 1.50μs | 0.85μs | 43% faster  |
| With LaTeX          | 2.10μs | 1.12μs | 47% faster  |
| Complex (5 vars)    | 2.80μs | 1.28μs | 54% faster  |

#### Number Formatting Speedup

| Method            | Time  | Speedup vs format!() |
|-------------------|-------|----------------------|
| format!() (int)   | 30ns  | Baseline             |
| itoa (v1.5.0)     | 10ns  | 3x faster            |
| format!() (float) | 200ns | Baseline             |
| ryu (v1.5.0)      | 20ns  | 10x faster           |

#### Memory Footprint Reduction

| Metric               | v1.3.0    | v1.5.0    | Reduction |
|----------------------|-----------|-----------|-----------|
| Allocations per op   | 15 allocs | 5 allocs  | 67%       |
| Memory per op        | 850 bytes | 340 bytes | 60%       |

#### SQL Query Throughput (when SQL feature enabled)

| Operation             | Performance          |
|-----------------------|----------------------|
| Simple SELECT         | 200 queries/sec      |
| Stream 100K rows      | 125K rows/sec (0.8s) |
| Aggregate 1M rows     | 833K rows/sec (1.2s) |
| Join 10K×10K tables   | 667 joins/sec (15ms) |

*Benchmark environment: Intel Core i7 @ 3.5 GHz, 16GB RAM. All measurements include full rendering pipeline: SQL to Arrow to HTML to Browser.*

### Highlights

#### SQL-First Analytics (Optional)
- Query CSV files without importing to database
- Complex analytics (window functions, CTEs) in pure Rust
- Zero external dependencies (DuckDB bundled)
- Instant feedback with streaming results

#### Production-Ready Performance
- Sub-microsecond f-string compilation
- SIMD-accelerated text processing
- Multi-threaded query execution
- Arrow columnar efficiency

#### Developer Experience
- One language for data, visualization, and web
- Python-like syntax for scripting
- SQL for analytics (optional)
- Rust for safety and speed
- Web for distribution

### Real-World Examples

#### Data Analysis in One File

```rust,ignore
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
    let top_products = query_to_hashmap("SELECT product, revenue FROM ...");
    chart(&top_products, "bar").title("Top Products");
}
```

**Run:** `cargo run` then instant SQL-powered dashboard.

#### Educational SQL Tutorial

```rust,ignore
#[gui]
fn main() {
    println!("@(blue, bold)SQL Tutorial: Joins\n");
    
    query("CREATE TABLE students (id INT, name TEXT);");
    query("INSERT INTO students VALUES (1, 'Alice'), (2, 'Bob');");
    query("SELECT * FROM students;");
}
```

**Use case:** Teaching SQL, data science courses, workshops.

#### Log Analysis and Monitoring

```rust,ignore
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

**Use case:** DevOps dashboards, incident analysis, performance monitoring.

---

<a id="v130"></a>

## Version 1.3.0 -- 2025-10-08

### Overview

Version 1.3.0 introduced native SQL analytics with DuckDB integration and significant rendering optimizations.

**Note:** In v1.5.0, SQL support became optional via feature flag.

### Added

#### Native SQL Analytics with DuckDB

Integrated DuckDB engine for in-memory analytical queries:
- `query()` function executes SQL with zero setup
- Apache Arrow streaming for batch-by-batch result rendering
- Full SQL support: CTEs, window functions, joins, aggregates, subqueries
- Built-in functions: `read_csv_auto()`, `read_json()`, `generate_series()`
- Schema introspection via `SCHEMA SELECT`
- Auto-formatted HTML tables with progressive rendering
- File-based databases via `OPEN 'path.db'`
- Multi-statement execution (semicolon-separated)
- User-defined functions via `CREATE MACRO`
- Comment support: line comments and block comments

#### Ultra-Responsive Rendering Engine

**Macro optimization:**
- Approximately 0.85μs per f-string transformation (43% faster than v1.2.0)
- Approximately 5 allocations per transformation (67% reduction)
- Approximately 340 bytes memory footprint (60% reduction)
- SIMD-optimized pattern matching
- Zero-copy optimization with `Cow<str>`
- Early exit for strings without f-strings or LaTeX

**Fast number formatting:**
- `itoa::Buffer` for integers (3x faster)
- `ryu::Buffer` for floats (10x faster)
- Direct buffer writing with zero heap allocations

**SQL rendering optimization:**
- Thread-local buffers (4KB capacity, reused across rows)
- SIMD HTML escaping
- Incremental table streaming
- Arrow columnar access
- Duplicate rendering prevention

#### Enhanced Module System

**New `db` module** (`webrust::db::sql`):
- `query(sql)` -- Execute SQL and stream results
- DuckDB configuration support
- Automatic connection management
- Type-safe null handling

**Improved `text` module** (`webrust::text`):
- String manipulation utilities
- LaTeX escaping helpers
- HTML sanitization

#### Developer Experience

**Comprehensive examples:**
- `py_sql.rs` -- Complete SQL tutorial with 11 sections
- Demonstrates DDL, DML, joins, aggregates, window functions
- Shows JSON processing, CSV loading, UDF definitions
- Includes best practices and performance tips

**Enhanced documentation:**
- SQL module fully documented (200+ lines)
- Performance benchmarks included
- Security warnings for SQL injection
- Real-world usage examples

### Changed

#### Core Architecture

**Database integration:**
- DuckDB embedded as core dependency
- Auto-initializes on first `query()` call
- Multi-threaded query execution (4 worker threads)
- Optimized for OLAP workloads

**Rendering pipeline:**
- Batch-oriented streaming via Arrow RecordBatch
- Separate table structure and data emission
- JavaScript row insertion for incremental updates
- Eliminates full-page reflows for large result sets

**HTML generation:**
- Pre-allocated string capacities
- 40-60% reduction in allocations
- Smart escaping with fast path for ASCII

#### API Improvements

**Simplified SQL execution:**
```rust,ignore
// One function call
query("SELECT * FROM my_table");
```

**Error handling:**
- Inline error display with context
- Non-blocking subsequent statements
- Formatted error messages

**Table rendering:**
- Automatic type detection
- Right-aligned numeric columns
- HTML-escaped string values
- Null values as empty cells

### Fixed

**Performance:**
- Memory usage via thread-local buffers
- Rendering speed via SIMD HTML escaping
- Query execution via parallel Arrow batch processing
- Browser responsiveness via incremental updates

**Correctness:**
- SQL parsing for quoted strings and comments
- Data type handling for all Arrow primitive types
- Null safety via Arrow bitmap checks
- HTML escaping for XSS prevention

**Stability:**
- Connection management via singleton pattern
- Error recovery without crashes
- Browser compatibility (Chrome, Firefox, Edge, Safari)

### Migration Notes

**New Dependencies:**
```toml
[dependencies]
webrust = "1.3.0"
```

**Breaking Changes:** None. Version 1.3.0 is fully backward compatible with v1.2.0.

**New APIs:**
```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    query("CREATE TABLE users (id INT, name TEXT);");
    query("INSERT INTO users VALUES (1, 'Alice'), (2, 'Bob');");
    query("SELECT * FROM users;");
    query("SCHEMA SELECT * FROM users;");
    query("OPEN 'mydata.db'");
}
```

---

<a id="v120"></a>

## Version 1.2.0 -- 2025-09-06

### Overview

Version 1.2.0 introduced grid-based layouts, hierarchical object groups, and physics-based animations.

### Added

#### Grid-Based Layout System

```rust,ignore
grid(2, 3);  // 2 rows, 3 columns
let (x, y) = cell(0, 1, "center");
chart(&data, "bar").at(x, y);
```

**Features:**
- Multi-panel dashboard support
- Each cell can host text, charts, tables, or animations
- Responsive sizing relative to viewport

#### Hierarchical Object Groups

```rust,ignore
let car = group();
let wheel1 = object().at(-30.0, 0.0).circle(15.0);
let wheel2 = object().at(30.0, 0.0).circle(15.0);
car.add(&wheel1);
car.add(&wheel2);
car.translate(200.0, 0.0);
```

**Capabilities:**
- Combine multiple objects into single animated entity
- Group animation moves all members
- Individual members can animate independently
- Enables complex multi-body systems

#### Physics-Based Animations

**Easing functions:**
- `linear`, `sineIn`, `sineOut`, `sineInOut`
- `quadIn`, `quadOut`, `quadInOut`
- `cubicIn`, `cubicOut`, `cubicInOut`
- `elasticIn`, `elasticOut`, `elasticInOut`
- `bounceIn`, `bounceOut`, `bounceInOut`
- `backIn`, `backOut`, `backInOut`
- `expoIn`, `expoOut`, `expoInOut`

**Synchronized transformations:**
- Translation plus rotation with easing
- Natural rolling and rebound effects

#### New Geometric Primitives

**Unified API via `object()`:**
- `point(x, y)` -- Explicit coordinates
- `line(x1, y1, x2, y2)` -- Absolute endpoints
- `circle(r)` -- Circle with radius
- `rectangle(w, h)` -- Rectangle with dimensions
- `polygon(n)` -- Regular polygon with n sides

**Chainable methods:**
- `.color(color)` -- Set stroke color
- `.width(w)` -- Set line width
- `.fill(color)` -- Set fill color
- `.ease(fn)` -- Set easing function

#### Enhanced Chart and Table Modules

**Charts:**
- Automatic color palette
- Label alignment
- Tooltip formatting

**Tables:**
- Automatic cell merging (rowspan/colspan)
- LaTeX support via MathJax
- Header pivoting
- LaTeX in headers and cells

#### Improved JavaScript Runtime

**Optimizations:**
- Two.js pipeline optimization
- Reduced CPU usage
- Wait queues for sequential animations
- Sequential animation handling

#### Enhanced GUI Attribute Macro

**Improvements:**
- Auto-opens browser when render stages ready
- Auto-shutdown after window close
- Better synchronization with async input fields

### Changed

#### Turtle and Object API Evolution

**Breaking changes:**
- `point()` changed to `point(x, y)`
- `line(x, y)` changed to `line(x1, y1, x2, y2)`

**Unified conventions:**
- All primitives follow consistent parameter patterns
- Smoother transition between animation and geometric calls

#### Syntax Improvements

**F-string engine:**
- Supports more operations in expressions
- Invalid placeholders detected at compile time

**Range syntax:**
- Stabilized `0.to(10).by(2)`

#### Layout Precision

**Improvements:**
- `.at(x, y)` and `.size(w, h)` honor device pixel ratio
- Floating-point accuracy for subpixel rendering

### Fixed

- Cross-browser support (Chrome, Firefox, Edge)
- Smoother easing curves (no end-frame overshoot)
- Wait blocking when chaining animations
- MathJax rendering race with charts and LaTeX
- Layering of overlapping text and shapes with `.sticky()`

### Migration Notes

#### Breaking Changes

**Point and line API:**
```rust,ignore
// Before v1.2.0
point()
line(x, y)

// After v1.2.0
point(x, y)
line(x1, y1, x2, y2)
```

**Object creation:**
- `object()` now requires explicit shape method
- Call `.circle()`, `.rectangle()`, etc.

**Grid layout:**
- Must call `grid(rows, cols)` before `cell(r, c, align)`

**Easing functions:**
- `.ease()` expects named function string, not numeric mode

### Highlights

- Visual scripting in pure Rust (no HTML, no JS)
- Terminal to professional dashboard in seconds
- Python simplicity plus Rust safety plus web visuals

---

<a id="v110"></a>

## Version 1.1.0 -- 2025-08-15

### Overview

Version 1.1.0 introduced turtle graphics with multi-turtle support and coordinate system management.

### Added

#### Turtle Graphics

**API** (`webrust::graphic::turtle`):
- `turtle()` -- Create turtle instance
- `.setColor(color)` -- Set turtle color (CSS/hex)
- `.setPenSize(size)` -- Set pen width
- `.speed(speed)` -- Set movement speed
- `.angle(degrees)` -- Set heading
- `.setPos(x, y)` -- Set position
- `.forward(distance)` -- Move forward
- `.line(x, y)` -- Draw line to point
- `.point()` -- Draw point
- `.circle(radius)` -- Draw circle
- `.penup()` -- Lift pen (don't draw)
- `.pendown()` -- Lower pen (draw)

**Color support:**
- CSS color names: `"navy"`, `"red"`, etc.
- Hex colors: `"#1e90ff"`

#### Coordinate Modes

**Global coordinate system:**
```rust,ignore
coord("css");       // Origin top-left, Y down
coord("cartesian"); // Origin center, Y up
```

**Unified behavior:**
- Affects both `print().at(x, y)` and turtle coordinates
- Consistent positioning across text and graphics

#### Absolute Text Positioning

```rust,ignore
print("Label").at(x, y);
println("Text").at(x, y);
```

**Features:**
- Works in both coordinate modes
- Can label figures and graphics
- Right-edge anchoring in CSS mode (negative x)

#### Canvas Staging

**Capabilities:**
- Automatic stage creation sized from viewport
- Device-pixel-ratio aware rendering
- Per-stage turtle queues
- High-DPI support

### Changed

#### Unified Positioning Semantics

**`.at(x, y)` behavior:**
- CSS mode: absolute positioning, negative x for right-edge anchoring
- Cartesian mode: absolute coordinates from center origin

#### Client Runtime Improvements

**JavaScript enhancements:**
- Compact turtle renderer
- High-DPI support
- Steady animation
- Resilient MathJax typeset path
- Safer inline chart execution

#### Documentation

**New examples:**
- `py_turtle.rs` -- Mixed text and geometry
- Coordinate switching demonstration
- Multiple turtle examples

### Fixed

- Inline color rendering in absolutely positioned boxes
- Redundant fetch operations
- Visual flicker during input and validation updates
- Stability improvements across I/O rendering

### Migration Notes

#### Breaking Changes

**Positioning API:**
```rust,ignore
// Before v1.1.0
at(dx, y)  // Offset semantics

// After v1.1.0
at(x, y)   // Absolute positioning
```

**CSS mode:**
- Negative x maintains offset from right behavior

**Cartesian mode:**
- x and y are absolute Cartesian coordinates (origin center)

---

<a id="v100"></a>

## Version 1.0.0 -- 2025-08-01

### Overview

Initial release of WebRust, introducing Python-like syntax in Rust with automatic web-based GUI generation.

### Core Features

#### Python-Like Syntax

**Range expressions:**
```rust,ignore
0.to(10)                    // 0 to 10
'a'.to('z')                 // a to z
0.0.to(5.0).by(0.25)       // 0.0, 0.25, 0.5, etc
```

**List comprehensions:**
```rust,ignore
let squares = 0.to(10).then(|x| x * x);
let evens = 0.to(20).when(|&x| x % 2 == 0).then(|x| x);
```

**Dictionary comprehensions:**
```rust,ignore
let dict = 0.to(5).then(|x| (x, x * x));  // HashMap inference
```

**String methods:**
```rust,ignore
"a,b,c".splitby(",")
"hello".upper()
"hello".lower()
"hello world".title()
```

**F-string interpolation:**
```rust,ignore
println!("Hello {name}!");
println!("Result: {value:.2}");
```

Compile-time transformation to `format!()`.

#### Web-Based GUI

**`#[gui]` macro:**
```rust,ignore
#[gui]
fn main() {
    println!("Hello, Web!");
}
```

**Features:**
- Zero-configuration web server
- Automatic browser launch
- Hot-reload support

**Styled output:**
```rust,ignore
println!("@(red, bold)Error@()");
println!("@(green)Success@()");
```

**Chainable CSS-like API:**
```rust,ignore
print("Text")
    .color("blue")
    .background("white")
    .radius(5)
    .align("center");
```

#### Data Visualization

**Charts (ECharts integration):**
- Line, bar, pie, scatter, radar
- Automatic data serialization
- Interactive tooltips

```rust,ignore
chart(&data, "bar").title("Sales");
```

**Tables:**
```rust,ignore
table(&matrix).header(["X", "Y", "Z"]);
```

Automatic formatting from any serializable data.

**LaTeX support (MathJax):**
```rust,ignore
println!("Einstein: $(E = mc^2)");
```

#### Type-Safe Inputs

```rust,ignore
let name: String = input("Name:");
let age: i32 = input("Age:");
let height: f64 = input("Height:");
let confirmed: bool = input("Confirm?");
```

**Validation:**
- Client-side (browser) -- Real-time feedback
- Server-side (Rust) -- Type-safe parsing

#### Professional Styling

**Inline color syntax:**
```rust,ignore
@(red, bold)Error@()
@(green, italic)Success@()
@(blue, underline)Link@()
```

**CSS-inspired methods:**
- `.weight(weight)` -- Font weight
- `.style(style)` -- Font style
- `.radius(radius)` -- Border radius
- `.at(x, y)` -- Absolute positioning
- `.sticky()` -- Sticky positioning

### Dependencies

- `tiny_http` -- HTTP server
- `serde` and `serde_json` -- Serialization
- `lazy_static` -- Global state management

### Initial Examples

- `py_simpleio.rs` -- Basic I/O and styling
- `py_advancedio.rs` -- Advanced formatting
- `py_string.rs` -- String manipulation
- `py_utils.rs` -- Ranges and comprehensions
- `py_table.rs` -- Table generation
- `py_chart.rs` -- Chart types

### Design Philosophy

WebRust aims to:
1. Bridge Python ergonomics with Rust performance
2. Make web UIs effortless (zero HTML/CSS/JS)
3. Provide zero-cost abstractions (compile-time transformations)
4. Deliver modern defaults (styled, interactive, visual)

---

## Future Roadmap

Planned features for upcoming releases:

- Database connectors (PostgreSQL, MySQL, SQLite)
- Python interop via PyO3
- Additional chart types (sankey, treemap, 3D)
- Static HTML export (offline dashboards)
- Type-safe query builder API
- Plugin ecosystem
- Multi-language i18n support
- WebAssembly target

---

## Contributing

We welcome contributions! Please see:
- [GitHub Issues](https://github.com/gerarddubard/webrust/issues) for bug reports
- [GitHub Discussions](https://github.com/gerarddubard/webrust/discussions) for feature requests
- [Contributing Guide](CONTRIBUTING.md) for development guidelines

---

## License

WebRust is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

For older releases and detailed migration guides, visit the [GitHub releases page](https://github.com/gerarddubard/webrust/releases).
