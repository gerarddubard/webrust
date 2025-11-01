# Changelog

All notable changes to **WebRust** are documented here.  
This project follows [Semantic Versioning](https://semver.org/).

## [2.0.0] – 2025-11-01

### Added
- **Redesigned `chart.rs` API** with direct per-type constructors:
  - `line`, `bar`, `area`, `curve`, `line_xy`, `area_xy`, `curve_xy`, `scatter`
  - `pie`, `doughnut`, `radar`, `gauge`, `funnel`, `matrix`, `treemap`, `tree`, `function`
- Uniform chaining for placement/sizing: `.at(x, y)`, `.size(w, h)`
- Cleaner radar labels on hover (`key: value %`), consistent tiny typography

### Changed
- **Replaced** `chart(data, "type")` with direct constructors (see Migration)
- Tuned pie/doughnut label spacing and connector lengths
- Harmonized tooltip behavior across chart types

### Fixed
- Radar label overlap near left-hand vertices
- Minor grid and heading spacing inconsistencies

### Migration Notes
- Replace all `chart(..., "type")` calls with the corresponding constructor:
  - `chart(v, "line")` → `line(labels, values)`
  - `chart(v, "radar")` → `radar(values, indicators)`
  - `chart(p, "pie")` → `pie(labels, values)`
  - etc.
- Headings: use text (`println(...)`) instead of `.title(...)` on charts.

---

## [1.9.0] – 2025-10-31

### Added
- Viewport background color via `#[gui(... !<bg>)]` (e.g., `#[gui(Arial 14px black !whitesmoke)]`)
- Restored radar tooltips and hover behavior

### Changed
- Faster locks with **`parking_lot`** (`rwlock` default feature)
- Macro engine uses SIMD scanning (`memchr`/`memmem`)
- Layout refinements for chart headings

### Fixed
- Pagination and table merge edge cases
- Frontend module loading stability (main.js, table.js, turtle.js)

### Performance
- Fewer allocations in f-string formatting
- Faster JSON and LaTeX handling
- Smaller memory footprint in table rendering

---

## [1.8.0] – 2025-10-15
- Modular JavaScript (`main.js`, `table.js`, `turtle.js`)
- Table improvements (multi-column sort, filter, pagination)
- Memory optimizations in `table.rs`

---

## [1.7.0] – 2025-10-01
- Macro optimization and grid layout overhaul

---

## [1.6.0] – 2025-09-01
- Stability and documentation improvements

---

## Unreleased
- WebSocket streaming
- Static export
- Advanced visualization presets (Sankey, 3D)
