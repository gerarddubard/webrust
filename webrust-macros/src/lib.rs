// webrust/webrust-macros/src/lib.rs
use memchr::{memchr, memchr2, memmem};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use smallvec::SmallVec;
use std::borrow::Cow;
use syn::{
    parse_macro_input,
    visit_mut::{visit_expr_mut, VisitMut},
    Expr, ExprCall, ExprLit, ExprPath, ItemFn, Lit,
};

type TS = proc_macro2::TokenStream;

#[inline]
fn has_fstring_or_latex(b: &[u8]) -> bool {
    memchr(b'{', b).is_some() || memmem::find(b, b"$(").is_some()
}

#[inline]
fn cut(s: &str) -> (&str, Option<&str>) {
    let b = s.as_bytes();
    let (n, mut i, mut p, mut a, mut br) = (b.len(), 0, 0, 0, 0);
    while i < n {
        match b[i] {
            b'(' => p += 1,
            b')' => p -= 1,
            b'<' => a += 1,
            b'>' => a -= 1,
            b'[' => br += 1,
            b']' => br -= 1,
            b':' if p == 0 && a == 0 && br == 0 => {
                if i + 1 < n && b[i + 1] == b':' {
                    i += 2;
                    continue;
                }
                let (e, x) = s.split_at(i);
                return (e.trim(), Some(x[1..].trim()));
            }
            _ => {}
        }
        i += 1;
    }
    (s.trim(), None)
}

#[inline]
fn latex_ranges(t: &str) -> SmallVec<[(usize, usize); 8]> {
    let b = t.as_bytes();
    let mut v = SmallVec::new();
    let mut i = 0;
    while let Some(pos) = memmem::find(&b[i..], b"$(") {
        let s = i + pos;
        let mut d = 1;
        let mut j = s + 2;
        while j < b.len() && d > 0 {
            match b[j] {
                b'(' => d += 1,
                b')' => d -= 1,
                _ => {}
            }
            j += 1;
        }
        v.push((s, j));
        i = j;
    }
    v
}

#[inline]
fn escape_braces_cow(s: &str) -> Cow<'_, str> {
    if memchr2(b'{', b'}', s.as_bytes()).is_none() {
        return Cow::Borrowed(s);
    }
    let mut o = String::with_capacity(s.len() + s.len() / 10 + 8);
    for ch in s.chars() {
        match ch {
            '{' => o.push_str("{{"),
            '}' => o.push_str("}}"),
            _ => o.push(ch),
        }
    }
    Cow::Owned(o)
}

#[inline]
fn find_matching_brace(b: &[u8], mut pos: usize) -> Option<usize> {
    let mut depth: i32 = 1;
    while depth > 0 {
        let off = memchr2(b'{', b'}', &b[pos..])?;
        pos += off;
        match b[pos] {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            _ => {}
        }
        pos += 1;
    }
    Some(pos)
}

fn trans(t: &str) -> (String, SmallVec<[TS; 8]>) {
    let b = t.as_bytes();
    if !has_fstring_or_latex(b) {
        return (t.to_string(), SmallVec::new());
    }
    let n = b.len();
    let rs = latex_ranges(t);
    let mut fmt = String::with_capacity(n + rs.len() * 16);
    let mut args: SmallVec<[TS; 8]> = SmallVec::new();
    let (mut r, mut i, mut last) = (0, 0, 0);
    while i < n {
        if r < rs.len() && i == rs[r].0 {
            let esc = escape_braces_cow(&t[last..rs[r].1]);
            fmt.push_str(esc.as_ref());
            i = rs[r].1;
            last = i;
            r += 1;
            continue;
        }
        match b[i] {
            b'{' => {
                if i + 1 < n && b[i + 1] == b'{' {
                    fmt.push_str(&t[last..i + 2]);
                    i += 2;
                    last = i;
                    continue;
                }
                fmt.push_str(&t[last..i]);
                i += 1;
                let s = i;
                let end = match find_matching_brace(b, i) {
                    Some(p) => p,
                    None => {
                        fmt.push_str("{:?}");
                        break;
                    }
                };
                i = end;
                let e = i - 1;
                let inner = t[s..e].trim();
                if inner.is_empty() {
                    fmt.push_str("{:?}");
                    last = i;
                    continue;
                }
                let (ex, sp) = cut(inner);
                if let Ok(expr) = syn::parse_str::<Expr>(ex) {
                    match sp {
                        Some("?") => {
                            fmt.push_str("{:?}");
                            args.push(expr.into_token_stream());
                        }
                        Some("c") => {
                            fmt.push_str("{}");
                            args.push(quote! { format!("{:?}", #expr) });
                            last = i;
                            continue;
                        }
                        Some("j") => {
                            fmt.push_str("{}");
                            args.push(quote! { __w_json(&#expr) });
                            last = i;
                            continue;
                        }
                        Some(sp) => {
                            fmt.push('{');
                            fmt.push(':');
                            fmt.push_str(sp);
                            fmt.push('}');
                            args.push(expr.into_token_stream());
                        }
                        None => {
                            fmt.push_str("{}");
                            args.push(expr.into_token_stream());
                        }
                    }
                } else {
                    fmt.push('{');
                    fmt.push_str(inner);
                    fmt.push('}');
                }
                last = i;
            }
            b'}' => {
                if i + 1 < n && b[i + 1] == b'}' {
                    fmt.push_str(&t[last..i + 2]);
                    i += 2;
                    last = i;
                } else {
                    fmt.push_str(&t[last..=i]);
                    i += 1;
                    last = i;
                }
            }
            _ => {
                i += 1;
            }
        }
    }
    if last < n {
        fmt.push_str(&t[last..]);
    }
    (fmt, args)
}

struct R;
impl VisitMut for R {
    fn visit_expr_mut(&mut self, e: &mut Expr) {
        if let Expr::Call(ExprCall { func, args, .. }) = e {
            if let Expr::Path(ExprPath { path, .. }) = func.as_ref() {
                if path.segments.len() == 1 {
                    let id = &path.segments[0].ident;
                    let id_str = id.to_string();
                    if id_str == "println" || id_str == "print" {
                        if let Some(Expr::Lit(ExprLit {
                            lit: Lit::Str(s), ..
                        })) = args.first()
                        {
                            let (f, a) = trans(&s.value());
                            let lit = syn::LitStr::new(&f, s.span());
                            *e = syn::parse2(quote! { #id(format!(#lit #(, #a)*)) }).unwrap();
                            return;
                        }
                    }
                }
            }
        }
        visit_expr_mut(self, e);
    }
}

#[inline]
fn parse_gui_args(ts: TokenStream) -> (String, String, String, String) {
    let mut font = "Arial".to_string();
    let mut size = "14px".to_string();
    let mut color = "black".to_string();
    let mut bg = "white".to_string();

    let s = ts.to_string();
    if s.is_empty() {
        return (font, size, color, bg);
    }

    for tok in s.split_whitespace() {
        let tok = tok.trim_matches(|c| c == ',' || c == '"');
        if tok.is_empty() {
            continue;
        }

        if tok.starts_with('!') {
            bg = tok[1..].into();
        } else if tok.chars().next().map_or(false, |c| c.is_uppercase()) {
            font = tok.replace('_', " ");
        } else if tok.chars().next().map_or(false, |c| c.is_ascii_digit()) {
            size = tok.into();
        } else {
            color = tok.into();
        }
    }

    (font, size, color, bg)
}

#[proc_macro_attribute]
pub fn gui(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as ItemFn);
    R.visit_item_fn_mut(&mut f);
    let (font, size, color, bg) = parse_gui_args(attr);
    let body = &f.block;
    let wrapped = quote! {{
        webrust::io::print::set_defaults(#color.to_string(), #font.to_string(), #size.to_string());

        fn __w_json<T: webrust::serde::Serialize>(v: &T) -> String {
            use webrust::serde_json::Value;
            fn write_number(n: &webrust::serde_json::Number, out: &mut String) {
                if let Some(i) = n.as_i64() {
                    let mut b = webrust::itoa::Buffer::new();
                    out.push_str(b.format(i));
                } else if let Some(u) = n.as_u64() {
                    let mut b = webrust::itoa::Buffer::new();
                    out.push_str(b.format(u));
                } else if let Some(f) = n.as_f64() {
                    let mut b = webrust::ryu::Buffer::new();
                    out.push_str(b.format(f));
                } else {
                    out.push_str(&n.to_string());
                }
            }
            fn fmt_into(val: &Value, depth: usize, out: &mut String) {
                match val {
                    Value::Array(arr) => {
                        if arr.is_empty() {
                            out.push_str("[]");
                            return;
                        }
                        if arr.len() <= 3 && arr.iter().all(|x| x.is_number()) {
                            out.push('[');
                            for (i, x) in arr.iter().enumerate() {
                                if i > 0 {
                                    out.push(' ');
                                }
                                if let Value::Number(n) = x {
                                    write_number(n, out)
                                } else {
                                    fmt_into(x, depth, out)
                                }
                            }
                            out.push(']');
                            return;
                        }
                        let ind = "    ".repeat(depth);
                        let inn = "    ".repeat(depth + 1);
                        out.push('[');
                        out.push('\n');
                        for x in arr.iter() {
                            out.push_str(&inn);
                            fmt_into(x, depth + 1, out);
                            out.push('\n');
                        }
                        out.push_str(&ind);
                        out.push(']');
                    }
                    Value::Object(obj) => {
                        if obj.is_empty() {
                            out.push_str("{}");
                            return;
                        }
                        let mut kv: Vec<_> = obj.iter().collect();
                        kv.sort_by(|a, b| a.0.cmp(b.0));
                        let ind = "    ".repeat(depth);
                        let inn = "    ".repeat(depth + 1);
                        out.push('{');
                        out.push('\n');
                        for (k, v) in kv.into_iter() {
                            out.push_str(&inn);
                            out.push('"');
                            out.push_str(k);
                            out.push_str(r#"": "#);
                            fmt_into(v, depth + 1, out);
                            out.push('\n');
                        }
                        out.push_str(&ind);
                        out.push('}');
                    }
                    Value::String(s) => {
                        out.push('"');
                        out.push_str(s);
                        out.push('"');
                    }
                    Value::Number(n) => write_number(n, out),
                    Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
                    Value::Null => out.push_str("null"),
                }
            }
            let val = webrust::serde_json::to_value(v).unwrap_or(Value::Null);
            let mut raw = String::with_capacity(128);
            fmt_into(&val, 0, &mut raw);
            let mut escaped = String::with_capacity(raw.len() + raw.len() / 10 + 32);
            for ch in raw.chars() {
                match ch {
                    '&' => escaped.push_str("&amp;"),
                    '<' => escaped.push_str("&lt;"),
                    '>' => escaped.push_str("&gt;"),
                    ' ' => escaped.push_str("&nbsp;"),
                    _ => escaped.push(ch),
                }
            }
            format!(
                r#"<div style="font-family:'Courier New',monospace;color:#1e40af;font-size:12px;line-height:1.3;white-space:pre;">{}</div>"#,
                escaped
            )
        }

        let style = webrust::io::gui::StyleConfig {
            bg: #bg.into(),
            color: #color.into(),
            font: #font.into(),
            size: #size.into(),
        };
        webrust::io::gui::start_gui_server_with_style(style, || { #body });
    }};
    f.block = syn::parse2(wrapped).unwrap();
    TokenStream::from(quote! { #[allow(unused_variables, dead_code)] #f })
}
