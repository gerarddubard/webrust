// webrust/src/io/print.rs
//! # Advanced Printing System with Rich Styling
//!
//! Provides sophisticated text output with CSS-like styling, LaTeX mathematical
//! expressions, and professional layout capabilities through the PrintBox system.
//!
//! ## Features
//!
//! - **Rich text styling** - Colors, fonts, and text decorations
//! - **LaTeX integration** - Mathematical expressions with `$(...)` syntax
//! - **PrintBox system** - Borders, alignment, spacing, and backgrounds
//! - **F-string processing** - Python-like string formatting
//! - **Professional layouts** - Table-like arrangements and visual grouping
//!
//! ## Text Styling
//!
//!
//! // Basic colors and styles
//! println("@(red, bold)Error message@(reset)");
//! println("@(green, italic)Success!@(reset)");
//! println("@(blue, underline)Important link@(reset)");
//!
//!
//! ## LaTeX Mathematical Expressions
//!
//!
//! println("Einstein's equation: $(E = mc^2)");
//! println("Quadratic formula: $(x = \\frac{-b \\pm \\sqrt{b^2-4ac}}{2a})");
//!
//!
//! ## PrintBox Advanced Styling
//!
//!
//! println("Professional output")
//!     .weight(2)                    // Border thickness
//!     .color("blue")               // Border color
//!     .background("lightblue")     // Background color
//!     .radius(8)                   // Rounded corners
//!     .width(200)                  // Fixed width
//!     .align("center");            // Text alignment
//!
//!
//! ## Implementation
//!
//! The system processes styling directives and converts them to HTML/CSS
//! for web rendering. LaTeX expressions are rendered via MathJax integration.
use crate::io::gui::{add_output_new_line, add_output_same_line};

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
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'@' && i + 1 < bytes.len() && bytes[i + 1] == b'(' {
            // cherche la ')'
            if let Some(close) = text[i + 2..].find(')') {
                let styles_end = i + 2 + close;
                let styles_raw = &text[i + 2..styles_end];
                let content_start = styles_end + 1;
                // contenu jusqu'au prochain "@(" ou fin
                let next_tag = text[content_start..].find("@(").map(|p| content_start + p).unwrap_or(text.len());
                let content = &text[content_start..next_tag];

                // styles -> CSS
                let mut css: Vec<String> = Vec::new();
                for tok in styles_raw.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
                    match tok.to_ascii_lowercase().as_str() {
                        "bold" => css.push("font-weight:bold".to_string()),
                        "italic" => css.push("font-style:italic".to_string()),
                        "underline" => css.push("text-decoration:underline".to_string()),
                        "strike" => css.push("text-decoration:line-through".to_string()),
                        "reset" => { out.push_str(content); css.clear(); }
                        _ => css.push(format!("color:{}", tok)),
                    }
                }

                if !css.is_empty() {
                    out.push_str(&format!(r#"<span style="{}">{}</span>"#, css.join(";"), content));
                } else {
                    out.push_str(content);
                }

                i = next_tag;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

pub fn process_styles(text: &str) -> String {
    let s = process_webrust_styles_only(&latex_from_dollar_paren(text));
    if s.contains('\n') && (s.contains('{') || s.contains('[')) {
        format!("<pre style=\"font-family:'Courier New',monospace;margin:0;display:inline;\">{}</pre>", s)
    } else { s }
}

#[derive(Clone)]
pub struct PrintBox {
    lines: Vec<String>,
    inline: bool,
    b_top: bool,
    b_right: bool,
    b_bottom: bool,
    b_left: bool,
    weight_px: u32,
    border_color: Option<String>,
    style: Option<String>,
    radius_px: u32,
    cell_width: Option<u32>,
    align: String,
    line_gap_px: Option<u32>,
    bg_color: Option<String>,
    emitted: bool,
}

impl PrintBox {
    fn new(lines: Vec<String>, inline: bool) -> Self {
        Self {
            lines,
            inline,
            b_top: true,
            b_right: true,
            b_bottom: true,
            b_left: true,
            weight_px: 0,
            border_color: None,
            style: None,
            radius_px: 0,
            cell_width: None,
            align: "center".into(),
            line_gap_px: None,
            bg_color: None,
            emitted: false,
        }
    }
    pub fn border(mut self, t: bool, r: bool, b: bool, l: bool) -> Self { self.b_top=t; self.b_right=r; self.b_bottom=b; self.b_left=l; self }
    pub fn weight(mut self, px: u32) -> Self { self.weight_px = px; self }
    pub fn stroke(self, px: u32) -> Self { self.weight(px) }
    pub fn thickness(self, px: u32) -> Self { self.weight(px) }
    pub fn color<S: Into<String>>(mut self, c: S) -> Self { self.border_color = Some(c.into()); self }
    pub fn style<S: Into<String>>(mut self, s: S) -> Self { self.style = Some(s.into()); self }
    pub fn radius(mut self, px: u32) -> Self { self.radius_px = px; self }
    pub fn width(mut self, px: u32) -> Self { self.cell_width = if px > 0 { Some(px) } else { None }; self }
    pub fn align<S: AsRef<str>>(mut self, v: S) -> Self { let a=v.as_ref().to_ascii_lowercase(); self.align = match a.as_str() {"left"|"center"|"right"=>a,_=>"center".into()}; self }
    pub fn space(mut self, px: u32) -> Self { self.line_gap_px = Some(px); self }
    pub fn background<S: Into<String>>(mut self, c: S) -> Self { self.bg_color = Some(c.into()); self }
    fn build_style(&self) -> String {
        let mut css = String::from("display:inline-block;white-space:nowrap;vertical-align:top;padding:2px 6px;");
        css.push_str(&format!("text-align:{};border-radius:{}px;", self.align, self.radius_px));
        if let Some(w)=self.cell_width { css.push_str(&format!("width:{}px;", w)); }
        if let Some(bg)=&self.bg_color { css.push_str(&format!("background-color:{};", bg)); }
        let sty = self.style.as_deref().unwrap_or("solid");
        let col = self.border_color.as_deref().unwrap_or("#cbd5e1");
        let s = self.weight_px;
        let mut side = |n:&str,on:bool| if on { css.push_str(&format!("border-{}:{}px {} {};", n, s, sty, col)); } else { css.push_str(&format!("border-{}:none;", n)); };
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