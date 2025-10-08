<!-- Stable anchors for IDEs: defined before any links -->
<a id="sec-getting-started"></a><a name="sec-getting-started"></a><span id="sec-getting-started"></span>
<a id="sec-core-philosophy"></a><a name="sec-core-philosophy"></a><span id="sec-core-philosophy"></span>

# 🚀 WebRust — **Python Meets Rust Meets Web**

### *The Revolutionary Framework Bridging Two Ecosystems*

<div style="text-align:center">

[![WebRust](https://img.shields.io/badge/🦀_WebRust-1.2.0-ff6b35?style=for-the-badge&labelColor=000&logoColor=white)](https://github.com/gerarddubard/webrust)
[![Rust](https://img.shields.io/badge/Rust-1.70+-000?style=for-the-badge&logo=rust)](https://rust-lang.org)
[![Web Ready](https://img.shields.io/badge/Web-Ready-4285f4?style=for-the-badge&logo=googlechrome&logoColor=white)](https://docs.rs/webrust)
[![Python Style](https://img.shields.io/badge/Python-Style-3776ab?style=for-the-badge&logo=python&logoColor=white)](https://crates.io/crates/webrust)

**🔥 Write like Python. Run like Rust. Deploy as Web. Zero configuration.**

[Documentation](https://docs.rs/webrust) | [Examples](https://github.com/gerarddubard/webrust/tree/main/examples) | [Crates.io](https://crates.io/crates/webrust)


<!-- IDE-friendly explicit anchors -->
<a id="getting-started"></a><a name="getting-started"></a><span id="getting-started"></span>
<a id="core-philosophy"></a><a name="core-philosophy"></a><span id="core-philosophy"></span>

</div>

---

## 🌟 **Why WebRust? A Manifesto for 2025**

### **The Great Divide in Programming**

The programming world is split between two philosophies:

**Python's camp** says: *"Life is short, use Python"* — prioritizing developer happiness, rapid prototyping, and readable syntax.

**Rust's camp** says: *"Performance and safety first"* — prioritizing zero-cost abstractions, memory safety, and compile-time guarantees.

**WebRust's answer**: *"Why choose? Have both."*

### **Why We're Still Stuck in the Console in 2025**

It's 2025. We have:
- 🚀 Blazingly fast computers with multi-core processors
- 🎨 Beautiful displays with millions of colors
- 🌐 Universal web browsers on every device
- 🧠 AI models running in real-time

Yet most educational tools, data analysis scripts, and prototypes still output to a monochrome terminal window invented in the 1970s.

**Why?** Because creating a proper GUI requires:
- Learning HTML/CSS/JavaScript (separate skillset)
- Managing web servers and deployment
- Writing boilerplate for state management
- Dealing with frontend/backend separation

**WebRust's philosophy**: Your code *is* the interface. The browser *is* your canvas. Forget the terminal.

### **A Bridge Between Two Worlds**

WebRust isn't just a library — it's a **proposal for the future of programming languages**:

1. **Syntax Evolution**: Languages should embrace ergonomics without sacrificing performance
2. **Ecosystem Integration**: Python developers should feel at home in Rust, and vice versa
3. **Modern Defaults**: In 2025, the default output shouldn't be plain text — it should be styled, interactive, and visual
4. **Zero-Configuration Philosophy**: Great tools should work out of the box

WebRust proves these aren't mutually exclusive goals. You can have Python's elegance with Rust's speed, running in a modern web interface, with zero setup.

---

## 🎯 **The Problem: Three Painful Paths**

### **Path 1: Terminal Applications (1970s Technology)**

```rust,ignore
use std::io;
println!(\"What's your name?\");
let mut input = String::new();
io::stdin().read_line(&mut input).expect(\"Failed to read input\");
let name = input.trim();
println!(\"Hello, {}!\", name);
```

**Reality Check**:
- ❌ No colors, no styling, no interactivity
- ❌ Can't embed charts, tables, or visualizations
- ❌ No mathematical notation support
- ❌ Copy-paste is painful
- ❌ Screenshots look unprofessional

**In 2025, this is like using a typewriter when you have a smartphone.**

### **Path 2: Web Frameworks (Complexity Explosion)**

```rust,ignore
use rocket::*;

#[get(\"/\")]
fn index() -> &'static str {
    \"<html><body>Hello!</body></html>\"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(\"/\", routes![index])
}
```

**Reality Check**:
- ❌ Need to learn 3 languages (HTML/CSS/JS)
- ❌ Separate frontend and backend logic
- ❌ State management becomes complex
- ❌ Deployment requires infrastructure
- ❌ Just wanted to print styled text!

**In 2025, this is like building a car when you just want to go to the store.**

### **Path 3: Desktop GUI Frameworks (Framework Lock-in)**

```rust,ignore
use eframe::egui;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(\"My App\");
            if ui.button(\"Click\").clicked() {
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

## ✨ **The WebRust Solution: Elegance Meets Power**

### **Same Problem, WebRust Way**

```rust,ignore
use webrust::prelude::*;

#[gui(bg=\"navy\", fg=\"white\", font=\"Courier New\")]
fn main() {
    println!(\"@(cyan, bold, italic)🎯 WebRust Personal Information\");
    
    let first_name: String = input(\"Your first name:\");
    let last_name: String = input(\"Your last name:\");
    let age: i32 = input(\"Your age:\");
    let height: f64 = input(\"Your height (meters):\");
    let married: bool = input(\"Married? (true/false):\");
    
    println!(\"@(white)Hello, @(green, bold){first_name} @(red, bold){last_name}!\");
    println!(\"You are @(yellow){age}@() years old and @(blue){height}@() m tall.\");
    
    // Show live status
    print(\"@(white)● {first_name} online\")
        .at(-20.0, 10.0)
        .background(\"green\")
        .radius(10)
        .sticky();
}
```

**What Just Happened?**

✅ **Python-like syntax** — `0.to(10)`, `.when()/.then()`, `.splitby()`  
✅ **Styled output** — Colors, fonts, positions with chainable API  
✅ **Type-safe inputs** — Real-time validation in the browser  
✅ **Automatic web UI** — Browser opens, server runs, zero config  
✅ **Rust performance** — Compiles to native code, runs blazingly fast  

**Run it**: `cargo run` → Browser opens automatically with a beautiful, styled interface.

---

<a id="core-philosophy"></a>
<a id="sec-core-philosophy"></a>
## 🧩 **Core Philosophy: Python Ergonomics in Rust**

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
// Python: \"a,b,c\".split(\",\")
let parts = \"a,b,c\".splitby(\",\");

// Python: \"hello world\".split()
let words = \"hello  world\".splitby(\"\");

// Python: \"L1\\nL2\\nL3\".split(\"\\n\")
let lines = \"L1\\nL2\\nL3\".splitby(\"\\n\");

// Python: \", \".join([\"a\", \"b\", \"c\")
let joined = parts.join(\", \");

// Python: \"hello\".upper()
let upper = \"hello\".upper();

// Python: \"hello world\".title()
let title = \"hello world\".title();
# }
```

**One method (`splitby`) handles all split patterns** — delimiter, whitespace, lines — just like Python!

### **4. F-String Interpolation**

```rust,ignore
# use webrust::prelude::*;
# #[gui] fn example() {
let name = \"Alice\";
let age = 30;
let pi = std::f64::consts::PI;

println!(\"Hello {name}, you are {age} years old!\");
println!(\"Next year: {age + 1}\");
println!(\"PI ≈ {pi:.2}\");                    // Format specifiers
println!(\"Data: {my_struct:j}\");             // JSON pretty-print
println!(\"Compact: {my_vec:c}\");             // Compact display
println!(\"Einstein: $(E = mc^2)\");           // LaTeX rendering!
# }
```

**Processed at compile-time** — no runtime overhead!

---

## 📊 **Beyond Text: Modern Visualizations**

### **Interactive Charts (ECharts Integration)**

```rust,ignore
# use webrust::prelude::*;
use std::collections::HashMap;
# #[gui] fn example() {

// Bar chart from HashMap
let sales = HashMap::from([
    (\"Q1\", 120.0), (\"Q2\", 200.0), (\"Q3\", 150.0), (\"Q4\", 300.0)
]);
chart(&sales, \"bar\")
    .title(\"Quarterly Sales\")
    .color(\"#2ecc71\");

// Line chart from Vec
let temps = vec![64.4, 67.1, 69.8, 72.5, 70.2];
chart(&temps, \"line\")
    .title(\"Temperature Trend\")
    .xlabels(vec![\"Mon\", \"Tue\", \"Wed\", \"Thu\", \"Fri\"]);

// Pie chart
let market_share = PieData(
    vec![\"Product A\".into(), \"Product B\".into(), \"Product C\".into()],
    vec![45.0, 30.0, 25.0]
);
chart(market_share, \"pie\").title(\"Market Share 2024\");
# }
```

**9+ chart types**: line, bar, pie, doughnut, radar, area, scatter, gauge, funnel, heatmap, candlestick, bubble.

### **Smart Tables (From Any Data)**

```rust,ignore
# use webrust::prelude::*;
use std::collections::HashMap;
# #[gui] fn example() {
// From vectors
let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
table(&matrix).header([\"X\", \"Y\", \"Z\"]);

// From nested structures (auto-flattened!)
let mut cities = HashMap::new();
cities.insert(\"Paris\", HashMap::from([
    (\"population\", \"2.2M\"),
    (\"country\", \"France\")
]));
table(&cities);  // Automatically formatted

// With LaTeX support
let physics = vec![
    (\"Einstein\", r\"$(E = mc^2)\"),
    (\"Schrödinger\", r\"$(i\\hbar\\frac{\\partial}{\\partial t}\\Psi = \\hat{H}\\Psi)\"),
];
table(&physics).header([\"Scientist\", \"Equation\"]);
# }
```

### **Turtle Graphics & Animations**

```rust,ignore
# use webrust::prelude::*;
# #[gui] fn example() {
coord(\"cartesian\");  // Mathematical coordinates

let turtle = object();
turtle.color(\"blue\").width(2.0);

// Draw animated square
for _ in 0.to(4) {
    turtle.forward(100.0);
    turtle.right(90.0);
}

// Animate with easing functions
turtle.rotate(360.0).ease(\"elasticOut\");
turtle.scale(1.5, 1.5).ease(\"sineInOut\");

// 🆕 NEW in 1.2.0: Hierarchical object groups!
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
car.translate(200.0, 0.0).ease(\"linear\");

// While individual parts animate independently!
wheel1.rotate(720.0).ease(\"linear\");  // Wheels spin
wheel2.rotate(720.0).ease(\"linear\");  // while car moves
# }
```

**20+ easing functions**: linear, sine, quad, cubic, elastic, bounce, back, expo, etc.

**🆕 Physics-based animations**: Synchronize rotation with translation for realistic rolling motion!

---

## 🏗️ **Architecture: How It Works**

### **The `#[gui]` Macro Magic**

When you write:

```rust,ignore
# use webrust::prelude::*;
#[gui(bg=\"navy\", fg=\"white\")]
fn main() {
    println!(\"Hello!\");
}
```

WebRust automatically:

1. **Transforms f-strings** at compile-time (`{var}` → `format!()`)
2. **Starts HTTP server** on `127.0.0.1:8080`
3. **Opens browser** automatically
4. **Serves modern UI** with MathJax, ECharts, D3.js
5. **Handles bidirectional communication** (Rust ↔ JavaScript)
6. **Auto-shuts down** when browser closes (3s timeout)

**You write Python-like Rust. WebRust handles the web.**

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
let age: i32 = input(\"Age:\");
let height: f64 = input(\"Height:\");
let ok: bool = input(\"Confirm:\");
# }
```

**Validation happens twice**:
1. **Client-side (JavaScript)** — Immediate feedback in browser
2. **Server-side (Rust)** — Type-safe parsing with helpful errors

---

## 🎨 **Professional Styling Out of the Box**

### **Inline Color Syntax**

```rust,ignore
# use webrust::prelude::*;
# #[gui] fn example() {
println!(\"@(red, bold)Error:@() File not found\");
println!(\"@(green)Success@() — Operation completed\");
println!(\"@(blue, italic)Info:@() Processing...\");
# }
```

### **CSS-Inspired Chainable API**

```rust,ignore
# use webrust::prelude::*;
# #[gui] fn example() {
println!(\"Important Notice\")
    .weight(3)              // Border thickness
    .color(\"navy\")        // Border color
    .background(\"yellow\") // Background
    .radius(10)             // Rounded corners
    .align(\"center\")      // Text alignment
    .width(400)             // Fixed width
    .at(100.0, 50.0);       // Absolute position
# }
```

### **Grid-Based Layouts**

```rust,ignore
# use webrust::prelude::*;
# #[gui] fn example() {
# let data1 = vec![1.0, 2.0]; let data2 = vec![vec![1, 2]];
grid(3, 4);  // 3 rows × 4 columns

let (x, y) = cell(0, 0, \"center\");
chart(&data1, \"bar\").at(x, y);

let (x, y) = cell(1, 2, \"top left\");
table(&data2).at(x, y);

let (x, y) = cell(2, 3, \"bottom right\");
println!(\"Footer\").at(x, y);
# }
```

**Build dashboards in minutes**, not hours.

---

## 🌍 **Real-World Use Cases**

### **1. Education & Teaching**

```rust,ignore
# use webrust::prelude::*;
#[gui]
fn main() {
    println!(\"@(blue, bold)Mathematical Sequences\\n\");
    
    // Fibonacci with visual table
    let mut fib = vec![0i64, 1i64];
    for i in 2..10 {
        fib.push(fib[i-1] + fib[i-2]);
    }
    table(&fib).header([\"Index\", \"Value\"]);
    
    // Golden ratio convergence chart
    let ratios: Vec<f64> = (1..fib.len())
        .when(|&i| fib[i-1] != 0)
        .then(|i| fib[i] as f64 / fib[i-1] as f64);
    chart(&ratios, \"line\").title(\"Convergence to φ\");
}
```

**Perfect for**: Math education, algorithm visualization, interactive textbooks.

### **2. Data Analysis & Reporting**

```rust,ignore
# use webrust::prelude::*;
use std::collections::HashMap;

#[gui]
fn main() {
    // 🆕 NEW in 1.2.0: Grid-based dashboard layout!
    grid(2, 3);  // 2 rows, 3 columns
    
    let sales_data = load_csv(\"sales.csv\");
    
    // Summary table in top-left cell
    let (x, y) = cell(0, 0, \"center\");
    table(&sales_data)
        .header([\"Product\", \"Q1\", \"Q2\", \"Q3\", \"Q4\")
        .at(x, y)
        .size(*CW * 95 / 100, *CH * 90 / 100);
    
    // Bar chart in top-middle cell
    let (x, y) = cell(0, 1, \"center\");
    let totals: HashMap<String, f64> = sales_data.iter()
        .then(|(product, q1, q2, q3, q4)| (product.clone(), q1+q2+q3+q4));
    chart(&totals, \"bar\")
        .title(\"Annual Sales by Product\")
        .at(x, y)
        .size(*CW * 95 / 100, *CH * 90 / 100);
    
    // Line chart showing trends in top-right
    let (x, y) = cell(0, 2, \"center\");
    let monthly = extract_monthly_data(&sales_data);
    chart(&monthly, \"line\")
        .title(\"Monthly Trend\")
        .at(x, y)
        .size(*CW * 95 / 100, *CH * 90 / 100);
    
    // KPI dashboard in bottom row
    let (x, y) = cell(1, 0, \"center\");
    println!(\"@(green, bold)Revenue: $1.2M (+15%)\")
        .background(\"lightgreen\")
        .radius(10)
        .at(x, y);
    
    let (x, y) = cell(1, 1, \"center\");
    println!(\"@(blue, bold)Users: 45K (+8%)\")
        .background(\"lightblue\")
        .radius(10)
        .at(x, y);
    
    let (x, y) = cell(1, 2, \"center\");
    println!(\"@(orange, bold)Conversion: 3.2% (+0.5%)\")
        .background(\"lightyellow\")
        .radius(10)
        .at(x, y);
}
```

**Perfect for**: Business reports, data exploration, executive dashboards, scientific papers.

**🆕 NEW**: Grid system makes professional multi-panel layouts trivial!

### **3. Scientific Computing**

```rust,ignore
# use webrust::prelude::*;
#[gui]
fn main() {
    coord(\"cartesian\");  // Mathematical coordinates
    grid(2, 2);
    
    println!(\"@(purple, bold)Physics Simulation\\n\");
    
    // LaTeX equations in top-left
    let (x, y) = cell(0, 0, \"top left\");
    println!(r\"Newton's law: $(F = ma)\")
        .at(x + 20.0, y + 20.0);
    println!(r\"Energy: $(E = \\frac{1}{2}mv^2)\")
        .at(x + 20.0, y + 50.0);
    
    // 🆕 NEW: Hierarchical animation - Solar system!
    let (cx, cy) = cell(0, 1, \"center\");
    
    // Sun (stationary)
    let sun = object();
    sun.color(\"gold\").fill(\"gold\").at(cx, cy).circle(20.0);
    
    // Earth system (planet + moon as hierarchy)
    let earth_system = group();
    
    let earth = object();
    earth.color(\"blue\").fill(\"lightblue\").at(cx + 80.0, cy).circle(12.0);
    
    let moon = object();
    moon.color(\"gray\").fill(\"lightgray\").at(cx + 80.0 + 20.0, cy).circle(4.0);
    
    earth_system.add(&earth);
    earth_system.add(&moon);
    
    // Earth orbits sun
    earth_system.rotate(360.0).ease(\"linear\");
    
    // Moon orbits earth (independent animation!)
    moon.translate(-20.0, 0.0)  // Move to earth center
        .rotate(720.0)          // Orbit twice as fast
        .translate(20.0, 0.0)   // Move back
        .ease(\"linear\");
    
    // Simulation data chart in bottom cells
    let (x, y) = cell(1, 0, \"center\");
    let positions = run_simulation();
    chart(&positions, \"line\")
        .title(\"Particle Trajectory\")
        .xlabel(\"Time (s)\")
        .ylabel(\"Position (m)\")
        .at(x, y)
        .size(*CW * 95 / 100, *CH * 90 / 100);
    
    // Vector diagram in bottom-right
    let (x, y) = cell(1, 1, \"center\");
    let arrow = object();
    arrow.color(\"red\").width(3.0).at(x, y);
    arrow.forward(100.0);  // Force vector
}
```

**Perfect for**: Physics simulations, chemistry visualizations, biology models, astronomy.

**🆕 NEW**: Hierarchical groups enable complex multi-body systems with realistic physics!

### **4. Prototyping & Demos**

```rust,ignore
# use webrust::prelude::*;
#[gui]
fn main() {
    println!(\"@(cyan, bold)Product Demo\\n\");
    
    let user_choice: String = input(\"Select feature (A/B/C):\");
    
    match user_choice.as_str() {
        \"A\" => demo_feature_a(),
        \"B\" => demo_feature_b(),
        \"C\" => demo_feature_c(),
        _ => println!(\"@(red)Invalid choice\"),
    }
}
```
```

**Perfect for**: Customer demos, proof-of-concepts, hackathons.

---

<a id="getting-started"></a>
<a id="sec-getting-started"></a>
## 🚀 **Getting Started**

### **Installation**

```toml
[dependencies]
webrust = \"1.2.0\"
```

### **Your First App (30 seconds)**

```rust,ignore
use webrust::prelude::*;

#[gui]
fn main() {
    let name: String = input(\"What's your name?\");
    println!(\"Hello, {name}! 🎉\");
    
    let nums = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    chart(&nums, \"line\").title(\"My First Chart\");
}
```

**Run**: `cargo run` → Browser opens → Professional UI appears.

---

## 📚 **Rich Examples**

WebRust includes comprehensive examples demonstrating every feature:

```bash
cargo run --example simpleio      # Inputs, styling, positioning
cargo run --example advancedio    # LaTeX, tables, advanced formatting
cargo run --example string        # String methods showcase
cargo run --example utils         # Ranges, enumerate, comprehensions
cargo run --example table         # Table generation & formatting
cargo run --example chart         # All 9+ chart types
cargo run --example turtle        # Turtle graphics & animations
cargo run --example mixed         # 🆕 Complete dashboard with grid layout!
```

Each example is extensively commented and demonstrates best practices.

**🆕 Don't miss `py_mixed`**: Shows the full power of the 1.2.0 layout system with a 3×3 grid containing tables, charts, LaTeX, and complex hierarchical animations (like bouncing balls with physics)!

---

## 🎯 **WebRust's Vision for Programming's Future**

### **1. Syntax Should Be Universal**

Good ideas shouldn't be language-specific. If Python's `range(10)` is intuitive, why can't Rust have `0.to(10)`? If JavaScript's template literals work, why not Rust's `{variable}`?

**WebRust proves syntax can evolve without breaking performance or safety.**

### **2. Ecosystems Should Talk**

Python developers bring 30 years of ergonomic patterns. Rust brings safety and speed. These communities should learn from each other, not stay isolated.

**WebRust builds bridges, not walls.**

### **3. Defaults Should Be Modern**

In 2025, \"Hello, World!\" shouldn't be monochrome terminal text. It should be:
- ✅ **Styled** (colors, fonts, layouts)
- ✅ **Interactive** (inputs, buttons, charts)
- ✅ **Shareable** (runs in browser, URL-accessible)
- ✅ **Visual** (supports math, graphics, animations)

**WebRust makes the modern default effortless.**

### **4. Complexity Should Be Optional**

Want to make a simple script? Use `println()`.
Want to add styling? Chain `.color()`.
Want a chart? Call `chart()`.
Want animations? Add `.ease()`.

**Each layer of complexity is opt-in**, not mandatory.

---

## 🔮 **What's Next?**

WebRust is actively evolving with:

- 🎨 **More chart types** (sankey, treemap, 3D plots)
- 🧩 **Component system** (reusable UI widgets)
- 🌐 **Static export** (generate standalone HTML)
- 📱 **Mobile optimization** (responsive by default)
- 🔌 **Plugin ecosystem** (community extensions)
- 🌍 **i18n support** (multi-language UIs)

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
- Keep it simple (zero-config, batteries-included)

---

## 📄 **License**

MIT License — see [LICENSE](https://github.com/gerarddubard/webrust/blob/main/https://github.com/gerarddubard/webrust/blob/main/LICENSE) for details.

---

## 🙏 **Acknowledgments**

WebRust stands on the shoulders of giants:

- **Python community** — For showing us ergonomics matter
- **Rust community** — For proving safety and speed coexist
- **Web standards** — For creating the universal platform
- **Open source** — For enabling collaboration

Special thanks to the creators of:
- [tiny_http](https://crates.io/crates/tiny_http) — Simple HTTP server
- [serde](https://crates.io/crates/serde) — Serialization framework
- [MathJax](https://www.mathjax.org/) — Beautiful math rendering
- [ECharts](https://echarts.apache.org/) — Powerful charting library
- [Two.js](https://two.js.org/) — 2D drawing API

---

## 🌟 **The Bottom Line**

**It's 2025.** We have the technology to make programming:
- More intuitive (Python-like syntax)
- More safe (Rust's type system)
- More visual (browser-based UIs)
- More accessible (zero configuration)

**WebRust proves it's possible.** We're not asking you to choose between Python and Rust, between simplicity and performance, between terminal and web.

**We're showing you can have it all.**

---

<div style="text-align:center">

### 🦀 **Write Python. Think Rust. Ship Web.** 🌐

**[Get Started Now](https://docs.rs/webrust)** | **[View Examples](https://github.com/gerarddubard/webrust/tree/main/examples)** | **[Join Discussion](https://github.com/gerarddubard/webrust/discussions)**

---

*Made with ❤️ for developers who believe programming should be joyful*

</div>
