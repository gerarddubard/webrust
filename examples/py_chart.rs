// webrust/examples/py_chart.rs
use std::collections::HashMap;
use webrust::prelude::*;

#[gui(bg = "grey", fg = "white", font = "Courier New", color = "black", size = "14px")]
fn main(){
    println("\n@(darkgrey, bold)WebRust Charts - Complete Demonstration").width(*CW).align("center");

    // === 1. Weekly Temperature Evolution ===
    println("\n@(blue, bold)1. Weekly Temperature Evolution");
    let temp = vec![64.4, 67.1, 69.8, 73.4, 75.2, 71.6, 68.0]; // Fahrenheit
    let days = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    let temp_data: Vec<(String, f64, String)> = days.iter().zip(temp.iter())
        .map(|(d, &t)| (d.to_string(), t,
                        if t < 68.0 { "Cool".to_string() }
                        else if t < 72.0 { "Pleasant".to_string() }
                        else { "Warm".to_string() }))
        .collect();
    table(&temp_data).header(["Day", "Temperature (°F)", "Category"]);
    chart(&temp, "line")
        .title("Weekly Temperature Trend")
        .x_axis_label("Day")
        .y_axis_label("°F")
        .x_labels(vec!["Mon","Tue","Wed","Thu","Fri","Sat","Sun"])
        .color("#e67e22")
        .series_name("Max Temperature");

    // === 2. Sales Distribution by Product ===
    println("\n@(green, bold)2. Sales Distribution by Product");
    let sales = HashMap::from([
        ("Smartphones", 45.0), ("Laptops", 30.0), ("Tablets", 15.0), ("Accessories", 10.0)
    ]);
    let total_sales: f64 = sales.values().sum();
    let sales_analysis: Vec<(String, f64, f64, String)> = sales.iter()
        .map(|(product, &amount)| {
            let percentage = (amount / total_sales) * 100.0;
            let status = if percentage > 40.0 { "Leader" }
            else if percentage > 20.0 { "Important" }
            else { "Niche" };
            (product.to_string(), amount, percentage, status.to_string())
        })
        .collect();
    table(&sales_analysis).header(["Product", "Sales (k$)", "Share (%)", "Category"]);
    let pie_data = PieData(
        vec!["Smartphones".to_string(), "Laptops".to_string(), "Tablets".to_string(), "Accessories".to_string()],
        vec![45.0, 30.0, 15.0, 10.0]
    );
    chart(pie_data, "pie").title("Market Share 2024");

    // === 3. Quarterly Performance Analysis ===
    println("\n@(purple, bold)3. Quarterly Performance Analysis");
    let quarters = HashMap::from([
        ("Q1 2024", 85.0), ("Q2 2024", 92.0), ("Q3 2024", 78.0), ("Q4 2024", 96.0)
    ]);
    let revenue_data: Vec<(String, f64, f64, String)> = {
        let mut sorted: Vec<_> = quarters.iter().collect();
        sorted.sort_by_key(|(k, _)| *k);
        sorted.windows(2).enumerate()
            .map(|(i, window)| {
                let (quarter, &revenue) = window[1];
                let growth = if i == 0 { 0.0 } else {
                    ((revenue - window[0].1) / window[0].1) * 100.0
                };
                let trend = if growth > 10.0 { "Strong Growth" }
                else if growth > 0.0 { "Growth" }
                else if growth > -10.0 { "Decline" }
                else { "Strong Decline" };
                (quarter.to_string(), revenue, growth, trend.to_string())
            })
            .collect()
    };
    table(&revenue_data).header(["Quarter", "Revenue (k$)", "Growth (%)", "Trend"]);
    chart(&quarters, "bar")
        .title("Revenue by Quarter")
        .x_axis_label("Quarter")
        .y_axis_label("Revenue (k$)")
        .color("#2ecc71");

    // === 4. Regional Sales Analysis with Pivot ===
    println("\n@(cyan, bold)4. Regional Sales Analysis");
    let regional_sales = vec![
        ("North", 25.0, 28.0, 22.0, 30.0, 105.0),
        ("South", 30.0, 32.0, 28.0, 35.0, 125.0),
        ("East", 20.0, 22.0, 18.0, 25.0, 85.0),
        ("West", 15.0, 18.0, 16.0, 20.0, 69.0),
        ("Total", 90.0, 100.0, 84.0, 110.0, 384.0),
    ];
    table(&regional_sales).header(["Region", "Q1", "Q2", "Q3", "Q4", "Annual"]);
    let regional_totals: HashMap<String, f64> = regional_sales.iter()
        .filter(|(region, _, _, _, _, _)| *region != "Total")
        .map(|(region, _, _, _, _, annual)| (region.to_string(), *annual))
        .collect();
    chart(&regional_totals, "bar")
        .title("Total Sales by Region")
        .x_axis_label("Region")
        .y_axis_label("Total Sales (k$)")
        .color("#3498db");

    // === 5. Mathematical Function Analysis ===
    println("\n@(red, bold)5. Mathematical Function Analysis");
    let key_points: Vec<(f64, f64)> = vec![
        (-2.0, 7.0),
        (0.0, -1.0),
        (1.0, -2.0),
        (3.0, 2.0),
    ];
    table(&key_points).header(["x", "f(x)", "Property"]).pivot();
    let x_values: Vec<f64> = (-20..=20).map(|i| i as f64 / 4.0).collect();
    let y_values: Vec<f64> = x_values.iter().map(|&x| x*x - 2.0*x - 1.0).collect();
    chart(&y_values, "line")
        .title("f(x) = x² - 2x - 1")
        .x_axis_label("x")
        .y_axis_label("f(x)")
        .x_labels(x_values.iter().map(|x| format!("{:.1}", x)).collect())
        .color("#9b59b6")
        .series_name("f(x)");

    // === 6. Price-Quality Correlation ===
    println("\n@(orange, bold)6. Price-Quality Correlation");
    let product_data = vec![
        ("Product A", 25.0, 3.2), ("Product B", 45.0, 4.1), ("Product C", 65.0, 4.5),
        ("Product D", 30.0, 3.8), ("Product E", 80.0, 4.7), ("Product F", 15.0, 2.9),
        ("Product G", 55.0, 4.3), ("Product H", 70.0, 4.6), ("Product I", 35.0, 3.5),
        ("Product J", 90.0, 4.8)
    ];
    let product_analysis: Vec<(String, f64, f64, String)> = product_data.iter()
        .map(|(name, price, rating)| {
            let value_ratio = rating / price * 100.0;
            let value_grade = if value_ratio > 8.0 { "Excellent" }
            else if value_ratio > 6.0 { "Good" }
            else if value_ratio > 4.0 { "Average" }
            else { "Poor" };
            (name.to_string(), *price, *rating, value_grade.to_string())
        })
        .collect();
    table(&product_analysis).header(["Product", "Price ($)", "Rating (/5)", "Value Grade"]);
    let prices: Vec<f64> = product_data.iter().map(|(_, p, _)| *p).collect();
    chart(&prices, "scatter")
        .title("Price vs Market Position")
        .x_axis_label("Position")
        .y_axis_label("Price ($)")
        .x_labels((1..=product_data.len()).map(|i| i.to_string()).collect())
        .color("#e74c3c")
        .series_name("Products");

    // === 7. Executive Dashboard ===
    println("\n@(magenta, bold)7. Executive Dashboard");
    let dashboard = vec![
        ("Temperature", "Weekly Trend", "64-75°F range", "7 day cycle"),
        ("Sales", "Market Share", "$100k total", "4 product lines"),
        ("Revenue", "Quarterly Growth", "Q4 strongest", "+23% vs Q3"),
        ("Regional", "Geographic Spread", "4 regions", "South leads"),
        ("Function", "Mathematical", "Parabola curve", "Min at x=1"),
        ("Products", "Price Analysis", "10 items", "Variable value ratio"),
    ];
    table(&dashboard).header(["Analysis", "Type", "Key Finding", "Details"]);

    println("\n@(bright_green, bold)Analysis Complete - Charts and Tables Optimized for Business Intelligence");
}