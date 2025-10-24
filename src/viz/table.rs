// webrust/src/viz/table.rs
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

// webrust/src/viz/table.rs
use itoa;
use memchr::memchr2;
use ryu;
use serde::Serialize;
use serde_json::{to_value, Map, Value};
use smallvec::SmallVec;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Eq)]
pub enum CellStatus {
    Key,
    Value,
    Empty,
}

#[derive(Clone)]
pub struct Cell {
    pub content: String,
    pub status: CellStatus,
    pub rowspan: usize,
    pub colspan: usize,
    pub is_number: bool,
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

#[inline]
fn push_escaped(out: &mut String, s: &str) {
    if !s
        .bytes()
        .any(|b| matches!(b, b'<' | b'>' | b'&' | b'"' | b'\''))
    {
        out.push_str(s);
        return;
    }
    for ch in s.chars() {
        match ch {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            _ => out.push(ch),
        }
    }
}

#[inline]
fn process_webrust_styles(text: &str) -> Cow<'_, str> {
    let b = text.as_bytes();
    if memchr2(b'$', b'@', b).is_none() {
        let mut out = String::with_capacity(text.len());
        push_escaped(&mut out, text);
        return Cow::Owned(out);
    }
    let n = b.len();
    let mut out = String::with_capacity(n + n / 4 + 16);
    let mut i = 0usize;
    let mut copy_from = 0usize;
    while i < n {
        if i + 1 < n && b[i] == b'$' && b[i + 1] == b'(' {
            push_escaped(&mut out, &text[copy_from..i]);
            i += 2;
            let start = i;
            let mut depth = 1usize;
            while i < n && depth > 0 {
                match b[i] {
                    b'(' => depth += 1,
                    b')' => depth -= 1,
                    _ => {}
                }
                i += 1;
            }
            if depth > 0 {
                push_escaped(&mut out, &text[start - 2..i]);
            } else {
                let inner = &text[start..i - 1];
                let display =
                    inner.len() > 50 || inner.contains("\\begin{") || inner.contains("\\[");
                if display {
                    out.push_str("$$");
                } else {
                    out.push('$');
                }
                out.push_str(inner);
                if display {
                    out.push_str("$$");
                } else {
                    out.push('$');
                }
            }
            copy_from = i;
            continue;
        }
        if i + 1 < n && b[i] == b'@' && b[i + 1] == b'(' {
            push_escaped(&mut out, &text[copy_from..i]);
            i += 2;
            let s = i;
            while i < n && b[i] != b')' {
                i += 1;
            }
            if i == n {
                push_escaped(&mut out, &text[s - 2..i]);
                copy_from = i;
                continue;
            }
            let styles_raw = &text[s..i];
            i += 1;
            let c0 = i;
            while i + 1 < n && !(b[i] == b'@' && b[i + 1] == b'(') {
                i += 1;
            }
            let content = &text[c0..i];
            out.push_str("<span style=\"");
            let mut css: SmallVec<[&str; 4]> = SmallVec::new();
            for tok in styles_raw
                .split(',')
                .map(|s| s.trim())
                .filter(|t| !t.is_empty())
            {
                css.push(tok);
            }
            for (idx, tok) in css.iter().enumerate() {
                if idx > 0 {
                    out.push(';');
                }
                match tok.to_ascii_lowercase().as_str() {
                    "bold" => out.push_str("font-weight:bold"),
                    "italic" => out.push_str("font-style:italic"),
                    "underline" => out.push_str("text-decoration:underline"),
                    "strike" => out.push_str("text-decoration:line-through"),
                    _ if tok.contains(':') || tok.starts_with("color:") => out.push_str(tok),
                    _ => {
                        out.push_str("color:");
                        out.push_str(tok);
                    }
                }
            }
            out.push_str("\">");
            push_escaped(&mut out, content);
            out.push_str("</span>");
            copy_from = i;
            continue;
        }
        i += 1;
    }
    if copy_from < n {
        push_escaped(&mut out, &text[copy_from..]);
    }
    Cow::Owned(out)
}

pub trait HeaderExt {
    fn into_headers(self) -> Vec<String>;
}

impl<I> HeaderExt for I
where
    I: IntoIterator,
    I::Item: ToString,
{
    fn into_headers(self) -> Vec<String> {
        self.into_iter()
            .map(|h| process_webrust_styles(&h.to_string()).into_owned())
            .collect()
    }
}

impl TableBuilder {
    pub fn new(data: Vec<Vec<Cell>>) -> Self {
        Self {
            data,
            headers: None,
            row_headers: None,
            rendered: false,
            merge_enabled: false,
            position: None,
            size: None,
        }
    }

    pub fn header(mut self, headers: impl HeaderExt) -> Self {
        self.headers = Some(headers.into_headers());
        self
    }

    pub fn merge(mut self) -> Self {
        self.merge_enabled = true;
        self
    }

    pub fn at(mut self, x: f64, y: f64) -> Self {
        self.position = Some((x, y));
        self
    }

    pub fn pivot(mut self) -> Self {
        if self.data.is_empty() {
            return self;
        }
        if let Some(headers) = self.headers.take() {
            self.row_headers = Some(headers);
        }
        let rows = self.data.len();
        let cols = self.data.iter().map(|r| r.len()).max().unwrap_or(0);
        let mut pivoted = Vec::with_capacity(cols);
        for j in 0..cols {
            let mut row = Vec::with_capacity(rows);
            for i in 0..rows {
                if j < self.data[i].len() {
                    row.push(self.data[i][j].clone());
                } else {
                    row.push(Cell::empty());
                }
            }
            pivoted.push(row);
        }
        self.data = pivoted;
        self
    }

    fn render(&mut self) -> String {
        if self.rendered || self.data.is_empty() {
            self.rendered = true;
            return "<p>(empty)</p>".into();
        }
        self.rendered = true;
        self.normalize_table_size();
        if self.merge_enabled {
            self.apply_merge();
        } else {
            self.compute_span();
        }
        let cells_count: usize = self.data.iter().map(|r| r.len()).sum();
        let mut html = String::with_capacity((cells_count << 6) + 256);
        html.push_str("<table class=\"webrust-table\">\n");
        if let Some(headers) = &self.headers {
            html.push_str("<thead><tr>");
            if self.row_headers.is_some() {
                html.push_str("<th class=\"no-border\"></th>");
            }
            for h in headers {
                html.push_str("<th class=\"webrust-th-header\">");
                html.push_str(h);
                html.push_str("</th>");
            }
            html.push_str("</tr></thead>\n");
        }
        html.push_str("<tbody>\n");
        let mut itoa_buf = itoa::Buffer::new();
        for (i, row) in self.data.iter().enumerate() {
            html.push_str("<tr>");
            if let Some(rh) = &self.row_headers {
                html.push_str("<th class=\"webrust-th-header\">");
                html.push_str(rh.get(i).map(|s| s.as_str()).unwrap_or(""));
                html.push_str("</th>");
            }
            for cell in row.iter().filter(|c| c.rowspan > 0 && c.colspan > 0) {
                let (tag, class) = match cell.status {
                    CellStatus::Key => ("th", "webrust-th-header"),
                    CellStatus::Value => (
                        "td",
                        if cell.is_number {
                            "webrust-td-number"
                        } else {
                            "webrust-td-value"
                        },
                    ),
                    CellStatus::Empty => ("td", "no-border"),
                };
                html.push('<');
                html.push_str(tag);
                html.push_str(" class=\"");
                html.push_str(class);
                html.push('"');
                if cell.rowspan > 1 {
                    html.push_str(" rowspan=\"");
                    html.push_str(itoa_buf.format(cell.rowspan));
                    html.push('"');
                }
                if cell.colspan > 1 {
                    html.push_str(" colspan=\"");
                    html.push_str(itoa_buf.format(cell.colspan));
                    html.push('"');
                }
                html.push('>');
                html.push_str(&cell.content);
                html.push_str("</");
                html.push_str(tag);
                html.push('>');
            }
            html.push_str("</tr>\n");
        }
        html.push_str("</tbody></table>");
        html
    }

    #[inline]
    fn normalize_table_size(&mut self) {
        if self.data.is_empty() {
            return;
        }
        let max_cols = self.data.iter().map(|row| row.len()).max().unwrap_or(0);
        for row in &mut self.data {
            row.resize_with(max_cols, Cell::empty);
        }
    }

    fn apply_merge(&mut self) {
        if self.data.is_empty() {
            return;
        }
        for row in &mut self.data {
            let mut j = 0usize;
            while j < row.len() {
                if row[j].colspan == 0
                    || (row[j].content.is_empty() && row[j].status != CellStatus::Empty)
                {
                    j += 1;
                    continue;
                }
                let content_len = row[j].content.len();
                let status = row[j].status.clone();
                let is_num = row[j].is_number;
                let mut span = 1usize;
                for k in (j + 1)..row.len() {
                    let c = &row[k];
                    if c.status != status
                        || c.is_number != is_num
                        || c.content.len() != content_len
                        || c.content != row[j].content
                    {
                        break;
                    }
                    row[k].colspan = 0;
                    span += 1;
                }
                row[j].colspan = span;
                j += span;
            }
        }
        let rows = self.data.len();
        let cols = self.data.iter().map(|r| r.len()).max().unwrap_or(0);
        for j in 0..cols {
            let mut i = 0usize;
            while i < rows {
                if i >= self.data.len()
                    || j >= self.data[i].len()
                    || self.data[i][j].rowspan == 0
                    || (self.data[i][j].content.is_empty()
                        && self.data[i][j].status != CellStatus::Empty)
                {
                    i += 1;
                    continue;
                }
                let content_len = self.data[i][j].content.len();
                let status = self.data[i][j].status.clone();
                let colspan = self.data[i][j].colspan;
                let is_num = self.data[i][j].is_number;
                let mut span = 1usize;
                for k in (i + 1)..rows {
                    if k >= self.data.len() || j >= self.data[k].len() {
                        break;
                    }
                    let c = &self.data[k][j];
                    if c.status != status
                        || c.colspan != colspan
                        || c.is_number != is_num
                        || c.content.len() != content_len
                        || c.content != self.data[i][j].content
                    {
                        break;
                    }
                    self.data[k][j].rowspan = 0;
                    span += 1;
                }
                self.data[i][j].rowspan = span;
                i += span;
            }
        }
    }

    #[inline]
    fn compute_span(&mut self) {
        for cell in self.data.iter_mut().flatten() {
            if cell.rowspan == 0 {
                cell.rowspan = 1;
            }
            if cell.colspan == 0 {
                cell.colspan = 1;
            }
        }
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
                let table_id = format!(
                    "wrtable_{}",
                    TABLE_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
                );
                let (left, top) = crate::layout::coord::to_screen_coords(x, y);
                let (width, height) = self.size.unwrap_or((300, 200));
                let mut b = itoa::Buffer::new();
                let mut html = String::with_capacity(table_html.len() + 512);
                html.push_str("<div id=\"");
                html.push_str(&table_id);
                html.push_str("\" style=\"position:absolute;left:");
                html.push_str(b.format(left as i32));
                html.push_str("px;top:");
                html.push_str(b.format(top as i32));
                html.push_str("px;width:");
                html.push_str(b.format(width as i32));
                html.push_str("px;height:");
                html.push_str(b.format(height as i32));
                html.push_str("px;overflow:hidden;transform:translate(-50%,-50%);box-sizing:border-box;\"><div style=\"width:100%;height:100%;overflow:auto;padding:2px;\">");
                html.push_str(&table_html);
                html.push_str("</div></div><script>(function(){var e=document.getElementById('");
                html.push_str(&table_id);
                html.push_str("');if(e){var t=e.querySelector('table');if(t){t.style.fontSize='9px';t.style.width='98%';t.style.margin='0 auto';var c=t.querySelectorAll('th,td');c.forEach(function(x){x.style.padding='3px 4px';x.style.fontSize='9px';x.style.whiteSpace='nowrap';x.style.lineHeight='1.2';});}}})();</script>");
                crate::io::gui::add_output(html);
            } else {
                let mut html = String::with_capacity(table_html.len() + 128);
                html.push_str(r#"<div class="webrust-line" style="margin-top:0px;margin-bottom:6px;line-height:1.2;">"#);
                html.push_str(&table_html);
                html.push_str("</div>");
                crate::io::gui::add_output_new_line(html);
            }
        }
    }
}

pub fn table<T: Serialize>(data: &T) -> TableBuilder {
    let val = to_value(data).unwrap_or(Value::Null);
    let cells = match &val {
        Value::Object(map) if !map.is_empty() && map.values().all(|v| v.is_object()) => {
            flatten_nested_object(map)
        }
        _ => value_to_cells(&val),
    };
    let mut builder = TableBuilder::new(cells);
    if let Value::Object(map) = &val {
        if map.values().any(|v| v.is_object()) {
            builder = builder.merge();
        }
    }
    builder
}

fn value_to_cells(value: &Value) -> Vec<Vec<Cell>> {
    match value {
        Value::Array(arr) if !arr.is_empty() => {
            if arr.iter().all(|v| {
                matches!(
                    v,
                    Value::Number(_) | Value::String(_) | Value::Bool(_) | Value::Null
                )
            }) {
                let mut row = Vec::with_capacity(arr.len());
                for v in arr {
                    row.push(Cell::val(v));
                }
                vec![row]
            } else if arr.iter().all(|v| v.is_array()) {
                let mut matrix: Vec<Vec<Cell>> = Vec::with_capacity(arr.len());
                let mut max_cols = 0usize;
                for v in arr {
                    if let Some(inner) = v.as_array() {
                        let mut r = Vec::with_capacity(inner.len());
                        for x in inner {
                            r.push(Cell::val(x));
                        }
                        max_cols = max_cols.max(r.len());
                        matrix.push(r);
                    }
                }
                for r in &mut matrix {
                    r.resize_with(max_cols, Cell::empty);
                }
                matrix
            } else {
                let mut out = Vec::with_capacity(arr.len());
                for v in arr {
                    out.push(vec![Cell::val(v)]);
                }
                out
            }
        }
        Value::Object(obj) => {
            let mut out = Vec::with_capacity(obj.len());
            for (k, v) in obj {
                let mut row = Vec::with_capacity(
                    1 + if v.is_array() {
                        v.as_array().map(|a| a.len()).unwrap_or(1)
                    } else {
                        1
                    },
                );
                row.push(Cell::key(k));
                match v {
                    Value::Array(arr) => row.extend(arr.iter().map(Cell::val)),
                    _ => row.push(Cell::val(v)),
                }
                out.push(row);
            }
            out
        }
        _ => vec![vec![Cell::val(value)]],
    }
}

fn flatten_nested_object(map: &Map<String, Value>) -> Vec<Vec<Cell>> {
    type KeyPath = Vec<String>;
    type KeyPathSet = BTreeMap<KeyPath, ()>;
    type DataMap = BTreeMap<String, BTreeMap<KeyPath, Vec<String>>>;
    fn walk(path: KeyPath, val: &Value) -> (KeyPathSet, DataMap) {
        let mut key_paths = BTreeMap::new();
        let mut data: DataMap = BTreeMap::new();
        if let Value::Object(map) = val {
            for (k, v) in map {
                let mut new_path = path.clone();
                new_path.push(k.clone());
                let (kp, d) = walk(new_path, v);
                key_paths.extend(kp);
                for (prop, vm) in d {
                    data.entry(prop).or_default().extend(vm);
                }
            }
        } else if let Some(prop_key) = path.last() {
            let col_path = path[..path.len() - 1].to_vec();
            key_paths.insert(col_path.clone(), ());
            let values = match val {
                Value::String(s) => s.split(',').map(|s| s.trim().to_string()).collect(),
                _ => vec![format_json(val)],
            };
            data.entry(prop_key.clone())
                .or_default()
                .insert(col_path, values);
        }
        (key_paths, data)
    }
    let mut key_paths = BTreeMap::new();
    let mut data: DataMap = BTreeMap::new();
    for (k, v) in map.iter() {
        let (kp, d) = walk(vec![k.to_string()], v);
        key_paths.extend(kp);
        for (prop, vm) in d {
            data.entry(prop).or_default().extend(vm);
        }
    }
    let sorted_key_paths: Vec<Vec<String>> = key_paths.into_keys().collect();
    let col_header_depth = sorted_key_paths.iter().map(|k| k.len()).max().unwrap_or(0);
    let mut table = Vec::with_capacity(
        col_header_depth
            + data
                .values()
                .map(|vm| vm.values().map(|v| v.len()).max().unwrap_or(1))
                .sum::<usize>(),
    );
    table.extend((0..col_header_depth).map(|d| {
        let mut row = Vec::with_capacity(1 + sorted_key_paths.len());
        row.push(Cell::empty());
        row.extend(
            sorted_key_paths
                .iter()
                .map(|key_path| Cell::key(&key_path.get(d).cloned().unwrap_or_default())),
        );
        row
    }));
    let prop_rows: Vec<Vec<Cell>> = data
        .into_iter()
        .flat_map(|(prop, values_map)| {
            let max_rows = values_map.values().map(|v| v.len()).max().unwrap_or(1);
            (0..max_rows)
                .map(|i| {
                    let mut row = Vec::with_capacity(1 + sorted_key_paths.len());
                    row.push(Cell::key(&prop));
                    row.extend(sorted_key_paths.iter().map(|key_path| {
                        Cell::val_from_str(
                            &values_map
                                .get(key_path)
                                .and_then(|v| v.get(i))
                                .cloned()
                                .unwrap_or_default(),
                        )
                    }));
                    row
                })
                .collect::<Vec<_>>()
        })
        .collect();
    table.extend(prop_rows);
    table
}

#[inline]
fn format_json(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                itoa::Buffer::new().format(i).to_string()
            } else if let Some(u) = n.as_u64() {
                itoa::Buffer::new().format(u).to_string()
            } else if let Some(f) = n.as_f64() {
                ryu::Buffer::new().format(f).to_string()
            } else {
                n.to_string()
            }
        }
        Value::Bool(b) => {
            if *b {
                "true".into()
            } else {
                "false".into()
            }
        }
        Value::Null => String::new(),
        _ => v.to_string(),
    }
}

impl Cell {
    #[inline]
    fn key(s: &str) -> Self {
        Self {
            content: process_webrust_styles(s).into_owned(),
            status: CellStatus::Key,
            rowspan: 1,
            colspan: 1,
            is_number: false,
        }
    }

    #[inline]
    fn val(v: &Value) -> Self {
        let content = format_json(v);
        let is_number = matches!(v, Value::Number(_));
        Self {
            content: process_webrust_styles(&content).into_owned(),
            status: CellStatus::Value,
            rowspan: 1,
            colspan: 1,
            is_number,
        }
    }

    #[inline]
    fn val_from_str(s: &str) -> Self {
        let is_number = s.trim().parse::<f64>().is_ok();
        Self {
            content: process_webrust_styles(s).into_owned(),
            status: CellStatus::Value,
            rowspan: 1,
            colspan: 1,
            is_number,
        }
    }

    #[inline]
    fn empty() -> Self {
        Self {
            content: String::new(),
            status: CellStatus::Empty,
            rowspan: 1,
            colspan: 1,
            is_number: false,
        }
    }
}
