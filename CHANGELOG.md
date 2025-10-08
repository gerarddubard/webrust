# 🦀 WebRust — Changelog

All notable changes to this project will be documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/).

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
- **Resolved occasional “wait” blocking** when chaining multiple animations.  
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
  - In **CSS mode**, negative `x` keeps the old “offset from right” behavior.  
  - In **Cartesian mode**, `x`/`y` are absolute Cartesian coordinates (origin center).

---

*See also:* [WebRust 1.2.0 Release Notes](#120--2025-10-08)
