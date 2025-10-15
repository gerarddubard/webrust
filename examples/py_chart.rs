// examples/py_chart.rs
// Run with: cargo run --example chart
//
// This example demonstrates WebRust's comprehensive charting capabilities
// powered by ECharts, covering 9 different chart types for data visualization.
//
// Chart types demonstrated:
// 1. Line Chart - Time series, trends, continuous data
// 2. Bar Chart - Comparisons, discrete categories
// 3. Pie Chart - Proportions, market share, composition
// 4. Doughnut Chart - Similar to pie, with center hole
// 5. Radar Chart - Multi-dimensional comparisons, profiles
// 6. Area Chart - Filled line charts, cumulative trends
// 7. Scatter Chart - Distribution, correlation, outliers
// 8. Gauge Chart - Single value indicators, KPIs, progress
// 9. Funnel Chart - Sequential process, conversion rates
//
// Core features:
// * chart(&data, "type") - Universal chart function
// * Specialized functions: doughnut_chart(), radar_chart(), area_chart()
// * Method chaining: .title().color().name().xlabels()
// * Automatic data formatting from Vec, HashMap, custom types
// * Responsive sizing and interactive tooltips
// * Professional styling with customizable colors
//
// Data format patterns:
// * Simple Vec: vec![120.0, 200.0, 150.0] - Values only
// * HashMap: HashMap::from([("Q1", 470.0), ...]) - Labeled data
// * PieData: PieData(labels, values) - Explicit label-value pairs
// * Tuple Vec: vec![(x1, y1), (x2, y2)] - Scatter data
//
// Method chaining options:
// * .title(str) - Chart title
// * .color(str) - Primary color (hex or CSS name)
// * .name(str) - Series name (for legend)
// * .xlabels(vec) - X-axis labels
// * .ylabels(vec) - Y-axis labels (radar chart)
// * .at(x, y) - Absolute positioning
// * .size(w, h) - Width and height percentages
//
// Choosing the right chart:
// * Line: Time series, trends over time
// * Bar: Category comparisons, rankings
// * Pie/Doughnut: Part-to-whole relationships, percentages
// * Radar: Multi-attribute comparisons (skills, features)
// * Area: Cumulative values, volume over time
// * Scatter: Correlations, distributions, outliers
// * Gauge: Single metric, progress indicators
// * Funnel: Sequential stages, conversion funnels
//
// Performance notes:
// * Server-side data preparation, client-side rendering
// * ECharts handles animation and interaction
// * Responsive to window resizing
// * Lazy loading - charts render as they appear
//
// Tips:
// * Use line charts for temporal data with continuous x-axis
// * Bar charts work best with 3-12 categories
// * Pie charts should have 3-7 slices maximum
// * Radar charts excel at comparing 4-8 dimensions
// * Scatter plots reveal correlations and outliers
// * Gauge charts are perfect for single KPI dashboards
// * Funnel charts show drop-off at each stage
// * Color consistency: Use brand colors across charts
// * Title clarity: Describe what the chart shows
//
// Color palette suggestions:
// * Primary: #3498db (blue), #2ecc71 (green), #e74c3c (red)
// * Accent: #9b59b6 (purple), #f39c12 (orange), #1abc9c (teal)
// * Neutral: #95a5a6 (gray), #34495e (dark gray)

use std::collections::HashMap;
use webrust::prelude::*;

#[gui(bg="navy", fg="white", font="Arial")]
fn main() {
    println("@(white, bold)WebRust Charts - 9 Chart Types")
        .weight(3)
        .background("dodgerblue")
        .width(*TW - 20)
        .align("center");

    // -------------------------------------------------------------------------
    // 1) Line Chart - Best for trends and time series
    // -------------------------------------------------------------------------
    let sales_data = vec![120.0, 200.0, 150.0, 300.0, 250.0];
    let months = vec!["Jan", "Feb", "Mar", "Apr", "May"];

    println("\n@(cyan, bold)1. Line Chart - Trends");
    chart(&sales_data, "line")
        .title("Sales Trend")
        .xlabels(months.clone())
        .color("#3498db")
        .name("Sales");

    // -------------------------------------------------------------------------
    // 2) Bar Chart - Best for comparing discrete categories
    // -------------------------------------------------------------------------
    println("\n@(cyan, bold)2. Bar Chart - Comparisons");
    let quarters = HashMap::from([
        ("Q1", 470.0), ("Q2", 620.0), ("Q3", 550.0), ("Q4", 680.0)
    ]);
    chart(&quarters, "bar")
        .title("Quarterly Revenue")
        .color("#2ecc71");

    // -------------------------------------------------------------------------
    // 3) Pie Chart - Best for part-to-whole relationships
    // -------------------------------------------------------------------------
    println("\n@(cyan, bold)3. Pie Chart - Proportions");
    let pie_data = PieData(
        vec!["Smartphones".to_string(), "Laptops".to_string(),
             "Tablets".to_string(), "Accessories".to_string()],
        vec![45.0, 30.0, 15.0, 10.0]
    );
    chart(pie_data, "pie")
        .title("Market Share 2024");

    // -------------------------------------------------------------------------
    // 4) Doughnut Chart - Like pie, with aesthetic center
    // -------------------------------------------------------------------------
    println("\n@(cyan, bold)4. Doughnut Chart - Categories");
    let categories = vec!["Web".to_string(), "Mobile".to_string(),
                          "Desktop".to_string(), "Other".to_string()];
    let cat_values = vec![45.0, 30.0, 20.0, 5.0];
    doughnut_chart(categories, cat_values)
        .title("Traffic by Channel");

    // -------------------------------------------------------------------------
    // 5) Radar Chart - Best for multi-dimensional comparisons
    // -------------------------------------------------------------------------
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

    // -------------------------------------------------------------------------
    // 6) Area Chart - Best for volume and cumulative trends
    // -------------------------------------------------------------------------
    println("\n@(cyan, bold)6. Area Chart - Evolution");
    let visitors = vec![1200.0, 1350.0, 1500.0, 1800.0, 2100.0];
    area_chart(&visitors)
        .title("Visitor Growth")
        .xlabels(months.clone())
        .color("#e67e22")
        .name("Visitors");

    // -------------------------------------------------------------------------
    // 7) Scatter Chart - Best for distribution and correlation
    // -------------------------------------------------------------------------
    println("\n@(cyan, bold)7. Scatter Chart - Distribution");
    let prices = vec![25.0, 45.0, 35.0, 65.0, 55.0];
    chart(&prices, "scatter")
        .title("Product Prices")
        .xlabels(vec!["A", "B", "C", "D", "E"])
        .color("#e74c3c")
        .name("Price");

    // -------------------------------------------------------------------------
    // 8) Gauge Chart - Best for single value indicators
    // -------------------------------------------------------------------------
    println("\n@(cyan, bold)8. Gauge Chart - Indicators");
    gauge_chart(87.5)
        .title("Customer Satisfaction");

    // -------------------------------------------------------------------------
    // 9) Funnel Chart - Best for sequential conversion processes
    // -------------------------------------------------------------------------
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

    println("\n@(bright_green, bold)✨ All 9 chart types are responsive and interactive!");
    println("@(gray, italic)Powered by ECharts - Professional data visualization");
}