// webrust/examples/py_mixed.rs
use std::collections::HashMap;
use webrust::prelude::*;

#[gui(bg="navy", fg="white", font="Arial", size = "10px")]
fn main() {
    coord("css");
    grid(3, 3);

    println("@(white, bold)WebRust Dashboard - 9 Chart Types")
        .weight(3)
        .background("dodgerblue")
        .width(*TW - 20)
        .align("center")
        .at(40.0, 0.0)
        .sticky();

    let (cx, cy) = cell(0, 0, "center");
    print("@(white)Matrix").background("navy").radius(6).width(*CW * 25 / 100).at(cx - 30.0, cy - 60.0);
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
    table(&matrix).at(cx, cy + 20.0).size(85, 85);

    let (cx, cy) = cell(0, 1, "center");
    print("@(white)Matrix with headers").background("navy").radius(6).width(*CW * 45 / 100).at(cx - 50.0, cy - 60.0);
    table(&matrix).header(["$(x)", "$(y)", "$(z)"]).at(cx, cy + 20.0).size(85, 85);

    let (cx, cy) = cell(0, 2, "center");
    print("@(white)Pivoted matrix with headers").background("navy").radius(6).width(*CW * 60 / 100).at(cx - 65.0, cy - 60.0);
    table(&matrix).header(["$(x)", "$(y)", "$(z)"])
        .pivot().header(["$(\\vec{u})", "$(\\vec{v})"])
        .size(85, 85)
        .at(cx, cy + 20.0);

    let (cx, cy) = cell(1, 0, "center");
    print("@(white)Rotating Cross").background("navy").radius(6).width(*CW * 35 / 100).at(cx - 45.0, cy - 80.0);

    let cross = object();
    cross.color("darkcyan").width(3.0).at(cx - 20.0, cy - 20.0);
    for _ in 0..4 {
        cross.forward(40.0);
        cross.left(90.0);
        cross.forward(15.0);
        cross.backward(30.0);
        cross.forward(15.0);
        cross.right(90.0);
        cross.backward(40.0);
        cross.right(90.0);
    }
    cross.speed(35.0).rotate(720.0);

    {
        let (cx, cy) = cell(1, 1, "center");
        print("@(white)Bayesian Update").background("navy").radius(6).width(*CW * 50 / 100).at(cx - 60.0, cy - 80.0);
        println("$(P(\\theta\\mid D)=\\frac{P(D\\mid \\theta)\\,P(\\theta)}{P(D)})")
            .width(*CW * 70 / 100)
            .align("center")
            .at(cx - 80.0, cy - 40.0);
    }

    {
        let (cx, cy) = cell(1, 2, "center");
        println("@(white, bold)Bouncing Balls")
            .background("darkslateblue")
            .radius(8)
            .width(*CW * 50 / 100)
            .at(cx - 45.0, cy - 80.0);
        let ground_y = cy + 60.0;
        let ball_configs: [(f64, f64, f64, &str); 4] = [
            (cx - 60.0, cy - 80.0, 10.0, "tomato"),
            (cx - 20.0, cy - 100.0, 12.0, "gold"),
            (cx + 20.0, cy - 70.0, 9.0, "limegreen"),
            (cx + 60.0, cy - 90.0, 11.0, "deepskyblue"),
        ];
        for &(x, y, radius, color) in &ball_configs {
            let ball = object();
            ball.color(color)
                .width(2.0)
                .fill(color)
                .at(x, y)
                .circle(radius);
            let e = 0.75;
            let min = 3.0;
            let mut h = ground_y - y - radius;
            ball.ease("sineIn").speed(140.0).translate(0.0, h);
            while h * e > min {
                h *= e;
                ball.ease("sineOut").speed(110.0).translate(0.0, -h);
                ball.ease("sineIn").speed(110.0).translate(0.0, h);
            }
        }
        let ground = object();
        ground.color("rgba(255,255,255,0.3)").width(1.0).line(cx - 90.0, ground_y, cx + 90.0, ground_y);
    }


    let (cx, cy) = cell(2, 0, "center");
    let quarters = HashMap::from([("Q1", 470.0), ("Q2", 620.0), ("Q3", 550.0), ("Q4", 680.0)]);
    chart(&quarters, "bar")
        .title("Quarterly Revenue")
        .color("limegreen")
        .at(cx, cy)
        .size(90, 80);

    let (cx, cy) = cell(2, 1, "center");
    let sales_data = vec![120.0, 200.0, 150.0, 300.0, 250.0];
    let months = vec!["Jan", "Feb", "Mar", "Apr", "May"];
    chart(&sales_data, "line")
        .title("Sales Trend")
        .xlabels(months.clone())
        .color("deepskyblue")
        .name("Sales")
        .at(cx, cy)
        .size(90, 80);

    let (cx, cy) = cell(2, 2, "center");
    let pie_data = PieData(
        vec!["Smartphones".to_string(), "Laptops".to_string(), "Tablets".to_string(), "Accessories".to_string()],
        vec![45.0, 30.0, 15.0, 10.0]
    );
    chart(pie_data, "pie")
        .title("Market Share 2024")
        .at(cx, cy)
        .size(90, 80);
}