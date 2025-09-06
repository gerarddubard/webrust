// webrust/examples/py_turtle.rs
use std::f64::consts::PI;
use webrust::prelude::*;

#[gui(bg="navy", fg="white", font="Courier New", color="black", size="10px")]
fn main() {
    coord("cartesian");

    // Helpers for positioning text in cartesian coords
    let left_edge  = -(*CW as f64)/2.0 + 12.0;
    let right_edge =  (*CW as f64)/2.0 - 12.0;
    let top_edge   =  (*CH as f64)/2.0 - 12.0;
    let bottom_edge= -(*CH as f64)/2.0 + 12.0;

    println("@(cyan, bold)🐢 WebRust Turtle Playground")
        .width(*CW/3)
        .align("center")
        .weight(2)
        .color("cyan")
        .radius(8)
        .background("midnightblue")
        .at(-75.0, top_edge-8.0);

    // Spiral
    let spiral = turtle();
    spiral.setColor("plum").setPenSize(2.0);
    let margin = 90.0;
    let spiral_cx = -(*CW as f64)/2.0 + 1.2*margin;
    let spiral_cy =  (*CH as f64)/2.0 - 1.4 * margin;
    let k = 3.5;
    let mut prev = None;
    for th in 0.0.to(10.0*PI).by(0.15) {
        let r = k * th;
        let x = spiral_cx + r * th.cos();
        let y = spiral_cy + r * th.sin();
        if let Some((x0, y0)) = prev { spiral.line(x0, y0, x, y); }
        prev = Some((x, y));
    }
    print("@(white, italic)Archimedean spiral (plum)")
        .background("indigo").radius(6).at(spiral_cx-25.0, spiral_cy-100.0);

    // Blue arrow + circle
    let arrow = turtle();
    arrow.setColor("deepskyblue").setPenSize(2.0).speed(220.0).angle(-25.0).setPos(-350.0,-50.0).forward(200.0);
    arrow.setPenSize(6.0).point();
    arrow.setPenSize(2.0).circle(40.0);
    print("@(white)Arrow + ring (deepskyblue)")
        .background("steelblue").radius(6).at(-260.0, -175.0);

    // Flower (12 circles)
    let flower = turtle();
    flower.setColor("red").setPenSize(2.0).speed(200.0);
    let (fx, fy) = (225.0, 60.0);
    let rad = 30.0;
    for k in 0.to(12) {
        let th = (k as f64 * 30.0).to_radians();
        let (ox, oy) = (rad*2.0*th.cos(), rad*2.0*th.sin());
        flower.setPos(fx+ox, fy+oy).circle(rad);
    }
    flower.setPos(fx, fy).point();
    print("@(white)Flower (12 circles)")
        .background("crimson").radius(6).at(fx-20.0, fy-85.0);

    // Sun with rays using penup/pendown
    let sun = turtle();
    sun.setColor("gold").setPenSize(2.0).speed(260.0);
    sun.setPos(0.0, 0.0).circle(60.0);
    for k in 0.to(24) {
        let th = (k as f64 * 15.0).to_radians();
        sun.angle(th.to_degrees());
        sun.setPos(0.0, 0.0).penup().forward(45.0);
        sun.pendown().forward(60.0);
    }
    print("@(black, bold)Sun with rays").background("gold").radius(6).at(0.0, -100.0);

    // Color fan
    let fan = turtle();
    fan.setPos(250.0, -180.0).setPenSize(4.0).speed(200.0);
    for (col, ang) in ["navy","crimson","orange","seagreen","royalblue","orchid","pink"]
        .iter()
        .zip([0.0,30.0,60.0,90.0,120.0,150.0,180.0])
    {
        fan.setColor(*col).angle(ang).forward(120.0);
        fan.setPos(250.0,-180.0);
    }
    print("@(white)Color fan").background("slateblue").radius(6).at(250.0, -175.0);
}