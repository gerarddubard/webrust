// webrust/src/io/print.rs
//! # Styled Printing & Absolute Positioning
//!
//! Fluent, CSS-like printing system for the WebRust terminal. Chain styles on `print(...)` 
//! and `println(...)`, and position text blocks anywhere on screen with `.at(x, y)`.
//!
//! ## Core Usage
//! ```rust
//! use webrust::prelude::*;
//!
//! println("@(green, bold)Success")
//!     .weight(3).style("double").radius(8)
//!     .color("green").background("honeydew");
//!
//! // Absolute placement that respects the global coord("css" | "cartesian") setting
//! coord("cartesian");
//! print("@(white)Center badge").background("indigo").radius(6).at(0.0, 0.0);
//! ```
//!
//! ## Coordinate System
//! The `.at(x, y)` method uses the global `coord(...)` setting (shared with turtle graphics):
//! - `coord("css")`: Origin at top-left, +y points down. If `x < 0`, the element is anchored
//!   **from the right edge** by `|x|` pixels (`.at(-20.0, y)` → 20px from right edge).
//! - `coord("cartesian")`: Origin at screen center, +y points up. Standard mathematical placement.
//!
//! ## Layout Constants
//! - `CW`, `CH` are exported for layout calculations (e.g., `.width(*CW / 2)`).
//!
//! ## Style Methods
//! - `.width(px)` — Set fixed width
//! - `.align("left"|"center"|"right"|"justify")` — Text alignment
//! - `.weight(px)` — Border thickness (aliases: `.stroke`, `.thickness`)
//! - `.color(color)` — Border color
//! - `.background(color)` — Background color
//! - `.style("solid"|"dashed"|"dotted"|"double")` — Border style
//! - `.radius(px)` — Border radius for rounded corners
//! - `.space(px)` — Line spacing
//! - `.border(top, right, bottom, left)` — Control individual border sides
//!
//! ## Text Markup
//! Inline text markup is automatically parsed and rendered:
//! - Style tags: `@(color|background:...|bold|italic|underline|strike)`
//! - LaTeX math: `$( ... )` for inline math expressions
//!
//! Both markup systems work together seamlessly within printed text.

use crate::coord::{current,CoordMode};
use crate::io::gui::{add_output_new_line,add_output_same_line};
use std::{process::Command,sync::LazyLock};

fn pshell(cmd:&str,d:u32)->u32{
    Command::new("powershell").args(&["-Command",cmd]).output().ok()
        .and_then(|o|String::from_utf8_lossy(&o.stdout).trim().parse().ok()).unwrap_or(d)
}
pub static CW:LazyLock<u32>=LazyLock::new(||pshell("Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.Screen]::PrimaryScreen.Bounds.Width",800)/2);
pub static CH:LazyLock<u32>=LazyLock::new(||pshell("Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.Screen]::PrimaryScreen.Bounds.Height",600)/2);

fn read_balanced(cs:&[char],i:&mut usize)->String{
    let(mut d,mut s)=(1,String::new());
    while *i<cs.len()&&d>0{
        let c=cs[*i];
        if c=='(' {d+=1;} else if c==')'{d-=1;}
        if d>0{s.push(c);}*i+=1;
    }
    s
}
fn latex_from_dollar_paren(t:&str)->String{
    let mut o=String::new();let cs:Vec<char>=t.chars().collect();let mut i=0;
    while i<cs.len(){
        if i+1<cs.len()&&cs[i]=='$'&&cs[i+1]=='(' {
            i+=2;let b=read_balanced(&cs,&mut i);
            let disp=b.contains("\\begin{")||b.contains("\\[")||b.len()>50;
            if disp{o.push_str("$$");o.push_str(&b);o.push_str("$$");}
            else{o.push('$');o.push_str(&b);o.push('$');}
        }else{o.push(cs[i]);i+=1;}
    }o
}

pub fn process_webrust_styles_only(t:&str)->String{
    let mut o=String::new();let cs:Vec<char>=t.chars().collect();let mut i=0;
    while i<cs.len(){
        if cs[i]=='@'&&i+1<cs.len()&&cs[i+1]=='(' {
            let mut close=None;for j in (i+2)..cs.len(){if cs[j]==')'{close=Some(j);break;}}
            if let Some(cl)=close{
                let styles_raw:String=cs[(i+2)..cl].iter().collect();let content_start=cl+1;
                let mut next=cs.len();for j in content_start..cs.len(){if j+1<cs.len()&&cs[j]=='@'&&cs[j+1]=='(' {next=j;break;}}
                let content:String=cs[content_start..next].iter().collect();let mut css=Vec::new();
                for tok in styles_raw.split(',').map(|s|s.trim()).filter(|s|!s.is_empty()){
                    match tok.to_ascii_lowercase().as_str(){
                        "bold"=>css.push("font-weight:bold".into()),
                        "italic"=>css.push("font-style:italic".into()),
                        "underline"=>css.push("text-decoration:underline".into()),
                        "strike"=>css.push("text-decoration:line-through".into()),
                        "reset"=>{o.push_str(&content);css.clear();}
                        _=>{
                            if tok.contains(':'){let p:Vec<&str>=tok.split(':').collect();if p.len()==2{
                                match p[0].trim(){"background"=>css.push(format!("background-color:{}",p[1].trim())),"color"=>css.push(format!("color:{}",p[1].trim())),_=>css.push(format!("color:{}",tok)),}
                            }else{css.push(format!("color:{}",tok));}}
                            else{css.push(format!("color:{}",tok));}
                        }
                    }
                }
                if !css.is_empty(){o.push_str(&format!(r#"<span style="{}">{}</span>"#,css.join(";"),content));}
                else{o.push_str(&content);}
                i=next;continue;
            }
        }
        o.push(cs[i]);i+=1;
    }o
}
pub fn html_escape_preserve_utf8(t:&str)->String{
    t.chars().map(|c|match c{'<'=>"&lt;".into(),'>'=>"&gt;".into(),'&'=>"&amp;".into(),'"'=>"&quot;".into(),'\''=>"&#x27;".into(),_=>c.to_string()}).collect()
}
pub fn process_styles(t:&str)->String{
    let s=process_webrust_styles_only(&latex_from_dollar_paren(t));
    if s.contains('\n')&&(s.contains('{')||s.contains('[')){format!("<pre style=\"font-family:'Courier New',monospace;margin:0;display:inline;\">{}</pre>",s)}else{s}
}

#[derive(Clone)]
pub struct PrintBox{
    lines:Vec<String>,inline:bool,
    b_top:bool,b_right:bool,b_bottom:bool,b_left:bool,
    weight_px:u32,border_color:Option<String>,style:Option<String>,radius_px:u32,
    cell_width:Option<u32>,align:String,line_gap_px:Option<u32>,bg_color:Option<String>,
    x:Option<f64>,y:Option<f64>,rx:Option<f64>,emitted:bool,
}
impl PrintBox{
    fn new(lines:Vec<String>,inline:bool)->Self{
        Self{lines,inline,b_top:true,b_right:true,b_bottom:true,b_left:true,weight_px:0,
            border_color:None,style:None,radius_px:0,cell_width:None,align:"center".into(),
            line_gap_px:None,bg_color:None,x:None,y:None,rx:None,emitted:false}
    }
    pub fn border(mut self,t:bool,r:bool,b:bool,l:bool)->Self{self.b_top=t;self.b_right=r;self.b_bottom=b;self.b_left=l;self}
    pub fn weight(mut self,px:u32)->Self{self.weight_px=px;self}
    pub fn stroke(self,px:u32)->Self{self.weight(px)}
    pub fn thickness(self,px:u32)->Self{self.weight(px)}
    pub fn color<S:Into<String>>(mut self,c:S)->Self{self.border_color=Some(c.into());self}
    pub fn style<S:Into<String>>(mut self,s:S)->Self{self.style=Some(s.into());self}
    pub fn radius(mut self,px:u32)->Self{self.radius_px=px;self}
    pub fn width(mut self,px:u32)->Self{self.cell_width=(px>0).then_some(px);self}
    pub fn align<S:AsRef<str>>(mut self,v:S)->Self{let a=v.as_ref().to_ascii_lowercase();self.align=match a.as_str(){"left"|"center"|"right"|"justify"=>a,_=>"center".into()};self}
    pub fn space(mut self,px:u32)->Self{self.line_gap_px=Some(px);self}
    pub fn background<S:Into<String>>(mut self,c:S)->Self{self.bg_color=Some(c.into());self}
    pub fn at<X:Into<f64>,Y:Into<f64>>(mut self,x:X,y:Y)->Self{
        let x=x.into();let y=y.into();
        match current(){
            CoordMode::Css=>{ if x<0.0{self.rx=Some(-x);self.x=None;}else{self.x=Some(x);self.rx=None;} }
            _=>{ self.x=Some(x);self.rx=None; }
        }
        self.y=Some(y);self
    }
    fn build_style(&self)->String{
        let mut css=String::from("display:inline-block;white-space:normal;vertical-align:top;padding:2px 6px;");
        css.push_str(&format!("text-align:{};border-radius:{}px;",self.align,self.radius_px));
        if let Some(w)=self.cell_width{css.push_str(&format!("width:{}px;",w));}
        if let Some(bg)=&self.bg_color{css.push_str(&format!("background-color:{};",bg));}
        let sty=self.style.as_deref().unwrap_or("solid");let col=self.border_color.as_deref().unwrap_or("#cbd5e1");let s=self.weight_px;
        let mut side=|n:&str,on:bool|if on{css.push_str(&format!("border-{}:{}px {} {};",n,s,sty,col));}else{css.push_str(&format!("border-{}:none;",n));};
        side("top",self.b_top);side("right",self.b_right);side("bottom",self.b_bottom);side("left",self.b_left);
        if let Some(y)=self.y{
            match (self.x,self.rx,current()){
                (Some(x),_,CoordMode::Cartesian)=>{
                    let lx=(*CW as f64)/2.0+x;let ty=(*CH as f64)/2.0-y;
                    css.push_str("position:absolute;");css.push_str(&format!("left:{}px;top:{}px;",lx.round() as i32,ty.round() as i32));
                }
                (Some(x),_,_)=>{
                    css.push_str("position:absolute;");css.push_str(&format!("left:{}px;top:{}px;",x.round() as i32,y.round() as i32));
                }
                (None,Some(r),CoordMode::Css)=>{
                    css.push_str("position:absolute;");css.push_str(&format!("right:{}px;top:{}px;",r.round() as i32,y.round() as i32));
                }
                (None,Some(r),_)=>{
                    let lx=(*CW as f64)/2.0 - r;let ty=(*CH as f64)/2.0 - y;
                    css.push_str("position:absolute;");css.push_str(&format!("left:{}px;top:{}px;",lx.round() as i32,ty.round() as i32));
                }
                _=>{}
            }
        }
        css
    }
}
impl Drop for PrintBox{
    fn drop(&mut self){
        if self.emitted{return;}
        let style=self.build_style();
        let absolute=self.y.is_some()&&(self.x.is_some()||self.rx.is_some());
        if absolute{
            for seg in &self.lines{
                add_output_new_line(format!(r#"<div class="webrust-abs" style="{style}"><span class="webrust-box">{}</span></div>"#,seg));
            }
        }else if self.inline{
            let gap=self.line_gap_px.map(|g|format!(r#" data-line-gap="{}""#,g)).unwrap_or_default();
            for seg in &self.lines{
                add_output_same_line(format!(r#"<span class="webrust-box"{gap} style="{style}">{}</span>"#,seg));
            }
        }else{
            let gap=self.line_gap_px.unwrap_or(6);
            for seg in &self.lines{
                add_output_new_line(format!(r#"<div class="webrust-line" style="display:block;margin:{gap}px 0;"><span class="webrust-box" style="{style}">{}</span></div>"#,seg));
            }
        }
        self.emitted=true;
    }
}

fn make_box<T:std::fmt::Display>(t:T,inline:bool)->PrintBox{
    let lines:Vec<String>=format!("{}",t).split('\n').map(process_styles).collect();
    PrintBox::new(lines,inline)
}
pub fn print_str<T:std::fmt::Display>(t:T)->PrintBox{make_box(t,true)}
pub fn println_str<T:std::fmt::Display>(t:T)->PrintBox{make_box(t,false)}
pub use print_str as print;
pub use println_str as println;
