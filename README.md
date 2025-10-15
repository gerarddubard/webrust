<!-- Stable anchors for IDEs: defined before any links -->
<a id="sec-getting-started"></a><a name="sec-getting-started"></a><span id="sec-getting-started"></span>
<a id="sec-core-philosophy"></a><a name="sec-core-philosophy"></a><span id="sec-core-philosophy"></span>

# 🚀 WebRust — **Python Meets Rust Meets Web Meets Data**

### *The Revolutionary Framework Bridging Ecosystems*

<div style="text-align:center">

[![WebRust](https://img.shields.io/badge/🦀_WebRust-1.3.0-ff6b35?style=for-the-badge&labelColor=000&logoColor=white)](https://github.com/gerarddubard/webrust)
[![Rust](https://img.shields.io/badge/Rust-1.70+-000?style=for-the-badge&logo=rust)](https://rust-lang.org)
[![Web Ready](https://img.shields.io/badge/Web-Ready-4285f4?style=for-the-badge&logo=googlechrome&logoColor=white)](https://docs.rs/webrust)
[![SQL Powered](https://img.shields.io/badge/SQL-DuckDB-ffd700?style=for-the-badge&logo=duckdb&logoColor=white)](https://duckdb.org)
[![Python Style](https://img.shields.io/badge/Python-Style-3776ab?style=for-the-badge&logo=python&logoColor=white)](https://crates.io/crates/webrust)

**🔥 Write like Python. Query like SQL. Run like Rust. Deploy as Web. Zero configuration.**

**🆕 NEW in 1.3.0**: Ultra-fast rendering + Native SQL analytics with DuckDB!

[Documentation](https://docs.rs/webrust) | [Examples](https://github.com/gerarddubard/webrust/tree/main/examples) | [Crates.io](https://crates.io/crates/webrust)


<!-- IDE-friendly explicit anchors -->
<a id="getting-started"></a><a name="getting-started"></a><span id="getting-started"></span>
<a id="core-philosophy"></a><a name="core-philosophy"></a><span id="core-philosophy"></span>

</div>

---

## 🌟 **Why WebRust? A Manifesto for 2025**

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

**Why?** Because historically, these were separate domains with different tools.

**WebRust's philosophy**: One language. One file. Instant results. Modern defaults.

### **A Bridge Between Three Worlds**

WebRust isn't just a library — it's a **proposal for the future of programming**:

1. **Syntax Evolution**: Languages should embrace ergonomics without sacrificing performance
2. **Ecosystem Integration**: Python, Rust, and SQL communities should learn from each other
3. **Modern Defaults**: In 2025, data tools should be visual, interactive, and instant
4. **Zero-Configuration Philosophy**: Great tools should work out of the box

WebRust proves these aren't mutually exclusive goals. You can have Python's elegance, Rust's speed, SQL's power, running in a modern web interface, with zero setup.

---

## 🎯 **The Problem: Four Painful Paths**

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
- ❌ Copy-paste is painful
- ❌ Screenshots look unprofessional

**In 2025, this is like using a typewriter when you have a smartphone.**

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

# Step 4: Web deployment (Flask/Django)
# ... 100 more lines of boilerplate
```

**Reality Check**:
- ❌ Three separate languages/tools
- ❌ Multiple data format conversions
- ❌ Complex dependency management
- ❌ Slow iteration cycle (query → export → import → visualize)
- ❌ Deployment requires web framework

**In 2025, this is like assembling furniture with 20 different tools.**

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
- ❌ State management becomes complex
- ❌ Deployment requires infrastructure
- ❌ Just wanted to visualize data!

**In 2025, this is like building a car when you just want to go to the store.**

### **Path 4: Desktop GUI Frameworks (Framework Lock-in)**

```rust,ignore
use eframe::egui;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My App");
            if ui.button("Click").clicked() {
                // Complex state management
            }
        });
    }
}
```

**Reality Check**:
- ❌ Learn framework-specific APIs
- ❌ Platform-specific quirks
- ❌ Distribution is complicated
- ❌ Can't easily share via URL
- ❌ Updates require reinstalling

**In 2025, this is like building native apps when the web exists.**

---

## ✨ **The WebRust Solution: Elegance Meets Power Meets Data**

### **Same Problems, WebRust Way**

```rust,ignore
use webrust::prelude::*;

#[gui(bg="navy", fg="white", font="Courier New")]
fn main() {
    println!("@(cyan, bold, italic)📊 Sales Analytics Dashboard");
    
    // 🆕 NEW in 1.3.0: Native SQL queries with DuckDB!
    query(r#"
        CREATE TABLE sales AS SELECT * FROM read_csv_auto('sales.csv');
        
        SELECT 
            product,
            SUM(amount) AS total_sales,
            COUNT(*) AS num_transactions
        FROM sales
        GROUP BY product
        ORDER BY total_sales DESC
    "#);
    
    // Instant interactive visualization
    let top_products = query_to_hashmap("SELECT product, total_sales FROM ...");
    chart(&top_products, "bar")
        .title("Top Products by Revenue")
        .color("limegreen");
    
    // All in one file, zero configuration
}
```

**What Just Happened?**

✅ **SQL analytics** — Native DuckDB integration with Arrow streaming  
✅ **Python-like syntax** — `0.to(10)`, `.when()/.then()`, `.splitby()`  
✅ **Styled output** — Colors, fonts, positions with chainable API  
✅ **Type-safe inputs** — Real-time validation in the browser  
✅ **Automatic web UI** — Browser opens, server runs, zero config  
✅ **Rust performance** — Compiles to native code, runs blazingly fast  
✅ **🆕 Ultra-responsive** — Optimized rendering engine (40-60% faster)

**Run it**: `cargo run` → Browser opens automatically → Professional SQL-powered dashboard.

---

## 🆕 **What's New in 1.3.0: Speed Meets SQL**

### **1. Native SQL Analytics with DuckDB**

WebRust 1.3.0 introduces a complete SQL engine powered by DuckDB and Apache Arrow:

```rust,ignore
# use webrust::prelude::*;
# #[gui] fn example() {
// Create and query data in-memory
query(r#"
    CREATE TABLE person (id INT, name TEXT, age INT);
    INSERT INTO person VALUES (1, 'Alice', 30), (2, 'Bob', 25);
    
    SELECT name, age FROM person WHERE age > 20 ORDER BY age DESC;
"#);

// Join multiple tables
query(r#"
    CREATE TABLE city (id INT, name TEXT);
    INSERT INTO city VALUES (1, 'Paris'), (2, 'Lyon');
    
    SELECT p.name, c.name AS city
    FROM person p
    JOIN city c ON p.city_id = c.id;
"#);

// Window functions and aggregates
query(r#"
    SELECT 
        name,
        age,
        RANK() OVER (ORDER BY age DESC) as age_rank
    FROM person;
"#);
# }
```

**Key Features**:
- ✅ **In-memory analytics**: Zero setup, zero dependencies
- ✅ **Arrow streaming**: Batch-by-batch rendering (handles millions of rows)
- ✅ **Standard SQL**: Full DuckDB SQL support (CTEs, window functions, joins)
- ✅ **Auto-formatted tables**: Results stream as styled HTML tables
- ✅ **Schema inspection**: `SCHEMA SELECT ...` to view column types
- ✅ **CSV/JSON support**: `read_csv_auto()`, `read_json()` built-in
- ✅ **Performance**: SIMD-optimized with parallel execution

**Why This Matters**:
- No PostgreSQL, MySQL, or SQLite installation needed
- No ORM complexity
- No context switching between tools
- Query → Visualize in the same file
- Perfect for data exploration, reports, and analytics

  **⚠️ Note on First Compilation**:
The initial compilation time is longer than in version 1.2.0 due to DuckDB's native 
integration (~5-10 minutes on first build). However, subsequent incremental builds remain 
fast, and the performance gains in SQL query execution far outweigh this one-time cost.

### **2. Ultra-Responsive Rendering**

WebRust 1.3.0 delivers **40-60% faster** rendering through aggressive optimizations:

**Macro System**:
- **~0.85μs** per f-string transformation (43% faster than 1.2.0)
- **~5 allocations** per transformation (67% reduction)
- **~340 bytes** memory footprint (60% reduction)
- SIMD pattern matching with `memchr` and `memchr2`
- Zero-copy optimization with `Cow<str>`

**Number Formatting**:
- `itoa::Buffer` for integers (3x faster than `format!`)
- `ryu::Buffer` for floats (10x faster than `format!`)
- Direct buffer writing, zero allocations

**SQL Rendering**:
- Thread-local buffers (4KB capacity, reused across rows)
- SIMD HTML escaping (zero-copy for clean strings)
- Incremental table streaming (progressive paint)
- Arrow columnar access (cache-friendly)

**User Experience**:
- Instant feedback on user input
- Smooth animations at 60fps
- No flicker during updates
- Responsive even with large datasets

---

<a id="core-philosophy"></a>
<a id="sec-core-philosophy"></a>
## 🧩 **Core Philosophy: Python + SQL Ergonomics in Rust**

WebRust demonstrates that **systems languages can be ergonomic** without sacrificing safety or performance.

### **1. Python-like Ranges**

```rust,ignore
# use webrust::prelude::*;
# fn example() {
// Python: for i in range(10)
for i in 0.to(10) { }

// Python: for i in range(0, 100, 5)
for i in 0.to(100).by(5) { }

// Python: for c in 'abcde'
for c in 'a'.to('f') { }

// Floats and negative steps too!
for x in 4.0.to(0.0).by(-0.5) { }
# }
```

### **2. List & Dictionary Comprehensions**

```rust,ignore
# use webrust::prelude::*;
use std::collections::HashMap;
# fn example() {

// Python: [x**2 for x in range(10)]
let squares: Vec<i32> = 0.to(10).then(|x| x * x);

// Python: [x for x in range(20) if x % 2 == 0]
let evens: Vec<i32> = 0.to(20)
    .when(|&x| x % 2 == 0)
    .then(|x| x);

// Python: {x: x**2 for x in range(5)}
let dict: HashMap<i32, i32> = 0.to(5).then(|x| (x, x * x));
# }
```

**The beauty**: These compile to standard Rust iterators — zero runtime cost!

### **3. Python String Methods**

```rust,ignore
# use webrust::prelude::*;
# fn example() {
// Python: "a,b,c".split(",")
let parts = "a,b,c".splitby(",");

// Python: "hello world".split()
let words = "hello  world".splitby("");

// Python: "L1\nL2\nL3".split("\n")
let lines = "L1\nL2\nL3".splitby("\n");

// Python: ", ".join(["a", "b", "c"])
let joined = parts.join(", ");

// Python: "hello".upper()
let upper = "hello".upper();

// Python: "hello world".title()
let title = "hello world".title();
# }
```

**One method (`splitby`) handles all split patterns** — delimiter, whitespace, lines — just like Python!

### **4. SQL-First Analytics**

```rust,ignore
# use webrust::prelude::*;
# #[gui] fn example() {
// SQL: Natural syntax for data queries
query(r#"
    -- Load CSV data
    CREATE TABLE sales AS SELECT * FROM read_csv_auto('data.csv');
    
    -- Analyze with window functions
    SELECT 
        product,
        quarter,
        revenue,
        SUM(revenue) OVER (PARTITION BY product) AS product_total,
        RANK() OVER (ORDER BY revenue DESC) AS revenue_rank
    FROM sales
    WHERE year = 2024
    ORDER BY revenue DESC;
"#);

// Inspect schema
query("SCHEMA SELECT * FROM sales");

// Aggregate and visualize
query(r#"
    SELECT 
        product,
        SUM(revenue) AS total
    FROM sales
    GROUP BY product
    ORDER BY total DESC
    LIMIT 10;
"#);
# }
```

**DuckDB + Arrow**: OLAP-grade analytics in pure Rust, zero external dependencies!

### **5. F-String Interpolation**

```rust,ignore
# use webrust::prelude::*;
# #[gui] fn example() {
let name = "Alice";
let age = 30;
let pi = std::f64::consts::PI;

println!("Hello {name}, you are {age} years old!");
println!("Next year: {age + 1}");
println!("PI ≈ {pi:.2}");                    // Format specifiers
println!("Data: {my_struct:j}");             // JSON pretty-print
println!("Compact: {my_vec:c}");             // Compact display
println!("Einstein: $(E = mc^2)");           // LaTeX rendering!
# }
```

**Processed at compile-time** — no runtime overhead!

---

## 📊 **Beyond Text: Modern Visualizations + SQL**

### **Interactive Charts (ECharts Integration)**

```rust,ignore
# use webrust::prelude::*;
use std::collections::HashMap;
# #[gui] fn example() {

// Bar chart from HashMap
let sales = HashMap::from([
    ("Q1", 120.0), ("Q2", 200.0), ("Q3", 150.0), ("Q4", 300.0)
]);
chart(&sales, "bar")
    .title("Quarterly Sales")
    .color("#2ecc71");

// Line chart from Vec
let temps = vec![64.4, 67.1, 69.8, 72.5, 70.2];
chart(&temps, "line")
    .title("Temperature Trend")
    .xlabels(vec!["Mon", "Tue", "Wed", "Thu", "Fri"]);

// Pie chart
let market_share = PieData(
    vec!["Product A".into(), "Product B".into(), "Product C".into()],
    vec![45.0, 30.0, 25.0]
);
chart(market_share, "pie").title("Market Share 2024");
# }
```

**9+ chart types**: line, bar, pie, doughnut, radar, area, scatter, gauge, funnel.

### **Smart Tables (From Any Data + SQL)**

```rust,ignore
# use webrust::prelude::*;
use std::collections::HashMap;
# #[gui] fn example() {
// From vectors
let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
table(&matrix).header(["X", "Y", "Z"]);

// 🆕 From SQL queries (auto-streamed as HTML)
query(r#"
    SELECT name, age, city
    FROM person
    ORDER BY age DESC
    LIMIT 10;
"#);  // → Renders as styled table automatically

// With LaTeX support
let physics = vec![
    ("Einstein", r"$(E = mc^2)"),
    ("Schrödinger", r"$(i\hbar\frac{\partial}{\partial t}\Psi = \hat{H}\Psi)"),
];
table(&physics).header(["Scientist", "Equation"]);
# }
```

### **Turtle Graphics & Animations**

```rust,ignore
# use webrust::prelude::*;
# #[gui] fn example() {
coord("cartesian");  // Mathematical coordinates

let turtle = object();
turtle.color("blue").width(2.0);

// Draw animated square
for _ in 0.to(4) {
    turtle.forward(100.0);
    turtle.right(90.0);
}

// Animate with easing functions
turtle.rotate(360.0).ease("elasticOut");
turtle.scale(1.5, 1.5).ease("sineInOut");

// Hierarchical object groups!
let car = group();

// Create complex parts
let wheel1 = object().at(-30.0, 0.0).circle(15.0);
let wheel2 = object().at(30.0, 0.0).circle(15.0);
let body = object().rectangle(80.0, 30.0);

// Compose them
car.add(&wheel1);
car.add(&wheel2);
car.add(&body);

// Animate the group (moves all parts together)
car.translate(200.0, 0.0).ease("linear");

// While individual parts animate independently!
wheel1.rotate(720.0).ease("linear");  // Wheels spin
wheel2.rotate(720.0).ease("linear");  // while car moves
# }
```

**20+ easing functions**: linear, sine, quad, cubic, elastic, bounce, back, expo, etc.

---

## 🏗️ **Architecture: How It Works**

### **The `#[gui]` Macro Magic**

When you write:

```rust,ignore
# use webrust::prelude::*;
#[gui(bg="navy", fg="white")]
fn main() {
    println!("Hello!");
    query("SELECT 1 as test;");
}
```

WebRust automatically:

1. **Transforms f-strings** at compile-time (`{var}` → `format!()`)
2. **Starts HTTP server** on `127.0.0.1:8080`
3. **Opens browser** automatically
4. **Serves modern UI** with MathJax, ECharts, D3.js, Two.js
5. **Initializes DuckDB** in-memory database
6. **Handles bidirectional communication** (Rust ↔ JavaScript)
7. **Auto-shuts down** when browser closes (3s timeout)

**You write Python-like Rust with SQL. WebRust handles the web and database.**

### **Zero-Cost Abstractions**

```rust,ignore
# use webrust::prelude::*;
# fn example() {
// This...
let squares: Vec<i32> = 0.to(10).then(|x| x * x);

// ...compiles to the same machine code as:
let squares: Vec<i32> = (0..10).map(|x| x * x).collect();
# }
```

**All Python-like syntax is compile-time sugar** — no runtime overhead!

### **Type-Safe Everything**

```rust,ignore
# use webrust::prelude::*;
# #[gui] fn example() {
let age: i32 = input("Age:");
let height: f64 = input("Height:");
let ok: bool = input("Confirm:");
# }
```

**Validation happens twice**:
1. **Client-side (JavaScript)** — Immediate feedback in browser
2. **Server-side (Rust)** — Type-safe parsing with helpful errors

### **SQL Performance Architecture**

```rust,ignore
// Under the hood, query() does:
// 1. Parse SQL (DuckDB parser)
// 2. Execute query (multi-threaded)
// 3. Stream results via Arrow batches
// 4. Format as HTML progressively
// 5. Send to browser incrementally

// All optimized with:
// - SIMD pattern matching
// - Zero-copy string handling
// - Thread-local buffers
// - Columnar data access
```

**Result**: Query millions of rows, display instantly, no memory bloat.

---

## 🌍 **Real-World Use Cases**

### **1. Data Analytics & Business Intelligence**

```rust,ignore
# use webrust::prelude::*;
use std::collections::HashMap;

#[gui(bg="navy", fg="white")]
fn main() {
    println!("@(cyan, bold)📊 Sales Analytics Dashboard");
    
    // 🆕 Load and analyze CSV data with SQL
    query(r#"
        CREATE TABLE sales AS SELECT * FROM read_csv_auto('sales.csv');
        
        -- Regional breakdown
        SELECT 
            region,
            SUM(amount) AS total_sales,
            COUNT(DISTINCT customer_id) AS unique_customers,
            AVG(amount) AS avg_transaction
        FROM sales
        WHERE date >= '2024-01-01'
        GROUP BY region
        ORDER BY total_sales DESC;
    "#);
    
    // 🆕 Time series analysis with window functions
    query(r#"
        SELECT 
            DATE_TRUNC('month', date) AS month,
            SUM(amount) AS monthly_total,
            SUM(SUM(amount)) OVER (
                ORDER BY DATE_TRUNC('month', date)
                ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
            ) AS running_total
        FROM sales
        GROUP BY month
        ORDER BY month;
    "#);
    
    // Grid-based dashboard
    grid(2, 2);
    
    // Top products chart
    let (x, y) = cell(0, 0, "center");
    let products = query_to_hashmap(r#"
        SELECT product, SUM(amount) AS total
        FROM sales GROUP BY product ORDER BY total DESC LIMIT 10
    "#);
    chart(&products, "bar")
        .title("Top 10 Products")
        .at(x, y)
        .size(*CW * 95 / 100, *CH * 90 / 100);
    
    // Trend line
    let (x, y) = cell(0, 1, "center");
    let monthly_data = query_to_vec("SELECT month, total FROM ...");
    chart(&monthly_data, "line")
        .title("Monthly Revenue Trend")
        .at(x, y)
        .size(*CW * 95 / 100, *CH * 90 / 100);
}
```

**Perfect for**: Business reports, data exploration, executive dashboards, KPI monitoring.

**Why WebRust Wins**: One file, SQL + charts, zero database setup, instant results.

### **2. Education & Data Science Teaching**

```rust,ignore
# use webrust::prelude::*;
#[gui]
fn main() {
    println!("@(blue, bold)📚 SQL Tutorial: Joins & Aggregations\n");
    
    // Create sample data
    query(r#"
        CREATE TABLE students (id INT, name TEXT, major TEXT);
        CREATE TABLE grades (student_id INT, course TEXT, grade INT);
        
        INSERT INTO students VALUES 
            (1, 'Alice', 'CS'), (2, 'Bob', 'Math'), (3, 'Charlie', 'CS');
        
        INSERT INTO grades VALUES
            (1, 'Algorithms', 95), (1, 'Databases', 88),
            (2, 'Calculus', 92), (3, 'Algorithms', 78);
    "#);
    
    println!("@(green)Step 1: Simple JOIN");
    query(r#"
        SELECT s.name, g.course, g.grade
        FROM students s
        JOIN grades g ON s.id = g.student_id
        ORDER BY s.name, g.grade DESC;
    "#);
    
    println!("\n@(green)Step 2: Aggregation with GROUP BY");
    query(r#"
        SELECT 
            s.name,
            AVG(g.grade) AS average_grade,
            COUNT(*) AS num_courses
        FROM students s
        JOIN grades g ON s.id = g.student_id
        GROUP BY s.name
        ORDER BY average_grade DESC;
    "#);
    
    println!("\n@(green)Step 3: Window Functions");
    query(r#"
        SELECT 
            name,
            course,
            grade,
            AVG(grade) OVER (PARTITION BY name) AS student_avg,
            RANK() OVER (ORDER BY grade DESC) AS overall_rank
        FROM students s
        JOIN grades g ON s.id = g.student_id;
    "#);
    
    // Visualize grade distribution
    let grade_dist = query_to_vec("SELECT grade FROM grades");
    chart(&grade_dist, "bar").title("Grade Distribution");
}
```

**Perfect for**: SQL education, data science courses, interactive textbooks, workshops.

**Why WebRust Wins**: Students see queries + results + visualizations instantly.

### **3. Scientific Computing with SQL**

```rust,ignore
# use webrust::prelude::*;
#[gui]
fn main() {
    println!("@(purple, bold)🔬 Experimental Data Analysis\n");
    
    // Load experimental measurements
    query(r#"
        CREATE TABLE measurements AS 
        SELECT * FROM read_csv_auto('experiment_data.csv');
        
        -- Statistical summary
        SELECT 
            experiment_id,
            COUNT(*) AS sample_size,
            AVG(value) AS mean,
            STDDEV(value) AS std_dev,
            MIN(value) AS min_value,
            MAX(value) AS max_value
        FROM measurements
        GROUP BY experiment_id;
    "#);
    
    // Outlier detection with window functions
    query(r#"
        WITH stats AS (
            SELECT 
                *,
                AVG(value) OVER (PARTITION BY experiment_id) AS mean,
                STDDEV(value) OVER (PARTITION BY experiment_id) AS std_dev
            FROM measurements
        )
        SELECT 
            experiment_id,
            timestamp,
            value,
            CASE 
                WHEN ABS(value - mean) > 3 * std_dev THEN 'Outlier'
                ELSE 'Normal'
            END AS classification
        FROM stats
        WHERE classification = 'Outlier';
    "#);
    
    // Time series visualization
    let timeseries = query_to_vec(r#"
        SELECT timestamp, AVG(value) AS avg_value
        FROM measurements
        GROUP BY timestamp
        ORDER BY timestamp
    "#);
    chart(&timeseries, "line").title("Measurement Time Series");
    
    // LaTeX formulas
    println!(r"Statistical significance: $(p < 0.05)");
    println!(r"Standard error: $(SE = \frac{\sigma}{\sqrt{n}})");
}
```

**Perfect for**: Lab data analysis, research notebooks, experimental reports.

**Why WebRust Wins**: SQL analytics + LaTeX + charts in one environment.

### **4. Log Analysis & Monitoring**

```rust,ignore
# use webrust::prelude::*;
#[gui]
fn main() {
    println!("@(red, bold)🔍 Server Log Analysis\n");
    
    // Parse and analyze logs
    query(r#"
        CREATE TABLE logs AS 
        SELECT * FROM read_csv_auto('server_logs.csv');
        
        -- Error rate by hour
        SELECT 
            DATE_TRUNC('hour', timestamp) AS hour,
            COUNT(*) AS total_requests,
            SUM(CASE WHEN status >= 400 THEN 1 ELSE 0 END) AS errors,
            ROUND(100.0 * SUM(CASE WHEN status >= 400 THEN 1 ELSE 0 END) / COUNT(*), 2) AS error_rate
        FROM logs
        WHERE timestamp >= NOW() - INTERVAL 24 HOURS
        GROUP BY hour
        ORDER BY hour DESC;
    "#);
    
    // Top error endpoints
    query(r#"
        SELECT 
            endpoint,
            status,
            COUNT(*) AS occurrences
        FROM logs
        WHERE status >= 400
        GROUP BY endpoint, status
        ORDER BY occurrences DESC
        LIMIT 10;
    "#);
    
    // Response time percentiles
    query(r#"
        SELECT 
            endpoint,
            PERCENTILE_CONT(0.50) WITHIN GROUP (ORDER BY response_time) AS p50,
            PERCENTILE_CONT(0.95) WITHIN GROUP (ORDER BY response_time) AS p95,
            PERCENTILE_CONT(0.99) WITHIN GROUP (ORDER BY response_time) AS p99
        FROM logs
        GROUP BY endpoint
        ORDER BY p99 DESC;
    "#);
}
```

**Perfect for**: DevOps dashboards, incident analysis, performance monitoring.

**Why WebRust Wins**: Parse logs, run SQL, visualize alerts — all in real-time.

---

<a id="getting-started"></a>
<a id="sec-getting-started"></a>
## 🚀 **Getting Started**

### **Installation**

```toml
[dependencies]
webrust = "1.3.0"
```

> **💡 First Build Note**: The initial compilation includes DuckDB's analytical engine, 
> which takes 5-10 minutes. This is a one-time cost — incremental builds are fast, and 
> you gain native SQL analytics with zero runtime dependencies.

### **Your First App (30 seconds)**

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    let name: String = input("What's your name?");
    println!("Hello, {name}! 🎉");
    
    // 🆕 NEW: SQL analytics built-in!
    query("SELECT 'WebRust' AS framework, 1.3 AS version;");
    
    let nums = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    chart(&nums, "line").title("My First Chart");
}
```

**Run**: `cargo run` → Browser opens → Professional UI with SQL support.

---

## 📚 **Rich Examples**

WebRust includes comprehensive examples demonstrating every feature:

```bash
cargo run --example simpleio      # Inputs, styling, positioning
cargo run --example latex         # LaTeX mathematical notation
cargo run --example advancedio    # Advanced I/O, tables, formatting
cargo run --example string        # Python-like string methods
cargo run --example utils         # Ranges, enumerate, comprehensions
cargo run --example table         # Table generation & formatting
cargo run --example chart         # All 9+ chart types
cargo run --example sql           # 🆕 SQL analytics with DuckDB!
cargo run --example turtle        # Turtle graphics & animations
cargo run --example mixed         # Complete dashboard with grid layout
```

Each example is extensively commented and demonstrates best practices.

**🆕 Must-see `py_sql`**: Shows the full power of DuckDB integration with:
- CREATE/INSERT/SELECT/JOIN operations
- Window functions and aggregates
- CSV loading and JSON processing
- Running totals and rankings
- Schema inspection
- UDF (macro) definitions

---

## 🛠️ **Development Notes**

### **Disk Space Requirements**

WebRust integrates DuckDB and other native libraries, resulting in larger build artifacts:
- **Debug builds**: ~15-20 GB (includes full debug symbols)
- **Release builds**: ~5-8 GB (optimized)
- **Final binary**: ~80-120 MB

**To manage disk space**:
```bash
# Clean release builds (keep debug for development)
cargo clean --release

# Full clean when needed
cargo clean
```

### **Recommended Build Configuration**

Add to your `Cargo.toml` for smaller builds:
```toml
[profile.release]
strip = true  # Reduces binary size by ~30%
lto = "thin"  # Better optimization
```

---

## 🎯 **WebRust's Vision for Programming's Future**

## 🎯 **WebRust's Vision for Programming's Future**

### **1. Syntax Should Be Universal**

Good ideas shouldn't be language-specific. If Python's `range(10)` is intuitive, why can't Rust have `0.to(10)`? If SQL's declarative queries work, why not integrate them natively?

**WebRust proves syntax can evolve without breaking performance or safety.**

### **2. Ecosystems Should Talk**

Python developers bring 30 years of ergonomic patterns. Rust brings safety and speed. SQL brings declarative data power. These communities should learn from each other, not stay isolated.

**WebRust builds bridges, not walls.**

### **3. Defaults Should Be Modern**

In 2025, "Hello, World!" shouldn't be monochrome terminal text. Data analysis shouldn't require three tools. It should be:
- ✅ **Styled** (colors, fonts, layouts)
- ✅ **Interactive** (inputs, buttons, charts)
- ✅ **Analytical** (SQL queries, aggregations)
- ✅ **Shareable** (runs in browser, URL-accessible)
- ✅ **Visual** (supports math, graphics, animations)

**WebRust makes the modern default effortless.**

### **4. Performance Should Be Invisible**

Users shouldn't think about performance. Fast should be the default:
- ✅ Sub-microsecond f-string compilation
- ✅ Zero-copy string handling
- ✅ SIMD-optimized rendering
- ✅ Arrow columnar processing
- ✅ Multi-threaded SQL execution

**WebRust makes speed automatic.**

### **5. Complexity Should Be Optional**

Want to make a simple script? Use `println()`.
Want to add styling? Chain `.color()`.
Want a chart? Call `chart()`.
Want SQL analytics? Call `query()`.
Want animations? Add `.ease()`.

**Each layer of complexity is opt-in**, not mandatory.

---

## 🔮 **What's Next?**

WebRust is actively evolving with:

- 🎨 **More chart types** (sankey, treemap, 3D plots, heatmaps)
- 🗄️ **Persistent databases** (SQLite, PostgreSQL connectors)
- 🧩 **Component system** (reusable UI widgets)
- 🌐 **Static export** (generate standalone HTML)
- 📱 **Mobile optimization** (responsive by default)
- 🔌 **Plugin ecosystem** (community extensions)
- 🌍 **i18n support** (multi-language UIs)
- ⚡ **WebAssembly target** (run in-browser)

**Join us in reimagining what programming can be.**

---

## 🤝 **Contributing**

WebRust welcomes contributions! Whether you:

- 🐛 Found a bug → [Open an issue](https://github.com/gerarddubard/webrust/issues)
- 💡 Have an idea → [Start a discussion](https://github.com/gerarddubard/webrust/discussions)
- 📝 Want to improve docs → PRs welcome!
- 🎨 Built something cool → [Share it](https://github.com/gerarddubard/webrust/discussions/show-and-tell)

**Community principles**:
- Keep it Pythonic (readable, intuitive)
- Keep it Rusty (safe, fast, expressive)
- Keep it analytical (SQL-powered)
- Keep it simple (zero-config, batteries-included)

---

## 📄 **License**

MIT License — see [LICENSE](https://github.com/gerarddubard/webrust/blob/main/LICENSE) for details.

---

## 🙏 **Acknowledgments**

WebRust stands on the shoulders of giants:

- **Python community** — For showing us ergonomics matter
- **Rust community** — For proving safety and speed coexist
- **SQL community** — For decades of declarative data wisdom
- **Web standards** — For creating the universal platform
- **Open source** — For enabling collaboration

Special thanks to the creators of:
- [DuckDB](https://duckdb.org/) — Blazing-fast analytical database
- [Apache Arrow](https://arrow.apache.org/) — Columnar data standard
- [tiny_http](https://crates.io/crates/tiny_http) — Simple HTTP server
- [serde](https://crates.io/crates/serde) — Serialization framework
- [MathJax](https://www.mathjax.org/) — Beautiful math rendering
- [ECharts](https://echarts.apache.org/) — Powerful charting library
- [Two.js](https://two.js.org/) — 2D drawing API

---

## 🌟 **The Bottom Line**

**It's 2025.** We have the technology to make programming:
- More intuitive (Python-like syntax)
- More powerful (SQL analytics)
- More safe (Rust's type system)
- More visual (browser-based UIs)
- More performant (SIMD optimization)
- More accessible (zero configuration)

**WebRust proves it's possible.** We're not asking you to choose between Python and Rust, between simplicity and performance, between terminal and web, between code and data.

**We're showing you can have it all.**

---

<div style="text-align:center">

### 🦀 **Write Python. Query SQL. Think Rust. Ship Web.** 🌐

**[Get Started Now](https://docs.rs/webrust)** | **[View Examples](https://github.com/gerarddubard/webrust/tree/main/examples)** | **[Join Discussion](https://github.com/gerarddubard/webrust/discussions)**

---

*Made with ❤️ for developers who believe programming should be joyful, powerful, and fast*

</div>
