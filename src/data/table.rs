// webrust/src/data/table.rs
//! # Smart Table Generation
//!
//! Automatic table formatting from any serializable data structure with
//! support for pivot operations, cell merging, and LaTeX rendering.
//!
//! ## Features
//!
//! - Automatic layout detection (vectors, matrices, hashmaps, nested structures)
//! - LaTeX integration with `$(...)` syntax
//! - Cell merging with `.merge()` for visual grouping
//! - Pivot operations for data transformation
//! - Absolute positioning with `.at(x, y)`
//!
//! ## Examples
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! use std::collections::HashMap;
//! # #[gui] fn example() {
//! // Simple matrix
//! let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
//! table(&matrix).header(["X", "Y", "Z"]);
//!
//! // Nested structures (auto-flattened)
//! let mut data = HashMap::new();
//! data.insert("Paris", HashMap::from([("pop", 2200000)]));
//! table(&data);  // Automatically formatted
//!
//! // With merge for grouping
//! let survey = vec![
//!     vec!["Good", "Service"],
//!     vec!["Good", "Quality"],
//!     vec!["Average", "Price"],
//! ];
//! table(&survey).merge();  // Groups adjacent identical cells
//! # }
//! ```

use serde::Serialize;
use serde_json::{to_value, Map, Value};
use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Eq)]
pub enum CellStatus { Key, Value, Empty }

#[derive(Clone)]
pub struct Cell {
    pub content: String,
    pub status: CellStatus,
    pub rowspan: usize,
    pub colspan: usize,
}

pub struct TableBuilder {
    data: Vec<Vec<Cell>>,
    headers: Option<Vec<String>>,
    row_headers: Option<Vec<String>>,
    rendered: bool,
    merge_enabled: bool,
    position: Option<(f64, f64)>,
    size: Option<(u32, u32)>,
}

fn process_webrust_styles(text: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if i + 1 < chars.len() && chars[i] == '$' && chars[i + 1] == '(' {
            i += 2;
            let mut buf = String::new();
            let mut d = 1;
            while i < chars.len() && d > 0 {
                if chars[i] == '(' { d += 1; }
                else if chars[i] == ')' { d -= 1; }
                if d > 0 { buf.push(chars[i]); }
                i += 1;
            }
            let display = buf.contains("\\begin{") || buf.contains("\\[") || buf.len() > 50;
            out.push_str(if display { "$$" } else { "$" });
            out.push_str(&buf);
            out.push_str(if display { "$$" } else { "$" });
        } else if chars[i] == '@' && i + 1 < chars.len() && chars[i + 1] == '(' {
            i += 2;
            let mut styles = String::new();
            while i < chars.len() && chars[i] != ')' { styles.push(chars[i]); i += 1; }
            i += 1;
            let content_start = i;
            while i < chars.len() && !(chars[i] == '@' && i + 1 < chars.len() && chars[i + 1] == '(') { i += 1; }
            let content = &text[content_start..i];
            let css: Vec<String> = styles.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).map(|tok| {
                match tok.to_ascii_lowercase().as_str() {
                    "bold" => "font-weight:bold".to_string(),
                    "italic" => "font-style:italic".to_string(),
                    "underline" => "text-decoration:underline".to_string(),
                    "strike" => "text-decoration:line-through".to_string(),
                    _ => format!("color:{}", tok),
                }
            }).collect();
            if !css.is_empty() {
                out.push_str(&format!(r#"<span style="{}">{}</span>"#, css.join(";"), content));
            } else { out.push_str(content); }
        } else { out.push(chars[i]); i += 1; }
    }
    out
}

impl TableBuilder {
    pub fn new(data: Vec<Vec<Cell>>) -> Self {
        Self { data, headers: None, row_headers: None, rendered: false, merge_enabled: false, position: None, size: None }
    }

    pub fn header<I, S>(mut self, headers: I) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        self.headers = Some(headers.into_iter().map(|h| process_webrust_styles(&h.into())).collect());
        self
    }

    pub fn merge(mut self) -> Self { self.merge_enabled = true; self }
    pub fn at(mut self, x: f64, y: f64) -> Self { self.position = Some((x, y)); self }

    pub fn pivot(mut self) -> Self {
        if self.data.is_empty() { return self; }
        if let Some(headers) = self.headers.take() { self.row_headers = Some(headers); }
        let (rows, cols) = (self.data.len(), self.data.iter().map(|r| r.len()).max().unwrap_or(0));
        let mut pivoted = vec![vec![Cell::empty(); rows]; cols];
        for (i, row) in self.data.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if j < cols && i < rows { pivoted[j][i] = cell.clone(); }
            }
        }
        self.data = pivoted; self
    }

    fn render(&mut self) -> String {
        if self.rendered || self.data.is_empty() {
            self.rendered = true;
            return "<p>(empty)</p>".into();
        }
        self.rendered = true;
        self.normalize_table_size();
        if self.merge_enabled { self.apply_merge(); } else { self.compute_span(); }

        let mut html = String::from("<table class=\"webrust-table\">\n");
        if let Some(headers) = &self.headers {
            html.push_str("<thead><tr>");
            if self.row_headers.is_some() { html.push_str("<th class=\"no-border\"></th>"); }
            headers.iter().for_each(|h| html.push_str(&format!("<th class=\"webrust-th-header\">{}</th>", h)));
            html.push_str("</tr></thead>\n");
        }
        html.push_str("<tbody>\n");
        for (i, row) in self.data.iter().enumerate() {
            html.push_str("<tr>");
            if let Some(rh) = &self.row_headers {
                html.push_str(&format!("<th class=\"webrust-th-header\">{}</th>", rh.get(i).unwrap_or(&String::new())));
            }
            for cell in row.iter().filter(|c| c.rowspan > 0 && c.colspan > 0) {
                let (tag, class) = match cell.status {
                    CellStatus::Key => ("th", "webrust-th-header"),
                    CellStatus::Value => ("td", if is_numeric(&cell.content) { "webrust-td-number" } else { "webrust-td-value" }),
                    CellStatus::Empty => ("td", "no-border")
                };
                let attr = match (cell.rowspan, cell.colspan) {
                    (r, c) if r > 1 || c > 1 => format!("{}{}",
                                                        if r > 1 { format!(" rowspan=\"{}\"", r) } else { String::new() },
                                                        if c > 1 { format!(" colspan=\"{}\"", c) } else { String::new() }),
                    _ => String::new(),
                };
                html.push_str(&format!("<{tag} class=\"{class}\"{attr}>{}</{tag}>", cell.content));
            }
            html.push_str("</tr>\n");
        }
        html.push_str("</tbody></table>");
        html
    }

    fn normalize_table_size(&mut self) {
        if self.data.is_empty() { return; }
        let max_cols = self.data.iter().map(|row| row.len()).max().unwrap_or(0);
        self.data.iter_mut().for_each(|row| while row.len() < max_cols { row.push(Cell::empty()); });
    }

    fn apply_merge(&mut self) {
        if self.data.is_empty() { return; }
        let (rows, cols) = (self.data.len(), self.data.iter().map(|r| r.len()).max().unwrap_or(0));
        for row in &mut self.data {
            let mut j = 0;
            while j < row.len() {
                if row[j].colspan == 0 || (row[j].content.is_empty() && row[j].status != CellStatus::Empty) { j += 1; continue; }
                let (content, status) = (row[j].content.clone(), row[j].status.clone());
                let mut span = 1;
                for k in (j + 1)..row.len() {
                    if row[k].content != content || row[k].status != status { break; }
                    row[k].colspan = 0; span += 1;
                }
                row[j].colspan = span; j += span;
            }
        }
        for j in 0..cols {
            let mut i = 0;
            while i < rows {
                if i >= self.data.len() || j >= self.data[i].len() || self.data[i][j].rowspan == 0 ||
                    (self.data[i][j].content.is_empty() && self.data[i][j].status != CellStatus::Empty) { i += 1; continue; }
                let (content, status, colspan) = (self.data[i][j].content.clone(), self.data[i][j].status.clone(), self.data[i][j].colspan);
                let mut span = 1;
                for k in (i + 1)..rows {
                    if k >= self.data.len() || j >= self.data[k].len() ||
                        self.data[k][j].content != content || self.data[k][j].status != status || self.data[k][j].colspan != colspan { break; }
                    self.data[k][j].rowspan = 0; span += 1;
                }
                self.data[i][j].rowspan = span; i += span;
            }
        }
    }

    fn compute_span(&mut self) {
        self.data.iter_mut().flatten().for_each(|cell| {
            if cell.rowspan == 0 { cell.rowspan = 1; }
            if cell.colspan == 0 { cell.colspan = 1; }
        });
    }
}

impl crate::layout::grid::Sizable for TableBuilder {
    fn set_size(&mut self, size: (u32, u32)) {
        self.size = Some(size);
    }
}

impl Drop for TableBuilder {
    fn drop(&mut self) {
        if !self.rendered {
            let table_html = self.render();
            if let Some((x, y)) = self.position {
                use std::sync::atomic::{AtomicUsize, Ordering};
                static TABLE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
                let table_id = format!("wrtable_{}", TABLE_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                let (left, top) = crate::layout::coord::to_screen_coords(x, y);
                let (width, height) = self.size.unwrap_or((300, 200));
                let positioned_html = format!(
                    r#"<div id="{}" style="position:absolute;left:{}px;top:{}px;width:{}px;height:{}px;overflow:hidden;transform:translate(-50%,-50%);box-sizing:border-box;"><div style="width:100%;height:100%;overflow:auto;padding:2px;">{}</div></div><script>(function(){{var e=document.getElementById('{}');if(e){{var t=e.querySelector('table');if(t){{t.style.fontSize='9px';t.style.width='98%';t.style.margin='0 auto';var cells=t.querySelectorAll('th,td');cells.forEach(function(c){{c.style.padding='3px 4px';c.style.fontSize='9px';c.style.whiteSpace='nowrap';c.style.lineHeight='1.2';}});}}}}}})()</script>"#,
                    table_id, left, top, width, height, table_html, table_id
                );
                crate::io::gui::add_output(positioned_html);
            } else {
                crate::io::gui::add_output(format!("SIMPLE_TABLE:{}", table_html));
            }
        }
    }
}

pub fn table<T: Serialize>(data: &T) -> TableBuilder {
    let val = to_value(data).unwrap_or(Value::Null);
    let cells = match &val {
        Value::Object(map) if !map.is_empty() && map.values().all(|v| v.is_object()) => flatten_nested_object(map),
        _ => value_to_cells(&val),
    };
    let mut builder = TableBuilder::new(cells);
    if let Value::Object(map) = &val { if map.values().any(|v| v.is_object()) { builder = builder.merge(); } }
    builder
}

fn value_to_cells(value: &Value) -> Vec<Vec<Cell>> {
    match value {
        Value::Array(arr) if !arr.is_empty() => {
            if arr.iter().all(|v| v.is_primitive() || v.is_string()) {
                vec![arr.iter().map(Cell::val).collect()]
            } else if arr.iter().all(|v| v.is_array()) {
                let mut matrix: Vec<Vec<Cell>> = arr.iter().filter_map(|v| v.as_array()).map(|inner| inner.iter().map(Cell::val).collect()).collect();
                if !matrix.is_empty() {
                    let max_cols = matrix.iter().map(|row| row.len()).max().unwrap_or(0);
                    matrix.iter_mut().for_each(|row| while row.len() < max_cols { row.push(Cell::empty()); });
                }
                matrix
            } else { arr.iter().map(|v| vec![Cell::val(v)]).collect() }
        }
        Value::Object(obj) => obj.iter().map(|(k, v)| {
            let mut row = vec![Cell::key(k)];
            match v { Value::Array(arr) => row.extend(arr.iter().map(Cell::val)), _ => row.push(Cell::val(v)) }
            row
        }).collect(),
        _ => vec![vec![Cell::val(value)]],
    }
}

fn flatten_nested_object(map: &Map<String, Value>) -> Vec<Vec<Cell>> {
    fn walk(path: Vec<String>, val: &Value) -> (BTreeMap<Vec<String>, ()>, BTreeMap<String, BTreeMap<Vec<String>, Vec<String>>>) {
        let mut key_paths = BTreeMap::new();
        let mut data: BTreeMap<String, BTreeMap<Vec<String>, Vec<String>>> = BTreeMap::new();
        if let Value::Object(map) = val {
            for (k, v) in map {
                let mut new_path = path.clone(); new_path.push(k.clone());
                let (sub_key_paths, sub_data) = walk(new_path, v);
                key_paths.extend(sub_key_paths);
                for (prop_key, values_map) in sub_data { data.entry(prop_key).or_default().extend(values_map); }
            }
        } else if let Some(prop_key) = path.last() {
            let col_path = path[..path.len() - 1].to_vec();
            key_paths.insert(col_path.clone(), ());
            let values = match val {
                Value::String(s) => s.split(',').map(|s| s.trim().to_string()).collect(),
                _ => vec![format_json(val)],
            };
            data.entry(prop_key.clone()).or_default().insert(col_path, values);
        }
        (key_paths, data)
    }
    let mut key_paths = BTreeMap::new();
    let mut data: BTreeMap<String, BTreeMap<Vec<String>, Vec<String>>> = BTreeMap::new();
    for (k, v) in map.iter() {
        let (sub_key_paths, sub_data) = walk(vec![k.to_string()], v);
        key_paths.extend(sub_key_paths);
        for (prop_key, values_map) in sub_data { data.entry(prop_key).or_default().extend(values_map); }
    }
    let sorted_key_paths: Vec<Vec<String>> = key_paths.into_keys().collect();
    let col_header_depth = sorted_key_paths.iter().map(|k| k.len()).max().unwrap_or(0);
    let mut table = (0..col_header_depth).map(|d| {
        let mut row = vec![Cell::empty()];
        row.extend(sorted_key_paths.iter().map(|key_path| Cell::key(&key_path.get(d).cloned().unwrap_or_default())));
        row
    }).collect::<Vec<_>>();
    let prop_rows: Vec<Vec<Cell>> = data.into_iter().flat_map(|(prop, values_map)| {
        let max_rows = values_map.values().map(|v| v.len()).max().unwrap_or(1);
        (0..max_rows).map(|i| {
            let mut row = vec![Cell::key(&prop)];
            row.extend(sorted_key_paths.iter().map(|key_path| Cell::val_from_str(&values_map.get(key_path).and_then(|v| v.get(i)).cloned().unwrap_or_default())));
            row
        }).collect::<Vec<_>>()
    }).collect();
    table.extend(prop_rows); table
}

fn format_json(v: &Value) -> String {
    match v { Value::String(s) => s.clone(), Value::Number(n) => n.to_string(), Value::Bool(b) => b.to_string(), Value::Null => String::new(), _ => v.to_string() }
}

impl Cell {
    fn key(s: &str) -> Self { Self { content: process_webrust_styles(s), status: CellStatus::Key, rowspan: 1, colspan: 1 } }
    fn val(v: &Value) -> Self { Self { content: process_webrust_styles(&format_json(v)), status: CellStatus::Value, rowspan: 1, colspan: 1 } }
    fn val_from_str(s: &str) -> Self { Self { content: process_webrust_styles(s), status: CellStatus::Value, rowspan: 1, colspan: 1 } }
    fn empty() -> Self { Self { content: String::new(), status: CellStatus::Empty, rowspan: 1, colspan: 1 } }
}

fn is_numeric(s: &str) -> bool { s.trim().parse::<f64>().is_ok() }
trait IsPrimitive { fn is_primitive(&self) -> bool; }
impl IsPrimitive for Value { fn is_primitive(&self) -> bool { matches!(self, Value::Number(_) | Value::Bool(_) | Value::Null) } }