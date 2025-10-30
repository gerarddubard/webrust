// webrust/src/graphic/turtle.rs
//! # Turtle Graphics and Object Animation
//!
//! Turtle-style drawing with pen control, geometric shapes, transformations,
//! and sophisticated animations with easing functions.
//!
//! ## Core Concepts
//!
//! - **Object**: Individual drawable entity with pen state and animation queue
//! - **ObjectGroup**: Collection of objects that can be animated together
//! - **Easing**: 20+ easing functions (linear, sine, quad, elastic, bounce, etc.)
//! - **Coordinate mode**: Respects global `coord()` setting
//!
//! ## Pen Control
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! # #[gui] fn example() {
//! let o = object();
//! o.color("red").width(2.0).fill("rgba(255,0,0,0.3)");
//! o.at(100.0, 100.0);          // Position
//! o.to(45.0);                  // Angle
//! o.forward(100.0);            // Move forward
//! o.right(90.0);               // Turn right
//! o.penup();                   // Lift pen
//! o.forward(50.0);             // Move without drawing
//! o.pendown();                 // Lower pen
//! # }
//! ```
//!
//! ## Geometric Shapes
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! # #[gui] fn example() {
//! let o = object();
//! o.at(0.0, 0.0).color("blue");
//! o.circle(50.0);              // Circle
//! o.rectangle(80.0, 60.0);     // Rectangle
//! o.square(40.0);              // Square
//! o.ellipse(60.0, 30.0);       // Ellipse
//! o.arc(50.0, 90.0);           // Arc (radius, sweep angle)
//! o.rhombus(50.0, 60.0);       // Rhombus (side, angle)
//! o.parallelogram(70.0, 40.0, 60.0);  // Length, width, angle
//! o.polygon(vec![[0.0, 0.0], [50.0, 0.0], [25.0, 50.0]]);
//! # }
//! ```
//!
//! ## Transformations with Animation
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! # #[gui] fn example() {
//! let o = object();
//! o.circle(50.0);
//!
//! // Animate with easing
//! o.speed(100.0)               // Pixels per second
//!     .rotate(360.0)           // Rotation in degrees
//!     .ease("elasticOut");     // Easing function
//!
//! o.translate(100.0, 50.0)     // Move
//!     .scale(1.5, 1.5)         // Scale
//!     .ease("sineInOut");
//!
//! o.reflect("x");              // Mirror horizontally
//! # }
//! ```
//!
//! ## Group Animations
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! # #[gui] fn example() {
//! let wheel1 = object();
//! wheel1.circle(20.0);
//!
//! let wheel2 = object();
//! wheel2.at(100.0, 0.0).circle(20.0);
//!
//! // Animate together
//! let car = group();
//! car.add(&wheel1);
//! car.add(&wheel2);
//! car.translate(200.0, 0.0).ease("linear");
//! car.rotate(45.0);
//! # }
//! ```
//!
//! ## Easing Functions
//!
//! Available easings: `linear`, `sineIn`, `sineOut`, `sineInOut`,
//! `quadIn`, `quadOut`, `quadInOut`, `cubicIn`, `cubicOut`, `cubicInOut`,
//! `quartIn`, `quartOut`, `quartInOut`, `quintIn`, `quintOut`, `quintInOut`,
//! `expoIn`, `expoOut`, `expoInOut`, `circIn`, `circOut`, `circInOut`,
//! `backIn`, `backOut`, `backInOut`, `elasticIn`, `elasticOut`, `elasticInOut`,
//! `bounceIn`, `bounceOut`, `bounceInOut`
//!
//! ## Cloning Objects
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! # #[gui] fn example() {
//! let original = object();
//! original.color("red").circle(30.0);
//!
//! let copy = original.clone();  // Clones shape and style
//! copy.at(100.0, 0.0);          // Independent positioning
//! # }
//! ```
//!
//! Objects automatically render when dropped, with animations queued
//! and executed via JavaScript in the browser.

#![allow(non_snake_case)]
use crate::io::gui::add_output_new_line;
use crate::io::print::{TH, TW};
use crate::layout::coord::{current, CoordMode};
use serde::Serialize;
use std::{
    cell::RefCell,
    collections::HashSet,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        LazyLock, Mutex,
    },
};

static STAGES: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));
static NEXT_ID: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(1));
static NEXT_GID: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(1));

#[derive(Serialize, Clone)]
struct StageMsg {
    id: String,
    w: u32,
    h: u32,
    coord: String,
}

#[derive(Serialize, Clone)]
struct Cmd {
    op: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    b: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    c: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    d: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    points: Option<Vec<[f64; 2]>>,
}
impl Cmd {
    fn style_mode<S: Into<String>>(s: S) -> Self {
        Self {
            op: "style_mode",
            a: None,
            b: None,
            c: None,
            d: None,
            s: Some(s.into()),
            points: None,
        }
    }
    fn color<S: Into<String>>(s: S) -> Self {
        Self {
            op: "color",
            a: None,
            b: None,
            c: None,
            d: None,
            s: Some(s.into()),
            points: None,
        }
    }
    fn fill<S: Into<String>>(s: S) -> Self {
        Self {
            op: "fill",
            a: None,
            b: None,
            c: None,
            d: None,
            s: Some(s.into()),
            points: None,
        }
    }
    fn width(w: f64) -> Self {
        Self {
            op: "width",
            a: Some(w),
            b: None,
            c: None,
            d: None,
            s: None,
            points: None,
        }
    }
    fn dash(down: f64, up: f64) -> Self {
        Self {
            op: "dash",
            a: Some(down),
            b: Some(up),
            c: None,
            d: None,
            s: None,
            points: None,
        }
    }
    fn speed(s: f64) -> Self {
        Self {
            op: "speed",
            a: Some(s),
            b: None,
            c: None,
            d: None,
            s: None,
            points: None,
        }
    }
    fn ease<S: Into<String>>(name: S) -> Self {
        Self {
            op: "ease",
            a: None,
            b: None,
            c: None,
            d: None,
            s: Some(name.into()),
            points: None,
        }
    }
    fn reverse() -> Self {
        Self {
            op: "reverse",
            a: None,
            b: None,
            c: None,
            d: None,
            s: None,
            points: None,
        }
    }
    fn wait(ms: f64) -> Self {
        Self {
            op: "wait",
            a: Some(ms),
            b: None,
            c: None,
            d: None,
            s: None,
            points: None,
        }
    }
    fn point(x: f64, y: f64) -> Self {
        Self {
            op: "point",
            a: Some(x),
            b: Some(y),
            c: None,
            d: None,
            s: None,
            points: None,
        }
    }
    fn line(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            op: "line",
            a: Some(x1),
            b: Some(y1),
            c: Some(x2),
            d: Some(y2),
            s: None,
            points: None,
        }
    }
    fn circle(radius: f64, ang: f64) -> Self {
        Self {
            op: "circle",
            a: Some(radius),
            b: None,
            c: Some(ang),
            d: None,
            s: None,
            points: None,
        }
    }
    fn arc(radius: f64, sweep: f64, start: f64) -> Self {
        Self {
            op: "arc",
            a: Some(radius),
            b: Some(sweep),
            c: Some(start),
            d: None,
            s: None,
            points: None,
        }
    }
    fn ellipse(rx: f64, ry: f64, ang: f64) -> Self {
        Self {
            op: "ellipse",
            a: Some(rx),
            b: Some(ry),
            c: Some(ang),
            d: None,
            s: None,
            points: None,
        }
    }
    fn parallelogram(len: f64, w: f64, alpha: f64, ang: f64) -> Self {
        Self {
            op: "parallelogram",
            a: Some(len),
            b: Some(w),
            c: Some(alpha),
            d: Some(ang),
            s: None,
            points: None,
        }
    }
    fn rectangle(len: f64, w: f64, ang: f64) -> Self {
        Self {
            op: "rectangle",
            a: Some(len),
            b: Some(w),
            c: Some(ang),
            d: None,
            s: None,
            points: None,
        }
    }
    fn square(side: f64, ang: f64) -> Self {
        Self {
            op: "square",
            a: Some(side),
            b: Some(ang),
            c: None,
            d: None,
            s: None,
            points: None,
        }
    }
    fn rhombus(side: f64, alpha: f64, ang: f64) -> Self {
        Self {
            op: "rhombus",
            a: Some(side),
            b: Some(alpha),
            c: Some(ang),
            d: None,
            s: None,
            points: None,
        }
    }
    fn polygon(pts: Vec<[f64; 2]>) -> Self {
        Self {
            op: "polygon",
            a: None,
            b: None,
            c: None,
            d: None,
            s: None,
            points: Some(pts),
        }
    }
    fn translate(dx: f64, dy: f64) -> Self {
        Self {
            op: "translate",
            a: Some(dx),
            b: Some(dy),
            c: None,
            d: None,
            s: None,
            points: None,
        }
    }
    fn rotate(angle: f64) -> Self {
        Self {
            op: "rotate",
            a: Some(angle),
            b: None,
            c: None,
            d: None,
            s: None,
            points: None,
        }
    }
    fn scale(sx: f64, sy: f64) -> Self {
        Self {
            op: "scale",
            a: Some(sx),
            b: Some(sy),
            c: None,
            d: None,
            s: None,
            points: None,
        }
    }
    fn reflect<S: Into<String>>(axis: S) -> Self {
        Self {
            op: "reflect",
            a: None,
            b: None,
            c: None,
            d: None,
            s: Some(axis.into()),
            points: None,
        }
    }
    fn pen_move(x: f64, y: f64, draw: bool) -> Self {
        Self {
            op: "pen_move",
            a: Some(x),
            b: Some(y),
            c: Some(if draw { 1.0 } else { 0.0 }),
            d: None,
            s: None,
            points: None,
        }
    }
    fn set_position(x: f64, y: f64) -> Self {
        Self {
            op: "set_position",
            a: Some(x),
            b: Some(y),
            c: None,
            d: None,
            s: None,
            points: None,
        }
    }
    fn set_pen_angle(angle: f64) -> Self {
        Self {
            op: "set_pen_angle",
            a: Some(angle),
            b: None,
            c: None,
            d: None,
            s: None,
            points: None,
        }
    }
    fn adjust_pen_angle(delta: f64) -> Self {
        Self {
            op: "adjust_pen_angle",
            a: Some(delta),
            b: None,
            c: None,
            d: None,
            s: None,
            points: None,
        }
    }
}

#[derive(Serialize)]
struct CmdsMsg {
    stage: String,
    oid: String,
    cmds: Vec<Cmd>,
    #[serde(skip_serializing_if = "Option::is_none")]
    w: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    h: Option<u32>,
}

#[derive(Serialize)]
struct GroupMsg {
    gid: String,
    members: Vec<String>,
    cmds: Vec<Cmd>,
}

fn emit_stage_once(id: &str, w: u32, h: u32) {
    let mut g = STAGES.lock().unwrap();
    if g.insert(id.to_string()) {
        let coord = match current() {
            CoordMode::Cartesian => "cartesian",
            _ => "css",
        }
        .to_string();
        let j = serde_json::to_string(&StageMsg {
            id: id.to_string(),
            w,
            h,
            coord,
        })
        .unwrap();
        add_output_new_line(format!("OBJECT_STAGE:{j}"));
    }
}

#[derive(Clone)]
struct PenState {
    x: f64,
    y: f64,
    angle: f64,
    pen_down: bool,
}
impl Default for PenState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
            pen_down: true,
        }
    }
}

pub struct Object {
    inner: Rc<Inner>,
    pen_state: RefCell<PenState>,
}
struct Inner {
    stage: String,
    oid: String,
    q: RefCell<Vec<Cmd>>,
}

impl Default for Object {
    fn default() -> Self {
        let oid = format!("o{}", NEXT_ID.fetch_add(1, Ordering::Relaxed));
        emit_stage_once("stage1", *TW, *TH);
        Self {
            inner: Rc::new(Inner {
                stage: "stage1".to_string(),
                oid,
                q: RefCell::new(Vec::new()),
            }),
            pen_state: RefCell::new(PenState::default()),
        }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        let oid = format!("o{}", NEXT_ID.fetch_add(1, Ordering::Relaxed));
        emit_stage_once("stage1", *TW, *TH);
        let mut q_copy = self.inner.q.borrow().clone();
        let st_copy = self.pen_state.borrow().clone();
        let mut new_q = Vec::with_capacity(q_copy.len() + 1);
        new_q.push(Cmd::style_mode("restyle_on"));
        new_q.append(&mut q_copy);
        Self {
            inner: Rc::new(Inner {
                stage: self.inner.stage.clone(),
                oid,
                q: RefCell::new(new_q),
            }),
            pen_state: RefCell::new(st_copy),
        }
    }
}

impl Object {
    pub fn color<S: Into<String>>(&self, c: S) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::color(c));
        self
    }
    pub fn fill<S: Into<String>>(&self, c: S) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::fill(c));
        self
    }
    pub fn width(&self, w: f64) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::width(w));
        self
    }
    pub fn dash(&self, down: f64, up: f64) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::dash(down, up));
        self
    }
    pub fn speed(&self, s: f64) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::speed(s));
        self
    }
    pub fn ease<S: Into<String>>(&self, name: S) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::ease(name));
        self
    }
    pub fn reverse(&self) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::reverse());
        self
    }
    pub fn wait(&self, ms: f64) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::wait(ms));
        self
    }
    pub fn at(&self, x: f64, y: f64) -> &Self {
        let mut st = self.pen_state.borrow_mut();
        st.x = x;
        st.y = y;
        self.inner.q.borrow_mut().push(Cmd::set_position(x, y));
        self
    }
    pub fn pos(&self) -> (f64, f64) {
        let st = self.pen_state.borrow();
        (st.x, st.y)
    }
    pub fn to(&self, a: f64) -> &Self {
        self.pen_state.borrow_mut().angle = a;
        self.inner.q.borrow_mut().push(Cmd::set_pen_angle(a));
        self
    }
    pub fn angle(&self) -> f64 {
        self.pen_state.borrow().angle
    }
    pub fn right(&self, a: f64) -> &Self {
        self.pen_state.borrow_mut().angle += a;
        self.inner.q.borrow_mut().push(Cmd::adjust_pen_angle(a));
        self
    }
    pub fn left(&self, a: f64) -> &Self {
        self.pen_state.borrow_mut().angle -= a;
        self.inner.q.borrow_mut().push(Cmd::adjust_pen_angle(-a));
        self
    }
    pub fn penup(&self) -> &Self {
        self.pen_state.borrow_mut().pen_down = false;
        self
    }
    pub fn pendown(&self) -> &Self {
        self.pen_state.borrow_mut().pen_down = true;
        self
    }
    pub fn forward(&self, d: f64) -> &Self {
        self.mv(d);
        self
    }
    pub fn backward(&self, d: f64) -> &Self {
        self.mv(-d);
        self
    }
    fn mv(&self, d: f64) {
        let mut st = self.pen_state.borrow_mut();
        let a = st.angle.to_radians();
        let nx = st.x + d * a.cos();
        let ny = st.y + d * a.sin();
        if st.pen_down {
            self.inner
                .q
                .borrow_mut()
                .push(Cmd::line(st.x, st.y, nx, ny));
        }
        st.x = nx;
        st.y = ny;
        self.inner
            .q
            .borrow_mut()
            .push(Cmd::pen_move(nx, ny, st.pen_down));
    }
    pub fn point(&self, x: f64, y: f64) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::point(x, y));
        self
    }
    pub fn line(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> &Self {
        let mut st = self.pen_state.borrow_mut();
        if (st.x - x1).abs() > 1e-9 || (st.y - y1).abs() > 1e-9 {
            self.inner.q.borrow_mut().push(Cmd::pen_move(x1, y1, false));
            st.x = x1;
            st.y = y1;
        }
        self.inner.q.borrow_mut().push(Cmd::line(x1, y1, x2, y2));
        st.x = x2;
        st.y = y2;
        self.inner.q.borrow_mut().push(Cmd::pen_move(x2, y2, false));
        self
    }
    pub fn circle(&self, r: f64) -> &Self {
        let ang = self.pen_state.borrow().angle;
        self.inner.q.borrow_mut().push(Cmd::circle(r, ang));
        self
    }
    pub fn arc(&self, r: f64, alpha: f64) -> &Self {
        let ang = self.pen_state.borrow().angle;
        self.inner.q.borrow_mut().push(Cmd::arc(r, alpha, ang));
        self
    }
    pub fn ellipse(&self, rx: f64, ry: f64) -> &Self {
        let ang = self.pen_state.borrow().angle;
        self.inner.q.borrow_mut().push(Cmd::ellipse(rx, ry, ang));
        self
    }
    pub fn parallelogram(&self, len: f64, w: f64, alpha: f64) -> &Self {
        let ang = self.pen_state.borrow().angle;
        self.inner
            .q
            .borrow_mut()
            .push(Cmd::parallelogram(len, w, alpha, ang));
        self
    }
    pub fn rectangle(&self, len: f64, w: f64) -> &Self {
        let ang = self.pen_state.borrow().angle;
        self.inner.q.borrow_mut().push(Cmd::rectangle(len, w, ang));
        self
    }
    pub fn square(&self, side: f64) -> &Self {
        let ang = self.pen_state.borrow().angle;
        self.inner.q.borrow_mut().push(Cmd::square(side, ang));
        self
    }
    pub fn rhombus(&self, side: f64, alpha: f64) -> &Self {
        let ang = self.pen_state.borrow().angle;
        self.inner
            .q
            .borrow_mut()
            .push(Cmd::rhombus(side, alpha, ang));
        self
    }
    pub fn polygon(&self, pts: Vec<[f64; 2]>) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::polygon(pts));
        self
    }
    pub fn translate(&self, dx: f64, dy: f64) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::translate(dx, dy));
        self
    }
    pub fn reflect<S: Into<String>>(&self, axis: S) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::reflect(axis));
        self
    }
    pub fn rotate(&self, a: f64) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::rotate(a));
        self
    }
    pub fn scale(&self, sx: f64, sy: f64) -> &Self {
        self.inner.q.borrow_mut().push(Cmd::scale(sx, sy));
        self
    }
    pub(crate) fn get_oid(&self) -> String {
        self.inner.oid.clone()
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        let q = std::mem::take(&mut *self.inner.q.borrow_mut());
        if q.is_empty() {
            return;
        }
        let j = serde_json::to_string(&CmdsMsg {
            stage: self.inner.stage.clone(),
            oid: self.inner.oid.clone(),
            cmds: q,
            w: Some(*TW),
            h: Some(*TH),
        })
        .unwrap();
        add_output_new_line(format!("OBJECT_CMDS:{j}"));
    }
}

pub struct ObjectGroup {
    gid: String,
    members: RefCell<Vec<String>>,
    cmds: RefCell<Vec<Cmd>>,
}

impl ObjectGroup {
    fn new() -> Self {
        Self {
            gid: format!("g{}", NEXT_GID.fetch_add(1, Ordering::Relaxed)),
            members: RefCell::new(Vec::new()),
            cmds: RefCell::new(Vec::new()),
        }
    }
    pub fn add(&self, obj: &Object) {
        self.members.borrow_mut().push(obj.get_oid());
    }
    pub fn remove(&self, obj: &Object) {
        self.members.borrow_mut().retain(|id| *id != obj.get_oid());
    }
    pub fn translate(&self, dx: f64, dy: f64) -> &Self {
        self.cmds.borrow_mut().push(Cmd::translate(dx, dy));
        self
    }
    pub fn rotate(&self, deg: f64) -> &Self {
        self.cmds.borrow_mut().push(Cmd::rotate(deg));
        self
    }
    pub fn scale(&self, sx: f64, sy: f64) -> &Self {
        self.cmds.borrow_mut().push(Cmd::scale(sx, sy));
        self
    }
    pub fn ease<S: Into<String>>(&self, name: S) -> &Self {
        self.cmds.borrow_mut().push(Cmd::ease(name));
        self
    }
    pub fn reverse(&self) -> &Self {
        self.cmds.borrow_mut().push(Cmd::reverse());
        self
    }
    pub fn wait(&self, ms: f64) -> &Self {
        self.cmds.borrow_mut().push(Cmd::wait(ms));
        self
    }
}

impl Drop for ObjectGroup {
    fn drop(&mut self) {
        let members = self.members.borrow().clone();
        let cmds = self.cmds.borrow().clone();
        if members.is_empty() || cmds.is_empty() {
            return;
        }
        let j = serde_json::to_string(&GroupMsg {
            gid: self.gid.clone(),
            members,
            cmds,
        })
        .unwrap();
        add_output_new_line(format!("OBJECT_GROUP:{j}"));
    }
}

pub fn object() -> Object {
    Object::default()
}
pub fn group() -> ObjectGroup {
    ObjectGroup::new()
}