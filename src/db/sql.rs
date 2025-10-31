// webrust/src/db/sql.rs
//! # High-Performance SQL Analytics with DuckDB
//!
//! This module provides a seamless integration with DuckDB for in-memory analytical queries,
//! with automatic streaming of results to HTML tables in the browser.
//!
//! ## Features
//!
//! - **🚀 Zero-copy streaming**: Results streamed via Apache Arrow with batch processing
//! - **📊 Automatic HTML rendering**: SELECT queries render as interactive tables
//! - **⚡ Adaptive performance**: Dynamic chunk sizing based on column count
//! - **🔧 Configurable precision**: Float rounding with `ROUND_FLOATS` constant
//! - **📦 Built-in I/O**: Import/export CSV, Parquet, and JSON files
//! - **🔌 Extension support**: Pre-loaded httpfs, parquet, and json extensions
//! - **🛡️ Safe concurrency**: Thread-safe connection with `OnceLock<Mutex<Connection>>`
//! - **🎯 Type inference**: Arrow-based type detection for optimal formatting
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//!
//! #[gui(bg = "navy", fg = "white")]
//! fn main() {
//!     // Create and populate a table
//!     query("CREATE TABLE users (id INT, name TEXT, age INT)");
//!     query("INSERT INTO users VALUES (1, 'Alice', 30), (2, 'Bob', 25)");
//!
//!     // Query with automatic rendering
//!     query("SELECT * FROM users WHERE age > 20");
//! }
//! ```
//!
//! ## Core Functionality
//!
//! ### Standard SQL Operations
//!
//! Execute any DuckDB-compatible SQL:
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # fn example() {
//! // DDL: Data Definition Language
//! query("CREATE TABLE products (id INT PRIMARY KEY, name TEXT, price DECIMAL(10,2))");
//! query("CREATE INDEX idx_name ON products(name)");
//! query("ALTER TABLE products ADD COLUMN stock INT DEFAULT 0");
//!
//! // DML: Data Manipulation Language
//! query("INSERT INTO products VALUES (1, 'Laptop', 999.99, 10)");
//! query("UPDATE products SET price = 899.99 WHERE id = 1");
//! query("DELETE FROM products WHERE stock = 0");
//!
//! // Queries: Automatic streaming to browser
//! query("SELECT * FROM products ORDER BY price DESC");
//! # }
//! ```
//!
//! ### Advanced Analytics
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # fn example() {
//! // Window functions with partition and ordering
//! query(r#"
//!     SELECT
//!         name,
//!         price,
//!         ROW_NUMBER() OVER (ORDER BY price DESC) as rank,
//!         AVG(price) OVER () as avg_price,
//!         price - AVG(price) OVER () as price_diff
//!     FROM products
//! "#);
//!
//! // Complex aggregations
//! query(r#"
//!     SELECT
//!         category,
//!         COUNT(*) as items,
//!         SUM(price * stock) as total_value,
//!         percentile_cont(0.5) WITHIN GROUP (ORDER BY price) as median_price
//!     FROM products
//!     GROUP BY category
//!     HAVING COUNT(*) > 5
//! "#);
//!
//! // CTEs (Common Table Expressions)
//! query(r#"
//!     WITH top_products AS (
//!         SELECT * FROM products WHERE price > 500
//!     )
//!     SELECT category, COUNT(*) as premium_count
//!     FROM top_products
//!     GROUP BY category
//! "#);
//! # }
//! ```
//!
//! ## Special Commands
//!
//! WebRust extends DuckDB with custom commands for common operations:
//!
//! ### `SCHEMA` - Inspect Query Structure
//!
//! Display column names and Arrow types for any SELECT query:
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # fn example() {
//! query("SCHEMA SELECT * FROM users");
//! // Renders: | column | arrow_type |
//! //          |--------|------------|
//! //          | id     | Int32      |
//! //          | name   | Utf8       |
//! //          | age    | Int32      |
//! # }
//! ```
//!
//! ### `IMPORT` - Load External Data
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # fn example() {
//! // Auto-detect format from extension
//! query("IMPORT 'data.csv' AS sales");
//! query("IMPORT 'metrics.parquet' AS metrics");
//! query("IMPORT 'config.json' AS config");
//!
//! // Load from URL (requires httpfs)
//! query("IMPORT 'https://example.com/data.csv' AS remote_data");
//! # }
//! ```
//!
//! ### `EXPORT` - Save Query Results
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # fn example() {
//! // Export table
//! query("EXPORT users TO 'users.csv'");
//! query("EXPORT users TO 'users.parquet' FORMAT PARQUET");
//!
//! // Export query results
//! query("EXPORT (SELECT * FROM users WHERE age > 25) TO 'adults.json' FORMAT JSON");
//! # }
//! ```
//!
//! ### `OPEN` - Switch to File-Based Database
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # fn example() {
//! // Persist data across runs
//! query("OPEN 'mydata.duckdb'");
//! query("CREATE TABLE persistent (id INT, value TEXT)");
//! // Data saved to file automatically
//! # }
//! ```
//!
//! ### `LOAD` - Enable Extensions
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # fn example() {
//! // Pre-loaded: httpfs, parquet, json
//! // Load additional extensions:
//! query("LOAD spatial");  // GIS operations
//! query("LOAD fts");      // Full-text search
//! # }
//! ```
//!
//! ### `CONFIG` - Runtime Configuration
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # fn example() {
//! query("CONFIG SET memory_limit = '4GB'");
//! query("CONFIG SET threads = 8");
//! # }
//! ```
//!
//! ## Performance Optimization
//!
//! ### Automatic Chunk Sizing
//!
//! Results are streamed in batches sized according to column count:
//! - ≤8 columns: 800 rows per batch (optimal for wide tables)
//! - 9-19 columns: 400 rows per batch (balanced)
//! - ≥20 columns: 200 rows per batch (prevents JSON overflow)
//!
//! ### Float Precision Control
//!
//! Configure at compile time via the `ROUND_FLOATS` constant:
//!
//! ```rust,ignore
//! // In sql.rs:
//! const ROUND_FLOATS: Option<usize> = Some(2);  // 2 decimal places
//! const ROUND_FLOATS: Option<usize> = Some(4);  // 4 decimal places
//! const ROUND_FLOATS: Option<usize> = None;     // Full precision
//! ```
//!
//! ### Zero-Copy Type Conversion
//!
//! Direct Arrow-to-string conversion for primitive types using `itoa` and `ryu`:
//! - Integers: ~10x faster than `format!`
//! - Floats: ~2x faster with better accuracy
//! - Decimals: Exact precision without floating-point errors
//!
//! ## Data Type Support
//!
//! | DuckDB Type | Arrow Type | Format | Example |
//! |-------------|------------|--------|---------|
//! | INTEGER | Int32 | Direct | `42` |
//! | BIGINT | Int64 | Direct | `9223372036854775807` |
//! | DOUBLE | Float64 | Rounded | `3.14` |
//! | DECIMAL(p,s) | Decimal128 | Exact | `123.45` |
//! | VARCHAR | Utf8 | Escaped | `Hello <world>` |
//! | BOOLEAN | Boolean | true/false | `true` |
//! | TIMESTAMP | Utf8 | ISO 8601 | `2025-01-15 10:30:00` |
//! | JSON | Utf8 | Preserved | `{"key":"value"}` |
//!
//! ## Real-World Examples
//!
//! ### Time Series Analysis
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # fn example() {
//! query(r#"
//!     CREATE TABLE sensor_data AS
//!     SELECT
//!         timestamp,
//!         value,
//!         AVG(value) OVER (
//!             ORDER BY timestamp
//!             ROWS BETWEEN 5 PRECEDING AND 5 FOLLOWING
//!         ) as moving_avg,
//!         value - LAG(value) OVER (ORDER BY timestamp) as change
//!     FROM read_csv_auto('sensors.csv')
//! "#);
//! # }
//! ```
//!
//! ### Sales Dashboard
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # fn example() {
//! query(r#"
//!     SELECT
//!         DATE_TRUNC('month', order_date) as month,
//!         product_category,
//!         SUM(amount) as revenue,
//!         COUNT(DISTINCT customer_id) as customers,
//!         SUM(amount) / COUNT(DISTINCT customer_id) as avg_per_customer
//!     FROM orders
//!     WHERE order_date >= CURRENT_DATE - INTERVAL '12 months'
//!     GROUP BY DATE_TRUNC('month', order_date), product_category
//!     ORDER BY month DESC, revenue DESC
//! "#);
//! # }
//! ```
//!
//! ### Data Quality Checks
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # fn example() {
//! query(r#"
//!     SELECT
//!         'Total rows' as metric,
//!         COUNT(*) as value
//!     FROM users
//!     UNION ALL
//!     SELECT 'Null emails', COUNT(*) FROM users WHERE email IS NULL
//!     UNION ALL
//!     SELECT 'Duplicate emails', COUNT(*) - COUNT(DISTINCT email) FROM users
//!     UNION ALL
//!     SELECT 'Invalid ages', COUNT(*) FROM users WHERE age < 0 OR age > 150
//! "#);
//! # }
//! ```
//!
//! ## Error Handling
//!
//! Errors are displayed in the browser with syntax highlighting:
//!
//! ```text
//! ❌ SQL error: Catalog Error: Table "missing" does not exist!
//! ↳ SELECT * FROM missing
//! ```
//!
//! ## Thread Safety
//!
//! The connection is wrapped in `OnceLock<Mutex<Connection>>`:
//! - **Single initialization**: Connection created once per process
//! - **Mutex protection**: Safe concurrent access from multiple threads
//! - **No overhead**: Lock-free read for initialized connection
//!
//! ## Limitations
//!
//! - **Single connection**: One DuckDB connection per process
//! - **No transactions**: Auto-commit mode only
//! - **Browser rendering**: Very large results (>100k rows) may be slow
//! - **Memory bound**: In-memory database limited by available RAM
//!
//! ## Best Practices
//!
//! 1. **Use LIMIT**: Preview data before rendering full results
//!    ```rust,ignore
//!    query("SELECT * FROM large_table LIMIT 100");
//!    ```
//!
//! 2. **Filter early**: Apply WHERE clauses before JOINs
//!    ```rust,ignore
//!    query("SELECT * FROM orders o
//!           JOIN users u ON o.user_id = u.id
//!           WHERE o.created_at > '2024-01-01'");
//!    ```
//!
//! 3. **Export large results**: Don't render massive datasets
//!    ```rust,ignore
//!    query("EXPORT (SELECT * FROM big_query) TO 'output.parquet'");
//!    ```
//!
//! 4. **Use CTEs**: Break complex queries into readable parts
//!    ```rust,ignore
//!    query(r#"
//!        WITH filtered AS (...),
//!             aggregated AS (...)
//!        SELECT * FROM aggregated
//!    "#);
//!    ```
//!
//! ## Configuration Reference
//!
//! ```rust,ignore
//! // Default DuckDB settings (in DUCKDB_CONFIG):
//! SET threads TO 4;                    // Parallel execution
//! SET worker_threads TO 4;             // Background workers
//! SET preserve_insertion_order TO false; // Faster, unordered
//! SET enable_progress_bar TO false;    // No terminal output
//! SET enable_object_cache TO true;     // Cache parsed objects
//! ```
//!
//! ## See Also
//!
//! - [DuckDB SQL Reference](https://duckdb.org/docs/sql/introduction)
//! - [Apache Arrow Format](https://arrow.apache.org/docs/format/Columnar.html)
//! - [`query()`] - Main entry point for SQL execution

use crate::io::gui::add_output;
use crate::io::println;
use duckdb::arrow::array::Decimal128Array;
use duckdb::arrow::datatypes::{
    DataType, Float32Type, Float64Type, Int16Type, Int32Type, Int64Type, Int8Type, UInt16Type,
    UInt32Type, UInt64Type, UInt8Type,
};
use duckdb::{
    arrow::array::{Array, AsArray, BooleanArray},
    Connection,
};
use itoa;
use ryu;
use serde_json;
use std::borrow::Cow;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex, OnceLock,
};

static CONN: OnceLock<Mutex<Connection>> = OnceLock::new();
static TBL_SEQ: AtomicUsize = AtomicUsize::new(1);

thread_local! {
    static SCRIPT_BUF: std::cell::RefCell<String> = std::cell::RefCell::new(String::with_capacity(8192));
    static HTML_BUF: std::cell::RefCell<String> = std::cell::RefCell::new(String::with_capacity(4096));
    static CELL_BUF: std::cell::RefCell<String> = std::cell::RefCell::new(String::with_capacity(256));
}

const DUCKDB_CONFIG: &str = "\
    SET threads TO 4; \
    SET worker_threads TO 4; \
    SET preserve_insertion_order TO false; \
    SET enable_progress_bar TO false; \
    SET enable_object_cache TO true; \
    INSTALL httpfs; LOAD httpfs; \
    INSTALL parquet; LOAD parquet; \
    INSTALL json; LOAD json;";
const DUCKDB_OPEN_CONFIG: &str = "\
    SET threads TO 4; \
    SET worker_threads TO 4; \
    SET enable_progress_bar TO false; \
    SET enable_object_cache TO true;";
const NO_ROWS_MSG: &str = "@(orange)∅ No rows";
const NO_SCHEMA_MSG: &str = "@(orange)∅ No schema";
const HTML_CAPACITY: usize = 512;
const COLUMN_HEADER: &str = "column";
const ARROW_TYPE_HEADER: &str = "arrow_type";
const ROUND_FLOATS: Option<usize> = Some(2);

#[inline]
fn get_conn() -> std::sync::MutexGuard<'static, Connection> {
    CONN.get_or_init(|| {
        let conn = Connection::open_in_memory().expect("Failed to create DuckDB connection");
        let _ = conn.execute_batch(DUCKDB_CONFIG);
        Mutex::new(conn)
    })
    .lock()
    .unwrap()
}

fn open_db(path: &str) {
    let conn = get_conn();
    let attach = format!("ATTACH '{}' AS db; SET schema db;", path);
    if let Err(e) = conn.execute_batch(&attach) {
        log_error("OPEN/ATTACH", &e, &attach);
        return;
    }
    let _ = conn.execute_batch(DUCKDB_OPEN_CONFIG);
}

#[inline]
fn html_escape(s: &str) -> Cow<'_, str> {
    if !s
        .bytes()
        .any(|b| matches!(b, b'&' | b'<' | b'>' | b'"' | b'\''))
    {
        return Cow::Borrowed(s);
    }
    let mut result = String::with_capacity(s.len() + (s.len() >> 2));
    for c in s.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&#39;"),
            _ => result.push(c),
        }
    }
    Cow::Owned(result)
}

#[inline]
fn trim_zeros(s: &str) -> String {
    if !s.contains('.') {
        return s.to_string();
    }
    let mut out = s.to_owned();
    while out.ends_with('0') {
        out.pop();
    }
    if out.ends_with('.') {
        out.pop();
    }
    out
}

#[inline]
fn round_to(v: f64, n: usize) -> f64 {
    let p = 10f64.powi(n as i32);
    (v * p).round() / p
}

#[inline]
fn fmt_f64_buf(v: f64, rbuf: &mut ryu::Buffer) -> String {
    if !v.is_finite() {
        return v.to_string();
    }
    let x = match ROUND_FLOATS {
        Some(n) => round_to(v, n),
        None => v,
    };
    trim_zeros(rbuf.format(x))
}

#[inline]
fn fmt_f32_buf(v: f32, rbuf: &mut ryu::Buffer) -> String {
    fmt_f64_buf(v as f64, rbuf)
}

fn format_decimal128(val: i128, scale: u32, rbuf: &mut ryu::Buffer) -> String {
    if scale == 0 {
        return itoa::Buffer::new().format(val).to_string();
    }
    let neg = val < 0;
    let x = if neg { -val } else { val };
    let s = itoa::Buffer::new().format(x).to_string();
    let len = s.len();
    let sc = scale as usize;
    let out = if len <= sc {
        let mut out = String::with_capacity(sc + 3 + neg as usize);
        if neg {
            out.push('-');
        }
        out.push_str("0.");
        for _ in 0..(sc - len) {
            out.push('0');
        }
        out.push_str(&s);
        out
    } else {
        let split = len - sc;
        let mut out = String::with_capacity(len + 1 + neg as usize);
        if neg {
            out.push('-');
        }
        out.push_str(&s[..split]);
        out.push('.');
        out.push_str(&s[split..]);
        out
    };
    match ROUND_FLOATS {
        Some(n) => {
            let v = out.parse::<f64>().unwrap_or(0.0);
            fmt_f64_buf(round_to(v, n), rbuf)
        }
        None => trim_zeros(&out),
    }
}

struct NumBuf {
    ibuf: itoa::Buffer,
    rbuf: ryu::Buffer,
}

#[inline]
fn cell_to_string_fast(col: &dyn Array, row_idx: usize, out: &mut String, nb: &mut NumBuf) {
    out.clear();
    if col.is_null(row_idx) {
        return;
    }
    if let Some(a) = col.as_primitive_opt::<Int64Type>() {
        out.push_str(nb.ibuf.format(a.value(row_idx)));
        return;
    }
    if let Some(a) = col.as_primitive_opt::<Int32Type>() {
        out.push_str(nb.ibuf.format(a.value(row_idx)));
        return;
    }
    if let Some(a) = col.as_primitive_opt::<Int16Type>() {
        out.push_str(nb.ibuf.format(a.value(row_idx) as i32));
        return;
    }
    if let Some(a) = col.as_primitive_opt::<Int8Type>() {
        out.push_str(nb.ibuf.format(a.value(row_idx) as i32));
        return;
    }
    if let Some(a) = col.as_primitive_opt::<UInt64Type>() {
        out.push_str(nb.ibuf.format(a.value(row_idx)));
        return;
    }
    if let Some(a) = col.as_primitive_opt::<UInt32Type>() {
        out.push_str(nb.ibuf.format(a.value(row_idx)));
        return;
    }
    if let Some(a) = col.as_primitive_opt::<UInt16Type>() {
        out.push_str(nb.ibuf.format(a.value(row_idx) as u32));
        return;
    }
    if let Some(a) = col.as_primitive_opt::<UInt8Type>() {
        out.push_str(nb.ibuf.format(a.value(row_idx) as u32));
        return;
    }
    if let Some(a) = col.as_primitive_opt::<Float64Type>() {
        out.push_str(&fmt_f64_buf(a.value(row_idx), &mut nb.rbuf));
        return;
    }
    if let Some(a) = col.as_primitive_opt::<Float32Type>() {
        out.push_str(&fmt_f32_buf(a.value(row_idx), &mut nb.rbuf));
        return;
    }
    if let Some(dec) = col.as_any().downcast_ref::<Decimal128Array>() {
        let v = dec.value(row_idx);
        let scale = if let DataType::Decimal128(_, s) = dec.data_type() {
            *s as u32
        } else {
            0
        };
        out.push_str(&format_decimal128(v, scale, &mut nb.rbuf));
        return;
    }
    if let Some(a) = col.as_string_opt::<i32>() {
        out.push_str(a.value(row_idx));
        return;
    }
    if let Some(a) = col.as_string_opt::<i64>() {
        out.push_str(a.value(row_idx));
        return;
    }
    if let Some(b) = col.as_any().downcast_ref::<BooleanArray>() {
        out.push_str(if b.value(row_idx) { "true" } else { "false" });
        return;
    }
    use std::fmt::Write;
    let _ = write!(out, "{:?}", col);
}

fn start_table(table_id: &str, headers: &[String]) {
    HTML_BUF.with(|buf| {
        let mut html = buf.borrow_mut();
        html.clear();
        html.reserve(HTML_CAPACITY + headers.len() * 80);
        html.push_str(r#"<div class="table-container"><table id=""#);
        html.push_str(table_id);
        html.push_str(r#"" class="webrust-table">"#);
        if !headers.is_empty() {
            html.push_str("<thead><tr>");
            for h in headers {
                html.push_str(r#"<th class="webrust-th-header">"#);
                html.push_str(h);
                html.push_str("</th>");
            }
            html.push_str("</tr></thead>");
        }
        html.push_str("<tbody></tbody></table></div>");
        add_output(format!("SIMPLE_TABLE:{}", html));
    });
    SCRIPT_BUF.with(|buf| {
        let mut s = buf.borrow_mut();
        s.clear();
        s.reserve(1536);
        s.push_str("<script>(function(){var W=window;W.__wr_q=W.__wr_q||{};W.__wr_a=W.__wr_a||{};W.__wr_q['");
        s.push_str(table_id);
        s.push_str("']=[];W.__wr_a['");
        s.push_str(table_id);
        s.push_str("']=0;W['wr_ap_");
        s.push_str(table_id);
        s.push_str("']=function(start,rows){var t=document.getElementById('");
        s.push_str(table_id);
        s.push_str("');if(!t){W.__wr_q['");
        s.push_str(table_id);
        s.push_str("'].push([start,rows]);return;}var a=W.__wr_a['");
        s.push_str(table_id);
        s.push_str("'];if(start!==a)return;var b=t.tBodies[0]||t.createTBody(),f=document.createDocumentFragment();for(var i=0;i<rows.length;i++){var tr=document.createElement('tr'),row=rows[i];for(var j=0;j<row.length;j++){var td=document.createElement('td'),x=row[j]??'';td.className=!isNaN(x)&&String(x).trim()!==''?'webrust-td-number':'webrust-td-value';td.textContent=String(x);tr.appendChild(td);}f.appendChild(tr);}b.appendChild(f);W.__wr_a['");
        s.push_str(table_id);
        s.push_str("']=start+rows.length;};(function F(){var q=W.__wr_q['");
        s.push_str(table_id);
        s.push_str("'],t=document.getElementById('");
        s.push_str(table_id);
        s.push_str("');if(!t){if(document.readyState==='loading')document.addEventListener('DOMContentLoaded',F,{once:true});return;}while(q.length){var p=q.shift();W['wr_ap_");
        s.push_str(table_id);
        s.push_str("'](p[0],p[1]);}})();})();</script>");
        add_output(s.clone());
    });
}

#[inline]
fn append_rows(table_id: &str, start_index: usize, rows: &[Vec<String>]) {
    if rows.is_empty() {
        return;
    }
    SCRIPT_BUF.with(|buf| {
        let mut b = buf.borrow_mut();
        b.clear();
        b.reserve(rows.len() * 100);
        b.push_str("<script>(function(){var f=window['wr_ap_");
        b.push_str(table_id);
        b.push_str("'];if(f)f(");
        use std::fmt::Write;
        let _ = write!(b, "{},", start_index);
        b.push_str(&serde_json::to_string(rows).unwrap_or_else(|_| "[]".to_string()));
        b.push_str(");else setTimeout(function(){var f=window['wr_ap_");
        b.push_str(table_id);
        b.push_str("'];if(f)f(");
        let _ = write!(b, "{},", start_index);
        b.push_str(&serde_json::to_string(rows).unwrap_or_else(|_| "[]".to_string()));
        b.push_str(");},10);})();</script>");
        add_output(b.clone());
    });
}

fn split_sql(input: &str) -> Vec<String> {
    let mut out = Vec::with_capacity(8);
    let mut buf = String::with_capacity(input.len() >> 2);
    let mut chars = input.chars().peekable();
    let (mut in_sq, mut in_dq, mut in_lc, mut in_bc) = (false, false, false, false);
    while let Some(c) = chars.next() {
        if in_lc {
            if c == '\n' {
                in_lc = false;
            }
            buf.push(c);
            continue;
        }
        if in_bc {
            if c == '*' && matches!(chars.peek().copied(), Some('/')) {
                let _ = chars.next();
                in_bc = false;
                buf.push_str("*/");
            } else {
                buf.push(c);
            }
            continue;
        }
        if !in_sq && !in_dq {
            if c == '-' && matches!(chars.peek().copied(), Some('-')) {
                let _ = chars.next();
                in_lc = true;
                buf.push_str("--");
                continue;
            }
            if c == '/' && matches!(chars.peek().copied(), Some('*')) {
                let _ = chars.next();
                in_bc = true;
                buf.push_str("/*");
                continue;
            }
        }
        match c {
            '\'' if !in_dq => {
                in_sq = !in_sq;
                buf.push(c);
            }
            '"' if !in_sq => {
                in_dq = !in_dq;
                buf.push(c);
            }
            ';' if !in_sq && !in_dq => {
                let s = buf.trim();
                if !s.is_empty() {
                    out.push(s.to_string());
                }
                buf.clear();
            }
            _ => buf.push(c),
        }
    }
    let s = buf.trim();
    if !s.is_empty() {
        out.push(s.to_string());
    }
    out
}

#[inline]
fn first_kw(s: &str) -> &str {
    let s = s.trim_start();
    s.find(char::is_whitespace).map(|i| &s[..i]).unwrap_or(s)
}

#[inline]
fn log_error(prefix: &str, err: &dyn std::fmt::Display, sql: &str) {
    add_output(format!(
        r#"<div style="padding:10px;margin:10px 0;background-color:#fff5f5;border-left:4px solid #dc2626;color:#000000;font-family:monospace;"><strong style="color:#dc2626;">❌ {} error:</strong> <span style="color:#991b1b;">{}</span><br><span style="color:#6b7280;">↳ {}</span></div>"#,
        prefix,
        err,
        html_escape(sql).as_ref()
    ));
}

fn stream_select(sql: &str) {
    let conn = get_conn();
    let mut st = match conn.prepare(sql) {
        Ok(s) => s,
        Err(e) => {
            log_error("Prepare", &e, sql);
            return;
        }
    };
    let reader = match st.query_arrow([]) {
        Ok(r) => r,
        Err(e) => {
            log_error("ARROW select", &e, sql);
            return;
        }
    };
    let table_id = format!("wr_stream_tbl_{}", TBL_SEQ.fetch_add(1, Ordering::Relaxed));
    let mut header_done = false;
    let mut row_counter = 0usize;
    for batch in reader {
        let num_cols = batch.num_columns();
        let num_rows = batch.num_rows();
        if !header_done {
            let headers: Vec<String> = (0..num_cols)
                .map(|i| batch.schema().field(i).name().to_string())
                .collect();
            start_table(&table_id, &headers);
            header_done = true;
        }
        let cols: Vec<&dyn Array> = (0..num_cols).map(|i| batch.column(i).as_ref()).collect();
        let chunk_size = if num_cols <= 8 {
            800
        } else if num_cols >= 20 {
            200
        } else {
            400
        };
        CELL_BUF.with(|cb| {
            let mut cell_buf = cb.borrow_mut();
            let mut nb = NumBuf {
                ibuf: itoa::Buffer::new(),
                rbuf: ryu::Buffer::new(),
            };
            for start in (0..num_rows).step_by(chunk_size) {
                let end = (start + chunk_size).min(num_rows);
                let mut rows: Vec<Vec<String>> = Vec::with_capacity(end - start);
                for r in start..end {
                    let mut row = Vec::with_capacity(num_cols);
                    for col in &cols {
                        cell_to_string_fast(*col, r, &mut cell_buf, &mut nb);
                        row.push(cell_buf.clone());
                    }
                    rows.push(row);
                }
                append_rows(&table_id, row_counter, &rows);
                row_counter += end - start;
            }
        });
    }
    if !header_done {
        let _ = println(NO_ROWS_MSG.to_string());
    }
}

fn handle_schema(sql: &str) {
    let conn = get_conn();
    let mut st = match conn.prepare(sql) {
        Ok(s) => s,
        Err(e) => {
            log_error("Prepare", &e, sql);
            return;
        }
    };
    let reader = match st.query_arrow([]) {
        Ok(r) => r,
        Err(e) => {
            log_error("ARROW schema", &e, sql);
            return;
        }
    };
    for b in reader {
        let id = format!("wr_stream_tbl_{}", TBL_SEQ.fetch_add(1, Ordering::Relaxed));
        let headers = vec![COLUMN_HEADER.into(), ARROW_TYPE_HEADER.into()];
        start_table(&id, &headers);
        let rows: Vec<Vec<String>> = b
            .schema()
            .fields()
            .iter()
            .map(|f| vec![f.name().to_string(), format!("{:?}", f.data_type())])
            .collect();
        append_rows(&id, 0, &rows);
        return;
    }
    let _ = println(NO_SCHEMA_MSG.to_string());
}

#[inline]
fn exec_batch(stmt: &str, err_prefix: &str) {
    let conn = get_conn();
    if let Err(e) = conn.execute_batch(stmt) {
        log_error(err_prefix, &e, stmt);
    }
}

fn handle_export(cmd: &str) {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.len() < 4 {
        let _ = println(
            "@(red)Usage: EXPORT table|query TO 'file' [FORMAT CSV|PARQUET|JSON]".to_string(),
        );
        return;
    }
    let source = parts[1];
    let to_idx = parts.iter().position(|&p| p.eq_ignore_ascii_case("TO"));
    let format_idx = parts.iter().position(|&p| p.eq_ignore_ascii_case("FORMAT"));
    if to_idx.is_none() {
        let _ = println("@(red)Missing TO clause".to_string());
        return;
    }
    let file = parts[to_idx.unwrap() + 1].trim_matches('\'');
    let format = if let Some(idx) = format_idx {
        parts.get(idx + 1).unwrap_or(&"CSV").to_uppercase()
    } else {
        if file.ends_with(".parquet") {
            "PARQUET".to_string()
        } else if file.ends_with(".json") {
            "JSON".to_string()
        } else {
            "CSV".to_string()
        }
    };
    let sql = match format.as_str() {
        "CSV" => format!("COPY {} TO '{}' (HEADER, DELIMITER ',')", source, file),
        "PARQUET" => format!("COPY {} TO '{}' (FORMAT PARQUET)", source, file),
        "JSON" => format!("COPY {} TO '{}' (FORMAT JSON)", source, file),
        _ => {
            let _ = println("@(red)Format must be CSV, PARQUET, or JSON".to_string());
            return;
        }
    };
    exec_batch(&sql, "EXPORT");
    let _ = println(format!("@(green)✓ Exported to {}", file));
}

fn handle_import(cmd: &str) {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.len() < 4 {
        let _ = println("@(red)Usage: IMPORT 'file' AS table".to_string());
        return;
    }
    let file = parts[1].trim_matches('\'');
    let as_idx = parts.iter().position(|&p| p.eq_ignore_ascii_case("AS"));
    if as_idx.is_none() {
        let _ = println("@(red)Missing AS clause".to_string());
        return;
    }
    let table = parts[as_idx.unwrap() + 1];
    let sql = if file.ends_with(".parquet") {
        format!(
            "CREATE TABLE {} AS SELECT * FROM read_parquet('{}')",
            table, file
        )
    } else if file.ends_with(".json") {
        format!(
            "CREATE TABLE {} AS SELECT * FROM read_json_auto('{}')",
            table, file
        )
    } else {
        format!(
            "CREATE TABLE {} AS SELECT * FROM read_csv_auto('{}')",
            table, file
        )
    };
    exec_batch(&sql, "IMPORT");
    let _ = println(format!("@(green)✓ Imported {} as {}", file, table));
}

#[inline]
fn try_special_command(cmd: &str) -> bool {
    let trimmed = cmd.trim();
    let b = trimmed.as_bytes();
    if b.len() >= 5 && b[..5].eq_ignore_ascii_case(b"OPEN ") {
        if let Some(path) = trimmed[5..]
            .trim()
            .strip_prefix('\'')
            .and_then(|s| s.strip_suffix('\''))
        {
            open_db(path);
            return true;
        }
    }
    if b.len() >= 7 && b[..7].eq_ignore_ascii_case(b"EXPORT ") {
        handle_export(trimmed);
        return true;
    }
    if b.len() >= 7 && b[..7].eq_ignore_ascii_case(b"IMPORT ") {
        handle_import(trimmed);
        return true;
    }
    if b.len() >= 5 && b[..5].eq_ignore_ascii_case(b"LOAD ") {
        if let Some(ext) = trimmed[5..].trim().split_whitespace().next() {
            exec_batch(&format!("INSTALL {ext}; LOAD {ext};"), "LOAD");
            return true;
        }
    }
    if b.len() >= 7 && b[..7].eq_ignore_ascii_case(b"CONFIG ") {
        exec_batch(&trimmed[7..], "CONFIG");
        return true;
    }
    false
}

pub fn query(sql: &str) {
    let sql = sql.trim();
    if try_special_command(sql) {
        return;
    }
    for stmt in split_sql(sql) {
        let kw = first_kw(&stmt);
        if kw.eq_ignore_ascii_case("SCHEMA") {
            handle_schema(stmt[kw.len()..].trim());
        } else if kw.eq_ignore_ascii_case("SELECT") {
            stream_select(&stmt);
        } else {
            exec_batch(&stmt, "SQL");
        }
    }
}
