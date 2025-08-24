// webrust/src/io/print.rs
//! # Advanced Printing System with Rich Styling
//!
//! Provides sophisticated text output with CSS-like styling, LaTeX mathematical
//! expressions, professional layout capabilities, and advanced text alignment
//! through the PrintBox system.
//!
//! ## Features
//!
//! - **Rich text styling** - Colors, fonts, and text decorations
//! - **LaTeX integration** - Mathematical expressions with `$(...)` syntax
//! - **PrintBox system** - Borders, alignment, spacing, and backgrounds
//! - **F-string processing** - Python-like string formatting
//! - **Professional layouts** - Table-like arrangements and visual grouping
//! - **Dynamic sizing** - Automatic screen dimension detection with CW/CH
//! - **Text alignment** - Left, center, right, and justify alignment
//! - **Advanced styling** - Border weights, styles, radius, and backgrounds
//!
//! ## Quick Start
//!
//!
//! use webrust::prelude::*;
//!
//! // Basic styled output
//! println("@(red, bold)Error message@(reset)");
//! println("@(green, italic)Success!@(reset)");
//!
//! // Advanced styling with alignment
//! println("Centered Text")
//!     .width(*CW)
//!     .align("center")
//!     .weight(2)
//!     .color("blue")
//!     .background("lightblue");
//!
//!
//! ## Dynamic Screen Dimensions
//!
//! The system automatically detects screen dimensions and provides constants:
//! - **`CW`** - Content Width (half of screen width)
//! - **`CH`** - Content Height (half of screen height)
//!
//! These are calculated dynamically via PowerShell on Windows systems:
//!
//!
//! // Full width centered text
//! println("Welcome to WebRust").width(*CW).align("center");
//!
//! // Quarter width left-aligned
//! println("Side panel").width(*CW / 4).align("left");
//!
//! // Half width right-aligned
//! println("Footer").width(*CW / 2).align("right");
//!
//!
//! ## Text Alignment System
//!
//! Four alignment modes provide complete layout control:
//!
//! ### Center Alignment
//!
//! println("Perfectly Centered")
//!     .width(*CW)
//!     .align("center")
//!     .weight(2)
//!     .color("blue");
//!
//!
//! ### Left Alignment  
//!
//! println("Left-aligned content")
//!     .width(*CW)
//!     .align("left")
//!     .background("lightgreen");
//!
//!
//! ### Right Alignment
//!
//! println("Right-aligned content")
//!     .width(*CW)
//!     .align("right")
//!     .color("red");
//!
//!
//! ### Justify Alignment
//!
//! let long_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit...";
//! println(long_text)
//!     .width(*CW)
//!     .align("justify")
//!     .background("ghostwhite");
//!
//!
//! ## Advanced Styling Options
//!
//! ### Border Weights (1-5px)
//!
//! println("Thin border").weight(1);      // 1px
//! println("Medium border").weight(3);    // 3px  
//! println("Thick border").weight(5);     // 5px
//!
//!
//! ### Border Styles
//!
//! println("Solid border").style("solid");
//! println("Dashed border").style("dashed");
//! println("Dotted border").style("dotted");
//! println("Double border").style("double");
//!
//!
//! ### Border Radius
//!
//! println("Sharp corners").radius(0);    // No rounding
//! println("Slight curve").radius(5);     // 5px radius
//! println("Very rounded").radius(20);    // 20px radius
//!
//!
//! ### Colors and Backgrounds
//!
//! println("Colored border")
//!     .color("crimson")
//!     .background("mistyrose");
//!
//! println("Professional styling")
//!     .color("darkslateblue")
//!     .background("ghostwhite");
//!
//!
//! ## Complete Styling Example
//!
//!
//! println("@(navy, bold)PROFESSIONAL ALERT")
//!     .width(*CW)                    // Full width
//!     .align("center")               // Center alignment
//!     .weight(4)                     // Thick border (4px)
//!     .color("navy")                 // Navy border color
//!     .style("double")               // Double border style
//!     .radius(8)                     // 8px rounded corners
//!     .background("lightcyan");      // Light cyan background
//!
//!
//! ## Text Styling with @(...) Syntax
//!
//! Rich inline text styling using WebRust's @(...) notation:
//!
//!
//! println("@(red, bold)Error:@(reset) @(blue)Connection failed@(reset)");
//! println("@(green, italic)Success:@(reset) @(yellow)File saved@(reset)");
//! println("@(purple, underline)Important:@(reset) Read the docs");
//!
//!
//! ### Available Text Styles
//! - **Colors**: `red`, `blue`, `green`, `yellow`, `purple`, `navy`, etc.
//! - **Decorations**: `bold`, `italic`, `underline`, `strike`
//! - **Special**: `reset` (clears all styling)
//! - **Custom**: `background:lightblue`, `color:crimson`
//!
//! ## LaTeX Mathematical Expressions
//!
//! Seamless mathematical notation with `$(...)` syntax:
//!
//!
//! println("Einstein's equation: $(E = mc^2)");
//! println("Quadratic formula: $(x = \\frac{-b \\pm \\sqrt{b^2-4ac}}{2a})");
//! println("Integral: $(\\int_0^\\infty e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2})");
//!
//!
//! ### LaTeX Display Modes
//! - **Inline**: Short expressions render inline: `$(x^2)`
//! - **Display**: Complex expressions auto-center: `$(\\begin{matrix}...)` 
//! - **Auto-detection**: Based on length and complexity
//!
//! ## Real-World Layout Examples
//!
//! ### Document Header
//!
//! println("@(navy, bold)WEBRUST FRAMEWORK v0.8.0")
//!     .width(*CW)
//!     .align("center")
//!     .weight(4)
//!     .color("navy")
//!     .style("double")
//!     .radius(8)
//!     .background("lightcyan");
//!
//!
//! ### Content Body
//!
//! let content = "WebRust revolutionizes Rust development by providing \
//!                Python-like simplicity without sacrificing performance...";
//! println(content)
//!     .width(*CW)
//!     .align("justify")
//!     .weight(1)
//!     .color("darkslateblue")
//!     .style("solid")
//!     .radius(5)
//!     .background("ghostwhite");
//!
//!
//! ### Document Footer
//!
//! println("Created with ❤️ by the WebRust Team")
//!     .width(*CW)
//!     .align("right")
//!     .weight(2)
//!     .color("crimson")
//!     .style("dotted")
//!     .radius(15)
//!     .background("seashell");
//!
//!
//! ## Width Variations
//!
//! Flexible width control for different layout needs:
//!
//!
//! // Full width (responsive)
//! println("Full width header").width(*CW);
//!
//! // Half width (two-column layout)  
//! println("Left column").width(*CW / 2);
//! println("Right column").width(*CW / 2);
//!
//! // Quarter width (four-column layout)
//! println("Col 1").width(*CW / 4);
//! println("Col 2").width(*CW / 4); 
//! println("Col 3").width(*CW / 4);
//! println("Col 4").width(*CW / 4);
//!
//! // Fixed width (absolute sizing)
//! println("Sidebar").width(200);
//!
//!
//! ## PrintBox Method Reference
//!
//! Complete API for the PrintBox styling system:
//!
//! ### Core Methods
//! - `.width(px)` - Set fixed width in pixels
//! - `.align(mode)` - Set text alignment: "left", "center", "right", "justify"
//! - `.weight(px)` - Set border thickness (1-5px recommended)
//! - `.color(color)` - Set border color (CSS color names/hex)
//! - `.background(color)` - Set background color
//!
//! ### Border Styling  
//! - `.style(type)` - Border style: "solid", "dashed", "dotted", "double"
//! - `.radius(px)` - Border radius for rounded corners
//! - `.border(t, r, b, l)` - Control individual borders (bool values)
//!
//! ### Spacing and Layout
//! - `.space(px)` - Set line spacing between elements
//! - `.stroke(px)` - Alias for `.weight()`
//! - `.thickness(px)` - Alias for `.weight()`
//!
//! ## Performance Notes
//!
//! - **Lazy initialization**: CW/CH calculated once on first use
//! - **HTML generation**: Styles compile to optimized HTML/CSS
//! - **MathJax integration**: LaTeX expressions rendered client-side
//! - **Memory efficient**: PrintBox uses drop-based output for minimal overhead
//!
//! ## Cross-Platform Compatibility
//!
//! - **Windows**: PowerShell-based screen detection (primary)
//! - **Fallback**: 800x600 default if detection fails
//! - **Web rendering**: All styling works in WebRust's web interface
//! - **CSS output**: Professional HTML/CSS generation
//!
//! ## Integration Examples
//!
//! ### Form Layout
//!
//! println("@(blue, bold)User Registration Form")
//!     .width(*CW)
//!     .align("center")
//!     .weight(3)
//!     .background("aliceblue");
//!
//! println("Please enter your information:")
//!     .width(*CW)
//!     .align("left")
//!     .weight(1)
//!     .color("gray");
//!
//!
//! ### Alert System
//!
//! // Success alert
//! println("@(white, bold)✅ SUCCESS")
//!     .width(*CW)
//!     .align("center")
//!     .weight(2)
//!     .color("green")
//!     .background("lightgreen");
//!
//! // Warning alert  
//! println("@(black, bold)⚠️ WARNING")
//!     .width(*CW)
//!     .align("center")
//!     .weight(3)
//!     .color("orange")
//!     .background("lightyellow");
//!
//! // Error alert
//! println("@(white, bold)❌ ERROR")
//!     .width(*CW)
//!     .align("center")
//!     .weight(4)
//!     .color("red")
//!     .background("lightcoral");
//!
//!
//! This advanced printing system brings professional document styling to Rust
//! with Python-like simplicity, making WebRust ideal for creating polished
//! user interfaces and formatted output.

use crate::io::gui::{add_output_new_line, add_output_same_line};
use std::sync::LazyLock;

pub static CW: LazyLock<u32> = LazyLock::new(|| {
    std::process::Command::new("powershell")
        .args(&["-Command", "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.Screen]::PrimaryScreen.Bounds.Width"])
        .output().ok().and_then(|out| String::from_utf8_lossy(&out.stdout).trim().parse().ok()).unwrap_or(800) / 2
});

pub static CH: LazyLock<u32> = LazyLock::new(|| {
    std::process::Command::new("powershell")
        .args(&["-Command", "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.Screen]::PrimaryScreen.Bounds.Height"])
        .output().ok().and_then(|out| String::from_utf8_lossy(&out.stdout).trim().parse().ok()).unwrap_or(600) / 2
});

fn read_balanced(chars: &[char], i: &mut usize) -> String {
    let mut d = 1;
    let mut s = String::new();
    while *i < chars.len() && d > 0 {
        let c = chars[*i];
        if c == '(' { d += 1; } else if c == ')' { d -= 1; }
        if d > 0 { s.push(c); }
        *i += 1;
    }
    s
}

fn latex_from_dollar_paren(text: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if i + 1 < chars.len() && chars[i] == '$' && chars[i + 1] == '(' {
            i += 2;
            let buf = read_balanced(&chars, &mut i);
            let display = buf.contains("\\begin{") || buf.contains("\\[") || buf.len() > 50;
            if display { out.push_str("$$"); out.push_str(&buf); out.push_str("$$"); }
            else { out.push('$'); out.push_str(&buf); out.push('$'); }
        } else { out.push(chars[i]); i += 1; }
    }
    out
}

pub fn process_webrust_styles_only(text: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '@' && i + 1 < chars.len() && chars[i + 1] == '(' {
            let mut close_pos = None;
            for j in (i + 2)..chars.len() {
                if chars[j] == ')' { close_pos = Some(j); break; }
            }
            if let Some(close) = close_pos {
                let styles_end = close;
                let styles_raw: String = chars[(i + 2)..styles_end].iter().collect();
                let content_start = styles_end + 1;
                let mut next_tag = chars.len();
                for j in content_start..chars.len() {
                    if j + 1 < chars.len() && chars[j] == '@' && chars[j + 1] == '(' {
                        next_tag = j; break;
                    }
                }
                let content: String = chars[content_start..next_tag].iter().collect();
                let mut css: Vec<String> = Vec::new();
                for tok in styles_raw.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
                    match tok.to_ascii_lowercase().as_str() {
                        "bold" => css.push("font-weight:bold".to_string()),
                        "italic" => css.push("font-style:italic".to_string()),
                        "underline" => css.push("text-decoration:underline".to_string()),
                        "strike" => css.push("text-decoration:line-through".to_string()),
                        "reset" => { out.push_str(&content); css.clear(); }
                        _ => {
                            if tok.contains(':') {
                                let parts: Vec<&str> = tok.split(':').collect();
                                if parts.len() == 2 {
                                    match parts[0].trim() {
                                        "background" => css.push(format!("background-color:{}", parts[1].trim())),
                                        "color" => css.push(format!("color:{}", parts[1].trim())),
                                        _ => css.push(format!("color:{}", tok)),
                                    }
                                } else { css.push(format!("color:{}", tok)); }
                            } else { css.push(format!("color:{}", tok)); }
                        }
                    }
                }
                if !css.is_empty() {
                    out.push_str(&format!(r#"<span style="{}">{}</span>"#, css.join(";"), content));
                } else { out.push_str(&content); }
                i = next_tag;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

pub fn html_escape_preserve_utf8(text: &str) -> String {
    text.chars().map(|c| match c {
        '<' => "&lt;".to_string(),
        '>' => "&gt;".to_string(),
        '&' => "&amp;".to_string(),
        '"' => "&quot;".to_string(),
        '\'' => "&#x27;".to_string(),
        _ => c.to_string(),
    }).collect()
}

pub fn process_styles(text: &str) -> String {
    let s = process_webrust_styles_only(&latex_from_dollar_paren(text));
    if s.contains('\n') && (s.contains('{') || s.contains('[')) {
        format!("<pre style=\"font-family:'Courier New',monospace;margin:0;display:inline;\">{}</pre>", s)
    } else { s }
}

#[derive(Clone)]
pub struct PrintBox {
    lines: Vec<String>, inline: bool, b_top: bool, b_right: bool, b_bottom: bool, b_left: bool,
    weight_px: u32, border_color: Option<String>, style: Option<String>, radius_px: u32,
    cell_width: Option<u32>, align: String, line_gap_px: Option<u32>, bg_color: Option<String>, emitted: bool,
}

impl PrintBox {
    fn new(lines: Vec<String>, inline: bool) -> Self {
        Self { lines, inline, b_top: true, b_right: true, b_bottom: true, b_left: true, weight_px: 0,
            border_color: None, style: None, radius_px: 0, cell_width: None, align: "center".into(),
            line_gap_px: None, bg_color: None, emitted: false }
    }
    pub fn border(mut self, t: bool, r: bool, b: bool, l: bool) -> Self { self.b_top=t; self.b_right=r; self.b_bottom=b; self.b_left=l; self }
    pub fn weight(mut self, px: u32) -> Self { self.weight_px = px; self }
    pub fn stroke(self, px: u32) -> Self { self.weight(px) }
    pub fn thickness(self, px: u32) -> Self { self.weight(px) }
    pub fn color<S: Into<String>>(mut self, c: S) -> Self { self.border_color = Some(c.into()); self }
    pub fn style<S: Into<String>>(mut self, s: S) -> Self { self.style = Some(s.into()); self }
    pub fn radius(mut self, px: u32) -> Self { self.radius_px = px; self }
    pub fn width(mut self, px: u32) -> Self { self.cell_width = if px > 0 { Some(px) } else { None }; self }
    pub fn align<S: AsRef<str>>(mut self, v: S) -> Self {
        let a = v.as_ref().to_ascii_lowercase();
        self.align = match a.as_str() { "left"|"center"|"right"|"justify" => a, _ => "center".into() };
        self
    }
    pub fn space(mut self, px: u32) -> Self { self.line_gap_px = Some(px); self }
    pub fn background<S: Into<String>>(mut self, c: S) -> Self { self.bg_color = Some(c.into()); self }
    fn build_style(&self) -> String {
        let mut css = String::from("display:inline-block;white-space:normal;vertical-align:top;padding:2px 6px;");
        css.push_str(&format!("text-align:{};border-radius:{}px;", self.align, self.radius_px));
        if let Some(w) = self.cell_width { css.push_str(&format!("width:{}px;", w)); }
        if let Some(bg) = &self.bg_color { css.push_str(&format!("background-color:{};", bg)); }
        let sty = self.style.as_deref().unwrap_or("solid");
        let col = self.border_color.as_deref().unwrap_or("#cbd5e1");
        let s = self.weight_px;
        let mut side = |n: &str, on: bool| {
            if on { css.push_str(&format!("border-{}:{}px {} {};", n, s, sty, col)); }
            else { css.push_str(&format!("border-{}:none;", n)); }
        };
        side("top", self.b_top); side("right", self.b_right); side("bottom", self.b_bottom); side("left", self.b_left);
        css
    }
}

impl Drop for PrintBox {
    fn drop(&mut self) {
        if self.emitted { return; }
        let style = self.build_style();
        if self.inline {
            let gap_attr = self.line_gap_px.map(|g| format!(" data-line-gap=\"{}\"", g)).unwrap_or_default();
            for seg in &self.lines {
                let html = format!(r#"<span class="webrust-box"{gap} style="{style}">{inner}</span>"#, gap=gap_attr, style=style, inner=seg);
                add_output_same_line(html);
            }
        } else {
            let gap = self.line_gap_px.unwrap_or(6);
            for seg in &self.lines {
                let html = format!(r#"<div class="webrust-line" style="display:block;margin:{gap}px 0;"><span class="webrust-box" style="{style}">{inner}</span></div>"#, gap=gap, style=style, inner=seg);
                add_output_new_line(html);
            }
        }
        self.emitted = true;
    }
}

pub fn print_str<T: std::fmt::Display>(text: T) -> PrintBox {
    let raw = format!("{}", text);
    let lines: Vec<String> = raw.split('\n').map(|s| process_styles(s)).collect();
    PrintBox::new(lines, true)
}

pub fn println_str<T: std::fmt::Display>(text: T) -> PrintBox {
    let raw = format!("{}", text);
    let lines: Vec<String> = raw.split('\n').map(|s| process_styles(s)).collect();
    PrintBox::new(lines, false)
}

pub use print_str as print;
pub use println_str as println;