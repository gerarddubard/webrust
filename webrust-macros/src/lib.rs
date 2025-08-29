// webrust/webrust-macros/src/lib.rs
//! # Procedural Macros for WebRust
//!
//! Provides the `#[gui]` attribute macro that transforms regular Rust functions
//! into web-based interactive applications with advanced f-string processing.
//!
//! ## Key Features
//!
//! - **F-string transformation** - Python-like `{variable}` syntax with Rust performance
//! - **Expression evaluation** - Complex Rust expressions in string literals
//! - **Format specifiers** - Rich formatting (`:c`, `:j`, `:e`, `:.2`, etc.)
//! - **GUI integration** - Automatic web server with responsive styling
//! - **LaTeX processing** - Mathematical expressions with `$(...)` syntax
//! - **Theme customization** - Professional styling with CSS-like controls
//!
//! ## The `#[gui]` Macro
//!
//! Transforms any function into a web application with zero boilerplate:
//!
//! ```ignore
//! #[gui]
//! fn main() {
//!     let name = "Alice";
//!     let age = 30;
//!     println("Hello {name}, you are {age} years old!");
//! }
//! ```
//!
//! ## Professional Theming
//!
//! ```ignore
//! #[gui(bg = "navy", fg = "white", font = "Courier New", color = "cyan", size = "14px")]
//! fn main() {
//!     println("Professional styled application!");
//! }
//! ```
//!
//! ## Advanced F-String Processing
//!
//! - **Variables**: `{name}` → `format!("{}", name)`
//! - **Expressions**: `{name.len()}` → `format!("{}", name.len())`  
//! - **Formatting**: `{pi:.2}` → `format!("{:.2}", pi)`
//! - **JSON**: `{data:j}` → Pretty JSON formatting
//! - **Compact**: `{data:c}` → Compact representation
//!
//! ## Mathematical Expressions
//!
//! ```ignore
//! println("Einstein's equation: $(E = mc^2)$");
//! println("Integral: $(\\int_0^\\infty e^{-x^2} dx)$");
//! ```
//!
//! LaTeX expressions are preserved and rendered with MathJax in the browser.
//!
//! ## Zero-Cost Abstractions
//!
//! All transformations happen at compile-time using `syn` and `quote`.
//! The resulting code is as fast as hand-written Rust with no runtime overhead.

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn, Expr, ExprCall, ExprLit, ExprPath, Lit, visit_mut::{VisitMut, visit_expr_mut}};

fn cut(s:&str)->(&str,Option<&str>){
    let b=s.as_bytes(); let (n,mut i,mut p,mut a,mut br)=(b.len(),0,0,0,0);
    while i<n{
        match b[i]{
            b'('=>p+=1, b')'=>p-=1, b'<'=>a+=1, b'>'=>a-=1, b'['=>br+=1, b']'=>br-=1,
            b':' if p==0&&a==0&&br==0=>{
                if i+1<n&&b[i+1]==b':'{i+=2;continue;}
                let (e,x)=s.split_at(i); return (e.trim(),Some(x[1..].trim()));
            }
            _=>{}
        } i+=1;
    } (s.trim(),None)
}

fn latex_ranges(t:&str)->Vec<(usize,usize)>{
    let b=t.as_bytes(); let n=b.len(); let (mut i,mut v)=(0,Vec::new());
    while i+1<n{
        if b[i]==b'$'&&b[i+1]==b'('{
            let s=i; i+=2; let mut d=1i32;
            while i<n&&d>0{ if b[i]==b'(' {d+=1} else if b[i]==b')' {d-=1} i+=1; }
            v.push((s,i));
        } else { i+=1; }
    } v
}

fn esc_braces(s:&str)->String{
    let mut o=String::with_capacity(s.len());
    for ch in s.chars(){ match ch{ '{'=>o.push_str("{{"), '}'=>o.push_str("}}"), _=>o.push(ch) } }
    o
}

fn trans(t:&str)->(String,Vec<proc_macro2::TokenStream>){
    let b=t.as_bytes(); let n=b.len(); let rs=latex_ranges(t);
    let (mut r,mut i,mut last)=(0,0,0); let mut fmt=String::with_capacity(n+16); let mut args=Vec::new();
    while i<n{
        if r<rs.len()&&i==rs[r].0{ fmt.push_str(&esc_braces(&t[last..rs[r].1])); i=rs[r].1; last=i; r+=1; continue; }
        match b[i]{
            b'{'=>{
                if i+1<n&&b[i+1]==b'{'{ fmt.push_str(&t[last..i+2]); i+=2; last=i; continue; }
                fmt.push_str(&t[last..i]); i+=1; let s=i; let mut d=1i32;
                while i<n&&d>0{ match b[i]{ b'{'=>d+=1, b'}'=>d-=1, _=>{} } i+=1; }
                if d!=0{ fmt.push_str("{:?}"); break; }
                let e=i-1; let inner=t[s..e].trim();
                if inner.is_empty(){ fmt.push_str("{:?}"); last=i; continue; }
                let (ex,sp)=cut(inner);
                if let Ok(expr)=syn::parse_str::<Expr>(ex){
                    match sp{
                        Some("?")=>fmt.push_str("{:?}"),
                        Some("c")=>{ fmt.push_str("{}"); args.push(quote!{ format!("{:?}", #expr) }); last=i; continue; }
                        Some("j")=>{ fmt.push_str("{}"); args.push(quote!{ __w_json(&#expr) }); last=i; continue; }
                        Some(sp)=>{ fmt.push('{'); fmt.push(':'); fmt.push_str(sp); fmt.push('}'); }
                        None=>fmt.push_str("{}"),
                    }
                    if !matches!(sp,Some("c"|"j")){ args.push(expr.into_token_stream()); }
                } else { fmt.push('{'); fmt.push_str(inner); fmt.push('}'); }
                last=i;
            }
            b'}'=>{
                if i+1<n&&b[i+1]==b'}'{ fmt.push_str(&t[last..i+2]); i+=2; last=i; }
                else { fmt.push_str(&t[last..=i]); i+=1; last=i; }
            }
            _=>{ i+=1; }
        }
    }
    if last<n{ fmt.push_str(&t[last..]); }
    (fmt,args)
}

struct R;
impl VisitMut for R{
    fn visit_expr_mut(&mut self,e:&mut Expr){
        if let Expr::Call(ExprCall{func,args,..})=e{
            if let Expr::Path(ExprPath{path,..})=func.as_ref(){
                if path.segments.len()==1 && (path.segments[0].ident=="println"||path.segments[0].ident=="print"){
                    if let Some(Expr::Lit(ExprLit{lit:Lit::Str(s),..}))=args.first(){
                        let (f,a)=trans(&s.value()); let lit=syn::LitStr::new(&f,s.span()); let name=&path.segments[0].ident;
                        *e=syn::parse2(quote!{ #name(format!(#lit #(, #a)*)) }).unwrap(); return;
                    }
                }
            }
        } visit_expr_mut(self,e);
    }
}

fn parse_args(ts:TokenStream)->(String,String,String,String,String){
    let (mut bg,mut fg,mut font,mut color,mut size)=(
        "white".to_string(),"lightgray".to_string(),"Arial, sans-serif".to_string(),"black".to_string(),"14px".to_string()
    );
    let s=ts.to_string(); if s.is_empty(){ return (bg,fg,font,color,size); }
    for p in s.split(','){
        if let Some((k,v))=p.split_once('='){
            let k=k.trim(); let v=v.trim().trim_matches('"');
            match k{ "bg"=>bg=v.into(),"fg"=>fg=v.into(),"font"=>font=v.into(),"color"=>color=v.into(),"size"=>size=v.into(), _=>{} }
        }
    } (bg,fg,font,color,size)
}

#[proc_macro_attribute]
pub fn gui(attr:TokenStream,input:TokenStream)->TokenStream{
    let mut f=parse_macro_input!(input as ItemFn);
    R.visit_item_fn_mut(&mut f);
    let (bg,fg,font,color,size)=parse_args(attr); let body=&f.block;
    let wrapped=quote!{{
        fn __w_json<T: ::serde::Serialize>(v:&T)->String{
            fn format_json_custom(val:&::serde_json::Value,depth:usize)->String{
                match val{
                    ::serde_json::Value::Array(arr)=>{
                        if arr.is_empty(){return "[]".into();}
                        if arr.len()<=3 && arr.iter().all(|v|matches!(v,::serde_json::Value::Number(_))){
                            return format!("[{}]",arr.iter().map(|v|format_json_custom(v,depth)).collect::<Vec<_>>().join(", "));
                        }
                        let indent="    ".repeat(depth); let inner="    ".repeat(depth+1);
                        let items=arr.iter().map(|v|format!("{}{}",inner,format_json_custom(v,depth+1))).collect::<Vec<_>>();
                        format!("[\n{}\n{}]",items.join(",\n"),indent)
                    }
                    ::serde_json::Value::Object(obj)=>{
                        if obj.is_empty(){return "{}".into();}
                        let indent="    ".repeat(depth); let inner="    ".repeat(depth+1);
                        let mut entries:Vec<_>=obj.iter().collect(); entries.sort_by_key(|(k,_)|*k);
                        let items=entries.iter().map(|(k,v)|format!(r#"{}"{}": {}"#,inner,k,format_json_custom(v,depth+1))).collect::<Vec<_>>();
                        format!("{{\n{}\n{}}}",items.join(",\n"),indent)
                    }
                    ::serde_json::Value::String(s)=>format!(r#""{}""#,s),
                    ::serde_json::Value::Number(n)=>n.to_string(),
                    ::serde_json::Value::Bool(b)=>b.to_string(),
                    ::serde_json::Value::Null=>"null".into(),
                }
            }
            let val=::serde_json::to_value(v).unwrap_or(::serde_json::Value::Null);
            let escaped=format_json_custom(&val,0).replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;").replace(' ',"&nbsp;");
            format!(r#"<div style="font-family:'Courier New',monospace;color:#1e40af;font-size:12px;line-height:1.3;white-space:pre;">{}</div>"#,escaped)
        }
        let style=::webrust::io::gui::StyleConfig{ bg:#bg.into(), fg:#fg.into(), font:#font.into(), color:#color.into(), size:#size.into() };
        ::webrust::io::gui::start_gui_server_with_style(style,||{ #body });
    }};
    f.block=syn::parse2(wrapped).unwrap();
    TokenStream::from(quote!{ #[allow(unused_variables,dead_code)] #f })
}