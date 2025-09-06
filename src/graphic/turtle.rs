// webrust/src/graphic/turtle.rs
//! # WebRust Turtle Graphics
//!
//! An animated turtle graphics engine rendered to HTML canvas inside the WebRust
//! terminal. Designed to feel Python-like while remaining idiomatic Rust.
//!
//! ## Key Features
//! - **Multiple turtles** per stage, each with independent state and command queue
//! - **Shared coordinate system** with the I/O layer: `coord("css")` or 
//!   `coord("cartesian")` applies to both turtles and text placement
//! - **Consistent stage sizing** using `CW`/`CH` constants for predictable layout
//! - **Smooth animation** with requestAnimationFrame loop in the browser
//!
//! ## Quick Example
//! ```rust
//! use webrust::prelude::*;
//!
//! coord("cartesian");
//!
//! let sun = turtle();
//! sun.setColor("gold").setPenSize(2.0).setPos(0.0, 0.0).circle(60.0);
//!
//! // Draw sun rays
//! for k in 0..24 {
//!     let angle = k as f64 * 15.0;
//!     sun.angle(angle).setPos(0.0, 0.0)
//!        .penup().forward(45.0)
//!        .pendown().forward(60.0);
//! }
//! ```
//!
//! ## API Reference
//! All distances are in pixels; angles are in degrees; speed is in pixels/second.
//!
//! ### Drawing Configuration
//! - `setColor(name: &str)` — Set stroke color (CSS names or hex: `"crimson"`, `"#ff0055"`)
//! - `setPenSize(px: f64)` — Set stroke width in pixels
//! - `speed(px_per_s: f64)` — Set forward movement animation speed
//!
//! ### Movement & Positioning  
//! - `angle(deg: f64)` — Set absolute heading (0° = right, counter-clockwise positive)
//! - `setPos(x, y)` — Teleport to position without drawing (respects coordinate mode)
//! - `forward(distance)` — Move forward by distance, drawing if pen is down
//!
//! ### Drawing Commands
//! - `line(x1, y1, x2, y2)` — Draw line segment immediately
//! - `point()` — Draw filled point at current position  
//! - `circle(radius)` — Draw circle centered at current position
//! - `penup()` / `pendown()` — Toggle drawing mode for `forward()` movement
//!
//! ## Coordinate Systems
//! - `coord("css")`: Origin at top-left, +y points downward
//! - `coord("cartesian")`: Origin at screen center, +y points upward
//!
//! The coordinate setting affects both turtle movement and text positioning 
//! from the I/O layer, enabling precise alignment between graphics and labels.
//!
//! ## Combining Text and Graphics
//! ```rust
//! use webrust::prelude::*;
//!
//! coord("cartesian");
//!
//! let flower = turtle();
//! flower.setColor("orchid").setPenSize(3.0)
//!       .setPos(-120.0, 80.0).circle(50.0);
//!
//! // Text label uses same coordinate system as turtle
//! print("@(white, italic)Flower")
//!     .background("indigo").radius(6)
//!     .at(-120.0, 10.0);
//! ```

#![allow(non_snake_case)]
use crate::coord::{current,CoordMode};
use crate::io::gui::add_output_new_line;
use crate::io::print::{CW,CH};
use serde::Serialize;
use std::{cell::RefCell,collections::HashSet,rc::Rc,sync::{LazyLock,Mutex},sync::atomic::{AtomicUsize,Ordering}};

static STAGES: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));
static NEXT_ID: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(1));

#[derive(Serialize,Clone)]
struct StageMsg{ id:String,w:u32,h:u32,coord:String }

#[derive(Serialize,Clone)]
struct Cmd{ op:&'static str,a:f64,b:f64,c:f64,d:f64,#[serde(skip_serializing_if="Option::is_none")] s:Option<String> }
impl Cmd{
    fn color<S:Into<String>>(s:S)->Self{Self{op:"color",a:0.0,b:0.0,c:0.0,d:0.0,s:Some(s.into())}}
    fn pen(a:f64)->Self{Self{op:"pen",a,b:0.0,c:0.0,d:0.0,s:None}}
    fn speed(a:f64)->Self{Self{op:"speed",a,b:0.0,c:0.0,d:0.0,s:None}}
    fn angle(a:f64)->Self{Self{op:"angle",a,b:0.0,c:0.0,d:0.0,s:None}}
    fn pos(a:f64,b:f64)->Self{Self{op:"pos",a,b,c:0.0,d:0.0,s:None}}
    fn fwd(a:f64)->Self{Self{op:"fwd",a,b:0.0,c:0.0,d:0.0,s:None}}
    fn line(a:f64,b:f64,c:f64,d:f64)->Self{Self{op:"line",a,b,c,d,s:None}}
    fn point()->Self{Self{op:"point",a:0.0,b:0.0,c:0.0,d:0.0,s:None}}
    fn circle(a:f64)->Self{Self{op:"circle",a,b:0.0,c:0.0,d:0.0,s:None}}
    fn penup()->Self{Self{op:"penup",a:0.0,b:0.0,c:0.0,d:0.0,s:None}}
    fn pendown()->Self{Self{op:"pendown",a:0.0,b:0.0,c:0.0,d:0.0,s:None}}
}

#[derive(Serialize)]
struct CmdsMsg{ stage:String,tid:String,cmds:Vec<Cmd>,#[serde(skip_serializing_if="Option::is_none")] w:Option<u32>,#[serde(skip_serializing_if="Option::is_none")] h:Option<u32> }

fn emit_stage_once(id:&str,w:u32,h:u32){
    let mut g=STAGES.lock().unwrap();
    if g.insert(id.to_string()){
        let coord=match current(){CoordMode::Cartesian=>"cartesian",_=>"css"}.to_string();
        let j=serde_json::to_string(&StageMsg{id:id.to_string(),w,h,coord}).unwrap();
        add_output_new_line(format!("TURTLE_STAGE:{j}"));
    }
}

pub struct Turtle{ inner:Rc<Inner> }
struct Inner{ stage:String,tid:String,q:RefCell<Vec<Cmd>> }

impl Turtle{
    fn new(stage:String)->Self{
        let tid=format!("t{}",NEXT_ID.fetch_add(1,Ordering::Relaxed));
        emit_stage_once(&stage,*CW,*CH);
        Self{inner:Rc::new(Inner{stage,tid,q:RefCell::new(Vec::new())})}
    }
    pub fn setColor<S:Into<String>>(&self,c:S)->&Self{self.inner.q.borrow_mut().push(Cmd::color(c));self}
    pub fn setPenSize(&self,px:f64)->&Self{self.inner.q.borrow_mut().push(Cmd::pen(px));self}
    pub fn speed(&self,px_per_s:f64)->&Self{self.inner.q.borrow_mut().push(Cmd::speed(px_per_s));self}
    pub fn angle(&self,deg:f64)->&Self{self.inner.q.borrow_mut().push(Cmd::angle(deg));self}
    pub fn setPos(&self,x:f64,y:f64)->&Self{self.inner.q.borrow_mut().push(Cmd::pos(x,y));self}
    pub fn forward(&self,d:f64)->&Self{self.inner.q.borrow_mut().push(Cmd::fwd(d));self}
    pub fn line(&self,x1:f64,y1:f64,x2:f64,y2:f64)->&Self{self.inner.q.borrow_mut().push(Cmd::line(x1,y1,x2,y2));self}
    pub fn point(&self)->&Self{self.inner.q.borrow_mut().push(Cmd::point());self}
    pub fn circle(&self,r:f64)->&Self{self.inner.q.borrow_mut().push(Cmd::circle(r));self}
    pub fn penup(&self)->&Self{self.inner.q.borrow_mut().push(Cmd::penup());self}
    pub fn pendown(&self)->&Self{self.inner.q.borrow_mut().push(Cmd::pendown());self}
    pub fn stage_id(&self)->&str{&self.inner.stage}
    pub fn id(&self)->&str{&self.inner.tid}
}
impl Drop for Turtle{
    fn drop(&mut self){
        let q=std::mem::take(&mut *self.inner.q.borrow_mut());
        if q.is_empty(){return;}
        let j=serde_json::to_string(&CmdsMsg{stage:self.inner.stage.clone(),tid:self.inner.tid.clone(),cmds:q,w:Some(*CW),h:Some(*CH)}).unwrap();
        add_output_new_line(format!("TURTLE_CMDS:{j}"));
    }
}

pub fn turtle()->Turtle{Turtle::new("stage1".to_string())}