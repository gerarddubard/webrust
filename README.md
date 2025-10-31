# WebRust: Python-Inspired Rust for Interactive Web Applications

[![WebRust](https://img.shields.io/badge/WebRust-1.9.0-ff6b35?style=flat-square)](https://github.com/gerarddubard/webrust)
[![Rust](https://img.shields.io/badge/Rust-1.70+-000?style=flat-square&logo=rust)](https://rust-lang.org)
[![Documentation](https://img.shields.io/badge/docs-latest-blue?style=flat-square)](https://docs.rs/webrust)
[![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)

**Links:** [Documentation](https://docs.rs/webrust) | [Examples](https://github.com/gerarddubard/webrust/tree/main/examples) | [Crates.io](https://crates.io/crates/webrust)

---

## Abstract

**WebRust** bridges Python’s ergonomics with Rust’s performance to build interactive, browser‑based apps **without writing HTML/CSS/JS**. Write expressive, Python‑like Rust that compiles to a single native binary; WebRust launches a local server and streams rich UI (text, tables, charts, turtle graphics, LaTeX) to your browser.

Version **1.9.0** adds **customizable browser background colors** via `#[gui(... !bg)]`, restores **radar chart tooltips**, and ships comprehensive **optimizations** across the proc‑macro pipeline and GUI runtime. The framework unifies data manipulation, visualization, and interactivity in a single, type‑safe environment.

---

## Table of Contents

1. [Introduction](#introduction)
2. [What's New in 1.9.0](#whats-new-in-190)
3. [Motivation](#motivation)
4. [Architecture](#architecture)
5. [Installation](#installation)
6. [Core Features](#core-features)
7. [Usage Examples](#usage-examples)
8. [Performance](#performance)
9. [Contributing](#contributing)
10. [License](#license)

---

## Introduction

### Overview

WebRust lets you write Python‑like Rust that runs natively with **full type safety**, while automatically generating a **rich web UI**. No frontend stack, no deployment boilerplate — just Rust.

### Key Characteristics

- **🐍 Python-Inspired Syntax**: ranges (`.to()`), filters (`.when()`), transforms (`.then()`)
- **🎨 Rich Text Rendering**: inline colors, borders, layout helpers, and LaTeX
- **📊 Interactive Visualizations**: 12+ chart types powered by ECharts **with full tooltip support**
- **🗄️ High-Performance SQL** *(optional)*: DuckDB integration with streaming results
- **🐢 Turtle Graphics**: object-oriented drawing with animation and easing
- **🎯 Zero Configuration**: automatic browser UI; no HTML/CSS/JS required
- **⚡ Native Performance**: Rust compile-time safety and speed
- **🎭 Full Theme Control**: **viewport** background theming via `#[gui(... !bg)]`

### Design Philosophy

1. **Ergonomics First** — write naturally
2. **Type Safety Always** — lean on Rust guarantees
3. **Visual by Default** — UI is automatic
4. **Performance Matters** — zero-copy and SIMD where it counts

---

## What's New in 1.9.0

### 🎨 Major Visual Enhancements

#### 1) **Browser Background Color Support**

The `#[gui]` attribute now supports **full browser viewport** background customization via the `!color` token.

```rust
#[gui(Arial 14px black !#1a1a2e)]  // Dark theme, full window background
fn main() {
    println("<white 18 b>Night Mode");
}
```

Supported formats: named colors (`!navy`), hex `!#fff`/`!#1a1a2e`, `rgb(...)`/`rgba(...)`.

Applied consistently to `<body>`, the main container, and chart areas (transparent by default). **Zero runtime overhead**.

#### 2) **Radar Chart Tooltips**

ECharts radar charts now show **tooltips on hover** (single or multi‑series). Values are formatted and color‑coded.

### ⚡ Optimizations & Polish

- **Proc‑macro engine** uses SIMD scanning (`memchr`/`memmem`) and SmallVec for fewer allocations
- **`parking_lot` + `RwLock`** (default feature) for fast concurrent reads
- **Layout spacing** tightened for pie/doughnut/radar/gauge headings
- Minor fixes in table pagination/merge

---

## Motivation

### The Problem

Modern visualization pipelines juggle many tools:

```
SQL → Python (pandas) → Plotly → Flask → Deploy
       ↓                   ↓        ↓
  type safety lost   frontend code  infra cost
```

### The WebRust Solution

```
Rust (WebRust) → Browser (automatic)
   type safety + performance + zero frontend
```

**One language, one binary, instant UI.**

---

## Architecture

### System Overview

```
User Rust code → Proc-macros (SIMD scanning) → Optimized Rust → Native binary
      │                                             │
      └───────────── WebRust runtime ───────────────┘
                       │       │
                 tiny_http   Browser
                  /api/*     main.js + modules (table.js, turtle.js)
```

### Theme Injection (v1.9.0)

```
#[gui(Font Size TextColor !Background)]
          │
    StyleConfig { bg, color, font, size }
          │
   Injected to browser (body + container)
```

---

## Installation

### Base

```toml
[dependencies]
webrust = "1.9.0"
```

### With SQL (DuckDB)

```toml
webrust = { version = "1.9.0", features = ["sql"] }
```

### With Compact Strings

```toml
webrust = { version = "1.9.0", features = ["compact"] }
```

### Locking

`rwlock` (using `parking_lot`) is **enabled by default**. To revert to `Mutex`:

```toml
webrust = { version = "1.9.0", default-features = false }
```

---

## Core Features

- Python‑like ranges and comprehensions
- Inline rich text + LaTeX with `$(...)`
- Interactive tables (sort/filter/paginate/merge/pivot)
- Charts (bar/line/pie/doughnut/scatter/radar/gauge/funnel/...)
- Turtle graphics with easing
- Optional SQL via DuckDB
- Auto‑launched browser UI

---

## Usage Examples

> **Note:** `chart.rs` no longer provides `.title(...)`. Use text elements for headings.

### 1) Interactive Table

```rust
use webrust::prelude::*;

#[gui]
fn main() {
    println("<dodgerblue b 20>Employees");
    let employees = vec![
        vec!["Alice", "25", "Engineer"],
        vec!["Bob", "30", "Designer"],
        vec!["Charlie", "28", "Manager"],
    ];

    table(&employees)
        .header(["Name", "Age", "Role"])
        .sort()
        .filter()
        .paginate()
        .page_size(5)
        .align("center");
}
```

### 2) Radar Chart (with Tooltips, No `.title()`)

```rust
use webrust::prelude::*;

#[gui]
fn main() {
    println("<purple b 18>Team A — Skill Profile");

    let indicators = vec![
        ("Technical".to_string(), 100.0),
        ("Communication".to_string(), 100.0),
        ("Leadership".to_string(), 100.0),
        ("Innovation".to_string(), 100.0),
        ("Quality".to_string(), 100.0),
    ];
    let values = vec![85.0, 90.0, 75.0, 95.0, 88.0];

    chart(&values, "radar")
        .radar_indicators(indicators)
        .tooltips(true);
}
```

### 3) Custom Theme (Viewport Background)

```rust
use webrust::prelude::*;

#[gui(Times_New_Roman 13px navy !#0b1020)]
fn main() {
    println("<white b 22>📊 Themed Dashboard");
    println("<gray 13>Visual clarity meets Rust performance");
}
```

### 4) Multi-Panel Layout with Charts (No `.title()`)

```rust
use webrust::prelude::*;

#[gui(Consolas 13px #e0e0e0 !#1a1a2e)]
fn main() {
    println("<cyan b i 22>🌙 Dark Mode Analytics");
    grid(2, 2);
    coord("css");

    // Top-left: table
    let (x, y) = cell(0, 0, "center");
    println("<white b 16>Metrics").at(x, y - 40.0);
    let metrics = vec![
        vec!["Users", "1,247"],
        vec!["Revenue", "$53,890"],
        vec!["Growth", "+12.3%"],
    ];
    table(&metrics).at(x, y + 20.0).size(45, 45);

    // Top-right: line chart
    let (x, y) = cell(0, 1, "center");
    println("<white b 16>Weekly Trend").at(x, y - 40.0);
    let trend = vec![120.0, 145.0, 132.0, 168.0, 191.0];
    chart(&trend, "line").at(x, y + 20.0).size(45, 45);

    // Bottom-left: gauge
    let (x, y) = cell(1, 0, "center");
    println("<white b 16>Completion").at(x, y - 40.0);
    gauge_chart(78.5).at(x, y + 20.0);

    // Bottom-right: radar with tooltips
    let (x, y) = cell(1, 1, "center");
    println("<white b 16>Project Health").at(x, y - 40.0);
    let scores = vec![85.0, 92.0, 78.0, 88.0, 95.0];
    let inds = vec!["Speed".to_string(), "Quality".to_string(), "Cost".to_string(), "Time".to_string(), "Risk".to_string()]
        .into_iter().map(|s| (s, 100.0)).collect::<Vec<_>>();
    chart(&scores, "radar").radar_indicators(inds).tooltips(true).at(x, y + 20.0);
}
```

---

## Performance

### Compilation (v1.9.0)

| Configuration        | First Build | Incremental | Size   |
|----------------------|-------------|-------------|--------|
| Default              | ~28s        | <4s         | ~2MB   |
| With SQL             | ~2m50s      | <9s         | ~15MB  |
| With compact feature | ~30s        | <4s         | ~2.1MB |

### Runtime

- SIMD f‑string/LaTeX scanning (`memchr`/`memmem`)
- Number formatting via `itoa`/`ryu`
- Fewer allocations with `SmallVec` and pre‑sized buffers
- `parking_lot::RwLock` (default) for concurrent reads

---

## Contributing

- **Issues**: <https://github.com/gerarddubard/webrust/issues>
- **Discussions**: <https://github.com/gerarddubard/webrust/discussions>

PRs welcome: examples, docs, optimizations, features.

---

## License

MIT — see [LICENSE](LICENSE).

