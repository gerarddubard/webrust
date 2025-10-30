// webrust/src/io/table.rs
//! # WebRust Table — Fast HTML table renderer
//!
//! `io::table` turns any `serde::Serialize` value into a responsive HTML table
//! with zero-copy rendering, optional interactive features (sort, filter,
//! pagination), and grid integration (`.at(x,y)`, `.size(w,h)`, `.align(...)`).
//!
//! ## Key features
//! - **Input-agnostic**: accepts arrays, matrices, objects (maps), and nested objects.
//! - **Typed cells**: integers/floats kept as numbers (no string boxing) for fast render
//!   and correct numeric sort.
//! - **Merging**: `.merge()` coalesces adjacent identical cells horizontally and vertically.
//! - **Pivot**: `.pivot()` transposes the matrix; if headers exist, they become row headers.
//! - **Interactive**: `.sort()`, `.filter()`, `.paginate()` inject minimal JS hooks
//!   (`.paginate()` defaults to 10 rows/page; adjust with `.page_size(n)`).
//! - **Positioning/Sizing**: `.at(x,y)` (absolute center anchor) and `.size(w,h)`.
//! - **Alignment**: `.align("left" | "center" | "right")` controls the outer container alignment.
//! - **Inline LaTeX**: `$( ... )` emitted as `$...$` or `$$...$$` (auto inline/display).
//! - **Safe by default**: full HTML escaping; no `unsafe`.
//!
//! ## Performance notes (v8)
//! - **SmallVec rows**: `SmallVec<[Cell; 12]>` to avoid heap allocs on short rows.
//! - **Compact strings**: optional `compact` feature uses `compact_str::CompactString` for text.
//! - **Zero-copy pivot**: moves cells with `mem::take` (no cloning).
//! - **Path building**: uses `smallvec![]` for nested key paths during flattening.
//! - **DataMap keys**: nested-object property map keyed by `Text` (less churn under `compact`).
//! - **Single-pass** escaping + LaTeX detection, O(n).
//! - **Type sampling**: column type detection on a bounded sample for stable, fast sorts.
//!
//! ## CSS & JS integration
//! - Table element: `class="webrust-table"` inside a `.table-container` wrapper;
//!   `.table-small` is added when `.size(w,h)` is set.
//! - Cells/classes: `webrust-th-header`, `webrust-td-value`, `webrust-td-number`, `no-border`.
//! - Sorting headers add `sort-header` and a `<span class="sort-indicator"></span>`.
//! - When interactive features are enabled, a `<script>` calls:
//!   `window.webrustInitTable(id, filterEnabled, pageSize, hasPagination)`,
//!   and a `<div class="pagination-controls"></div>` is appended when paginated.
//!
//! ## Feature flags
//! - `compact`: use `CompactString` for text cells to reduce allocations.
//!
//! ## API overview
//! - `table(&T) -> TableBuilder` where `T: Serialize`
//! - Builder:
//!   - `.header(iterable_of_displayables)`
//!   - `.merge()`
//!   - `.pivot()`
//!   - `.sort()`
//!   - `.filter()`
//!   - `.paginate()` and/or `.page_size(n: usize)`
//!   - `.at(x: f64, y: f64)`
//!   - `.size(w: u32, h: u32)`
//!   - `.align("left" | "center" | "right")`
//!
//! ## Examples
//! Basic:
//! ```rust
//! use webrust::prelude::*;
//! let data = vec![ vec!["Alice", 25], vec!["Bob", 31] ];
//! table(&data).header(["Name", "Age"]).align("center");
//! ```
//!
//! Interactive — flux (relative) + align:
//! ```rust
//! use webrust::prelude::*;
//! let employees = vec![
//!     vec!["Alice", 25, "Engineer"],
//!     vec!["Bob",   30, "Designer"],
//!     vec!["Chloé", 28, "Manager"],
//! ];
//! table(&employees)
//!     .header(["Name","Age","Role"])
//!     .sort().filter().paginate()
//!     .size(480, 260)
//!     .align("right");
//! ```
//!
//! Interactive — absolute positioning:
//! ```rust
//! use webrust::prelude::*;
//! let employees = vec![
//!     vec!["Alice", 25, "Engineer"],
//!     vec!["Bob",   30, "Designer"],
//!     vec!["Chloé", 28, "Manager"],
//! ];
//! table(&employees)
//!     .header(["Name","Age","Role"])
//!     .sort().filter().paginate()
//!     .size(480, 260)
//!     .at(320.0, 200.0);
//! ```
//!
//! Nested object and merge:
//! ```rust
//! use serde_json::json;
//! let nested = json!({
//!   "Paris":     { "Population": 2148_000, "Attractions": "Louvre,Eiffel" },
//!   "Lyon":      { "Population":  518_000, "Attractions": "Old Town" },
//!   "Marseille": { "Population":  861_000, "Attractions": "Vieux-Port" }
//! });
//! table(&nested).merge().align("center");
//! ```
//!
//! Pivot:
//! ```rust
//! let matrix = vec![ vec![1,2,3], vec![4,5,6] ];
//! table(&matrix).header(["A","B","C"]).pivot().align("left");
//! ```
//!
//! LaTeX inline:
//! ```rust
//! let equations = vec![ vec!["$(a^2+b^2=c^2)$", "$(\\int_0^1 x^2 dx)$"] ];
//! table(&equations).header(["Pythagoras","Integral"]).align("center");
//! ```
//!
//! ## Behavior details
//! - Objects (`Map`) become rows `key | values...`. Arrays of arrays become matrices.
//! - Nested objects are flattened into multi-row headers + property rows.
//! - Numeric detection is performed on a sample of rows to choose sort semantics.
//! - Empty input renders `"<p>(empty)</p>"`.
//!
//! ## Safety
//! - All HTML-special characters are escaped.
//! - No `unsafe` code; no external JS executed beyond the emitted `webrustInitTable` hook.
//!
//! ⚠️ Do not combine `.at(x,y)` and `.align(..)` in the same builder.

use crate::io::gui::add_output_new_line;
use itoa;
use memchr::memchr;
use ryu;
use serde::Serialize;
use serde_json::{to_value, Map, Value};
use smallvec::{smallvec, SmallVec};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(feature = "compact")]
use compact_str::CompactString as Text;
#[cfg(not(feature = "compact"))]
type Text = String;

const EMPTY_TABLE: &str = "<p>(empty)</p>";
const TABLE_CONTAINER: &str = "table-container";
const SAMPLE_SIZE: usize = 10;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellStatus { Key, Value, Empty }

#[derive(Clone, Copy, PartialEq, Eq)]
enum Align { Left, Center, Right }

#[derive(Clone, PartialEq)]
enum CellContent { Text(Text), NumI(i64), NumU(u64), NumF(f64), Empty }

#[derive(Clone, PartialEq)]
pub struct Cell {
    content: CellContent,
    status: CellStatus,
    rowspan: u16,
    colspan: u16,
}

impl Default for Cell { fn default() -> Self { Self::empty() } }

type Row = SmallVec<[Cell; 12]>;

pub struct TableBuilder {
    data: Vec<Row>,
    headers: Option<Vec<Text>>,
    row_headers: Option<Vec<Text>>,
    rendered: bool,
    merge_enabled: bool,
    sort_enabled: bool,
    filter_enabled: bool,
    page_size: Option<usize>,
    position: Option<(f64, f64)>,
    size: Option<(u32, u32)>,
    align: Option<Align>,
}

static PAD_STYLES: [&str; 5] = [
    "",
    " style=\"padding:3px 5px\"",
    " style=\"padding:4px 6px\"",
    " style=\"padding:5px 7px\"",
    " style=\"padding:8px 10px\"",
];

#[inline]
fn has_special_chars(s: &str) -> bool {
    s.as_bytes().iter().any(|&b| matches!(b, b'<' | b'>' | b'&' | b'"' | b'\''))
}

#[inline]
fn push_escaped(out: &mut String, s: &str) {
    if !has_special_chars(s) { out.push_str(s); return; }
    let bytes = s.as_bytes();
    let mut last = 0;
    for (i, &b) in bytes.iter().enumerate() {
        let rep = match b { b'<' => "&lt;", b'>' => "&gt;", b'&' => "&amp;", b'\"' => "&quot;", b'\'' => "&#x27;", _ => continue };
        if last < i { out.push_str(&s[last..i]); }
        out.push_str(rep);
        last = i + 1;
    }
    if last < s.len() { out.push_str(&s[last..]); }
}

#[inline]
fn process_webrust_styles(text: &str) -> Cow<'_, str> {
    let b = text.as_bytes();
    match memchr(b'$', b) {
        None if !has_special_chars(text) => return Cow::Borrowed(text),
        None => { let mut out = String::with_capacity(text.len() + 16); push_escaped(&mut out, text); return Cow::Owned(out) }
        _ => {}
    }
    let n = b.len();
    let mut out = String::with_capacity(n + (n >> 2) + 16);
    let mut i = 0;
    let mut copy_from = 0;
    while i < n {
        if i + 1 < n && b[i] == b'$' && b[i + 1] == b'(' {
            push_escaped(&mut out, &text[copy_from..i]);
            i += 2;
            let start = i;
            let mut depth = 1;
            while i < n && depth > 0 { match b[i] { b'(' => depth += 1, b')' => depth -= 1, _ => {} } i += 1; }
            if depth > 0 { push_escaped(&mut out, &text[start - 2..i]); }
            else {
                let inner = &text[start..i - 1];
                let display = inner.len() > 50 || inner.contains("\\begin{") || inner.contains("\\[");
                out.push_str(if display { "$$" } else { "$" });
                out.push_str(inner);
                out.push_str(if display { "$$" } else { "$" });
            }
            copy_from = i;
            continue;
        }
        i += 1;
    }
    if copy_from < n { push_escaped(&mut out, &text[copy_from..]); }
    Cow::Owned(out)
}

#[inline]
const fn pad_and_font_from_size(size: Option<(u32, u32)>) -> (usize, f32) {
    match size {
        Some((w, h)) => {
            let s = if w < h { w } else { h };
            match s {
                0..=90 => (1, 0.72),
                91..=120 => (2, 0.80),
                121..=160 => (3, 0.88),
                161..=220 => (3, 0.92),
                _ => (4, 0.96),
            }
        }
        None => (0, 1.0),
    }
}

#[inline(always)]
fn write_pad_attr(out: &mut String, pad_idx: usize) {
    if pad_idx > 0 && pad_idx < PAD_STYLES.len() { out.push_str(PAD_STYLES[pad_idx]); }
}

pub trait HeaderExt { fn into_headers(self) -> Vec<Text>; }

impl<I> HeaderExt for I where I: IntoIterator, I::Item: ToString {
    fn into_headers(self) -> Vec<Text> {
        self.into_iter().map(|h| Text::from(process_webrust_styles(&h.to_string()).as_ref())).collect()
    }
}

impl TableBuilder {
    pub fn new(data: Vec<Row>) -> Self {
        Self {
            data, headers: None, row_headers: None, rendered: false,
            merge_enabled: false, sort_enabled: false, filter_enabled: false,
            page_size: None, position: None, size: None, align: None,
        }
    }
    #[inline] pub fn header(mut self, headers: impl HeaderExt) -> Self { self.headers = Some(headers.into_headers()); self }
    #[inline] pub fn merge(mut self) -> Self { self.merge_enabled = true; self }
    #[inline] pub fn sort(mut self) -> Self { self.sort_enabled = true; self }
    #[inline] pub fn filter(mut self) -> Self { self.filter_enabled = true; self }
    #[inline] pub fn paginate(mut self) -> Self { self.page_size = Some(10); self }
    #[inline] pub fn page_size(mut self, size: usize) -> Self { self.page_size = Some(size); self }
    #[inline] pub fn at(mut self, x: f64, y: f64) -> Self { self.position = Some((x, y)); self }
    #[inline] pub fn size(mut self, w: u32, h: u32) -> Self { self.size = Some((w, h)); self }
    #[inline]
    pub fn align<S: AsRef<str>>(mut self, v: S) -> Self {
        self.align = match v.as_ref().to_ascii_lowercase().as_str() {
            "left" => Some(Align::Left),
            "center" => Some(Align::Center),
            "right" => Some(Align::Right),
            _ => None,
        };
        self
    }
    pub fn pivot(mut self) -> Self {
        if self.data.is_empty() { return self; }
        if let Some(headers) = self.headers.take() { self.row_headers = Some(headers); }
        let mut old = std::mem::take(&mut self.data);
        let rows = old.len();
        let cols = old.iter().map(|r| r.len()).max().unwrap_or(0);
        let mut pivoted: Vec<Row> = Vec::with_capacity(cols);
        for j in 0..cols {
            let mut row: Row = SmallVec::with_capacity(rows.min(12));
            for i in 0..rows {
                let cell = if j < old[i].len() { std::mem::take(&mut old[i][j]) } else { Cell::empty() };
                row.push(cell);
            }
            pivoted.push(row);
        }
        self.data = pivoted;
        self
    }
    #[inline]
    fn detect_column_type(&self, col_idx: usize) -> &'static str {
        let sample = self.data.len().min(SAMPLE_SIZE);
        let half = (sample / 2) + 1;
        let (mut n, mut d) = (0, 0);
        for (seen, row) in self.data.iter().take(sample).enumerate() {
            if let Some(cell) = row.get(col_idx) {
                if cell.is_number() { n += 1; if n >= half { return "number"; } }
                else if is_date_like(cell.as_str_hint()) { d += 1; if d >= half { return "date"; } }
            }
            if seen + 1 - n - d >= half { return "string"; }
        }
        if n > sample / 2 { "number" } else if d > sample / 2 { "date" } else { "string" }
    }
    fn render(&mut self) -> String {
        if self.rendered || self.data.is_empty() { self.rendered = true; return EMPTY_TABLE.into(); }
        self.rendered = true;
        self.normalize_table_size();
        if self.merge_enabled { self.apply_merge(); } else { self.compute_span(); }
        static C: AtomicUsize = AtomicUsize::new(1);
        let inter = self.sort_enabled || self.filter_enabled || self.page_size.is_some();
        let table_id = if inter { format!("tbl{}", C.fetch_add(1, Ordering::Relaxed)) } else { String::new() };
        let (pad_idx, font_mul) = pad_and_font_from_size(self.size);
        let cells: usize = self.data.iter().map(|r| r.len()).sum();
        let mut html = String::with_capacity((cells << 6) + 768);
        let container = if self.size.is_some() { "table-container table-small" } else { TABLE_CONTAINER };
        html.push_str("<div class=\""); html.push_str(container); html.push_str("\"><table");
        if inter { html.push_str(" id=\""); html.push_str(&table_id); html.push('"'); }
        html.push_str(" class=\"webrust-table\"");
        if font_mul < 0.999 { let _ = write!(html, " style=\"font-size:{:.0}%\"", font_mul * 100.0); }
        html.push_str(">\n");
        if let Some(headers) = &self.headers {
            html.push_str("<thead><tr>");
            if self.row_headers.is_some() { html.push_str("<th class=\"no-border\""); write_pad_attr(&mut html, pad_idx); html.push_str("></th>"); }
            for (idx, h) in headers.iter().enumerate() {
                html.push_str("<th class=\"webrust-th-header");
                if self.sort_enabled { let _ = write!(html, " sort-header\" data-col=\"{}\" data-type=\"{}\"", idx, self.detect_column_type(idx)); } else { html.push('"'); }
                write_pad_attr(&mut html, pad_idx);
                html.push('>');
                html.push_str(h.as_str());
                if self.sort_enabled { html.push_str("<span class=\"sort-indicator\"></span>"); }
                html.push_str("</th>");
            }
            html.push_str("</tr></thead>\n");
            if self.filter_enabled {
                html.push_str("<thead><tr>");
                if self.row_headers.is_some() { html.push_str("<th class=\"no-border\""); write_pad_attr(&mut html, pad_idx); html.push_str("></th>"); }
                for idx in 0..headers.len() {
                    html.push_str("<th class=\"filter-cell\"");
                    if pad_idx > 0 { write_pad_attr(&mut html, pad_idx.saturating_sub(1)); }
                    let _ = write!(html, "><input type=\"text\" class=\"filter-input\" data-col=\"{}\" placeholder=\"Filter...\"></th>", idx);
                }
                html.push_str("</tr></thead>\n");
            }
        }
        html.push_str("<tbody>\n");
        let mut itb_attr = itoa::Buffer::new();
        let mut itb_num = itoa::Buffer::new();
        let mut rbuf_num = ryu::Buffer::new();
        for (i, row) in self.data.iter().enumerate() {
            html.push_str("<tr>");
            if let Some(rh) = &self.row_headers {
                html.push_str("<th class=\"webrust-th-header\""); write_pad_attr(&mut html, pad_idx); html.push('>'); html.push_str(rh.get(i).map(|s| s.as_str()).unwrap_or("")); html.push_str("</th>");
            }
            for cell in row.iter().filter(|c| c.rowspan > 0 && c.colspan > 0) {
                cell.write_html(&mut html, pad_idx, &mut itb_attr, &mut itb_num, &mut rbuf_num);
            }
            html.push_str("</tr>\n");
        }
        html.push_str("</tbody></table>");
        if inter {
            if self.page_size.is_some() { html.push_str("<div class=\"pagination-controls\"></div>"); }
            html.push_str("</div>");
            let _ = write!(html, r#"<script>window.webrustInitTable('{}',{},{},{});</script>"#, table_id, self.filter_enabled, self.page_size.unwrap_or(999_999), self.page_size.is_some());
        } else {
            html.push_str("</div>");
        }
        html
    }
    #[inline]
    fn normalize_table_size(&mut self) {
        if self.data.is_empty() { return; }
        let max_cols = self.data.iter().map(|row| row.len()).max().unwrap_or(0);
        for row in &mut self.data { row.resize_with(max_cols, Cell::empty); }
    }
    fn apply_merge(&mut self) {
        if self.data.is_empty() { return; }
        for row in &mut self.data {
            let mut j = 0;
            while j < row.len() {
                if row[j].colspan == 0 || row[j].content.is_empty_or_null() { j += 1; continue; }
                let status = row[j].status;
                let content = row[j].content.clone();
                let mut span = 1;
                for k in (j + 1)..row.len() {
                    if row[k].status != status || row[k].content != content { break; }
                    row[k].colspan = 0; span += 1;
                }
                row[j].colspan = span as u16;
                j += span;
            }
        }
        let rows = self.data.len();
        let cols = self.data.iter().map(|r| r.len()).max().unwrap_or(0);
        for j in 0..cols {
            let mut i = 0;
            while i < rows {
                if i >= self.data.len() || j >= self.data[i].len() || self.data[i][j].rowspan == 0 || self.data[i][j].content.is_empty_or_null() { i += 1; continue; }
                let status = self.data[i][j].status;
                let colspan = self.data[i][j].colspan;
                let content = self.data[i][j].content.clone();
                let mut span = 1;
                for k in (i + 1)..rows {
                    if k >= self.data.len() || j >= self.data[k].len() { break; }
                    if self.data[k][j].status != status || self.data[k][j].colspan != colspan || self.data[k][j].content != content { break; }
                    self.data[k][j].rowspan = 0; span += 1;
                }
                self.data[i][j].rowspan = span as u16;
                i += span;
            }
        }
    }
    #[inline]
    fn compute_span(&mut self) {
        for cell in self.data.iter_mut().flatten() {
            if cell.rowspan == 0 { cell.rowspan = 1; }
            if cell.colspan == 0 { cell.colspan = 1; }
        }
    }
}

impl CellContent {
    #[inline] fn is_number(&self) -> bool { matches!(self, CellContent::NumI(_) | CellContent::NumU(_) | CellContent::NumF(_)) }
    #[inline] fn is_empty_or_null(&self) -> bool { matches!(self, CellContent::Empty) || matches!(self, CellContent::Text(t) if t.is_empty()) }
    #[inline] fn as_str_hint(&self) -> &str { match self { CellContent::Text(t) => t.as_str(), _ => "" } }
}

impl Cell {
    #[inline(always)]
    fn write_html(
        &self,
        html: &mut String,
        pad_idx: usize,
        itb_attr: &mut itoa::Buffer,
        itb_num: &mut itoa::Buffer,
        rbuf_num: &mut ryu::Buffer,
    ) {
        let (tag, class) = match self.status {
            CellStatus::Key => ("th", "webrust-th-header"),
            CellStatus::Value => ("td", if self.content.is_number() { "webrust-td-number" } else { "webrust-td-value" }),
            CellStatus::Empty => ("td", "no-border"),
        };
        html.push('<'); html.push_str(tag); html.push_str(" class=\""); html.push_str(class); html.push('"');
        if self.rowspan > 1 { html.push_str(" rowspan=\""); html.push_str(itb_attr.format(self.rowspan)); html.push('"'); }
        if self.colspan > 1 { html.push_str(" colspan=\""); html.push_str(itb_attr.format(self.colspan)); html.push('"'); }
        write_pad_attr(html, pad_idx);
        html.push('>');
        match &self.content {
            CellContent::Text(t) => html.push_str(t.as_str()),
            CellContent::NumI(i) => html.push_str(itb_num.format(*i)),
            CellContent::NumU(u) => html.push_str(itb_num.format(*u)),
            CellContent::NumF(f) => html.push_str(rbuf_num.format(*f)),
            CellContent::Empty => {}
        }
        html.push_str("</"); html.push_str(tag); html.push('>');
    }
    #[inline]
    fn key(s: &str) -> Self {
        Self { content: CellContent::Text(Text::from(process_webrust_styles(s).as_ref())), status: CellStatus::Key, rowspan: 1, colspan: 1 }
    }
    fn val(v: &Value) -> Self {
        let content = match v {
            Value::String(s) => CellContent::Text(Text::from(process_webrust_styles(s).as_ref())),
            Value::Number(n) => if let Some(i) = n.as_i64() { CellContent::NumI(i) }
            else if let Some(u) = n.as_u64() { CellContent::NumU(u) }
            else if let Some(f) = n.as_f64() { CellContent::NumF(f) }
            else { CellContent::Text(Text::from(n.to_string())) },
            Value::Bool(b) => CellContent::Text(Text::from(if *b { "true" } else { "false" })),
            Value::Null => CellContent::Empty,
            _ => CellContent::Text(Text::from(v.to_string())),
        };
        Self { content, status: CellStatus::Value, rowspan: 1, colspan: 1 }
    }
    #[inline]
    fn val_from_str(s: &str) -> Self {
        let t = s.trim();
        let content = if let Ok(i) = t.parse::<i64>() { CellContent::NumI(i) }
        else if let Ok(u) = t.parse::<u64>() { CellContent::NumU(u) }
        else if let Ok(f) = t.parse::<f64>() { CellContent::NumF(f) }
        else { CellContent::Text(Text::from(process_webrust_styles(s).as_ref())) };
        Self { content, status: CellStatus::Value, rowspan: 1, colspan: 1 }
    }
    #[inline] fn is_number(&self) -> bool { self.content.is_number() }
    #[inline] fn as_str_hint(&self) -> &str { self.content.as_str_hint() }
    #[inline] pub const fn empty() -> Self { Self { content: CellContent::Empty, status: CellStatus::Empty, rowspan: 1, colspan: 1 } }
}

impl crate::layout::grid::Sizable for TableBuilder {
    fn set_size(&mut self, size: (u32, u32)) { self.size = Some(size); }
}

impl Drop for TableBuilder {
    fn drop(&mut self) {
        if self.rendered { return; }
        let table_html = self.render();
        let align_style = match self.align {
            Some(Align::Center) => "margin:0 auto;display:block;width:fit-content;",
            Some(Align::Right) => "margin-left:auto;display:block;width:fit-content;",
            Some(Align::Left) => "margin-left:0;display:block;width:fit-content;",
            None => "",
        };
        let html = match (self.position, self.size) {
            (Some((x, y)), size_opt) => {
                let (w, h) = size_opt.unwrap_or((400, 300));
                let left = if x < 0.0 { format!("right:{}px", -x.round() as i32) } else { format!("left:{}px", x.round() as i32) };
                let top = y.round() as i32;
                let abs_style = format!("position:absolute;{left};top:{top}px;width:{}px;height:{}px;transform:translate(-50%,-50%);display:flex;flex-direction:column;overflow:auto;{align_style}", w, h);
                format!("<div style=\"{abs_style}\">{table_html}</div>")
            }
            (None, Some((w, h))) => format!("<div class=\"table-container\" style=\"width:{}px;height:{}px;display:flex;flex-direction:column;overflow:auto;{align_style}\">{table_html}</div>", w, h),
            (None, None) => if align_style.is_empty() {
                format!("<div class=\"table-container\">{table_html}</div>")
            } else {
                format!("<div class=\"table-container\" style=\"{align_style}\">{table_html}</div>")
            },
        };
        add_output_new_line(html);
    }
}

pub fn table<T: Serialize>(data: &T) -> TableBuilder {
    let val = to_value(data).unwrap_or(Value::Null);
    let cells = match &val {
        Value::Object(map) if !map.is_empty() && map.values().all(|v| v.is_object()) => flatten_nested_object(map),
        _ => value_to_rows(&val),
    };
    let mut builder = TableBuilder::new(cells);
    if let Value::Object(map) = &val { if map.values().any(|v| v.is_object()) { builder = builder.merge(); } }
    builder
}

fn value_to_rows(value: &Value) -> Vec<Row> {
    match value {
        Value::Array(arr) if !arr.is_empty() => {
            if arr.iter().all(|v| matches!(v, Value::Number(_) | Value::String(_) | Value::Bool(_) | Value::Null)) {
                let mut row: Row = SmallVec::with_capacity(arr.len().min(12));
                row.extend(arr.iter().map(Cell::val));
                vec![row]
            } else if arr.iter().all(|v| v.is_array()) {
                let mut matrix: Vec<Row> = Vec::with_capacity(arr.len());
                let mut max_cols = 0;
                for inner in arr.iter().filter_map(Value::as_array) {
                    let mut r: Row = SmallVec::with_capacity(inner.len().min(12));
                    r.extend(inner.iter().map(Cell::val));
                    max_cols = max_cols.max(r.len());
                    matrix.push(r);
                }
                for r in &mut matrix { r.resize_with(max_cols, Cell::empty); }
                matrix
            } else {
                arr.iter().map(|v| { let mut row: Row = SmallVec::with_capacity(1); row.push(Cell::val(v)); row }).collect()
            }
        }
        Value::Object(obj) => obj.iter().map(|(k, v)| {
            let cap = 1 + if v.is_array() { v.as_array().map(|a| a.len()).unwrap_or(1) } else { 1 };
            let mut row: Row = SmallVec::with_capacity(cap.min(12));
            row.push(Cell::key(k));
            match v { Value::Array(arr) => row.extend(arr.iter().map(Cell::val)), _ => row.push(Cell::val(v)) }
            row
        }).collect(),
        _ => { let mut row: Row = SmallVec::with_capacity(1); row.push(Cell::val(value)); vec![row] }
    }
}

fn flatten_nested_object(map: &Map<String, Value>) -> Vec<Row> {
    type KeyPath<'a> = SmallVec<[&'a str; 8]>;
    type KeyPathSet<'a> = BTreeMap<KeyPath<'a>, ()>;
    type DataMap<'a> = BTreeMap<Text, BTreeMap<KeyPath<'a>, Vec<Text>>>;

    #[inline]
    fn num_to_text(n: &serde_json::Number, itb: &mut itoa::Buffer, rbuf: &mut ryu::Buffer) -> Text {
        if let Some(i) = n.as_i64() { Text::from(itb.format(i)) }
        else if let Some(u) = n.as_u64() { Text::from(itb.format(u)) }
        else if let Some(f) = n.as_f64() { Text::from(rbuf.format(f)) }
        else { Text::from(n.to_string()) }
    }

    #[inline]
    fn scalar_to_text(v: &Value, itb: &mut itoa::Buffer, rbuf: &mut ryu::Buffer) -> Text {
        match v {
            Value::String(s) => Text::from(process_webrust_styles(s).as_ref()),
            Value::Number(n) => num_to_text(n, itb, rbuf),
            Value::Bool(true) => Text::from("true"),
            Value::Bool(false) => Text::from("false"),
            Value::Null => Text::new(),
            _ => Text::from(v.to_string()),
        }
    }

    #[inline]
    fn split_or_scalar(v: &Value, itb: &mut itoa::Buffer, rbuf: &mut ryu::Buffer) -> Vec<Text> {
        match v {
            Value::String(s) => {
                let bs = s.as_bytes();
                if memchr(b',', bs).is_some() { s.split(',').map(|t| Text::from(process_webrust_styles(t.trim()).as_ref())).collect() }
                else { vec![Text::from(process_webrust_styles(s).as_ref())] }
            }
            _ => vec![scalar_to_text(v, itb, rbuf)],
        }
    }

    fn walk<'a>(path: KeyPath<'a>, val: &'a Value, itb: &mut itoa::Buffer, rbuf: &mut ryu::Buffer) -> (KeyPathSet<'a>, DataMap<'a>) {
        let mut key_paths = BTreeMap::new();
        let mut data: DataMap = BTreeMap::new();
        if let Value::Object(m) = val {
            for (k, v) in m {
                let mut new_path = path.clone();
                new_path.push(k.as_str());
                let (kp, d) = walk(new_path, v, itb, rbuf);
                key_paths.extend(kp);
                for (prop, vm) in d { data.entry(prop).or_default().extend(vm); }
            }
        } else if let Some(prop_key) = path.last() {
            let col_path: KeyPath = path[..path.len() - 1].iter().copied().collect();
            key_paths.insert(col_path.clone(), ());
            let values = split_or_scalar(val, itb, rbuf);
            data.entry(Text::from(*prop_key)).or_default().insert(col_path, values);
        }
        (key_paths, data)
    }

    let mut key_paths = BTreeMap::new();
    let mut data: DataMap = BTreeMap::new();
    let mut itb = itoa::Buffer::new();
    let mut rbuf = ryu::Buffer::new();

    for (k, v) in map.iter() {
        let (kp, d) = walk(smallvec![k.as_str()], v, &mut itb, &mut rbuf);
        key_paths.extend(kp);
        for (prop, vm) in d { data.entry(prop).or_default().extend(vm); }
    }

    let sorted_key_paths: Vec<KeyPath> = key_paths.into_keys().collect();
    let col_header_depth = sorted_key_paths.iter().map(|k| k.len()).max().unwrap_or(0);
    let keys_len = sorted_key_paths.len();

    let total_prop_rows: usize = data.values().map(|vm| vm.values().map(|v| v.len()).max().unwrap_or(1)).sum();
    let mut table: Vec<Row> = Vec::with_capacity(col_header_depth + total_prop_rows);

    for d in 0..col_header_depth {
        let mut row: Row = SmallVec::with_capacity(1 + keys_len.min(12));
        row.push(Cell::empty());
        for key_path in &sorted_key_paths {
            let s = key_path.get(d).copied().unwrap_or_default();
            row.push(Cell::key(s));
        }
        table.push(row);
    }

    for (prop, values_map) in data.into_iter() {
        let max_rows = values_map.values().map(|v| v.len()).max().unwrap_or(1);
        for i in 0..max_rows {
            let mut row: Row = SmallVec::with_capacity(1 + keys_len.min(12));
            row.push(Cell::key(prop.as_str()));
            for key_path in &sorted_key_paths {
                let s = values_map.get(key_path).and_then(|vv| vv.get(i)).cloned().unwrap_or_else(Text::new);
                row.push(Cell::val_from_str(s.as_str()));
            }
            table.push(row);
        }
    }

    table
}

#[inline]
fn is_date_like(s: &str) -> bool {
    let len = s.len();
    len >= 8 && len <= 20 && memchr(b'-', s.as_bytes()).is_some()
}