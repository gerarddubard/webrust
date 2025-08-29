// webrust/src/io/mod.rs
//! # Input/Output and Data Visualization Module
//!
//! Provides comprehensive I/O capabilities including web-based GUI with integrated
//! data visualization, advanced printing with professional styling, dynamic layout
//! system, and type-safe input validation.
//!
//! ## Key Features
//!
//! - **Interactive data visualization** - Integrated ECharts for professional charts
//! - **Dynamic screen sizing** - Automatic CW/CH detection for responsive layouts
//! - **Professional styling** - CSS-like borders, colors, backgrounds, radius
//! - **Text alignment** - Left, center, right, and justify alignment
//! - **Rich text formatting** - Colors, fonts, LaTeX mathematical expressions
//! - **Type-safe input** - Real-time validation with error handling
//! - **Smart server management** - Automatic browser launch with chart integration
//!
//! ## Modules
//!
//! - [`gui`] - Web-based interface with integrated chart rendering and smart server management
//! - [`print`] - Advanced printing with CSS-like styling, LaTeX, and dynamic layouts   
//! - [`input`] - Type-safe input with real-time validation
//!
//! ## Complete Business Application
//!
//! ```ignore
//! use webrust::prelude::*;
//! use std::collections::HashMap;
//!
//! #[gui(bg = "navy", fg = "white", font = "Arial", color = "cyan", size = "14px")]
//! fn main() {
//!     // Professional header
//!     println("@(white, bold)BUSINESS INTELLIGENCE DASHBOARD")
//!         .width(*CW)
//!         .align("center")
//!         .weight(4)
//!         .color("white")
//!         .style("double")
//!         .radius(8)
//!         .background("navy");
//!     
//!     // Data analysis with table and chart
//!     let quarterly_data = HashMap::from([
//!         ("Q1", 85.0), ("Q2", 92.0), ("Q3", 78.0), ("Q4", 96.0)
//!     ]);
//!     
//!     // Detailed table analysis
//!     table(&quarterly_data).header(["Quarter", "Revenue (k$)"]);
//!     
//!     // Interactive visualization
//!     chart(&quarterly_data, "bar")
//!         .title("Quarterly Performance Analysis")
//!         .x_axis_label("Quarter")
//!         .y_axis_label("Revenue (k$)")
//!         .color("#2ecc71");
//!     
//!     // Type-safe input
//!     let target: f64 = input("Enter Q1 2025 target (k$):");
//!     
//!     // Mathematical analysis
//!     println("Growth formula: $(R_{new} = R_{old} \\times (1 + r)^t)$");
//! }
//! ```
//!
//! ## Integrated Visualization Workflow
//!
//! ### Data → Table → Chart Pipeline
//! ```ignore
//! // 1. Data preparation
//! let sales_performance = vec![
//!     ("North", 105.0), ("South", 125.0), ("East", 85.0), ("West", 69.0)
//! ];
//!
//! // 2. Detailed analysis table
//! table(&sales_performance).header(["Region", "Sales (k$)"]);
//!
//! // 3. Visual representation
//! let sales_map: HashMap<String, f64> = sales_performance.into_iter()
//!     .map(|(r, s)| (r.to_string(), s))
//!     .collect();
//! chart(&sales_map, "bar")
//!     .title("Regional Sales Performance")
//!     .color("#3498db");
//! ```
//!
//! ### Professional Report Generation
//! ```ignore
//! // Executive summary with styling
//! println("@(navy, bold)EXECUTIVE SUMMARY")
//!     .width(*CW)
//!     .align("center")
//!     .weight(3)
//!     .style("solid")
//!     .background("lightsteelblue");
//!
//! // Key metrics table
//! let metrics = vec![
//!     ("Total Revenue", "$384k", "+12%"),
//!     ("Active Users", "15.2k", "+8%"),
//!     ("Conversion Rate", "3.8%", "+0.3%"),
//! ];
//! table(&metrics).header(["Metric", "Value", "Growth"]);
//!
//! // Trend visualization
//! let monthly_trend = vec![32.0, 35.0, 38.0, 41.0, 39.0, 43.0];
//! chart(&monthly_trend, "line")
//!     .title("6-Month Growth Trajectory")
//!     .x_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
//!     .series_name("Revenue")
//!     .color("#e67e22");
//! ```
//!
//! ## Advanced Chart Integration
//!
//! ### Multiple Chart Types
//! ```ignore
//! // Line chart for trends
//! chart(&time_series_data, "line")
//!     .title("Revenue Trend Analysis")
//!     .x_axis_label("Month")
//!     .y_axis_label("Revenue ($)")
//!     .series_name("Monthly Revenue");
//!
//! // Pie chart for market share
//! let market_data = PieData(
//!     vec!["Desktop".to_string(), "Mobile".to_string(), "Tablet".to_string()],
//!     vec![50.0, 35.0, 15.0]
//! );
//! chart(market_data, "pie").title("Traffic Sources");
//!
//! // Scatter plot for correlation analysis
//! chart(&price_quality_data, "scatter")
//!     .title("Price vs Quality Correlation")
//!     .x_axis_label("Quality Score")
//!     .y_axis_label("Price ($)")
//!     .color("#e74c3c");
//! ```
//!
//! ### Interactive Dashboard Layout
//! ```ignore
//! // Four-quadrant dashboard
//! for (i, (title, data)) in dashboard_sections.iter().enumerate() {
//!     println(title)
//!         .width(*CW / 2)
//!         .align("center")
//!         .weight(2)
//!         .background("aliceblue")
//!         .radius(5);
//!         
//!     chart(data, "bar")
//!         .title(&format!("Section {}", i + 1))
//!         .color(colors[i]);
//! }
//! ```
//!
//! ## Professional Styling System
//!
//! ### Complete CSS-like Control
//! ```ignore
//! // Alert systems
//! println("@(white, bold)SUCCESS")
//!     .width(*CW)
//!     .align("center")
//!     .weight(3)
//!     .color("green")
//!     .style("solid")
//!     .radius(10)
//!     .background("lightgreen");
//!
//! // Document sections
//! println("Financial Analysis")
//!     .width(*CW)
//!     .align("justify")
//!     .weight(1)
//!     .color("darkslateblue")
//!     .style("dashed")
//!     .radius(5)
//!     .background("ghostwhite");
//! ```
//!
//! ### Responsive Grid Layouts
//! ```ignore
//! // Three-column KPI display
//! let kpis = [("Revenue", "$1.2M"), ("Users", "45k"), ("Growth", "+15%")];
//! for (label, value) in kpis {
//!     println(&format!("{}\n{}", label, value))
//!         .width(*CW / 3)
//!         .align("center")
//!         .weight(2)
//!         .color("darkblue")
//!         .background("lightcyan")
//!         .radius(8);
//! }
//! ```
//!
//! ## Mathematical and Scientific Applications
//!
//! ```ignore
//! // LaTeX mathematical expressions
//! println("Compound interest: $(A = P(1 + r)^t)$");
//! println("Normal distribution: $(f(x) = \\frac{1}{\\sqrt{2\\pi\\sigma^2}} e^{-\\frac{(x-\\mu)^2}{2\\sigma^2}})$");
//!
//! // Scientific data visualization
//! let experimental_data = vec![2.3, 4.1, 3.8, 5.2, 4.9, 6.1];
//! chart(&experimental_data, "scatter")
//!     .title("Experimental Results")
//!     .x_axis_label("Trial Number")
//!     .y_axis_label("Measurement (units)")
//!     .color("#9b59b6");
//! ```
//!
//! The I/O module provides a complete platform for professional business intelligence,
//! scientific computing, and interactive data analysis with publication-ready output
//! and modern web-based visualization capabilities.

pub mod gui;
pub mod print;
pub mod input;

pub use crate::io::gui::*;
pub use print::{print, println, process_webrust_styles_only, PrintBox, CW, CH};
pub use input::input;