use std::collections::HashMap;
use webrust::prelude::*;

#[gui(Arial, 14px, black, !white)]
fn main() {
    println("<white !navy r8 p6 w400 h50>WebRust Charts — 9 Chart Types").align("center");

    let sales_data = vec![120.0, 200.0, 150.0, 300.0, 250.0];
    let months = vec!["Jan", "Feb", "Mar", "Apr", "May"];

    // 1. Line Chart
    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w360 h40 m15>1. Line Chart — Trends").align("center");
    chart(&sales_data, "line")
        .xlabels(months.clone())
        .color("#3498db")
        .name("Sales");
    println("<green !lightgreen r4 p4 w280 h36>Sales Trend").align("center");
    println("<gray t1 dashed |silver w{*TW} m25>").align("center");

    // 2. Bar Chart
    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w360 h40 m15>2. Bar Chart — Comparisons").align("center");
    let quarters = HashMap::from([("Q1", 470.0), ("Q2", 620.0), ("Q3", 550.0), ("Q4", 680.0)]);
    chart(&quarters, "bar").color("#2ecc71");
    println("<green !lightgreen r4 p4 w280 h36>Quarterly Revenue").align("center");
    println("<gray t1 dashed |silver w{*TW} m25>").align("center");

    // 3. Pie Chart
    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w360 h40 m15>3. Pie Chart — Proportions").align("center");
    let pie_data = PieData(
        vec![
            "Smartphones".to_string(),
            "Laptops".to_string(),
            "Tablets".to_string(),
            "Accessories".to_string(),
        ],
        vec![45.0, 30.0, 15.0, 10.0],
    );
    chart(pie_data, "pie");
    println("<green !lightgreen r4 p4 w280 h36>Market Share 2024").align("center");
    println("<gray t1 dashed |silver w{*TW} m25>").align("center");

    // 4. Doughnut Chart
    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w360 h40 m15>4. Doughnut Chart — Categories").align("center");
    let categories = vec!["Web", "Mobile", "Desktop", "Other"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();
    let cat_values = vec![45.0, 30.0, 20.0, 5.0];
    doughnut_chart(categories, cat_values);
    println("<green !lightgreen r4 p4 w280 h36>Traffic by Channel").align("center");
    println("<gray t1 dashed |silver w{*TW} m25>").align("center");

    // 5. Radar Chart
    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w360 h40 m15>5. Radar Chart — Multi-dimensional").align("center");
    let skills = vec![85.0, 90.0, 75.0, 95.0, 80.0];
    let indicators = vec![
        ("Technical".to_string(), 100.0),
        ("Communication".to_string(), 100.0),
        ("Leadership".to_string(), 100.0),
        ("Innovation".to_string(), 100.0),
        ("Quality".to_string(), 100.0),
    ];
    radar_chart(skills, indicators).color("#9b59b6").name("Scores");
    println("<green !lightgreen r4 p4 w280 h36>Team Profile A").align("center");
    println("<gray t1 dashed |silver w{*TW} m25>").align("center");

    // 6. Area Chart
    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w360 h40 m15>6. Area Chart — Evolution").align("center");
    let visitors = vec![1200.0, 1350.0, 1500.0, 1800.0, 2100.0];
    area_chart(&visitors)
        .xlabels(months.clone())
        .color("#e67e22")
        .name("Visitors");
    println("<green !lightgreen r4 p4 w280 h36>Visitor Growth").align("center");
    println("<gray t1 dashed |silver w{*TW} m25>").align("center");

    // 7. Scatter Chart
    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w360 h40 m15>7. Scatter Chart — Distribution").align("center");
    let prices = vec![25.0, 45.0, 35.0, 65.0, 55.0];
    chart(&prices, "scatter")
        .xlabels(vec!["A", "B", "C", "D", "E"])
        .color("#e74c3c")
        .name("Price");
    println("<green !lightgreen r4 p4 w280 h36>Product Prices").align("center");
    println("<gray t1 dashed |silver w{*TW} m25>").align("center");

    // 8. Gauge Chart
    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w360 h40 m15>8. Gauge Chart — Indicators").align("center");
    gauge_chart(87.5);
    println("<green !lightgreen r4 p4 w280 h36>Customer Satisfaction").align("center");
    println("<gray t1 dashed |silver w{*TW} m25>").align("center");

    // 9. Funnel Chart
    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w360 h40 m15>9. Funnel Chart — Conversion").align("center");
    let stages = vec!["Visitors", "Signups", "Active", "Purchases"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();
    let funnel_values = vec![10000.0, 5000.0, 2500.0, 350.0];
    funnel_chart(stages, funnel_values);
    println("<green !lightgreen r4 p4 w280 h36>Conversion Funnel").align("center");

    // Footer
    println("<white !navy r6 p6 w420 h50 m20>✨ All 9 chart types are responsive and interactive!").align("center");
    println("<gray i w420 mc>Powered by ECharts — Professional Data Visualization").align("center");
}