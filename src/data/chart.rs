// webrust/src/data/chart.rs
//! # Interactive Data Visualization
//!
//! Provides seamless chart generation with ECharts integration for modern,
//! interactive data visualization directly in WebRust applications.
//!
//! ## Key Features
//!
//! - **Multiple chart types** - Line, bar, pie, and scatter plots
//! - **Fluent API** - Method chaining for elegant configuration
//! - **Type-safe data** - Automatic conversion from Rust data structures
//! - **Professional styling** - Customizable colors, labels, and themes  
//! - **Interactive** - Hover tooltips, zoom, pan, and responsive design
//! - **Zero configuration** - Works out-of-the-box with `#[gui]`
//!
//! ## Quick Examples
//!
//! ```ignore
//! use webrust::prelude::*;
//! use std::collections::HashMap;
//!
//! #[gui]
//! fn main() {
//!     // Line chart from vector
//!     let temperatures = vec![18.0, 22.0, 25.0, 24.0, 20.0];
//!     chart(&temperatures, "line")
//!         .title("Daily Temperatures")
//!         .x_axis_label("Day")
//!         .y_axis_label("°C")
//!         .color("#e67e22");
//!
//!     // Bar chart from HashMap
//!     let mut sales = HashMap::new();
//!     sales.insert("Q1", 85.0);
//!     sales.insert("Q2", 92.0);
//!     chart(&sales, "bar").title("Quarterly Sales");
//!
//!     // Pie chart with custom data
//!     let pie_data = PieData(
//!         vec!["Desktop".to_string(), "Mobile".to_string()],
//!         vec![60.0, 40.0]
//!     );
//!     chart(pie_data, "pie").title("Device Usage");
//! }
//! ```
//!
//! ## Chart Types
//!
//! ### Line Charts
//! Perfect for time series, trends, and continuous data:
//! ```ignore
//! chart(&data, "line")
//!     .title("Stock Price Trend")
//!     .x_labels(vec!["Jan", "Feb", "Mar", "Apr"])
//!     .color("#3498db")
//!     .series_name("AAPL");
//! ```
//!
//! ### Bar Charts  
//! Ideal for categorical comparisons:
//! ```ignore
//! chart(&revenue_data, "bar")
//!     .title("Revenue by Region")
//!     .x_axis_label("Region")
//!     .y_axis_label("Revenue ($M)")
//!     .color("#2ecc71");
//! ```
//!
//! ### Pie Charts
//! Great for showing proportions and market share:
//! ```ignore
//! let segments = PieData(
//!     vec!["Product A".to_string(), "Product B".to_string()],
//!     vec![65.0, 35.0]
//! );
//! chart(segments, "pie").title("Market Share 2024");
//! ```
//!
//! ### Scatter Plots
//! Excellent for correlation analysis:
//! ```ignore
//! chart(&price_data, "scatter")
//!     .title("Price vs Quality Correlation")
//!     .x_axis_label("Quality Score")
//!     .y_axis_label("Price ($)")
//!     .color("#e74c3c");
//! ```
//!
//! ## Fluent Configuration
//!
//! All charts support method chaining for intuitive setup:
//! ```ignore
//! chart(&data, "line")
//!     .title("Professional Chart")           // Chart title
//!     .x_axis_label("Time Period")          // X-axis label
//!     .y_axis_label("Value")               // Y-axis label  
//!     .x_labels(vec!["Q1", "Q2", "Q3"])    // Custom X-axis labels
//!     .color("#9b59b6")                    // Chart color
//!     .series_name("Revenue");             // Legend name
//! ```
//!
//! ## Data Type Support
//!
//! Automatic conversion from common Rust types:
//! - `Vec<T>` - Direct numeric sequences
//! - `&[T]` - Array references  
//! - `HashMap<K, V>` - Key-value pairs with automatic sorting
//! - `PieData` - Structured pie chart data
//!
//! ## Integration Benefits
//!
//! - **Automatic rendering** - Charts appear instantly in GUI
//! - **Responsive design** - Scales with window resizing
//! - **Professional appearance** - Publication-ready styling
//! - **Interactive features** - Hover effects, tooltips, zoom
//! - **Memory efficient** - Optimized chart generation and cleanup
//!
//! Charts integrate seamlessly with WebRust's table system for complete
//! data analysis workflows combining tabular data and visual representations.

use serde::Serialize;
use std::{collections::HashMap, hash::Hash, sync::atomic::{AtomicUsize, Ordering}};

static CHART_COUNTER: AtomicUsize = AtomicUsize::new(0);
fn next_chart_id()->String{ format!("chart_{}", CHART_COUNTER.fetch_add(1,Ordering::Relaxed)+1) }

#[derive(Serialize,Clone)]
pub struct Chart{
    kind:String,title:String,x_label:String,y_label:String,
    x_labels:Vec<String>,color:String,data:Vec<f64>,
    series_name:Option<String>,pie_labels:Vec<String>,
}

impl Chart{
    pub fn title<T:Into<String>>(mut self,t:T)->Self{ self.title=t.into(); self }
    pub fn x_axis_label<T:Into<String>>(mut self,l:T)->Self{ self.x_label=l.into(); self }
    pub fn y_axis_label<T:Into<String>>(mut self,l:T)->Self{ self.y_label=l.into(); self }
    pub fn x_labels<T:Into<String>>(mut self,labels:Vec<T>)->Self{ self.x_labels=labels.into_iter().map(Into::into).collect(); self }
    pub fn color<T:Into<String>>(mut self,c:T)->Self{ self.color=c.into(); self }
    pub fn series_name<T:Into<String>>(mut self,n:T)->Self{ self.series_name=Some(n.into()); self }

    fn generate_script(&self,div_id:&str)->String{
        if self.kind=="pie"{
            let items:Vec<_>=self.pie_labels.iter().zip(self.data.iter())
                .map(|(name,value)|serde_json::json!({"name":name,"value":value})).collect();
            let data=serde_json::to_string(&items).unwrap();
            let title=serde_json::to_string(&self.title).unwrap();
            format!(r#"<script>
setTimeout(function(){{
  if(!window.echarts){{setTimeout(arguments.callee,100);return;}}
  var data={};
  var el=document.getElementById('{}'); if(!el){{setTimeout(arguments.callee,100);return;}}
  var c=echarts.init(el);
  c.setOption({{
    title:{{text:{},left:'center'}},tooltip:{{trigger:'item'}},legend:{{bottom:0}},
    series:[{{name:{},type:'pie',radius:'55%',data:data}}]
  }});
  window.addEventListener('resize',function(){{c.resize();}});
}},200);</script>"#,data,div_id,title,title)
        }else{
            let chart_json=serde_json::to_string(self).unwrap();
            let sym=match self.kind.as_str(){"line"=>",symbol:'none'","scatter"=>",symbolSize:8",_=>""};
            format!(r#"<script>
setTimeout(function(){{
  if(!window.echarts){{setTimeout(arguments.callee,100);return;}}
  var cd={};
  var el=document.getElementById('{}'); if(!el){{setTimeout(arguments.callee,100);return;}}
  var c=echarts.init(el);
  c.setOption({{
    title:{{text:cd.title,left:'center'}},tooltip:{{trigger:'axis'}},
    legend:{{show:!!cd.series_name,top:24}},
    xAxis:{{type:'category',name:cd.x_label,data:cd.x_labels}},
    yAxis:{{type:'value',name:cd.y_label}},
    series:[{{name:cd.series_name||'',type:cd.kind,data:cd.data,itemStyle:{{color:cd.color}}{}}}]
  }});
  window.addEventListener('resize',function(){{c.resize();}});
}},200);</script>"#,chart_json,div_id,sym)
        }
    }
}

impl Drop for Chart{
    fn drop(&mut self){
        let id=next_chart_id();
        let html=format!(r#"<div class="chart"><div id="{}" style="height:400px"></div></div>{}"#,id,self.generate_script(&id));
        crate::io::gui::add_output(html);
    }
}

pub trait ChartData{ fn to_chart(self,kind:&str)->Chart; }

fn basic(kind:&str,data:Vec<f64>)->Chart{
    Chart{ kind:kind.into(), title:String::new(), x_label:String::new(), y_label:String::new(),
        x_labels:Vec::new(), color:"#3498db".into(), data, series_name:None, pie_labels:Vec::new() }
}

impl<T:Into<f64>+Copy> ChartData for &[T]{ fn to_chart(self,kind:&str)->Chart{ basic(kind,self.iter().map(|&x|x.into()).collect()) } }
impl<T:Into<f64>+Copy> ChartData for &Vec<T>{ fn to_chart(self,kind:&str)->Chart{ basic(kind,self.iter().map(|&x|x.into()).collect()) } }
impl<T:Into<f64>> ChartData for Vec<T>{ fn to_chart(self,kind:&str)->Chart{ basic(kind,self.into_iter().map(Into::into).collect()) } }

impl<K:ToString+Eq+Hash,V:Into<f64>+Copy> ChartData for &HashMap<K,V>{
    fn to_chart(self,kind:&str)->Chart{
        let mut pairs:Vec<(String,f64)>=self.iter().map(|(k,v)|(k.to_string(),(*v).into())).collect();
        pairs.sort_by(|a,b|a.0.cmp(&b.0));
        let mut ch=basic(kind,pairs.iter().map(|(_,v)|*v).collect());
        ch.x_labels=pairs.into_iter().map(|(k,_)|k).collect(); ch
    }
}

pub struct PieData(pub Vec<String>,pub Vec<f64>);
impl ChartData for PieData{ fn to_chart(self,_:&str)->Chart{ let mut c=basic("pie",self.1); c.pie_labels=self.0; c } }

pub fn chart<D:ChartData>(data:D,kind:&str)->Chart{ data.to_chart(kind) }