# Changelog

All notable changes to **WebRust** are documented here.  
This project follows [Semantic Versioning](https://semver.org/).

## [1.9.0] – 2025‑10‑31

### Added
- Background color support in `#[gui(...)]` via the `!<bg>` syntax (e.g. `#[gui(Arial 14px black !whitesmoke)]`)
- Radar chart tooltips restored by default (ECharts backend)

### Changed
- Replaced standard locks with **`parking_lot`** for faster concurrency
- Introduced new `rwlock` feature (enabled by default) for concurrent reads
- Macro engine (`webrust‑macros`) rewritten for SIMD scanning (`memchr`/`memmem`)
- Minor layout refinements for pie/doughnut/radar/gauge titles
- Updated dependencies to latest stable releases

### Fixed
- Minor pagination and table merge edge cases
- Frontend module loading stability (main.js, table.js, turtle.js)

### Performance
- Fewer allocations in f‑string formatting
- Faster JSON and LaTeX handling
- Smaller memory footprint in table rendering

### Migration Notes
- If you previously customized background rendering, use `#[gui(... !color)]`
- To revert to `Mutex`, disable default features:
  ```toml
  webrust = { version = "1.9.0", default-features = false }
  ```

---

## [1.8.0] – 2025‑10‑15

### Highlights
- Modular JavaScript architecture (`main.js`, `table.js`, `turtle.js`)
- Major table improvements (multi‑column sort, filter, pagination)
- Refactored `table.rs` for better memory efficiency

---

## [1.7.0] – 2025‑10‑01
- Macro optimization and grid layout overhaul

---

## [1.6.0] – 2025‑09‑01
- Stability and documentation improvements

---

## Unreleased
- WebSocket streaming
- Static export
- Advanced visualization presets (Sankey, Treemap, 3D)
