# WebRust v0.9.0 - Interactive Data Visualization Framework

**Python-like simplicity meets Rust performance with integrated business intelligence capabilities**

WebRust transforms Rust development by providing Python's elegance with zero-cost abstractions, now featuring interactive data visualization, professional table generation, and complete business intelligence workflows.

## Mission

WebRust aims to **lower the barrier to Rust adoption** by providing familiar Python-like ergonomics while maintaining Rust's performance and safety guarantees. Our goal is to demonstrate that systems programming can be both powerful and approachable.

By bridging the gap between Python's ease of use and Rust's capabilities, WebRust serves as both a practical development tool and a proof-of-concept for ergonomic systems programming paradigms.

## What's New in v0.9.0

### Interactive Data Visualization
- **Professional Charts** - Line, bar, pie, and scatter plots with ECharts integration
- **Seamless Integration** - Charts work directly with tables and GUI system
- **Business Intelligence** - Complete dashboard creation capabilities
- **Real-time Rendering** - Interactive charts with hover effects and zoom

### Complete Analysis Workflow

use webrust::prelude::*;

#[gui]
fn main() {
    let sales = HashMap::from([("Q1", 85.0), ("Q2", 92.0), ("Q3", 78.0), ("Q4", 96.0)]);
    
    // Professional table analysis
    table(&sales).header(["Quarter", "Revenue (k$)"]);
    
    // Interactive visualization  
    chart(&sales, "bar")
        .title("Quarterly Performance")
        .color("#2ecc71");
}


## Core Features

### Zero-Configuration GUI
Transform any Rust function into a professional web application:


#[gui(bg = "navy", fg = "white", font = "Arial")]
fn main() {
    println("Hello, WebRust!");
}


### Professional Styling
CSS-like styling with responsive layouts:


println("BUSINESS DASHBOARD")
    .width(*CW)                    // Dynamic width
    .align("center")               // Center alignment
    .weight(4)                     // Border thickness
    .style("double")               // Border style
    .radius(8)                     // Rounded corners
    .background("navy");           // Background color


### Smart Data Visualization
Automatic table generation from any data structure:


let mut scores = HashMap::new();
scores.insert("Alice", 95);
scores.insert("Bob", 87);

table(&scores).header(["Student", "Score"]);


### Python-like Strings
Familiar string methods with Rust performance:


let result = "python,rust,go".split_by(",").join(" → ");  // "python → rust → go"
let clean = "  hello world  ".trim().title();             // "Hello World"
let padded = "42".zfill(6);                               // "000042"


### Mathematical Expressions
LaTeX integration for scientific computing:


println("Einstein's equation: $(E = mc^2)$");
println("Quadratic formula: $(x = \\frac{-b \\pm \\sqrt{b^2-4ac}}{2a})$");


## Business Intelligence Example

Create professional dashboards with minimal code:


use webrust::prelude::*;
use std::collections::HashMap;

#[gui(bg = "grey", fg = "white", font = "Courier New")]
fn main() {
    // Executive header
    println("QUARTERLY BUSINESS REVIEW")
        .width(*CW).align("center");
    
    // Revenue analysis
    let revenue = HashMap::from([
        ("Q1 2024", 85.0), ("Q2 2024", 92.0), 
        ("Q3 2024", 78.0), ("Q4 2024", 96.0)
    ]);
    
    // Detailed table
    table(&revenue).header(["Quarter", "Revenue (k$)"]);
    
    // Interactive chart
    chart(&revenue, "bar")
        .title("Quarterly Revenue Performance")
        .x_axis_label("Quarter")
        .y_axis_label("Revenue (k$)")
        .color("#2ecc71");
    
    // Regional breakdown
    let regions = vec![
        ("North", 25.0, 28.0, 22.0, 30.0, 105.0),
        ("South", 30.0, 32.0, 28.0, 35.0, 125.0),
        ("East", 20.0, 22.0, 18.0, 25.0, 85.0),
        ("West", 15.0, 18.0, 16.0, 20.0, 69.0),
    ];
    
    table(&regions).header(["Region", "Q1", "Q2", "Q3", "Q4", "Total"]);
    
    // Market share visualization
    let market_share = PieData(
        vec!["North".to_string(), "South".to_string(), "East".to_string(), "West".to_string()],
        vec![105.0, 125.0, 85.0, 69.0]
    );
    chart(market_share, "pie").title("Regional Market Share");
}


## Quick Start

### 1. Add WebRust to your project:
toml
[dependencies]
webrust = "0.9.0"
serde = "1.0.219"
serde_json = "1.0.143"

### 2. Create your first interactive dashboard:

use webrust::prelude::*;

#[gui]
fn main() {
    let data = vec![10.0, 20.0, 15.0, 25.0];
    table(&data).header(["Q1", "Q2", "Q3", "Q4"]);
    chart(&data, "line").title("Growth Trend");
}


### 3. Run and enjoy:
bash
cargo run


Your browser opens automatically with an interactive dashboard!

## Chart Types

### Line Charts
Perfect for trends and time series:

chart(&monthly_data, "line")
    .title("Revenue Trend")
    .x_axis_label("Month")
    .y_axis_label("Revenue ($)")
    .series_name("Monthly Revenue")
    .color("#3498db");


### Bar Charts
Great for comparisons:

chart(&category_data, "bar")
    .title("Sales by Category")
    .color("#2ecc71");


### Pie Charts
Ideal for proportions:

let pie_data = PieData(
    vec!["Desktop".to_string(), "Mobile".to_string()],
    vec![60.0, 40.0]
);
chart(pie_data, "pie").title("Traffic Sources");


### Scatter Plots
Perfect for correlations:

chart(&correlation_data, "scatter")
    .title("Price vs Quality")
    .x_axis_label("Quality Score")
    .y_axis_label("Price ($)")
    .color("#e74c3c");


## Professional Styling

WebRust provides complete control over visual presentation:

### Borders and Layout

println("Professional Content")
    .width(*CW / 2)              // Responsive width
    .align("justify")            // Text alignment
    .weight(3)                   // Border thickness (1-5)
    .style("dashed")             // Border style
    .radius(12)                  // Rounded corners
    .color("darkblue")           // Border color
    .background("lightcyan");    // Background


### Color System

println("Error: Something went wrong!");
println("Success: Data saved!");
println("Info: Processing...");


## Smart Tables

Automatic table generation with professional formatting:


// From vectors
let data = vec![("Alice", 95), ("Bob", 87), ("Charlie", 92)];
table(&data).header(["Name", "Score"]);

// From HashMaps
let mut scores = HashMap::new();
scores.insert("Math", 95);
scores.insert("Science", 88);
table(&scores);

// Pivot tables
let sales_data = vec![
    ("North", 25.0, 28.0, 22.0, 30.0),
    ("South", 30.0, 32.0, 28.0, 35.0),
];
table(&sales_data).header(["Region", "Q1", "Q2", "Q3", "Q4"]);


## String Processing

Python-compatible string methods with Rust performance:


// Smart splitting - one method, multiple patterns
let csv = "name,age,city".split_by(",");           // Comma-separated
let words = "hello  world\ttab".split_by("");      // Whitespace
let lines = "L1\nL2\nL3".split_by("\n");          // Line breaks

// Case transformations
let text = "hello world";
println!("{}", text.upper());       // "HELLO WORLD"
println!("{}", text.title());       // "Hello World"
println!("{}", text.capitalize());  // "Hello world"

// Padding and formatting
println!("{}", "42".zfill(6));              // "000042"
println!("{}", "text".center(10, '*'));     // "***text***"
println!("{}", "left".ljust(10, '-'));      // "left------"

// Validation
println!("{}", "hello".isalpha());          // true
println!("{}", "12345".isdigit());          // true
println!("{}", "user@host.com".endswith(".com")); // true


## Scientific Computing

LaTeX integration for mathematical expressions:


#[gui]
fn main() {
    println("Fundamental equations:");
    println("• Newton's second law: $(F = ma)$");
    println("• Schrödinger equation: $(i\\hbar\\frac{\\partial}{\\partial t}\\Psi = \\hat{H}\\Psi)$");
    println("• Maxwell's equations: $(\\nabla \\cdot \\mathbf{E} = \\frac{\\rho}{\\varepsilon_0})$");
    
    // Experimental data visualization
    let experimental_results = vec![2.1, 4.3, 3.8, 5.1, 4.9, 6.2];
    chart(&experimental_results, "scatter")
        .title("Experimental Data Analysis")
        .x_axis_label("Trial Number")
        .y_axis_label("Measurement")
        .color("#9b59b6");
}


## Installation & Setup

### Prerequisites
- Rust 1.70+
- Web browser (Chrome, Firefox, Safari, Edge)

### Add to Cargo.toml
toml
[dependencies]
webrust = "0.9.0"


### Optional Features
toml
[dependencies]
webrust = { version = "0.9.0", features = ["full"] }


## Real-World Applications

### Financial Dashboard

#[gui(bg = "navy", fg = "white")]
fn main() {
    let portfolio = HashMap::from([
        ("AAPL", 15000.0), ("GOOGL", 12000.0), ("MSFT", 8000.0)
    ]);
    
    table(&portfolio).header(["Stock", "Value ($)"]);
    chart(&portfolio, "pie").title("Portfolio Allocation");
}


### Scientific Analysis

#[gui]
fn main() {
    let temperature_data = vec![20.1, 21.5, 19.8, 22.3, 20.9];
    
    table(&temperature_data).header(["Day 1", "Day 2", "Day 3", "Day 4", "Day 5"]);
    chart(&temperature_data, "line")
        .title("Temperature Monitoring")
        .y_axis_label("Temperature (°C)")
        .color("#e67e22");
}


### Sales Analytics

#[gui]
fn main() {
    let monthly_sales = HashMap::from([
        ("Jan", 45000), ("Feb", 52000), ("Mar", 48000),
        ("Apr", 61000), ("May", 59000), ("Jun", 67000)
    ]);
    
    table(&monthly_sales).header(["Month", "Sales ($)"]);
    chart(&monthly_sales, "bar")
        .title("Monthly Sales Performance")
        .color("#27ae60");
}


## Architecture

WebRust v0.9.0 features a modular architecture:

- **Core GUI** (`webrust::io::gui`) - Web server and browser management
- **Data Visualization** (`webrust::data::chart`) - Interactive chart generation
- **Table System** (`webrust::data::table`) - Intelligent table formatting
- **String Processing** (`webrust::data::string`) - Python-like string methods
- **Styling Engine** (`webrust::io::print`) - Professional layout and design
- **Macro System** (`webrust-macros`) - F-string processing and GUI transformation

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

WebRust is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## WebRust v0.9.0 - Where Python Meets Rust Performance

**Experience the perfect fusion of Python's elegance and Rust's power with professional business intelligence capabilities.**

---

*Built with care for the Rust community*
