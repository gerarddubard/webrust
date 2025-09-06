// webrust/src/data/table.rs
//! # Professional Table Generation System
//!
//! A comprehensive, intelligent table generation system that automatically transforms 
//! any serializable data into beautiful, interactive tables with advanced formatting,
//! LaTeX support, and professional styling.
//!
//! ## 🎯 Core Features
//!
//! - **🔍 Smart Data Analysis** - Automatically detects optimal table layout
//! - **📊 Multi-Level Headers** - Column and row headers with hierarchical support  
//! - **🔄 Pivot Operations** - Transpose data for different analytical views
//! - **🎨 Visual Grouping** - Merge identical cells for clean presentation
//! - **📐 LaTeX Integration** - Mathematical formulas and scientific notation
//! - **🎪 webrust Styling** - Colors, formatting, and visual enhancement
//! - **🏗️ Nested Structures** - Complex hierarchical data handling
//! - **📱 Professional Output** - Publication-quality HTML tables
//!
//! ## 📋 Supported Data Types
//!
//! ### Basic Collections
//! - **Vectors** - `Vec<T>`, `Vec<Vec<T>>` (matrices)
//! - **HashMaps** - `HashMap<K,V>`, nested HashMaps
//! - **Arrays** - Fixed-size arrays and nested arrays
//!
//! ### Data Types
//! - **Numbers** - All numeric types (i32, f64, etc.)
//! - **Strings** - Text data with formatting support
//! - **Booleans** - True/false values
//! - **Custom Types** - Any type implementing `Serialize`
//!
//! ## 🚀 Quick Start Examples
//!
//! ### Simple Vector Operations
//!
//! // Basic vector
//! let numbers = vec![10, 20, 30, 40, 50];
//! table(&numbers);
//!
//! // With headers
//! table(&numbers).header(["A", "B", "C", "D", "E"]);
//!
//! // Pivoted view
//! table(&numbers).header(["A", "B", "C", "D", "E"]).pivot();
//!
//! // Pivoted with new headers
//! table(&numbers).header(["A", "B", "C", "D", "E"]).pivot().header(["Values"]);
//!
//!
//! ### Matrix Data
//!
//! // 2x3 Matrix
//! let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
//! table(&matrix);
//!
//! // With headers
//! table(&matrix).header(["x", "y", "z"]);
//!
//! // Pivoted with LaTeX headers
//! table(&matrix).header(["x", "y", "z"]).pivot().header(["$(\\vec{u})", "$(\\vec{v})"]);
//!
//!
//! ### HashMap Collections
//!
//! // Simple HashMap
//! let mut scores = HashMap::new();
//! scores.insert("Alice", 95);
//! scores.insert("Bob", 87);
//! scores.insert("Charlie", 92);
//! table(&scores);
//!
//! // HashMap with vectors
//! let mut grades = HashMap::new();
//! grades.insert("Math", vec![18, 16, 19]);
//! grades.insert("Physics", vec![15, 17, 16]);
//! table(&grades);
//! table(&grades).pivot();  // Different analytical view
//!
//!
//! ## 🔄 Advanced Operations
//!
//! ### Employee Data Management
//!
//! let employees = vec![
//!     vec!["Alice", "25", "Engineer"],
//!     vec!["Bob", "30", "Designer"],
//!     vec!["Charlie", "35", "Manager"],
//! ];
//!
//! table(&employees).header(["Name", "Age", "Job"]);
//! table(&employees).header(["Name", "Age", "Job"]).pivot();  // Jobs as columns
//!
//!
//! ### Complex Nested Structures
//!
//! // 2-level nesting with LaTeX in keys
//! let mut complex = HashMap::new();
//! let mut paris_data = HashMap::new();
//! paris_data.insert("population", 2_200_000);
//! paris_data.insert("area ($(km^2))", 105);  // LaTeX in keys!
//! complex.insert("Paris", paris_data);
//!
//! let mut lyon_data = HashMap::new();
//! lyon_data.insert("population", 515_000);
//! lyon_data.insert("area ($(km^2))", 47);
//! complex.insert("Lyon", lyon_data);
//!
//! table(&complex);  // Automatic layout
//! table(&complex).pivot();  // Alternative view
//!
//!
//! ### 3-Level Hierarchical Data
//!
//! // Complex city data structure
//! let mut cities_data = HashMap::new();
//! let mut france = HashMap::new();
//! let mut paris = HashMap::new();
//! paris.insert("population".to_string(), "2.2M".to_string());
//! paris.insert("attractions".to_string(), "Eiffel Tower, Louvre".to_string());
//! france.insert("Paris".to_string(), paris);
//!
//! let mut marseille = HashMap::new();
//! marseille.insert("population".to_string(), "870K".to_string());
//! marseille.insert("attractions".to_string(), "Old Port, Calanques".to_string());
//! france.insert("Marseille".to_string(), marseille);
//!
//! cities_data.insert("France".to_string(), france);
//! // ... add USA data similarly
//!
//! table(&cities_data);  // Automatically flattens complex structure
//! table(&cities_data).pivot();  // Different analytical perspective
//!
//!
//! ## 🎨 Visual Grouping with `.merge()`
//!
//! ### Survey Data Grouping
//!
//! let survey_data = vec![
//!     vec!["Excellent", "Customer Service"],
//!     vec!["Excellent", "Product Quality"],
//!     vec!["Excellent", "Website Design"],
//!     vec!["Good", "Delivery Speed"],
//!     vec!["Good", "Ordering Process"],
//!     vec!["Average", "Price"],
//!     vec!["Average", "Support Hours"],
//! ];
//!
//! // Default - each cell separate
//! table(&survey_data).header(["Rating", "Aspect"]);
//!
//! // With visual grouping - identical adjacent cells merge
//! table(&survey_data).header(["Rating", "Aspect"]).merge();
//!
//!
//! ### Color Matrix Visualization
//!
//! let color_matrix = vec![
//!     vec!["Red", "Red", "Blue"],
//!     vec!["Red", "Red", "Blue"],
//!     vec!["Green", "Green", "Blue"],
//! ];
//!
//! table(&color_matrix).header(["1", "2", "3"]).merge();
//! // Perfect for visualizing color regions or patterns
//!
//!
//! ## 📐 Mathematical Examples
//!
//! ### Boolean Logic Tables
//!
//! let truth_table = vec![
//!     vec!["0", "0", "0", "0"],
//!     vec!["0", "1", "0", "1"],
//!     vec!["1", "0", "0", "1"],
//!     vec!["1", "1", "1", "1"],
//! ];
//! table(&truth_table).header([
//!     "$(A)",
//!     "$(B)", 
//!     "$(A \\land B)",
//!     "$(A \\lor B)"
//! ]);
//!
//!
//! ### Pascal's Triangle Generation
//!
//! let mut pascal_triangle = Vec::new();
//! for n in 0.to(9) {
//!     let mut row = Vec::new();
//!     for k in 0.to(n + 1) {
//!         let mut c = 1u32;
//!         for i in 0.to(k) {
//!             c = c * (n - i) as u32 / (i + 1) as u32;
//!         }
//!         row.push(c.to_string());
//!     }
//!     pascal_triangle.push(row);
//! }
//! table(&pascal_triangle);  // Preserves triangular structure
//!
//!
//! ### Multiplication Tables
//!
//! let headers: Vec<String> = (1..=9).map(|i| format!("x{}", i)).collect();
//! let data: Vec<Vec<u32>> = (1..=9).map(|i| (1..=9).map(|j| i * j).collect()).collect();
//! table(&data).header(headers.clone()).pivot().header(headers);
//!
//!
//! ## 🔬 Advanced LaTeX Examples
//!
//! ### Simple Trigonometric Values
//!
//! let trig = vec![
//!     vec!["$(0)",      "$(0)",           "$(1)",           "$(0)"],
//!     vec!["$(\\pi/4)",  "$(\\sqrt{2}/2)", "$(\\sqrt{2}/2)", "$(1)"],
//!     vec!["$(\\pi/2)",  "$(1)",           "$(0)",           "not defined"],
//! ];
//! table(&trig).header([
//!     "$(\\theta)",
//!     "$(\\sin\\theta)",
//!     "$(\\cos\\theta)",
//!     "$(\\tan\\theta)",
//! ]);
//!
//!
//! ### Simple Physics Formulas
//!
//! let formulas = vec![
//!     vec!["Energy", "$(E = mc^2)"],
//!     vec!["Force", "$(F = ma)"],
//!     vec!["Momentum", "$(p = mv)"],
//!     vec!["Power", "$(P = Fv)"],
//! ];
//! table(&formulas).header(["Concept", "Formula"]);
//!
//!
//! ### Complex Equations (Raw Strings Recommended)
//!
//! let equations = vec![
//!     vec!["Maxwell 1", r"$(\nabla \cdot \mathbf{E} = \frac{\rho}{\epsilon_0})"],
//!     vec!["Maxwell 2", r"$(\nabla \cdot \mathbf{B} = 0)"],
//!     vec!["Maxwell 3", r"$(\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t})"],
//!     vec!["Schrödinger", r"$(i\hbar\frac{\partial}{\partial t}\Psi = \hat{H}\Psi)"],
//! ];
//! table(&equations).header(["Equation", "Mathematical Form"]);
//!
//!
//! ### 2D Transformation Matrices
//!
//! let transforms_2d = vec![
//!     vec!["Rotation", r"$(\begin{pmatrix} \cos\theta & -\sin\theta \\ \sin\theta & \cos\theta \end{pmatrix})"],
//!     vec!["Scaling", r"$(\begin{pmatrix} s_x & 0 \\ 0 & s_y \end{pmatrix})"],
//!     vec!["Reflection X", r"$(\begin{pmatrix} 1 & 0 \\ 0 & -1 \end{pmatrix})"],
//! ];
//! table(&transforms_2d).header(["Transform", "2D Matrix"]);
//!
//!
//! ### 3D Homogeneous Matrices (Raw Strings Essential)
//!
//! let transforms_3d = vec![
//!     vec!["3D Rotation Z", r"$(\begin{pmatrix} \cos\theta & -\sin\theta & 0 & 0 \\ \sin\theta & \cos\theta & 0 & 0 \\ 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & 1 \end{pmatrix})"],
//!     vec!["3D Scaling", r"$(\begin{pmatrix} s_x & 0 & 0 & 0 \\ 0 & s_y & 0 & 0 \\ 0 & 0 & s_z & 0 \\ 0 & 0 & 0 & 1 \end{pmatrix})"],
//!     vec!["3D Translation", r"$(\begin{pmatrix} 1 & 0 & 0 & t_x \\ 0 & 1 & 0 & t_y \\ 0 & 0 & 1 & t_z \\ 0 & 0 & 0 & 1 \end{pmatrix})"],
//! ];
//! table(&transforms_3d).header(["Transform", "3D Homogeneous Matrix"]);
//!
//!
//! ### Greek Alphabet Reference
//!
//! let greek = vec![
//!     vec!["Alpha", "$(\\alpha)", "$(A)"],
//!     vec!["Beta", "$(\\beta)", "$(B)"],
//!     vec!["Gamma", "$(\\gamma)", "$(\\Gamma)"],
//!     vec!["Delta", "$(\\delta)", "$(\\Delta)"],
//!     vec!["Epsilon", "$(\\epsilon)", "$(E)"],
//!     vec!["Pi", "$(\\pi)", "$(\\Pi)"],
//! ];
//! table(&greek).header(["Name", "Lowercase", "Uppercase"]);
//!
//!
//! ### Complex Mathematical Expressions
//!
//! let complex_math = vec![
//!     vec!["Fourier Transform", r"$(\mathcal{F}\{f(t)\} = \int_{-\infty}^{\infty} f(t) e^{-2\pi i \xi t} dt)"],
//!     vec!["Gaussian Integral", r"$(\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi})"],
//!     vec!["Euler's Identity", "$(e^{i\\pi} + 1 = 0)"], // Simple enough without raw string
//!     vec!["Riemann Zeta", r"$(\zeta(s) = \sum_{n=1}^{\infty} \frac{1}{n^s})"],
//! ];
//! table(&complex_math).header(["Name", "Expression"]);
//!
//!
//! ## 🎛️ API Reference
//!
//! ### Core Functions
//! - `table<T: Serialize>(data: &T) -> TableBuilder` - Create table from any serializable data
//!
//! ### TableBuilder Methods
//! - `.header<I, S>(headers: I) -> Self` - Add column headers
//! - `.pivot() -> Self` - Transpose rows and columns
//! - `.merge() -> Self` - Enable visual grouping of identical cells
//!
//! ## 🚦 Best Practices
//!
//! ### When to Use `.merge()`
//! - **Survey data** with repeated categories (Excellent, Good, Average)
//! - **Color matrices** with adjacent identical values
//! - **Status classifications** with grouped entries
//!
//! ### When to Use `.pivot()`
//! - **Employee data** - transform employees to columns
//! - **Grade analysis** - subjects as rows vs columns
//! - **City comparisons** - different geographical perspectives
//!
//! ### LaTeX Guidelines
//! - **Simple formulas**: `"$(E = mc^2)"` - regular strings work fine
//! - **Complex expressions**: `r"$(\frac{\partial}{\partial t})"` - raw strings for clarity
//! - **Matrices**: `r"$(\begin{pmatrix}...)"` - raw strings essential for readability
//!
//! ## 💡 Pro Tips
//!
//! 1. **Raw strings**: Use `r"..."` for complex LaTeX with many backslashes
//! 2. **LaTeX in keys**: Mathematical notation works in HashMap keys too
//! 3. **Automatic nesting**: Trust the system to handle complex hierarchies
//! 4. **Pivot for analysis**: Different views reveal different insights
//! 5. **Merge for clarity**: Visual grouping improves readability
//!
//! ## 🔧 Implementation Details
//!
//! - **Data Analysis** - Uses serde_json for structure inspection
//! - **HTML Generation** - Professional CSS-styled tables
//! - **LaTeX Rendering** - MathJax integration for mathematical notation
//! - **Styling Support** - webrust color and formatting system
//! - **Automatic Layout** - Intelligent row/column organization
//! - **Type Detection** - Numbers, text, and mixed data handling
//!
//! The table system handles everything from simple vectors to complex nested
//! structures, automatically choosing the most readable presentation format.

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
        Self { data, headers: None, row_headers: None, rendered: false, merge_enabled: false }
    }
    pub fn header<I, S>(mut self, headers: I) -> Self where I: IntoIterator<Item = S>, S: Into<String> {
        self.headers = Some(headers.into_iter().map(|h| process_webrust_styles(&h.into())).collect()); self
    }
    pub fn merge(mut self) -> Self { self.merge_enabled = true; self }
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
        if self.rendered || self.data.is_empty() { self.rendered = true; return "<p>(empty)</p>".into(); }
        self.rendered = true;
        self.normalize_table_size();
        if self.merge_enabled { self.apply_merge(); } else { self.compute_span(); }
        let mut html = String::from("<table class=\"webrust-table\">\n");
        if let Some(headers) = &self.headers {
            html.push_str("<thead><tr>");
            if self.row_headers.is_some() { html.push_str("<th class=\"no-border\"></th>"); }
            headers.iter().for_each(|h| html.push_str(&format!("<th class=\"webrust-th-header\">{}</th>", esc(h))));
            html.push_str("</tr></thead>\n");
        }
        html.push_str("<tbody>\n");
        for (i, row) in self.data.iter().enumerate() {
            html.push_str("<tr>");
            if let Some(rh) = &self.row_headers {
                html.push_str(&format!("<th class=\"webrust-th-header\">{}</th>", esc(rh.get(i).unwrap_or(&String::new()))));
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
        html.push_str("</tbody></table>"); html
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

impl Drop for TableBuilder {
    fn drop(&mut self) {
        if !self.rendered { crate::prelude::add_output(format!("SIMPLE_TABLE:{}", self.render())); }
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

fn esc(s: &str) -> String { s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#39;") }
fn is_numeric(s: &str) -> bool { s.trim().parse::<f64>().is_ok() }
trait IsPrimitive { fn is_primitive(&self) -> bool; }
impl IsPrimitive for Value { fn is_primitive(&self) -> bool { matches!(self, Value::Number(_) | Value::Bool(_) | Value::Null) } }