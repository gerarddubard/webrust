# 🚀 WebRust — **Python Meets Rust Meets Web Meets Data**

### *The Revolutionary Framework Bridging Ecosystems*

[![WebRust](https://img.shields.io/badge/🦀_WebRust-1.4.0-ff6b35?style=for-the-badge&labelColor=000&logoColor=white)](https://github.com/gerarddubard/webrust)
[![Rust](https://img.shields.io/badge/Rust-1.70+-000?style=for-the-badge&logo=rust)](https://rust-lang.org)
[![Web Ready](https://img.shields.io/badge/Web-Ready-4285f4?style=for-the-badge&logo=googlechrome&logoColor=white)](https://docs.rs/webrust)
[![SQL Optional](https://img.shields.io/badge/SQL-Optional-ffd700?style=for-the-badge&logo=duckdb&logoColor=white)](https://duckdb.org)
[![Python Style](https://img.shields.io/badge/Python-Style-3776ab?style=for-the-badge&logo=python&logoColor=white)](https://crates.io/crates/webrust)

**🔥 Write like Python. Run like Rust. Deploy as Web. Zero configuration.**

**🆕 NEW in 1.4.0**: Ultra-fast compilation + Optional SQL analytics!

[Documentation](https://docs.rs/webrust) | [Examples](https://github.com/gerarddubard/webrust/tree/main/examples) | [Crates.io](https://crates.io/crates/webrust)

---

## Table of Contents

1. [Why WebRust?](#-why-webrust)
2. [The Problem](#-the-problem-four-painful-paths)
3. [The Solution](#-the-solution-webrust)
4. [Core Philosophy](#-core-philosophy-python-ergonomics-in-rust)
5. [Installation](#-installation)
6. [Quick Start](#-quick-start)
7. [API Reference](#-api-reference)
8. [Examples](#-examples)
9. [Use Cases](#-use-cases)
10. [Performance](#-performance)
11. [Roadmap](#-roadmap)
12. [Contributing](#-contributing)

---

## 🌟 Why WebRust?

### **The Great Divide in Programming**

The programming world is split between multiple philosophies:

**Python's camp** says: *"Life is short, use Python"* — prioritizing developer happiness, rapid prototyping, and readable syntax.

**Rust's camp** says: *"Performance and safety first"* — prioritizing zero-cost abstractions, memory safety, and compile-time guarantees.

**SQL's camp** says: *"Data is everything"* — prioritizing declarative queries, relational algebra, and analytical power.

**WebRust's answer**: *"Why choose? Have them all."*

### **Why We're Still Stuck in Fragmented Workflows in 2025**

It's 2025. We have:
- 🚀 Blazingly fast computers with multi-core processors
- 🎨 Beautiful displays with millions of colors
- 🌐 Universal web browsers on every device
- 🧠 AI models running in real-time
- 📊 Massive datasets requiring instant analysis

Yet most data workflows require:
- **Three separate tools**: Python for scripting, SQL for queries, JavaScript for visualization
- **Multiple context switches**: Write SQL, export CSV, import to Python, generate charts
- **Complex infrastructure**: Database servers, web servers, frontend frameworks
- **Hours of setup**: Configure connections, manage dependencies, debug integrations

**WebRust's philosophy**: One language. One file. Instant results. Modern defaults.

---

## 🎯 The Problem: Four Painful Paths

### **Path 1: Terminal Applications (1970s Technology)**

```rust,ignore
use std::io;
println!("What's your name?");
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read input");
let name = input.trim();
println!("Hello, {}!", name);
```

**Reality Check**:
- ❌ No colors, no styling, no interactivity
- ❌ Can't embed charts, tables, or visualizations
- ❌ No mathematical notation support

### **Path 2: Traditional Data Analysis (Tool Soup)**

```python
# Step 1: SQL query (PostgreSQL/MySQL)
import psycopg2
conn = psycopg2.connect("dbname=sales user=admin")
cursor = conn.execute("SELECT product, SUM(amount) FROM sales GROUP BY product")
results = cursor.fetchall()

# Step 2: Python processing (pandas)
import pandas as pd
df = pd.DataFrame(results, columns=['product', 'total'])

# Step 3: Visualization (matplotlib/plotly)
import matplotlib.pyplot as plt
plt.bar(df['product'], df['total'])
plt.savefig('chart.png')
```

**Reality Check**:
- ❌ Three separate languages/tools
- ❌ Multiple data format conversions
- ❌ Complex dependency management

### **Path 3: Web Frameworks (Complexity Explosion)**

```rust,ignore
use rocket::*;

#[get("/")]
fn index() -> &'static str {
    "<html><body>Hello!</body></html>"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
```

**Reality Check**:
- ❌ Need to learn 3 languages (HTML/CSS/JS)
- ❌ Separate frontend and backend logic
- ❌ Just wanted to visualize data!

---

## ✨ The Solution: WebRust

```rust,ignore
use webrust::prelude::*;

#[gui(bg="navy", fg="white", font="Courier New")]
fn main() {
    println!("@(cyan, bold, italic)📊 Data Dashboard");
    
    let name: String = input("What's your name?");
    println!("Hello, {name}! 🎉");
    
    let data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    chart(&data, "line").title("Trend Analysis");
    
    let squares: Vec<i32> = 0.to(10).then(|x| x * x);
    table(&squares).header(["Index", "Square"]);
}
```

**What You Get**:
✅ **Python-like syntax** — `0.to(10)`, `.when()/.then()`, `.splitby()`  
✅ **Styled output** — Colors, fonts, positions with chainable API  
✅ **Type-safe inputs** — Real-time validation in the browser  
✅ **Interactive charts** — 9+ chart types with ECharts  
✅ **Automatic web UI** — Browser opens, server runs, zero config  
✅ **Rust performance** — Compiles to native code, blazingly fast  
✅ **Ultra-fast compile** — ~30 seconds initial build

**Run**: `cargo run` → Browser opens instantly → Professional UI

---

## 🧩 Core Philosophy: Python Ergonomics in Rust

### **1. Python-like Ranges**

```rust,ignore
use webrust::prelude::*;

// Python: for i in range(10)
for i in 0.to(10) { }

// Python: for i in range(0, 100, 5)
for i in 0.to(100).by(5) { }

// Python: for c in 'abcde'
for c in 'a'.to('f') { }

// Floats and negative steps too!
for x in 4.0.to(0.0).by(-0.5) { }
```

### **2. List & Dictionary Comprehensions**

```rust,ignore
use webrust::prelude::*;
use std::collections::HashMap;

// Python: [x**2 for x in range(10)]
let squares: Vec<i32> = 0.to(10).then(|x| x * x);

// Python: [x for x in range(20) if x % 2 == 0]
let evens: Vec<i32> = 0.to(20)
    .when(|&x| x % 2 == 0)
    .then(|x| x);

// Python: {x: x**2 for x in range(5)}
let dict: HashMap<i32, i32> = 0.to(5).then(|x| (x, x * x));
```

**Zero runtime cost** — compiles to standard Rust iterators!

### **3. Python String Methods**

```rust,ignore
use webrust::prelude::*;

// Python: "a,b,c".split(",")
let parts = "a,b,c".splitby(",");

// Python: "hello world".split()
let words = "hello  world".splitby("");

// Python: ", ".join(["a", "b", "c"])
let joined = parts.join(", ");

// Python: "hello".upper()
let upper = "hello".upper();
```

### **4. F-String Interpolation**

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    let name = "Alice";
    let age = 30;
    let pi = std::f64::consts::PI;
    
    println!("Hello {name}, you are {age} years old!");
    println!("Next year: {age + 1}");
    println!("PI ≈ {pi:.2}");                    // Format specifiers
    println!("Data: {my_struct:j}");             // JSON pretty-print
    println!("Einstein: $(E = mc^2)");           // LaTeX rendering!
}
```

### **5. SQL Integration (Optional)**

When `features = ["sql"]` is enabled:

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

---

## 🚀 Installation

### **Option 1: Fast Install (Recommended)**

```toml
[dependencies]
webrust = "1.4.0"
```

✅ Compiles in **~30 seconds**  
✅ Python-like syntax  
✅ Web GUI with charts & tables  
✅ LaTeX rendering  
✅ Turtle graphics

### **Option 2: With SQL Analytics**

```toml
[dependencies]
webrust = { version = "1.4.0", features = ["sql"] }
```

✅ Everything from Option 1  
✅ **Plus**: DuckDB integration  
✅ **Plus**: Native SQL queries  
⚠️ First compile: **2-5 minutes**

---

## 🎬 Quick Start

### **Your First App (30 seconds)**

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    let name: String = input("What's your name?");
    println!("Hello, {name}! 🎉");
    
    let nums = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    chart(&nums, "line").title("My First Chart");
}
```

**Run**: `cargo run` → Browser opens → Professional UI appears instantly.

---

## 🧭 API Reference

### **Module: `db` — SQL (DuckDB)**

*Requires `features = ["sql"]`*

#### `query(sql: &str)`
Execute one or more SQL queries with instant display in browser.

```rust,ignore
query("SELECT * FROM users WHERE age > 18");

query(r#"
    CREATE TABLE sales AS SELECT * FROM read_csv_auto('data.csv');
    SELECT product, SUM(amount) FROM sales GROUP BY product;
"#);
```

#### `query_to_hashmap(sql: &str) -> HashMap<String, f64>`
Execute query and return results as HashMap (useful for charts).

```rust,ignore
let data = query_to_hashmap("SELECT region, SUM(sales) FROM ...");
chart(&data, "bar");
```

#### `query_to_vec(sql: &str) -> Vec<(String, f64)>`
Execute query and return results as Vec of tuples.

---

### **Module: `graphic` — Turtle & Animations**

#### `object() -> DrawableObject`
Create a drawable object with turtle graphics.

**Chainable methods:**

**Position & Movement:**
- `.at(x: f64, y: f64)` — Set position
- `.to(x: f64, y: f64)` — Move to position
- `.forward(distance: f64)` — Move forward
- `.right(angle: f64)` — Turn right (degrees)
- `.left(angle: f64)` — Turn left (degrees)
- `.penup()` — Lift pen (don't draw)
- `.pendown()` — Lower pen (draw)

**Shapes:**
- `.circle(radius: f64)` — Draw circle
- `.rectangle(width: f64, height: f64)` — Draw rectangle
- `.square(size: f64)` — Draw square
- `.ellipse(rx: f64, ry: f64)` — Draw ellipse
- `.arc(radius: f64, angle: f64)` — Draw arc
- `.rhombus(width: f64, height: f64)` — Draw rhombus
- `.parallelogram(width: f64, height: f64, skew: f64)` — Draw parallelogram
- `.polygon(sides: i32, radius: f64)` — Draw regular polygon

**Style:**
- `.color(color: &str)` — Set color ("red", "#ff0000")
- `.width(width: f64)` — Set line width
- `.fill(color: &str)` — Fill shape with color

**Transformations & Animation:**
- `.speed(speed: f64)` — Set animation speed
- `.rotate(angle: f64)` — Rotate object
- `.translate(x: f64, y: f64)` — Move object
- `.scale(sx: f64, sy: f64)` — Scale object
- `.reflect(axis: &str)` — Reflect ("x", "y")
- `.ease(easing: &str)` — Animation easing function

**Easing functions:** `linear`, `sineIn`, `sineOut`, `sineInOut`, `quadIn`, `quadOut`, `quadInOut`, `cubicIn`, `cubicOut`, `cubicInOut`, `elasticIn`, `elasticOut`, `elasticInOut`, `bounceIn`, `bounceOut`, `bounceInOut`, `backIn`, `backOut`, `backInOut`, `expoIn`, `expoOut`, `expoInOut`

**Example:**
```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    coord("cartesian");
    
    let turtle = object();
    turtle.color("blue").width(2.0);
    
    for _ in 0.to(4) {
        turtle.forward(100.0);
        turtle.right(90.0);
    }
    
    turtle.rotate(360.0).ease("elasticOut");
}
```

#### `group() -> Group`
Create a hierarchical group of objects.

**Methods:**
- `.add(&object)` — Add object to group
- `.translate(x: f64, y: f64)` — Move entire group
- `.rotate(angle: f64)` — Rotate entire group
- `.ease(easing: &str)` — Animate entire group

**Example:**
```rust,ignore
let car = group();
let wheel1 = object().at(-30.0, 0.0).circle(15.0);
let wheel2 = object().at(30.0, 0.0).circle(15.0);
car.add(&wheel1);
car.add(&wheel2);
car.translate(200.0, 0.0);
```

---

### **Module: `io` — I/O, Styled Printing & GUI Server**

#### `print!(...)` / `println!(...)`
Print with f-string interpolation, inline styles, and LaTeX support.

**Basic usage:**
```rust,ignore
println!("Hello {name}!");
println!("Result: {value:.2}");
```

**Inline styles:**
```rust,ignore
println!("@(red)Error: File not found");
println!("@(cyan, bold)🎉 Success!");
println!("@(green, italic, underline)Important note");
```

**Available styles:** `red`, `green`, `blue`, `cyan`, `yellow`, `magenta`, `white`, `black`, `bold`, `italic`, `underline`

**LaTeX math:**
```rust,ignore
println!("Einstein: $(E = mc^2)");
println!("Quadratic: $(x = \frac{-b \pm \sqrt{b^2-4ac}}{2a})");
```

**Format specifiers:**
- `{var:j}` — JSON pretty-print
- `{var:c}` — Compact display
- `{var:.2}` — Float with 2 decimals

#### `input<T>(prompt: &str) -> T`
Type-safe input with browser validation.

```rust,ignore
let name: String = input("Enter your name:");
let age: i32 = input("Enter your age:");
let height: f64 = input("Enter your height:");
let confirmed: bool = input("Confirm?");
```

**Validation happens twice:**
1. Client-side (JavaScript) — Immediate feedback
2. Server-side (Rust) — Type-safe parsing

#### `try_input<T>(prompt: &str) -> Result<T, E>`
Input with error handling.

```rust,ignore
match try_input::<i32>("Age:") {
    Ok(age) => println!("Age: {age}"),
    Err(e) => println!("Invalid input: {e}"),
}
```

#### `input_with_validation<T>(prompt: &str, validator: F) -> T`
Input with custom validation function.

```rust,ignore
let age = input_with_validation("Age:", |&x: &i32| x >= 18 && x <= 100);
```

#### `add_output(text: &str)`
Add output to browser without newline.

#### `add_output_new_line(text: &str)`
Add output to browser with newline.

#### `add_output_same_line(text: &str)`
Replace current line in browser.

#### `start_gui_server()`
Start GUI server with default styling.

#### `start_gui_server_with_style(config: StyleConfig)`
Start GUI server with custom styling.

---

### **Module: `iter` — Ranges, Enumerate, Comprehensions**

#### Ranges: `start.to(end).by(step)`

**Integer ranges:**
```rust,ignore
for i in 0.to(10) { }           // 0, 1, 2, ..., 9
for i in 0.to(100).by(5) { }    // 0, 5, 10, ..., 95
```

**Float ranges:**
```rust,ignore
for x in 0.0.to(10.0).by(0.5) { }    // 0.0, 0.5, 1.0, ..., 9.5
for x in 5.0.to(0.0).by(-0.5) { }    // 5.0, 4.5, 4.0, ..., 0.5
```

**Character ranges:**
```rust,ignore
for c in 'a'.to('z') { }    // a, b, c, ..., y
for c in 'A'.to('F') { }    // A, B, C, D, E
```

#### Comprehensions: `.when(pred).then(mapper)`

**Map transformation:**
```rust,ignore
let squares: Vec<i32> = 0.to(10).then(|x| x * x);
```

**Filter and transform:**
```rust,ignore
let evens: Vec<i32> = 0.to(20)
    .when(|&x| x % 2 == 0)
    .then(|x| x);
```

**Dictionary construction:**
```rust,ignore
use std::collections::HashMap;
let dict: HashMap<i32, i32> = 0.to(5).then(|x| (x, x * x));
```

#### `enumerate(iter) -> EnumeratedIter`
Add indices to iterator.

```rust,ignore
for (i, value) in enumerate(vec![10, 20, 30]) {
    println!("{i}: {value}");
}
```

---

### **Module: `layout` — Coordinates & Grids**

#### `coord(mode: &str)`
Set coordinate system mode.

```rust,ignore
coord("css");         // CSS coordinates (top-left origin, Y down)
coord("cartesian");   // Cartesian coordinates (center origin, Y up)
```

#### `grid(rows: i32, cols: i32)`
Create responsive grid layout.

```rust,ignore
grid(2, 3);  // 2 rows, 3 columns
```

#### `cell(row: i32, col: i32, anchor: &str) -> (f64, f64)`
Calculate position for grid cell.

**Anchor options:** `"topleft"`, `"top"`, `"topright"`, `"left"`, `"center"`, `"right"`, `"bottomleft"`, `"bottom"`, `"bottomright"`

```rust,ignore
let (x, y) = cell(0, 1, "center");
chart(&data, "bar").at(x, y);
```

#### `grid_size() -> (f64, f64)`
Get total grid dimensions.

#### `cell_size() -> (f64, f64)`
Get individual cell dimensions.

#### `size_pct(width_pct: f64, height_pct: f64) -> (f64, f64)`
Calculate size as percentage of cell.

---

### **Module: `text` — String Methods (Python-like)**

#### `.splitby(pattern: &str) -> Vec<String>`
Split string by pattern.

```rust,ignore
let parts = "a,b,c".splitby(",");           // ["a", "b", "c"]
let words = "hello  world".splitby("");     // ["hello", "world"] (whitespace)
let lines = "L1\nL2\nL3".splitby("\n");    // ["L1", "L2", "L3"]
```

#### `.join(separator: &str) -> String`
Join vector of strings.

```rust,ignore
let joined = vec!["a", "b", "c"].join(", ");  // "a, b, c"
```

#### `.upper() -> String`
Convert to uppercase.

```rust,ignore
let upper = "hello".upper();  // "HELLO"
```

#### `.lower() -> String`
Convert to lowercase.

```rust,ignore
let lower = "HELLO".lower();  // "hello"
```

#### `.title() -> String`
Convert to title case.

```rust,ignore
let title = "hello world".title();  // "Hello World"
```

---

### **Module: `viz` — Tables & Charts**

#### Tables

##### `table(data) -> Table`
Create table from data.

**Data types supported:**
- `Vec<Vec<T>>` — 2D vector
- `Vec<(T, U)>` — Vector of tuples
- `HashMap<K, V>` — HashMap

**Chainable methods:**
- `.header(headers: &[&str])` — Set column headers
- `.merge()` — Merge adjacent cells with same value
- `.at(x: f64, y: f64)` — Position table
- `.pivot()` — Transpose table
- `.latex()` — Enable LaTeX rendering in cells

**Example:**
```rust,ignore
let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
table(&matrix).header(["X", "Y", "Z"]);

let physics = vec![
    ("Einstein", r"$(E = mc^2)"),
    ("Schrödinger", r"$(i\hbar\frac{\partial}{\partial t}\Psi)"),
];
table(&physics).header(["Scientist", "Equation"]).latex();
```

#### Charts

##### `chart(data, chart_type: &str) -> Chart`
Create interactive chart.

**Chart types supported:**
- `"line"` — Line chart
- `"bar"` — Bar chart
- `"pie"` — Pie chart
- `"area"` — Area chart
- `"doughnut"` — Doughnut chart
- `"funnel"` — Funnel chart
- `"radar"` — Radar/spider chart
- `"gauge"` — Gauge chart
- `"scatter"` — Scatter plot
- `"heatmap"` — Heatmap

**Data types supported:**
- `Vec<f64>` — Simple values
- `Vec<(String, f64)>` — Labeled values
- `HashMap<String, f64>` — Key-value pairs
- `PieData(labels, values)` — Pie chart data

**Chainable methods:**
- `.title(title: &str)` — Set chart title
- `.xlabels(labels: Vec<&str>)` — Set X-axis labels
- `.ylabels(labels: Vec<&str>)` — Set Y-axis labels
- `.color(color: &str)` — Set primary color
- `.colors(colors: Vec<&str>)` — Set multiple colors
- `.at(x: f64, y: f64)` — Position chart
- `.size(width: f64, height: f64)` — Set chart dimensions
- `.legend(show: bool)` — Show/hide legend
- `.tooltip(show: bool)` — Show/hide tooltips

**Example:**
```rust,ignore
use std::collections::HashMap;

// Bar chart
let sales = HashMap::from([
    ("Q1", 120.0), ("Q2", 200.0),
    ("Q3", 150.0), ("Q4", 300.0)
]);
chart(&sales, "bar")
    .title("Quarterly Sales")
    .color("#2ecc71");

// Line chart with labels
let temps = vec![64.4, 67.1, 69.8, 72.5, 70.2];
chart(&temps, "line")
    .title("Temperature Trend")
    .xlabels(vec!["Mon", "Tue", "Wed", "Thu", "Fri"]);

// Pie chart
let market = PieData(
    vec!["Product A".into(), "Product B".into(), "Product C".into()],
    vec![45.0, 30.0, 25.0]
);
chart(market, "pie").title("Market Share");
```

---

## 📚 Examples

```bash
# Core features (fast compile):
cargo run --example simpleio      # Inputs, styling, positioning
cargo run --example latex         # LaTeX mathematical notation
cargo run --example advancedio    # Advanced I/O, tables, formatting
cargo run --example string        # Python-like string methods
cargo run --example utils         # Ranges, enumerate, comprehensions
cargo run --example table         # Table generation & formatting
cargo run --example chart         # All 9+ chart types
cargo run --example turtle        # Turtle graphics & animations
cargo run --example mixed         # Complete dashboard layout

# SQL features (requires --features sql):
cargo run --example sql --features sql    # SQL analytics with DuckDB
```

---

## 🌍 Use Cases

### **1. Rapid Prototyping**

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    println!("@(cyan, bold)🎨 Product Demo");
    
    let choice: String = input("Select feature (A/B/C):");
    
    match choice.as_str() {
        "A" => {
            let data = vec![10.0, 25.0, 40.0, 35.0];
            chart(&data, "bar").title("Performance");
        }
        "B" => {
            coord("cartesian");
            let circle = object().circle(50.0);
            circle.rotate(360.0).ease("elasticOut");
        }
        _ => println!("@(red)Invalid choice"),
    }
}
```

### **2. Data Visualization**

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
}
```

### **3. Educational Tools**

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    println!("@(purple, bold)📚 Fibonacci Sequence");
    
    let mut fib = vec![0i64, 1i64];
    for i in 2..15 {
        fib.push(fib[i-1] + fib[i-2]);
    }
    
    table(&fib).header(["Index", "Value"]);
    
    println!(r"Golden ratio: $(\phi = \frac{1+\sqrt{5}}{2})");
}
```

### **4. Scientific Computing**

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    coord("cartesian");
    
    let v0 = 50.0;
    let angle = 45.0_f64.to_radians();
    let g = 9.81;
    
    let trajectory: Vec<(f64, f64)> = (0..100).then(|i| {
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
}
```

### **5. Data Analytics with SQL** *(requires `features = ["sql"]`)*

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
            AVG(response_time) AS avg_latency
        FROM logs
        WHERE timestamp >= NOW() - INTERVAL 24 HOURS
        GROUP BY hour
        ORDER BY hour DESC
    "#);
}
```

---

## ⚡ Performance

### **Compilation Time**

| Configuration    | First Build  | Subsequent Builds |
|------------------|--------------|-------------------|
| Default (no SQL) | ~30 seconds  | ~1-2 seconds      |
| With SQL feature | 2-5 minutes  | ~1-2 seconds      |

### **Runtime Performance (v1.4.0)**

- F-string transformation: ~0.85μs per operation (43% faster)
- Memory allocations: ~5 per transformation (67% reduction)
- Memory footprint: ~340 bytes per transformation (60% reduction)

**Techniques:**
- SIMD pattern matching via `memchr`
- Zero-copy optimization with `Cow<str>`
- Optimized number formatting (`itoa`, `ryu`)

**Result**: 60fps animations, instant feedback

---

## 🔮 Roadmap

- 🎨 **More chart types** (sankey, treemap, 3D plots)
- 🗄️ **Database connectors** (PostgreSQL, MySQL)
- 🧩 **Component system** (reusable widgets)
- 🌐 **Static export** (standalone HTML)
- 📱 **Mobile optimization** (responsive)

---

## 🤝 Contributing

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/gerarddubard/webrust/issues)
- 💡 **Feature Requests**: [GitHub Discussions](https://github.com/gerarddubard/webrust/discussions)
- 📝 **Documentation**: PRs welcome!
- 🎨 **Show & Tell**: [Share your projects](https://github.com/gerarddubard/webrust/discussions/show-and-tell)

**Community principles**:
- Keep it Pythonic (readable, intuitive)
- Keep it Rusty (safe, fast)
- Keep it simple (zero-config)
- Keep it optional (SQL when needed)

---

## 📄 License

MIT License — see [LICENSE](https://github.com/gerarddubard/webrust/blob/main/LICENSE).

---

## 🙏 Acknowledgments

- **Python community** — For ergonomics
- **Rust community** — For safety & speed
- **SQL community** — For declarative data power
- [DuckDB](https://duckdb.org/) — Analytical database
- [Apache Arrow](https://arrow.apache.org/) — Columnar data
- [tiny_http](https://crates.io/crates/tiny_http) — HTTP server
- [serde](https://crates.io/crates/serde) — Serialization
- [MathJax](https://www.mathjax.org/) — Math rendering
- [ECharts](https://echarts.apache.org/) — Charts
- [Two.js](https://two.js.org/) — 2D graphics

---

## 🌟 The Bottom Line

**It's 2025.** Programming should be:
- **Intuitive** (Python-like syntax)
- **Safe** (Rust's type system)
- **Visual** (browser-based UIs)
- **Fast** (sub-second compilation by default)
- **Powerful** (SQL when you need it)
- **Accessible** (zero configuration)

**WebRust proves it's possible.**

---

### 🦀 **Write Python. Think Rust. Ship Web.** 🌐

**[Get Started Now](https://docs.rs/webrust)** | **[View Examples](https://github.com/gerarddubard/webrust/tree/main/examples)** | **[Join Discussion](https://github.com/gerarddubard/webrust/discussions)**

---

*Made with ❤️ for developers who believe programming should be joyful and fast*