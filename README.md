# WebRust 1.1.0 — Python-like Rust for the Browser Terminal

WebRust brings Python-style ergonomics to Rust, with a zero-setup web UI: styled printing, validated inputs, rich text (with LaTeX), tables/charts, and now **canvas-powered turtle graphics**. It’s great for teaching, demos, dashboards, and quick interactive tools—without giving up Rust’s type safety.

---

## ✨ Highlights

- **One macro to start**: `#[gui(...)]` spins up the UI automatically.
- **Fluent styled printing**: `print("...").color("crimson").radius(8).at(x, y)`.
- **Shared coordinate system**: `coord("css" | "cartesian")` applies to **text** and **turtle**.
- **Absolute placement** with `.at(x, y)` and screen helpers `CW` / `CH`.
- **Inline styling** `@(bold, color:navy)` and **LaTeX** `$( ... )`.
- **Inputs with validation** shown inline in the terminal.
- **Tables & charts** (ECharts) for quick dashboards.
- **Turtle graphics**: multiple turtles, animation, `penup/pendown`, CSS colors.

---

## 📦 Install

```toml
# Cargo.toml
[dependencies]
webrust = "1.1.0"
```

---

## 🚀 Quick Start

```rust
use webrust::prelude::*;

#[gui(bg="navy", fg="white", font="Courier New", color="black", size="12px")]
fn main() {
    println("@(cyan, bold)Hello WebRust!");

    // Ask for input with type safety
    let name: String = input("Your name:");
    println("Welcome, @(yellow,bold){name}");

    // Switch coordinate mode globally (affects text & turtle)
    coord("cartesian");

    // Absolute placement in the terminal
    print("@(white)Centered badge")
        .background("indigo")
        .radius(6)
        .at(0.0, 0.0); // (x,y) in current coordinate mode

    // A quick LaTeX inline example
    println("Einstein: $(E = mc^2)");
}
```

Run:

```bash
cargo run
```

---

## 🧭 Coordinates & Layout

- `coord("css")` (default): origin **top-left**, +x→right, +y→down.  
  In this mode, `.at(x, y)` supports **right anchoring**: if `x` is **negative**, it means “from the right” (e.g. `.at(-20.0, 8.0)` → 20px from right edge).

- `coord("cartesian")`: origin **center**, +x→right, +y→**up**.

Screen helpers:
- `CW`, `CH` — layout width/height used by the terminal (exported as `LazyLock<u32>`).

Example:

```rust
coord("cartesian");

// top-center banner
println("@(white,bold)DASHBOARD")
    .background("midnightblue")
    .radius(8)
    .at(0.0, (*CH as f64)/2.0 - 30.0);

// two columns
println("Left").width(*CW / 2).align("left");
println("Right").width(*CW / 2).align("right");
```

---

## 🖍️ Styled Printing

Chain CSS-like styles:

```rust
println("@(green, bold)Success")
    .weight(3)
    .style("double")   // solid | dashed | dotted | double
    .radius(8)
    .color("seagreen") // border color
    .background("honeydew")
    .align("center")   // left | center | right | justify
    .at(0.0, -120.0);  // absolute placement
```

Inline text syntax:

- Colors & decorations: `@(red, bold, italic, underline, strike)`
- Custom CSS style tokens: `@(color:crimson, background:mintcream)`
- LaTeX: `$(x = \frac{-b \pm \sqrt{b^2-4ac}}{2a})` (auto inline/display)

---

## ⌨️ Inputs

```rust
let age: i32 = input("Your age:");
let price: f64 = input("Unit price:");
let ready: bool = input("Ready? (true/false):");
println("Ok, {age} {price} {ready}");
```

Inputs render inline in the terminal; values are validated and echoed.

---

## 📊 Tables & Charts (Optional)

Build simple tables and ECharts-powered charts from your data structures.  
(If you’ve used earlier versions, your existing code continues to work.)

---

## 🐢 Turtle Graphics (NEW)

Draw animated scenes on a canvas inside the terminal. Multiple turtles can move independently. The turtle system respects the global `coord("css" | "cartesian")`.

### Example: mixed text + drawing

```rust
use std::f64::consts::PI;
use webrust::prelude::*;

#[gui(bg="navy", fg="white", font="Courier New", size="10px")]
fn main() {
    coord("cartesian");

    let sun = turtle();
    sun.setColor("gold").setPenSize(2.0).setPos(0.0, 0.0).circle(60.0);

    for k in 0..24 {
        let ang = k as f64 * 15.0;
        sun.angle(ang).setPos(0.0, 0.0).penup().forward(45.0).pendown().forward(60.0);
    }

    print("@(black,bold)Sun with rays")
        .background("gold")
        .radius(6)
        .at(0.0, -100.0);
}
```

### Turtle API (overview)

- `setColor(name: &str)` — any CSS color name or hex.
- `setPenSize(px: f64)`
- `speed(px_per_s: f64)` — affects `forward` animation speed.
- `angle(deg: f64)` — absolute heading (0° = east, CCW positive).
- `setPos(x, y)` — jump without drawing.
- `forward(d)` — move with drawing if pen is down.
- `line(x1, y1, x2, y2)` — immediate segment.
- `point()` — filled point at current position.
- `circle(r)` — outline circle centered at current position.
- `penup()` / `pendown()` — toggle drawing while moving.

---

## 🧪 Example: Playground

```rust
#[gui(bg="navy", fg="white", font="Courier New", size="10px")]
fn main() {
    coord("cartesian");

    let spiral = turtle();
    spiral.setColor("plum").setPenSize(2.0);
    let k = 3.5;
    let mut prev = None;
    for th in 0.0.to(10.0*PI).by(0.15) {
        let r = k*th;
        let x = -(*CW as f64)/2.0 + 100.0 + r*th.cos();
        let y =  (*CH as f64)/2.0 -  90.0 + r*th.sin();
        if let Some((x0,y0)) = prev { spiral.line(x0,y0,x,y); }
        prev = Some((x,y));
    }

    print("@(white,italic)Archimedean spiral")
        .background("indigo")
        .radius(6)
        .at(-(*CW as f64)/2.0 + 70.0,  (*CH as f64)/2.0 - 180.0);
}
```

---

## 🧩 Version 1.1.0 — What’s New

- **Unified coordinate system**: `coord("css" | "cartesian")` affects **both** text and turtle.
- **`.at(x, y)` everywhere**: one method for absolute placement; in CSS mode, **negative `x` = from right edge**.
- **Turtle graphics module**:
  - Multi-turtle support with independent queues.
  - New `penup()` / `pendown()` for gapless rays, dotted paths, etc.
  - CSS color names and hex strings for `setColor(...)`.
- **CW/CH integration** across I/O and graphics for consistent sizing.
- Internal performance & robustness improvements.

---

## 🖥️ Platform Notes

- On Windows, `CW` / `CH` default to **half** the primary screen size (PowerShell detection, with a fallback to 800×600).
- Runs in the browser automatically; assets live under `webrust/static/` (e.g., `style.css`, `script.js`).

---

## 📚 Examples

```bash
cargo run --example turtle
cargo run --example py_simpleio
```

---

## 📜 License

MIT. See `LICENSE` for details.

---

## 🧭 Contributing

Issues and PRs are welcome! If you’re adding features, keep the API Python-friendly, chainable, and consistent with the shared coordinate model.
