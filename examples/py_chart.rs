// webrust/examples/py_chart.rs
use std::collections::HashMap;
use webrust::prelude::*;

#[gui(bg="navy", fg="white", font="Arial")]
fn main() {
    println("@(white, bold)WebRust Charts - 9 Chart Types")
        .weight(3)
        .background("dodgerblue")
        .width(*TW - 20)
        .align("center");

    let sales_data = vec![120.0, 200.0, 150.0, 300.0, 250.0];
    let months = vec!["Jan", "Feb", "Mar", "Apr", "May"];

    println("\n@(cyan, bold)1. Line Chart - Trends");
    chart(&sales_data, "line")
        .title("Sales Trend")
        .xlabels(months.clone())
        .color("#3498db")
        .name("Sales");

    println("\n@(cyan, bold)2. Bar Chart - Comparisons");
    let quarters = HashMap::from([
        ("Q1", 470.0), ("Q2", 620.0), ("Q3", 550.0), ("Q4", 680.0)
    ]);
    chart(&quarters, "bar")
        .title("Quarterly Revenue")
        .color("#2ecc71");

    println("\n@(cyan, bold)3. Pie Chart - Proportions");
    let pie_data = PieData(
        vec!["Smartphones".to_string(), "Laptops".to_string(), "Tablets".to_string(), "Accessories".to_string()],
        vec![45.0, 30.0, 15.0, 10.0]
    );
    chart(pie_data, "pie")
        .title("Market Share 2024");

    println("\n@(cyan, bold)4. Doughnut Chart - Categories");
    let categories = vec!["Web".to_string(), "Mobile".to_string(), "Desktop".to_string(), "Other".to_string()];
    let cat_values = vec![45.0, 30.0, 20.0, 5.0];
    doughnut_chart(categories, cat_values)
        .title("Traffic by Channel");

    println("\n@(cyan, bold)5. Radar Chart - Multi-dimensional");
    let skills = vec![85.0, 90.0, 75.0, 95.0, 80.0];
    let indicators = vec![
        ("Technical".to_string(), 100.0),
        ("Communication".to_string(), 100.0),
        ("Leadership".to_string(), 100.0),
        ("Innovation".to_string(), 100.0),
        ("Quality".to_string(), 100.0),
    ];
    radar_chart(skills, indicators)
        .title("Team Profile A")
        .color("#9b59b6")
        .name("Scores");

    println("\n@(cyan, bold)6. Area Chart - Evolution");
    let visitors = vec![1200.0, 1350.0, 1500.0, 1800.0, 2100.0];
    area_chart(&visitors)
        .title("Visitor Growth")
        .xlabels(months.clone())
        .color("#e67e22")
        .name("Visitors");

    println("\n@(cyan, bold)7. Scatter Chart - Distribution");
    let prices = vec![25.0, 45.0, 35.0, 65.0, 55.0];
    chart(&prices, "scatter")
        .title("Product Prices")
        .xlabels(vec!["A", "B", "C", "D", "E"])
        .color("#e74c3c")
        .name("Price");

    println("\n@(cyan, bold)8. Gauge Chart - Indicators");
    gauge_chart(87.5)
        .title("Customer Satisfaction");

    println("\n@(cyan, bold)9. Funnel Chart - Conversion");
    let stages = vec![
        "Visitors".to_string(),
        "Signups".to_string(),
        "Active".to_string(),
        "Purchases".to_string(),
    ];
    let funnel_values = vec![10000.0, 5000.0, 2500.0, 350.0];
    funnel_chart(stages, funnel_values)
        .title("Conversion Funnel");

    println("\n@(bright_green, bold)All chart types are available and responsive!");
}