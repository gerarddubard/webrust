// webrust/src/data/mod.rs
//! # Data Structures and Interactive Visualization
//!
//! Provides intelligent data visualization, interactive chart generation,
//! professional table formatting, and Python-like string manipulation 
//! with zero-cost abstractions.
//!
//! ## Key Features
//!
//! - **Interactive charts** - Line, bar, pie, and scatter plots with ECharts
//! - **Smart table generation** - Automatic visualization from any data structure
//! - **Python-like strings** - Familiar methods with Rust performance
//! - **Intelligent splitting** - One `split_by()` method for all patterns
//! - **Rich string operations** - Case conversion, padding, validation
//! - **Fluent chaining** - Method chaining for elegant data processing
//! - **Type safety** - All operations maintain Rust's safety guarantees
//!
//! ## Modules
//!
//! - [`chart`] - Interactive data visualization with ECharts integration
//! - [`table`] - Intelligent table generation with pivot, merge, and formatting
//! - [`string`] - Python-like string methods with smart pattern recognition
//!
//! ## Complete Data Analysis Workflow
//!
//! ```ignore
//! use webrust::prelude::*;
//! use std::collections::HashMap;
//!
//! #[gui]
//! fn main() {
//!     // Data preparation
//!     let sales_data = HashMap::from([
//!         ("Q1", 85.0), ("Q2", 92.0), ("Q3", 78.0), ("Q4", 96.0)
//!     ]);
//!     
//!     // Table analysis first
//!     table(&sales_data).header(["Quarter", "Revenue (k$)"]);
//!     
//!     // Interactive visualization
//!     chart(&sales_data, "bar")
//!         .title("Quarterly Performance")
//!         .x_axis_label("Quarter")
//!         .y_axis_label("Revenue")
//!         .color("#2ecc71");
//! }
//! ```
//!
//! ## Interactive Chart Types
//!
//! ### Professional Business Charts
//! ```ignore
//! // Line charts for trends
//! chart(&revenue_trend, "line")
//!     .title("Revenue Growth")
//!     .series_name("Monthly Revenue");
//!     
//! // Bar charts for comparisons
//! chart(&regional_sales, "bar")
//!     .title("Sales by Region")
//!     .color("#3498db");
//!     
//! // Pie charts for proportions
//! let market_share = PieData(
//!     vec!["Product A".to_string(), "Product B".to_string()],
//!     vec![65.0, 35.0]
//! );
//! chart(market_share, "pie").title("Market Share");
//!
//! // Scatter plots for correlations
//! chart(&price_quality_data, "scatter")
//!     .title("Price vs Quality Analysis")
//!     .color("#e74c3c");
//! ```
//!
//! ### Seamless Table Integration
//! ```ignore
//! let product_analysis = vec![
//!     ("Product A", 25.0, 4.2, "Excellent"),
//!     ("Product B", 45.0, 4.1, "Good"),
//!     ("Product C", 65.0, 4.5, "Excellent"),
//! ];
//!
//! // Detailed analysis table
//! table(&product_analysis).header(["Product", "Price", "Rating", "Grade"]);
//!
//! // Visual representation
//! let prices: Vec<f64> = product_analysis.iter().map(|(_, p, _, _)| *p).collect();
//! chart(&prices, "scatter").title("Price Distribution");
//! ```
//!
//! ## Advanced Data Processing
//!
//! ### Python-like String Operations
//! ```ignore
//! // CSV processing pipeline
//! let csv_data = "name,price,rating\nProduct A,25.0,4.2\nProduct B,45.0,4.1";
//! let rows = csv_data.split_by("\n");
//! let headers = rows[0].split_by(",");
//!
//! // Clean and process data
//! let clean_data: Vec<Vec<String>> = rows[1..]
//!     .iter()
//!     .map(|row| {
//!         row.split_by(",")
//!             .iter()
//!             .map(|field| field.trim().to_string())
//!             .collect()
//!     })
//!     .collect();
//!     
//! table(&clean_data).header(["Product", "Price ($)", "Rating"]);
//! ```
//!
//! ### Smart Pattern Recognition
//! ```ignore
//! // Multi-pattern splitting
//! let langs = "python,rust,go".split_by(",");           // Delimiter
//! let words = "hello  world\ttab".split_by("");         // Whitespace
//! let lines = "L1\nL2\nL3".split_by("\n");             // Lines
//!
//! // Fluent data transformation
//! let result = "sales,marketing,dev"
//!     .split_by(",")
//!     .join(" → ");                                     // "sales → marketing → dev"
//! ```
//!
//! ## Business Intelligence Features
//!
//! ### Executive Dashboard
//! ```ignore
//! // KPI summary table
//! let kpis = vec![
//!     ("Revenue", "$1.2M", "+15%", "Strong Growth"),
//!     ("Users", "45K", "+8%", "Steady Growth"), 
//!     ("Conversion", "3.2%", "+0.5%", "Improving"),
//! ];
//! table(&kpis).header(["Metric", "Current", "Change", "Status"]);
//!
//! // Visual trend analysis
//! let monthly_revenue = vec![85.0, 92.0, 78.0, 96.0, 103.0, 110.0];
//! chart(&monthly_revenue, "line")
//!     .title("6-Month Revenue Trend")
//!     .x_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
//!     .color("#27ae60");
//! ```
//!
//! ### Regional Analysis with Pivot
//! ```ignore
//! let regional_data = vec![
//!     ("North", 25.0, 28.0, 22.0, 30.0, 105.0),
//!     ("South", 30.0, 32.0, 28.0, 35.0, 125.0),
//!     ("East", 20.0, 22.0, 18.0, 25.0, 85.0),
//!     ("West", 15.0, 18.0, 16.0, 20.0, 69.0),
//! ];
//!
//! // Pivot table analysis
//! table(&regional_data).header(["Region", "Q1", "Q2", "Q3", "Q4", "Total"]);
//!
//! // Regional comparison chart
//! let totals: HashMap<String, f64> = regional_data.iter()
//!     .map(|(region, _, _, _, _, total)| (region.to_string(), *total))
//!     .collect();
//! chart(&totals, "bar").title("Total Sales by Region");
//! ```
//!
//! ## Performance and Integration
//!
//! All data operations provide:
//! - **Zero-cost abstractions** - Compile-time optimizations
//! - **Memory efficiency** - Smart allocation and reuse
//! - **Type safety** - Compile-time guarantees
//! - **Interactive rendering** - Real-time chart updates
//! - **Professional styling** - Publication-ready output
//! - **Responsive design** - Automatic layout adaptation
//!
//! The data module creates a complete business intelligence platform,
//! combining Python's data processing elegance with Rust's performance
//! and interactive web-based visualization for modern data analysis workflows.

pub mod table;
pub mod string;
pub mod chart;

pub use table::*;
pub use string::*;
pub use chart::*;