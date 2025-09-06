## [1.1.0] - 2025-09-06

### Added
- **Turtle graphics** (`webrust::graphic::turtle`): multi-turtle, independent motion, and smooth animation.
    - API: `turtle()` with `.setColor()`, `.setPenSize()`, `.speed()`, `.angle()`, `.setPos()`, `.forward()`, `.line()`, `.point()`, `.circle()`, **`.penup()`**, **`.pendown()`**.
    - Any valid CSS/HTML color name or hex is accepted by `.setColor("navy")`, `.setColor("#1e90ff")`, etc.
- **Global coordinate modes**: `coord("css")` (origin top-left) or `coord("cartesian")` (origin center, +y up).
    - Affects both `print()/println().at(x, y)` and all turtle coordinates for a unified experience.
- **Absolute text positioning**: `print(...).at(x, y)` now works in both coordinate modes and can be mixed with drawings to label figures.
- **Right-edge anchoring (CSS mode)**: pass a **negative `x`** to `.at(x, y)` to pin the box `|x|` pixels from the right edge.
- **Canvas staging**: automatic stage creation sized from `CW`/`CH`, device-pixel-ratio aware rendering, and per-stage turtle queues.

### Changed
- **Unified `.at(x, y)` semantics**: single method for both modes (removed the need for separate APIs).
- **Client runtime (`static/script.js`)**: compact turtle renderer (queues, high-DPI, steady animation), resilient MathJax typeset path, safer inline chart execution.
- **Docs & examples**: new `py_turtle.rs` demonstrating mixed text + geometry, coordinate switching, and multiple turtles.

### Fixed
- Inline color application inside absolutely-positioned boxes (e.g., `@(white) …`) now renders reliably.
- Avoided redundant fetch work and visual flicker during input/validation updates.
- Minor stability and performance tweaks across I/O rendering paths.

### Migration Notes
- If you previously relied on `at(dx, y)` semantics: the API is now `at(x, y)`.
    - In **CSS mode**, negative `x` keeps the old “offset from right” behavior.
    - In **Cartesian mode**, `x`/`y` are absolute Cartesian coordinates (origin center).

