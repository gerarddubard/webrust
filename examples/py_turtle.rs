// webrust/examples/py_turtle.rs

use std::f64::consts::PI;
use webrust::prelude::*;

#[gui(Courier New 10px black !white)]
fn main() {
    print("<white b w250 t2 !midnightblue>🐢 WebRust Object Playground").at(*TW / 3, 0.0);

    coord("cartesian");
    grid(3, 4);

    let lw = *CW * 50 / 100;

    let (_x, _y) = cell(0, 0, "tc");

    let (cx, cy) = cell(0, 0, "center");
    let o = object();
    o.color("darkgreen")
        .width(2.0)
        .at(cx, cy + 25.0)
        .dash(10.0, 5.0);
    o.line(cx - 30.0, cy - 15.0, cx + 75.0, cy + 55.0);
    let (lx, ly) = cell(0, 0, "bottom");
    print("<white w{lw} !darkgreen r6>Line").at(lx - 20.0, ly + 10.0);

    let (cx, cy) = cell(0, 1, "center");
    let o = object();
    o.color("royalblue").width(2.0).at(cx, cy + 25.0);
    let s = 24.0;
    o.polygon(vec![
        [cx, cy + s + 25.0],
        [cx + s, cy + 25.0],
        [cx, cy - s + 25.0],
        [cx - s, cy + 25.0],
    ]);
    let (lx, ly) = cell(0, 1, "bottom");
    print("<white w{lw} !royalblue r6>Polygon").at(lx - 20.0, ly + 10.0);

    let (cx, cy) = cell(0, 2, "center");
    let o = object();
    o.color("tomato")
        .width(2.0)
        .fill("rgba(255,99,71,0.30)")
        .at(cx, cy + 25.0);
    o.to(0.0).rectangle(70.0, 40.0);
    let (lx, ly) = cell(0, 2, "bottom");
    print("<white w{lw} !tomato r6>Rectangle").at(lx - 20.0, ly + 10.0);

    let (cx, cy) = cell(0, 3, "center");
    let o = object();
    o.color("gold")
        .width(2.0)
        .fill("rgba(255,215,0,0.25)")
        .at(cx, cy + 25.0);
    o.square(36.0);
    let (lx, ly) = cell(0, 3, "bottom");
    print("<black w{lw} !gold r6>Square").at(lx - 20.0, ly + 10.0);

    let (cx, cy) = cell(1, 0, "bottom");
    let wheel_radius = 12.0;
    let wheel_distance = 60.0;

    let scooter = group();

    let front_wheel = object();
    front_wheel
        .color("deepskyblue")
        .width(2.0)
        .at(cx - wheel_distance / 2.0, cy + 45.0);
    front_wheel.circle(wheel_radius);
    for spoke in 0..6 {
        let angle = spoke as f64 * 60.0;
        let rad = angle.to_radians();
        let x1 = cx - wheel_distance / 2.0;
        let y1 = cy + 45.0;
        let x2 = x1 + wheel_radius * rad.cos();
        let y2 = y1 + wheel_radius * rad.sin();
        front_wheel.line(x1, y1, x2, y2);
    }

    let back_wheel = object();
    back_wheel
        .color("deepskyblue")
        .width(2.0)
        .at(cx + wheel_distance / 2.0, cy + 45.0);
    back_wheel.circle(wheel_radius);
    for spoke in 0..6 {
        let angle = spoke as f64 * 60.0;
        let rad = angle.to_radians();
        let x1 = cx + wheel_distance / 2.0;
        let y1 = cy + 45.0;
        let x2 = x1 + wheel_radius * rad.cos();
        let y2 = y1 + wheel_radius * rad.sin();
        back_wheel.line(x1, y1, x2, y2);
    }

    let frame = object();
    frame.color("deepskyblue").width(2.5);
    frame.line(
        cx - wheel_distance / 2.0,
        cy + 45.0,
        cx + wheel_distance / 2.0,
        cy + 45.0,
    );
    frame.line(
        cx + wheel_distance / 2.0,
        cy + 45.0,
        cx + wheel_distance / 2.0,
        cy + 95.0,
    );

    scooter.add(&front_wheel);
    scooter.add(&back_wheel);
    scooter.add(&frame);

    let to_deg = |dist: f64| dist / wheel_radius * (180.0 / PI);

    scooter.translate(-35.0, 0.0);

    let d1 = 100.0;
    front_wheel.speed(45.0).rotate(to_deg(d1)).ease("linear");
    back_wheel.speed(45.0).rotate(to_deg(d1)).ease("linear");
    scooter.translate(d1, 0.0).ease("easeInOut");

    let d2 = -75.0;
    front_wheel.speed(45.0).rotate(to_deg(d2)).ease("linear");
    back_wheel.speed(45.0).rotate(to_deg(d2)).ease("linear");
    scooter.translate(d2, 0.0).ease("easeInOut");

    let (lx, ly) = cell(1, 0, "bottom");
    print("<white w{lw} !deepskyblue r6>Scooter").at(lx - 20.0, ly + 10.0);

    let (cx, cy) = cell(1, 1, "center");
    let o = object();
    o.color("magenta").width(2.0).speed(25.0).at(cx, cy + 25.0);
    o.parallelogram(70.0, 36.0, 0.0);
    o.rotate(25.0).ease("sineOut");
    let (lx, ly) = cell(1, 1, "bottom");
    print("<white w{lw} !magenta r6>Parallelogram").at(lx - 20.0, ly + 10.0);

    let (cx, cy) = cell(1, 2, "center");
    let o = object();
    o.color("yellow").width(2.0).at(cx, cy + 25.0);
    o.to(-30.0).ellipse(28.0, 16.0).speed(30.0);
    o.translate(-10.0, -8.0)
        .scale(1.3, 0.9)
        .rotate(20.0)
        .ease("elastic");
    let (lx, ly) = cell(1, 2, "bottom");
    print("<black w{lw} !orange r6>Ellipse").at(lx - 20.0, ly + 10.0);

    let (cx, cy) = cell(1, 3, "bottom");
    let arc_demo = object();
    arc_demo.color("dodgerblue").width(3.0).at(cx, cy + 35.0);
    arc_demo.to(0.0).arc(12.0, 180.0);
    let arc_clone = arc_demo.clone();
    arc_clone
        .width(2.0)
        .fill("lightskyblue")
        .translate(0.0, 60.0)
        .scale(2.0, 4.0)
        .rotate(-180.0)
        .translate(0.0, -30.0)
        .ease("sineOut");
    let (lx, ly) = cell(1, 3, "bottom");
    print("<white w{lw} !dodgerblue r6>Arc").at(lx - 20.0, ly + 10.0);

    let (cx, cy) = cell(2, 0, "center");
    let o = object();
    o.width(4.0).speed(200.0);
    let cols = [
        "navy",
        "crimson",
        "orange",
        "seagreen",
        "royalblue",
        "orchid",
        "pink",
    ];
    let angs = [0.0, 30.0, 60.0, 90.0, 120.0, 150.0, 180.0];
    for i in 0..cols.len() {
        o.color(cols[i])
            .to(angs[i])
            .at(cx, cy - 5.0)
            .forward(70.0)
            .wait(120.0);
    }
    let (lx, ly) = cell(2, 0, "bottom");
    print("<white w{lw} !slateblue r6>Color Fan").at(lx - 30.0, ly + 10.0);

    let (cx, cy) = cell(2, 1, "center");
    let flower = object();
    flower.color("red").width(2.0).at(cx, cy + 25.0);
    let petal_r = 16.0;
    for k in 0.to(12) {
        let th = (k as f64 * 30.0).to_radians();
        let (ox, oy) = (petal_r * 2.0 * th.cos(), petal_r * 2.0 * th.sin());
        flower
            .penup()
            .at(cx + ox, cy + oy + 25.0)
            .pendown()
            .to(k as f64 * 30.0)
            .circle(petal_r);
    }
    flower.at(cx, cy + 25.0).point(cx, cy + 25.0);
    flower.rotate(360.0).ease("sineInOut").reverse();
    let diamond = object();
    diamond
        .color("blue")
        .width(1.0)
        .fill("rgba(144,238,144,0.25)")
        .at(cx, cy + 25.0)
        .to(105.0)
        .rhombus(15.0, 22.0)
        .rotate(-540.0)
        .ease("back");
    let (lx, ly) = cell(2, 1, "bottom");
    print("<white w{lw} !crimson r6>Flower").at(lx - 30.0, ly + 10.0);

    let (cx, cy) = cell(2, 2, "center");
    let o = object();
    o.color("plum").width(2.0).speed(80.0);
    let k = 1.8;
    let step = 0.15;
    for u in step.to(8.0 * PI).by(step) {
        o.line(
            cx + k * (u - step) * (u - step).cos(),
            cy + k * (u - step) * (u - step).sin() + 25.0,
            cx + k * u * u.cos(),
            cy + k * u * u.sin() + 25.0,
        );
    }
    o.speed(50.0).rotate(720.0).ease("easeOut").reverse();
    let (lx, ly) = cell(2, 2, "bottom");
    print("<white w{lw} !indigo r6>Spiral").at(lx - 30.0, ly + 10.0);

    let (cx, cy) = cell(2, 3, "center");
    let r = 24.0;
    let sun_core = object();
    sun_core
        .color("gold")
        .width(2.0)
        .speed(20.0)
        .at(cx, cy + 25.0)
        .circle(r);
    for _ in 0.to(4) {
        sun_core.scale(1.5, 1.5).scale(0.63, 0.63).ease("sineInOut");
    }
    let sun_rays = object();
    sun_rays.color("gold").width(2.0).at(cx, cy + 25.0);
    for k in 0.to(24) {
        let th = k as f64 * 15.0;
        sun_rays.to(th).penup().forward(r - 6.0);
        sun_rays.pendown().forward(r + 6.0);
        sun_rays.at(cx, cy + 25.0);
    }
    let (lx, ly) = cell(2, 3, "bottom");
    print("<black w{lw} !gold r6>Sun").at(lx - 30.0, ly + 10.0);
}
