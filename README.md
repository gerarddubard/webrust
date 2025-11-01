# WebRust

**Python-like Rust for Web Applications** – A bridge between Python's simplicity and Rust's power.

[![WebRust](https://img.shields.io/badge/WebRust-2.0.0-ff6b35?style=flat-square)](https://github.com/gerarddubard/webrust)
[![Rust](https://img.shields.io/badge/Rust-1.70+-000?style=flat-square&logo=rust)](https://rust-lang.org)
[![Documentation](https://img.shields.io/badge/docs-latest-blue?style=flat-square)](https://docs.rs/webrust)
[![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)

## Why WebRust?

Write a single `.rs` file and instantly see interactive charts, tables, animations, and LaTeX in your browser. No HTML, CSS, or JavaScript required.

```rust, no run
use webrust::prelude::*;

#[gui]
fn main() {
    println("<white !green r8 p6 w480 h50>Hello, WebRust !").align("center");
    
    let sales = vec![120.0, 200.0, 150.0, 300.0, 250.0];
    bar(&["Jan", "Feb", "Mar", "Apr", "May"], &sales);
}
```

Run with `cargo run` – browser opens automatically at `http://127.0.0.1:8080`.

## Quick Start

```toml
[dependencies]
webrust = "2.0.0"
```

## Key Features

- **📊 17+ Chart Types** – Line, bar, pie, radar, treemap, heatmap, function plots
- **📋 Smart Tables** – Auto-headers, pivot, sort, filter, pagination, LaTeX support
- **🎨 Rich Formatting** – Colors, borders, backgrounds with `<red b !white>` syntax
- **🔢 LaTeX Math** – Inline formulas: `$(E = mc^2)`
- **🐢 Turtle Graphics** – Object-based animations with 30+ easing functions
- **🗄️ SQL Analytics** – Optional DuckDB integration for powerful queries
- **⚡ High Performance** – SIMD parsing, parking_lot locks, zero-copy rendering

---

## Coordinate Systems

WebRust supports two coordinate modes that affect all positioning operations.

| Mode                 | Origin          | Description                                   |
|----------------------|-----------------|-----------------------------------------------|
| `coord("css")`       | Top-left (0, 0) | **Default**. +y increases downward            |
| `coord("cartesian")` | Center (0, 0)   | Mathematical coordinates. +y increases upward |

```rust, no run
use webrust::prelude::*;

#[gui]
fn main() {
    coord("css");        // Default mode
    print("Top-left").at(0.0, 0.0);
    print("bottom-right").at(*TW + 40, *TH - 20);

    coord("cartesian");  // Switch to center origin
    print("Center").at(0.0, 0.0);
}
```

---

## Text Output & Formatting

### Print Methods

| Method                                  | Description                           |
|-----------------------------------------|---------------------------------------|
| `print(text)`                           | Print inline (no newline)             |
| `println(text)`                         | Print with newline                    |
| `.at(x, y)`                             | Position absolutely                   |
| `.align("left" \| "center" \| "right")` | Text alignment                        |
| `.sticky()`                             | Keep visible on scroll (with `.at()`) |

### Formatting Syntax

Format: `<attributes>content</>`

#### Text Styling

| Tag           | Description      | Example                              |
|---------------|------------------|--------------------------------------|
| `<color>`     | Text color       | `<red>`, `<blue>`, `<darkgreen>`     |
| `!color`      | Background color | `<white !navy>`, `<black !gold>`     |
| `b`           | Bold             | `<red b>`                            |
| `i`           | Italic           | `<blue i>`                           |
| `u`           | Underline        | `<green u>`                          |
| `s`           | Strikethrough    | `<gray s>`                           |
| `14`          | Font size (px)   | `<red 18>`                           |
| `Courier New` | Font family      | `<Courier New>` (use `_` for spaces) |

#### Layout & Box Model

| Tag                                   | Description         | Example               |
|---------------------------------------|---------------------|-----------------------|
| `t2`                                  | Border thickness    | `<t2>`, `<t3>`        |
| `\|color`                             | Border color        | `<\|red>`, `<\|blue>` |
| `solid`, `dashed`, `dotted`, `double` | Border style        | `<dashed>`            |
| `r8`                                  | Border radius       | `<r8>`, `<r10>`       |
| `w200`                                | Width (px)          | `<w200>`, `<w{*TW}>`  |
| `h50`                                 | Height (px)         | `<h50>`               |
| `p6`                                  | Padding (all sides) | `<p6>`                |
| `m10`                                 | Margin (all sides)  | `<m10>`               |
| `pt8`, `pb12`                         | Padding top/bottom  | `<pt8>`, `<pb12>`     |
| `pl5`, `pr15`                         | Padding left/right  | `<pl5>`, `<pr15>`     |
| `mt10`, `mb20`                        | Margin top/bottom   | `<mt10>`, `<mb20>`    |
| `ml5`, `mr15`                         | Margin left/right   | `<ml5>`, `<mr15>`     |

#### Border Positioning

| Style | Description               |
|-------|---------------------------|
| `B`   | Bottom border only        |
| `T`   | Top border only           |
| `L`   | Left border only          |
| `R`   | Right border only         |
| `H`   | Horizontal (top + bottom) |
| `V`   | Vertical (left + right)   |

#### Text Position in Box

| Tag  | Description             |
|------|-------------------------|
| `tl` | Top-left                |
| `tc` | Top-center              |
| `tr` | Top-right               |
| `ml` | Middle-left             |
| `mc` | Middle-center (default) |
| `mr` | Middle-right            |
| `bl` | Bottom-left             |
| `bc` | Bottom-center           |
| `br` | Bottom-right            |
| `j`  | Justify text            |

#### Screen Constants

| Constant | Description              |
|----------|--------------------------|
| `{*TW}`  | Total width / 2          |
| `{*TH}`  | Total height / 2         |
| `{*CW}`  | Content width (TW - 40)  |
| `{*CH}`  | Content height (TW - 40) |
#### LaTeX Math

Syntax: `$(expression)`

```rust, no run
println(r"Einstein: $(E = mc^2)");
println(r"Quadratic: $(x = \frac{-b \pm \sqrt{b^2-4ac}}{2a})");
```

---

## Chart API

### Available Chart Functions

| Function   | Description                    | Signature                                     |
|------------|--------------------------------|-----------------------------------------------|
| `line`     | Line chart (category axis)     | `line(&[&str], &[f64])`                       |
| `line_xy`  | Line chart (numeric x,y)       | `line_xy(&[f64], &[f64])`                     |
| `curve`    | Smooth curve (category)        | `curve(&[&str], &[f64])`                      |
| `curve_xy` | Smooth curve (numeric x,y)     | `curve_xy(&[f64], &[f64])`                    |
| `area`     | Area chart (category)          | `area(&[&str], &[f64])`                       |
| `area_xy`  | Area chart (numeric x,y)       | `area_xy(&[f64], &[f64])`                     |
| `bar`      | Bar chart                      | `bar(&[&str], &[f64])`                        |
| `scatter`  | Scatter plot                   | `scatter(&[f64], &[f64])`                     |
| `pie`      | Pie chart                      | `pie(&[&str], &[f64])`                        |
| `doughnut` | Doughnut chart                 | `doughnut(&[&str], &[f64])`                   |
| `radar`    | Radar chart                    | `radar(&[f64], &[(&str, f64)])`               |
| `gauge`    | Gauge meter (0-100)            | `gauge(f64)`                                  |
| `funnel`   | Funnel chart                   | `funnel(&[(&str, f64)])`                      |
| `matrix`   | Heatmap                        | `matrix(&[Vec<f64>], &[&str], &[&str])`       |
| `treemap`  | Treemap (flat or hierarchical) | `treemap(&[(&str, f64)])`                     |
| `tree`     | Tree diagram                   | `tree(&[&str])` or `tree(&serde_json::Value)` |
| `function` | Function plot                  | `function(impl Fn(f64)->f64, f64, f64, f64)`  |

### Chart Builder Methods

| Method                 | Description              |
|------------------------|--------------------------|
| `.at(x, y)`            | Position chart           |
| `.size(width, height)` | Dimensions in percentage |

```rust, no run
use webrust::prelude::*;

#[gui]
fn main() {
    let months = vec!["Jan", "Feb", "Mar", "Apr", "May"];
    let sales = vec![120.0, 200.0, 150.0, 300.0, 250.0];
    
    line(&months, &sales);
    pie(&["A", "B", "C"], &[45.0, 30.0, 25.0]);
}
```

---

## Table API

### Table Creation

```rust, no run
table(&data)  // Accepts: arrays, matrices, structs, nested objects
```

### Table Builder Methods

| Method                                  | Description                               |
|-----------------------------------------|-------------------------------------------|
| `.header(&[...])`                       | Set column headers (supports LaTeX)       |
| `.pivot()`                              | Transpose rows and columns                |
| `.merge()`                              | Merge consecutive identical cells         |
| `.sort()`                               | Enable multi-column sorting (shift+click) |
| `.filter()`                             | Enable global search/filter               |
| `.paginate()`                           | Enable pagination (default 10 rows)       |
| `.page_size(n)`                         | Set rows per page                         |
| `.at(x, y)`                             | Position table                            |
| `.size(w, h)`                           | Set dimensions in pixels                  |
| `.align("left" \| "center" \| "right")` | Alignment                                 |

```rust, no run
use webrust::prelude::*;

#[gui]
fn main() {
    let data = vec![vec![1, 2, 3], vec![4, 5, 6]];
    
    table(&data)
        .header(["$(x)", "$(y)", "$(z)"])
        .pivot()
        .header(["Row 1", "Row 2"]);
}
```

---

## Turtle Graphics

### Object Creation

```rust, no run
let obj = object();  // Create drawable object
let grp = group();   // Create object group
```

### Pen Control Methods

| Method                | Description                       |
|-----------------------|-----------------------------------|
| `.color(color)`       | Set stroke color                  |
| `.fill(color)`        | Set fill color                    |
| `.width(w)`           | Set line width                    |
| `.dash(down, up)`     | Set dash pattern                  |
| `.at(x, y)`           | Set position                      |
| `.pos()`              | Get current position `(f64, f64)` |
| `.to(angle)`          | Set pen angle (degrees)           |
| `.angle()`            | Get current angle                 |
| `.left(angle)`        | Turn left                         |
| `.right(angle)`       | Turn right                        |
| `.forward(distance)`  | Move forward                      |
| `.backward(distance)` | Move backward                     |
| `.penup()`            | Lift pen (no drawing)             |
| `.pendown()`          | Lower pen (drawing)               |

### Shape Methods

| Method                              | Description                  |
|-------------------------------------|------------------------------|
| `.point(x, y)`                      | Draw point                   |
| `.line(x1, y1, x2, y2)`             | Draw line                    |
| `.circle(radius)`                   | Draw circle                  |
| `.arc(radius, sweep_angle)`         | Draw arc                     |
| `.ellipse(rx, ry)`                  | Draw ellipse                 |
| `.rectangle(length, width)`         | Draw rectangle               |
| `.square(side)`                     | Draw square                  |
| `.rhombus(side, angle)`             | Draw rhombus                 |
| `.parallelogram(len, width, angle)` | Draw parallelogram           |
| `.polygon(points)`                  | Draw polygon `Vec<[f64; 2]>` |

### Animation Methods

| Method               | Description                |
|----------------------|----------------------------|
| `.speed(px_per_sec)` | Set animation speed        |
| `.ease(name)`        | Set easing function        |
| `.translate(dx, dy)` | Animate translation        |
| `.rotate(angle)`     | Animate rotation           |
| `.scale(sx, sy)`     | Animate scaling            |
| `.reflect(axis)`     | Mirror ("x", "y", or "xy") |
| `.reverse()`         | Reverse animation          |
| `.wait(ms)`          | Wait duration              |

### Available Easing Functions

`linear`, `sineIn`, `sineOut`, `sineInOut`, `quadIn`, `quadOut`, `quadInOut`, `cubicIn`, `cubicOut`, `cubicInOut`, `quartIn`, `quartOut`, `quartInOut`, `quintIn`, `quintOut`, `quintInOut`, `expoIn`, `expoOut`, `expoInOut`, `circIn`, `circOut`, `circInOut`, `backIn`, `backOut`, `backInOut`, `elasticIn`, `elasticOut`, `elasticInOut`, `bounceIn`, `bounceOut`, `bounceInOut`

### Group Methods

| Method               | Description               |
|----------------------|---------------------------|
| `.add(&object)`      | Add object to group       |
| `.remove(&object)`   | Remove object from group  |
| `.translate(dx, dy)` | Animate group translation |
| `.rotate(angle)`     | Animate group rotation    |
| `.scale(sx, sy)`     | Animate group scaling     |
| `.ease(name)`        | Set easing for group      |
| `.reverse()`         | Reverse group animation   |
| `.wait(ms)`          | Wait duration             |

```rust, no run
use webrust::prelude::*;

#[gui]
fn main() {
    let ball = object();
    ball.color("red").fill("tomato").at(100.0, 50.0).circle(20.0);
    ball.ease("bounceOut").speed(140.0).translate(0.0, 200.0);
}
```

---

## Grid Layout

### Grid Functions

| Function                 | Description                        |
|--------------------------|------------------------------------|
| `grid(rows, cols)`       | Create grid layout                 |
| `cell(row, col, anchor)` | Get cell coordinates `(f64, f64)`  |
| `size_pct(w_pct, h_pct)` | Get size in pixels from percentage |

### Grid Constants

| Constant | Description |
|----------|-------------|
| `*CW`    | Cell width  |
| `*CH`    | Cell height |

### Cell Anchors

`"top left"` / `"tl"`, `"top center"` / `"tc"`, `"top right"` / `"tr"`,
`"middle left"` / `"ml"`, `"center"` / `"mc"`, `"middle right"` / `"mr"`,
`"bottom left"` / `"bl"`, `"bottom center"` / `"bc"`, `"bottom right"` / `"br"`,
`"left"`, `"right"`, `"top"`, `"bottom"`

### Sizable Trait

Applied to charts and tables for grid integration:

```rust, no run
chart.size(80, 90)  // 80% of cell width, 90% of cell height
```

```rust, no run
use webrust::prelude::*;

#[gui]
fn main() {
    grid(1, 2);

    let (cx, cy) = cell(0, 0, "center");
    print("<white !navy r3 p1>cell (0,0)").at(cx, cy);

    let (cx, cy) = cell(0, 1, "top-right");
    bar(&["A", "B"], &[10.0, 20.0]).at(cx, cy).size(80, 80);
}
```

---

## SQL Analytics (Optional Feature)

Enable with: `webrust = { version = "2.0.0", features = ["sql"] }`

### SQL Function

```rust, no run
query(sql_string)  // Execute SQL and display results as table
```

Supports DuckDB features: window functions, CTEs, JSON operations, aggregates, joins.

```rust, no run
use webrust::prelude::*;

#[gui]
fn main() {
    query("CREATE TABLE sales(product TEXT, amount REAL)");
    query("INSERT INTO sales VALUES ('Laptop', 999), ('Mouse', 25)");
    query("SELECT product, amount FROM sales ORDER BY amount DESC");
}
```

---

## Architecture

```
SQL queries → Python (pandas) → Plotly → Flask → Deploy
     ↓             ↓              ↓        ↓       ↓
  Context    Type loss      Boilerplate  Frontend  Infrastructure

                          vs

Rust (WebRust) → Browser (automatic)
      ↓                    ↓
Type safety        Rich visualization
Performance        Zero deployment
```

**Single file, single language, instant visualization.**

---

## Performance Optimizations

- **SIMD acceleration** – `memchr`/`memmem` for fast string parsing
- **Parking lot locks** – 40% faster than std in high contention
- **RwLock by default** – Concurrent reads for better throughput
- **SmallVec** – Avoid heap allocations for small collections (tables use `SmallVec<[Cell; 12]>`)
- **Zero-copy rendering** – Direct serialization where possible
- **Compact strings** – Optional `compact` feature reduces string allocations

---

## Feature Flags

```toml
[features]
default = ["rwlock"]
sql = ["duckdb"]          # Enable SQL analytics
compact = ["compact_str"] # Use compact string representation
rwlock = []               # Use RwLock for concurrent reads (default)
```

To use `Mutex` instead of `RwLock`:
```toml
webrust = { version = "2.0.0", default-features = false }
```

---

## Examples Directory

Run with `cargo run --example <name>`:

| Example       | Description          |
|---------------|----------------------|
| `simpleio`    | Basic I/O operations |
| `advancedio`  | Advanced formatting  |
| `latexio`     | LaTeX rendering      |
| `string`      | String manipulation  |
| `utils`       | Utility functions    |
| `table`       | Table operations     |
| `chart`       | All 17+ chart types  |
| `turtle`      | Graphics & animation |
| `mixed`       | Complete dashboard   |
| `dyntable`    | Dynamic tables       |
| `simplesql`   | Basic SQL            |
| `advancedsql` | Advanced SQL         |

---

## Use Cases

### ✅ Perfect For

- Rapid prototyping and MVPs
- Data exploration and analysis
- Educational demonstrations
- Interactive presentations
- Small to medium datasets (<100K rows)
- Visualization-focused applications

### ⚠️ Consider Alternatives For

- Large-scale production web applications
- Complex multi-page applications
- Mobile-first responsive designs
- Real-time collaborative features

### 💡 With SQL Feature

- Analytical dashboards
- Complex joins and aggregations
- OLAP queries with window functions
- Large CSV/Parquet processing (100K+ rows)
- ETL workflows

---

## What's New in 2.0.0

- ✨ Background color support via `!<color>` syntax
- 🎯 Radar chart tooltips restored
- ⚡ Replaced standard locks with `parking_lot`
- 🔧 New `rwlock` feature for concurrent reads
- 📊 Macro engine rewritten with SIMD scanning
- 🌳 Enhanced tree visualization with unified API
- 🐛 Fixed treemap rendering artifacts
- 🚀 Performance improvements across the board

See [CHANGELOG.md](CHANGELOG.md) for complete details.

---

## Documentation

- **API Documentation**: [docs.rs/webrust](https://docs.rs/webrust)
- **Repository**: [github.com/gerarddubard/webrust](https://github.com/gerarddubard/webrust)
- **Examples**: [github.com/gerarddubard/webrust/tree/main/examples](https://github.com/gerarddubard/webrust/tree/main/examples)

---

## License

MIT License – see [LICENSE](LICENSE) for details.

---

## Acknowledgments

WebRust is built upon excellent open-source projects:
- [ECharts](https://echarts.apache.org/) – Visualization library
- [KaTeX](https://katex.org/) – LaTeX rendering
- [tiny_http](https://crates.io/crates/tiny_http) – HTTP server
- [DuckDB](https://duckdb.org/) – Analytical database

---

**Made with ❤️ for Rustaceans who miss Python's simplicity but crave Rust's power.**
