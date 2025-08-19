// webrust/webrust-macros/src/lib.rs
//! # Procedural Macros for webrust
//!
//! Provides the `#[gui]` attribute macro that transforms regular Rust functions
//! into web-based interactive applications with advanced f-string processing.
//!
//! ## Features
//!
//! - **F-string transformation** - Converts `{variable}` syntax to proper formatting
//! - **Expression evaluation** - Supports complex Rust expressions in strings
//! - **Format specifiers** - Rich formatting options (`:c`, `:j`, `:e`, `:.2`, etc.)
//! - **GUI integration** - Automatic web server and styling setup
//! - **LaTeX processing** - Handles `$(...)` mathematical expressions
//! - **Theme support** - Customizable colors, fonts, and styling
//!
//! ## The `#[gui]` Macro
//!
//! Transforms a regular `main()` function into a web application:
//!
//!
//! #[gui]
//! fn main() {
//!     let name = "Alice";
//!     let age = 30;
//!     println("Hello {name}, you are {age} years old!");
//! }
//!
//!
//! ## Theme Configuration
//!
//!
//! #[gui(bg = "navy", fg = "white", font = "Arial", color = "cyan", size = "14px")]
//! fn main() {
//!     println("Styled application!");
//! }
//!
//!
//! ## F-String Processing
//!
//! The macro intelligently processes string literals in `print!` and `println!` calls:
//!
//! - **Variables**: `{name}` → `format!("{}", name)`
//! - **Expressions**: `{name.len()}` → `format!("{}", name.len())`
//! - **Formatting**: `{pi:.2}` → `format!("{:.2}", pi)`
//! - **Complex**: `{numbers:j}` → `webrust_format_json_proper(&numbers)`
//!
//! ## LaTeX Integration
//!
//! Automatically detects and preserves `$(...)` mathematical expressions
//! while processing the rest of the string for f-string substitutions.
//!
//! ## Implementation
//!
//! Uses `syn` for AST manipulation and `quote` for code generation.
//! Implements a visitor pattern to transform expression trees while
//! preserving LaTeX regions and handling complex nested expressions.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, visit_mut::{self, VisitMut}, Expr, ExprCall, ExprPath, Lit, ExprLit};

struct FStringTransformer;

impl VisitMut for FStringTransformer {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Call(ExprCall { func, args, .. }) = expr {
            if let Expr::Path(ExprPath { path, .. }) = func.as_ref() {
                if path.segments.len() == 1 && (path.segments[0].ident == "println" || path.segments[0].ident == "print") {
                    if let Some(Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. })) = args.first() {
                        let template = lit_str.value();
                        let tr = extract_and_process_variables(&template);
                        if !tr.is_empty() {
                            let name = &path.segments[0].ident;
                            let mut rep = quote! { let mut result = String::from(#template); };
                            for (orig, expr_str, fmt) in tr {
                                let tokens = match expr_str.parse::<proc_macro2::TokenStream>() {
                                    Ok(t) => t, Err(_) => quote! { #expr_str }
                                };
                                let fe = match fmt.as_deref() {
                                    Some(":c") => quote! { format!("{:?}", #tokens) },
                                    Some(":j") => quote! { webrust_format_json_proper(&#tokens) },
                                    Some(":.2") => quote! { format!("{:.2}", #tokens) },
                                    Some(":.6") => quote! { format!("{:.6}", #tokens) },
                                    Some(":e") => quote! { format!("{:e}", #tokens) },
                                    Some(":.0") => quote! { format!("{:.0}", #tokens) },
                                    Some(":04") => quote! { format!("{:04}", #tokens) },
                                    Some(":x") => quote! { format!("{:x}", #tokens) },
                                    Some(":X") => quote! { format!("{:X}", #tokens) },
                                    Some(":b") => quote! { format!("{:b}", #tokens) },
                                    Some(":o") => quote! { format!("{:o}", #tokens) },
                                    None => quote! { format!("{}", #tokens) },
                                    _ => quote! { format!("{}", #tokens) },
                                };
                                rep = quote! { #rep result = result.replace(#orig, &#fe); };
                            }
                            let new_expr = quote! { #name({ #rep result }) };
                            *expr = syn::parse2(new_expr).unwrap();
                            return;
                        }
                    }
                }
            }
        }
        visit_mut::visit_expr_mut(self, expr);
    }
}

fn find_latex_ranges(s: &str) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    let c: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i + 1 < c.len() {
        if c[i] == '$' && c[i + 1] == '(' {
            let start = i;
            i += 2;
            let mut lvl = 1;
            while i < c.len() && lvl > 0 {
                if c[i] == '(' { lvl += 1; }
                else if c[i] == ')' { lvl -= 1; }
                i += 1;
            }
            if lvl == 0 { v.push((start, i)); }
        } else { i += 1; }
    }
    v
}

fn is_valid_rust_expression(expr: &str) -> bool {
    let t = expr.trim();
    if t.is_empty() { return false; }
    let ok = |c: char| c.is_alphanumeric() || "_.:+-*/%=!<>&|^?()[] \t".contains(c);
    t.chars().all(ok)
}

fn extract_and_process_variables(tpl: &str) -> Vec<(String, String, Option<String>)> {
    let ranges = find_latex_ranges(tpl);
    let inside = |pos: usize| -> bool { ranges.iter().any(|(s, e)| pos >= *s && pos < *e) };
    let mut out = Vec::new();
    let c: Vec<char> = tpl.chars().collect();
    let mut i = 0;
    while i < c.len() {
        if c[i] == '{' {
            let start = i;
            i += 1;
            let mut b = 1;
            let mut v = Vec::new();
            while i < c.len() && b > 0 {
                if c[i] == '{' { b += 1; }
                else if c[i] == '}' { b -= 1; }
                if b > 0 { v.push(c[i]); }
                i += 1;
            }
            if b == 0 && !inside(start) {
                let var: String = v.iter().collect();
                if let Some(p) = var.find(':') {
                    if p > 0 {
                        let vp = var[..p].trim();
                        let fp = &var[p..];
                        if !vp.is_empty() && is_valid_rust_expression(vp) {
                            if start <= i && i <= c.len() {
                                let orig: String = c[start..i].iter().collect();
                                out.push((orig, vp.to_string(), Some(fp.to_string())));
                            }
                        }
                    }
                } else if !var.trim().is_empty() && is_valid_rust_expression(var.trim()) {
                    if start <= i && i <= c.len() {
                        let orig: String = c[start..i].iter().collect();
                        out.push((orig, var.trim().to_string(), None));
                    }
                }
            }
        } else {
            i += 1;
        }
    }
    out
}

fn parse_gui_args(args: TokenStream) -> (String, String, String, String, String) {
    let mut bg = "white".to_string();
    let mut fg = "lightgray".to_string();
    let mut font = "Arial, sans-serif".to_string();
    let mut color = "black".to_string();
    let mut size = "14px".to_string();
    if !args.is_empty() {
        let args_str = args.to_string();
        for pair in args_str.split(',') {
            let pair = pair.trim();
            if let Some((key, value)) = pair.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                match key {
                    "bg" => bg = value.to_string(),
                    "fg" => fg = value.to_string(),
                    "font" => font = value.to_string(),
                    "color" => color = value.to_string(),
                    "size" => size = value.to_string(),
                    _ => {}
                }
            }
        }
    }
    (bg, fg, font, color, size)
}

#[proc_macro_attribute]
pub fn gui(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as ItemFn);
    FStringTransformer.visit_item_fn_mut(&mut f);
    let b = &f.block;
    let (bg, fg, font, color, size) = parse_gui_args(args);
    let expanded = quote! {
        #[allow(unused_variables)]
        fn main() {
            use ::webrust::serde;
            use ::webrust::serde_json;
            
            fn webrust_format_json_clean(s: &str) -> String {
                format!(r#"<pre style="font-family:'Courier New',monospace;color:#333;background:#f8f9fa;padding:8px;border-radius:4px;border:1px solid #e2e8f0;white-space:pre-wrap;">{}</pre>"#, s)
            }
            fn webrust_format_json_proper<T: serde::Serialize>(data: &T) -> String {
                let json_value = serde_json::to_value(data).unwrap_or(serde_json::Value::Null);
                webrust_format_json_value(&json_value, 0)
            }
            fn webrust_format_json_value(value: &serde_json::Value, indent: usize) -> String {
                match value {
                    serde_json::Value::Array(arr) => {
                        if arr.is_empty() { return "[]".to_string(); }
                        if arr.iter().all(|v| matches!(v, serde_json::Value::Number(_))) {
                            let items: Vec<String> = arr.iter().map(|v| webrust_format_json_value(v, 0)).collect();
                            return format!("[{}]", items.join(", "));
                        }
                        if arr.iter().all(|v| {
                            if let serde_json::Value::Array(inner) = v {
                                inner.iter().all(|iv| matches!(iv, serde_json::Value::Number(_)))
                            } else { false }
                        }) {
                            let indent_str = "&nbsp;".repeat((indent + 1) * 4);
                            let items: Vec<String> = arr.iter().map(|v| webrust_format_json_value(v, indent + 1)).collect();
                            return format!("[\n{}{}\n{}]", indent_str, items.join(&format!(",\n{}", indent_str)), "&nbsp;".repeat(indent * 4));
                        }
                        let indent_str = "&nbsp;".repeat((indent + 1) * 4);
                        let items: Vec<String> = arr.iter().map(|v| format!("{}{}", indent_str, webrust_format_json_value(v, indent + 1))).collect();
                        format!("[\n{}\n{}]", items.join(",\n"), "&nbsp;".repeat(indent * 4))
                    }
                    serde_json::Value::Object(obj) => {
                        if obj.is_empty() { return "{}".to_string(); }
                        let indent_str = "&nbsp;".repeat((indent + 1) * 4);
                        let mut entries: Vec<_> = obj.iter().collect();
                        entries.sort_by_key(|(k, _)| *k);
                        let items: Vec<String> = entries.iter().map(|(k, v)| format!("{}\"{}\": {}", indent_str, k, webrust_format_json_value(v, indent + 1))).collect();
                        format!("{{\n{}\n{}}}", items.join(",\n"), "&nbsp;".repeat(indent * 4))
                    }
                    serde_json::Value::String(s) => format!("\"{}\"", s),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Null => "null".to_string(),
                }
            }
            fn println<T: std::fmt::Display>(text: T) -> webrust::print::PrintBox { webrust::print::println_str(text) }
            fn print<T: std::fmt::Display>(text: T) -> webrust::print::PrintBox { webrust::print::print_str(text) }
            use webrust::input::input_with_validation as input;
            let style_config = webrust::io::gui::StyleConfig {
                bg: #bg.to_string(),
                fg: #fg.to_string(),
                font: #font.to_string(),
                color: #color.to_string(),
                size: #size.to_string(),
            };
            webrust::io::gui::start_gui_server_with_style(style_config, || { #b });
        }
    };
    TokenStream::from(expanded)
}