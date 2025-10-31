// webrust/src/viz/chart.rs
//! # WebRust Charts — ECharts-based plotting with a fluent Rust API
//!
// //! `viz::chart` renders charts by emitting a container `<div>` plus a small
//! initialization `<script>` that configures [Apache ECharts] in the browser.
//!
//! ## Supported chart kinds
//! - **Line / Bar / Scatter**: `"line"`, `"bar"`, `"scatter"`
//! - **Area**: `"area"` and `"area_stacked"` (via `.stacked(true)`)
//! - **Pie / Doughnut**: `"pie"`, `"doughnut"` (with `.hole(%)` for inner radius)
//! - **Polar area**: `"polar_area"`
//! - **Radar**
//! - **Heatmap**
//! - **Gauge**
//! - **Funnel**
//! - **Candlestick**
//! - **Bubble**
//!
//! ## Data adapters
//! Implemented via the `ChartData` trait:
//! - `&[T]`, `&Vec<T>`, `Vec<T>` where `T: Into<f64>`
//! - `&HashMap<K, V>` (keys become x-axis labels; values are numbers)
//! - `PieData(Vec<String>, Vec<f64>)` for labeled pie-like series
//!
//! ## Builder methods
//! - `.xlabel(str)`, `.ylabel(str)`, `.xlabels(Vec<String>)`
//! - `.color(css)` — series color (default `#3498db`)
//! - `.name(str)` — series or chart name
//! - `.add(name, data: Vec<f64>, color: Option<String>)` — multi-series
//! - `.stacked(true)` — for area/line/bar variants
//! - `.indicators(Vec<(String, f64)>)` — radar axes
//! - `.heatdata(Vec<Vec<f64>>)` — heatmap values
//! - `.candledata(Vec<CandlestickPoint>)`
//! - `.bubbledata(Vec<BubblePoint>)`
//! - `.hole(percent)` — doughnut inner radius
//! - `.at(x, y)` — absolute positioning (center-anchored)
//! - `.size(w, h)` — fixed size in pixels
//!
//! ## Integration details
//! - A unique `<div id="chart_X">` is emitted plus a script that waits for ECharts
//!   to be available and then calls `echarts.init(..).setOption({...})`.
//! - When `.at(x,y)` is used, the container is absolutely positioned and centered
//!   on `(x,y)` after converting from the current coordinate mode.
//! - Without `.size(..)`, sensible defaults are applied based on container classes.
//!
//! ## Examples
//! Line:
//! ```rust
//! use webrust::prelude::*;
//! chart(vec![1, 3, 2, 5, 4], "line")
//!     .xlabels(vec!["Mon","Tue","Wed","Thu","Fri"])
//!     .color("#1abc9c")
//!     .name("Visits");
//! ```
//!
//! Pie / Doughnut:
//! ```rust
//! use webrust::prelude::*;
//! chart(PieData(vec!["A".into(),"B".into(),"C".into()], vec![30.0, 45.0, 25.0]), "pie");
//! doughnut_chart(vec!["Chrome".into(),"Firefox".into()], vec![64.0, 36.0]).hole(55);
//! ```
//!
//! Multi-series area (stacked):
//! ```rust
//! use webrust::prelude::*;
//! chart(vec![10.0,12.0,9.0], "area")
//!     .xlabels(vec!["Q1","Q2","Q3"])
//!     .add("Project A", vec![10.0,12.0,9.0], None)
//!     .add("Project B", vec![8.0,11.0,7.0], Some("#e67e22".into()))
//!     .stacked(true);
//! ```
//!
//! Candlestick:
//! ```rust
//! use webrust::prelude::*;
//! let data = vec![
//!   CandlestickPoint{open:10.0, close:12.0, low:9.5, high:12.5},
//!   CandlestickPoint{open:12.0, close:11.8, low:11.0, high:12.2},
//! ];
//! candlestick_chart(data, vec!["2025-10-01".into(),"2025-10-02".into()]);
//! ```
//!
//! Bubble:
//! ```rust
//! use webrust::prelude::*;
//! let pts = vec![
//!   BubblePoint{x:10.0,y:20.0,size:15.0,name:Some("A".into())},
//!   BubblePoint{x:12.0,y:18.0,size:30.0,name:Some("B".into())},
//! ];
//! bubble_chart(pts).xlabel("X").ylabel("Y").color("#9b59b6").name("Cities");
//! ```
//!
//! ## Requirements
//! - The page must load Apache ECharts on `window.echarts` (provided by WebRust’s `index.html`).
//!
//! ## Safety
//! - All chart configuration is serialized to JSON. No `unsafe` code is used.
//!

use serde::Serialize;
use std::{
    collections::HashMap,
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

static CHART_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_chart_id() -> String {
    format!(
        "chart_{}",
        CHART_COUNTER.fetch_add(1, Ordering::Relaxed) + 1
    )
}

#[derive(Serialize, Clone)]
pub struct SeriesData {
    name: String,
    data: Vec<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct RadarIndicator {
    name: String,
    max: f64,
}

#[derive(Serialize, Clone)]
pub struct CandlestickPoint {
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
}

#[derive(Serialize, Clone)]
pub struct BubblePoint {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct Chart {
    kind: String,
    xlabel: String,
    ylabel: String,
    xlabels: Vec<String>,
    color: String,
    data: Vec<f64>,
    name: Option<String>,
    pie_labels: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_series: Option<Vec<SeriesData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indicators: Option<Vec<RadarIndicator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heatdata: Option<Vec<Vec<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    candledata: Option<Vec<CandlestickPoint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bubbledata: Option<Vec<BubblePoint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hole: Option<u32>,
    #[serde(skip)]
    position: Option<(f64, f64)>,
    #[serde(skip)]
    size: Option<(u32, u32)>,
}

impl Chart {
    fn create_labeled_items(&self) -> String {
        let items: Vec<_> = self
            .pie_labels
            .iter()
            .zip(self.data.iter())
            .map(|(name, value)| serde_json::json!({"name": name, "value": value}))
            .collect();
        serde_json::to_string(&items).unwrap()
    }

    pub fn xlabel<T: Into<String>>(mut self, l: T) -> Self {
        self.xlabel = l.into();
        self
    }
    pub fn ylabel<T: Into<String>>(mut self, l: T) -> Self {
        self.ylabel = l.into();
        self
    }
    pub fn xlabels<T: Into<String>>(mut self, labels: Vec<T>) -> Self {
        self.xlabels = labels.into_iter().map(Into::into).collect();
        self
    }
    pub fn color<T: Into<String>>(mut self, c: T) -> Self {
        self.color = c.into();
        self
    }
    pub fn name<T: Into<String>>(mut self, n: T) -> Self {
        self.name = Some(n.into());
        self
    }
    pub fn at(mut self, x: f64, y: f64) -> Self {
        self.position = Some((x, y));
        self
    }
    pub fn stacked(mut self, stacked: bool) -> Self {
        if stacked && !self.kind.contains("stacked") {
            self.kind = format!("{}_stacked", self.kind);
        }
        self
    }
    pub fn add<T: Into<String>>(mut self, name: T, data: Vec<f64>, color: Option<String>) -> Self {
        let series = SeriesData {
            name: name.into(),
            data,
            color,
        };
        if let Some(ref mut multi) = self.multi_series {
            multi.push(series);
        } else {
            self.multi_series = Some(vec![series]);
        }
        self
    }
    pub fn indicators(mut self, indicators: Vec<(String, f64)>) -> Self {
        self.indicators = Some(
            indicators
                .into_iter()
                .map(|(name, max)| RadarIndicator { name, max })
                .collect(),
        );
        self
    }
    pub fn heatdata(mut self, data: Vec<Vec<f64>>) -> Self {
        self.heatdata = Some(data);
        self
    }
    pub fn candledata(mut self, data: Vec<CandlestickPoint>) -> Self {
        self.candledata = Some(data);
        self
    }
    pub fn bubbledata(mut self, data: Vec<BubblePoint>) -> Self {
        self.bubbledata = Some(data);
        self
    }
    pub fn hole(mut self, radius: u32) -> Self {
        self.hole = Some(radius);
        self
    }

    fn generate_script(&self, div_id: &str) -> String {
        let start = format!(
            r#"<script>
requestAnimationFrame(function init(){{if(!window.echarts||!document.getElementById('{}')){{requestAnimationFrame(init);return;}}var c=echarts.init(document.getElementById('{}'));
"#,
            div_id, div_id
        );
        let opts = match self.kind.as_str() {
            "pie" => self.generate_pie_options(),
            "doughnut" => self.generate_doughnut_options(),
            "polar_area" => self.generate_polar_area_options(),
            "area" | "area_stacked" => self.generate_area_options(),
            "radar" => self.generate_radar_options(),
            "heatmap" => self.generate_heatmap_options(),
            "gauge" => self.generate_gauge_options(),
            "funnel" => self.generate_funnel_options(),
            "candlestick" => self.generate_candlestick_options(),
            "bubble" => self.generate_bubble_options(),
            _ => {
                if self.multi_series.is_some() {
                    self.generate_multi_series_options()
                } else {
                    self.generate_standard_options()
                }
            }
        };
        format!(
            "{}{}\nwindow.addEventListener('resize',function(){{c.resize();}});\n}});</script>",
            start, opts
        )
    }

    fn generate_pie_options(&self) -> String {
        let data = self.create_labeled_items();
        format!(
            r#"c.setOption({{tooltip:{{trigger:'item',formatter:'{{b}}: {{c}} ({{d}}%)'}},legend:{{show:false}},series:[{{type:'pie',radius:'50%',center:['50%','50%'],avoidLabelOverlap:true,label:{{fontSize:6,formatter:'{{b}}'}},labelLine:{{show:true,length:4,length2:2}},data:{}}}]}});"#,
            data
        )
    }

    fn generate_doughnut_options(&self) -> String {
        let data = self.create_labeled_items();
        let inner_radius = self.hole.unwrap_or(50);
        format!(
            r#"c.setOption({{tooltip:{{trigger:'item',formatter:'{{b}}: {{c}} ({{d}}%)'}},legend:{{show:false}},series:[{{type:'pie',radius:['{}%','70%'],center:['50%','50%'],avoidLabelOverlap:true,label:{{fontSize:6,formatter:'{{b}}'}},labelLine:{{show:true,length:4,length2:2}},emphasis:{{label:{{show:true,fontSize:8,fontWeight:'bold'}}}},data:{}}}]}});"#,
            inner_radius, data
        )
    }

    fn generate_polar_area_options(&self) -> String {
        format!(
            r#"c.setOption({{tooltip:{{trigger:'item'}},angleAxis:{{}},radiusAxis:{{type:'category',data:{}}},polar:{{}},legend:{{show:false}},series:[{{type:'bar',data:{},coordinateSystem:'polar',label:{{show:false}}}}]}});"#,
            serde_json::to_string(&self.pie_labels).unwrap(),
            serde_json::to_string(&self.data).unwrap()
        )
    }

    fn generate_area_options(&self) -> String {
        if self.multi_series.is_some() {
            return self.generate_multi_series_options();
        }
        let x_labels = serde_json::to_string(&self.xlabels).unwrap();
        let data = serde_json::to_string(&self.data).unwrap();
        format!(
            r#"c.setOption({{tooltip:{{trigger:'axis'}},grid:{{left:30,right:10,top:10,bottom:20}},xAxis:{{type:'category',boundaryGap:false,data:{},axisLabel:{{fontSize:7}}}},yAxis:{{type:'value',axisLabel:{{fontSize:7}}}},series:[{{data:{},type:'line',areaStyle:{{}},smooth:true,itemStyle:{{color:'{}'}},lineStyle:{{width:2}}}}]}});"#,
            x_labels, data, self.color
        )
    }

    fn generate_radar_options(&self) -> String {
        let ind = if let Some(ref i) = self.indicators {
            serde_json::to_string(i).unwrap()
        } else {
            "[]".into()
        };
        let data = serde_json::to_string(&self.data).unwrap();
        let name = self.name.as_deref().unwrap_or("");
        format!(
            r#"c.setOption({{tooltip:{{trigger:'item'}},legend:{{show:false}},radar:{{indicator:{}}},series:[{{type:'radar',data:[{{value:{},name:'{}'}}],symbol:'circle',symbolSize:6,itemStyle:{{color:'{}',borderWidth:1,borderColor:'#333'}},lineStyle:{{width:2}},areaStyle:{{opacity:0.3}}}}]}});"#,
            ind, data, name, self.color
        )
    }

    fn generate_heatmap_options(&self) -> String {
        let heatmap_data = if let Some(ref data) = self.heatdata {
            let mut values = Vec::new();
            for (i, row) in data.iter().enumerate() {
                for (j, &value) in row.iter().enumerate() {
                    values.push(serde_json::json!([j, i, value]));
                }
            }
            serde_json::to_string(&values).unwrap()
        } else {
            "[]".to_string()
        };
        let x_labels = serde_json::to_string(&self.xlabels).unwrap();
        let y_count = self.heatdata.as_ref().map(|d| d.len()).unwrap_or(0);
        let y_labels: Vec<String> = (0..y_count).map(|i| format!("Row {}", i)).collect();
        let y_labels_json = serde_json::to_string(&y_labels).unwrap();
        format!(
            r#"c.setOption({{tooltip:{{position:'top'}},grid:{{height:'60%',top:'10%'}},xAxis:{{type:'category',data:{},splitArea:{{show:true}}}},yAxis:{{type:'category',data:{},splitArea:{{show:true}}}},visualMap:{{min:0,max:100,calculable:true,orient:'horizontal',left:'center',bottom:'5%'}},series:[{{type:'heatmap',data:{},label:{{show:true}}}}]}});"#,
            x_labels, y_labels_json, heatmap_data
        )
    }

    fn generate_gauge_options(&self) -> String {
        let value = self.data.get(0).copied().unwrap_or(0.0);
        format!(
            r#"c.setOption({{series:[{{type:'gauge',startAngle:200,endAngle:-20,min:0,max:100,splitNumber:10,itemStyle:{{color:'{}'}},progress:{{show:true,width:18}},pointer:{{show:false}},axisLine:{{lineStyle:{{width:18}}}},axisTick:{{distance:-30,splitNumber:5,lineStyle:{{width:2,color:'#999'}}}},splitLine:{{distance:-40,length:14,lineStyle:{{width:3,color:'#999'}}}},axisLabel:{{distance:-20,color:'#999',fontSize:10}},anchor:{{show:false}},detail:{{valueAnimation:true,width:'60%',lineHeight:20,borderRadius:8,offsetCenter:[0,'-15%'],fontSize:20,fontWeight:'bolder',formatter:'{{value}}%',color:'inherit'}},data:[{{value:{}}}]}}]}});"#,
            self.color, value
        )
    }

    fn generate_funnel_options(&self) -> String {
        let data = self.create_labeled_items();
        format!(
            r#"c.setOption({{tooltip:{{trigger:'item',formatter:'{{b}}: {{c}} ({{d}}%)'}},legend:{{show:false}},series:[{{type:'funnel',left:'10%',top:10,bottom:10,width:'80%',sort:'descending',gap:2,label:{{show:true,position:'inside',fontSize:7}},labelLine:{{length:5,lineStyle:{{width:1}}}},itemStyle:{{borderColor:'#fff',borderWidth:1}},emphasis:{{label:{{fontSize:8}}}},data:{}}}]}});"#,
            data
        )
    }

    fn generate_candlestick_options(&self) -> String {
        let candlestick_data = if let Some(ref data) = self.candledata {
            let values: Vec<_> = data
                .iter()
                .map(|c| serde_json::json!([c.open, c.close, c.low, c.high]))
                .collect();
            serde_json::to_string(&values).unwrap()
        } else {
            "[]".to_string()
        };
        let x_labels = serde_json::to_string(&self.xlabels).unwrap();
        format!(
            r#"c.setOption({{tooltip:{{trigger:'axis',axisPointer:{{type:'cross'}}}},grid:{{left:45,right:10,top:15,bottom:35}},xAxis:{{type:'category',data:{},scale:true,axisLabel:{{fontSize:7}}}},yAxis:{{scale:true,axisLabel:{{fontSize:7}}}},series:[{{type:'candlestick',data:{},itemStyle:{{color:'#ec0000',color0:'#00da3c',borderColor:'#8a0000',borderColor0:'#008f28'}}}}]}});"#,
            x_labels, candlestick_data
        )
    }

    fn generate_bubble_options(&self) -> String {
        let bubble_data = if let Some(ref data) = self.bubbledata {
            let values: Vec<_> = data
                .iter()
                .map(|b| {
                    serde_json::json!([b.x, b.y, b.size, b.name.as_ref().unwrap_or(&String::new())])
                })
                .collect();
            serde_json::to_string(&values).unwrap()
        } else {
            "[]".to_string()
        };
        format!(
            r#"c.setOption({{tooltip:{{trigger:'item',formatter:function(params){{return params.data[3]+'<br/>X: '+params.data[0]+'<br/>Y: '+params.data[1]+'<br/>Taille: '+params.data[2];}}}},grid:{{left:45,right:10,top:15,bottom:35}},xAxis:{{name:'{}',nameTextStyle:{{fontSize:7}},splitLine:{{show:true}},axisLabel:{{fontSize:7}}}},yAxis:{{name:'{}',nameTextStyle:{{fontSize:7}},splitLine:{{show:true}},axisLabel:{{fontSize:7}}}},series:[{{name:'{}',type:'scatter',symbolSize:function(data){{return Math.sqrt(data[2])*2;}},data:{},itemStyle:{{color:'{}',opacity:0.8}},emphasis:{{itemStyle:{{shadowBlur:10,shadowOffsetX:0,shadowColor:'rgba(0, 0, 0, 0.5)'}}}}}}]}});"#,
            self.xlabel,
            self.ylabel,
            self.name.as_ref().unwrap_or(&String::new()),
            bubble_data,
            self.color
        )
    }

    fn generate_standard_options(&self) -> String {
        let x_labels = serde_json::to_string(&self.xlabels).unwrap();
        let data = serde_json::to_string(&self.data).unwrap();
        let chart_type = if self.kind == "scatter" {
            "scatter"
        } else {
            self.kind.as_str()
        };
        format!(
            r#"c.setOption({{tooltip:{{trigger:'axis'}},grid:{{left:30,right:10,top:10,bottom:20}},xAxis:{{type:'category',data:{},axisLabel:{{fontSize:7}}}},yAxis:{{type:'value',axisLabel:{{fontSize:7}}}},series:[{{data:{},type:'{}',itemStyle:{{color:'{}'}},lineStyle:{{width:2}}}}]}});"#,
            x_labels, data, chart_type, self.color
        )
    }

    fn generate_multi_series_options(&self) -> String {
        let multi_series = self.multi_series.as_ref().unwrap();
        let series_json: Vec<_> = multi_series
            .iter()
            .map(|s| {
                let base_type = if self.kind.contains("area") {
                    "line"
                } else {
                    self.kind.split('_').next().unwrap_or(&self.kind)
                };
                format!(
                    r#"{{name:'{}',type:'{}',data:{},itemStyle:{{color:'{}'}},{}}}"#,
                    s.name,
                    base_type,
                    serde_json::to_string(&s.data).unwrap(),
                    s.color.as_ref().unwrap_or(&self.color),
                    if self.kind.contains("area") {
                        "areaStyle:{}"
                    } else {
                        ""
                    }
                )
            })
            .collect();
        let x_labels = serde_json::to_string(&self.xlabels).unwrap();
        let legend_data: Vec<_> = multi_series.iter().map(|s| &s.name).collect();
        let legend_json = serde_json::to_string(&legend_data).unwrap();
        format!(
            r#"c.setOption({{tooltip:{{trigger:'axis'}},legend:{{data:{},top:5,textStyle:{{fontSize:7}}}},grid:{{left:30,right:10,top:30,bottom:20}},xAxis:{{type:'category',data:{},axisLabel:{{fontSize:7}}}},yAxis:{{type:'value',axisLabel:{{fontSize:7}}}},series:[{}]}});"#,
            legend_json,
            x_labels,
            series_json.join(",")
        )
    }
}

impl crate::layout::grid::Sizable for Chart {
    fn set_size(&mut self, size: (u32, u32)) {
        self.size = Some(size);
    }
}

impl Drop for Chart {
    fn drop(&mut self) {
        let id = next_chart_id();
        let div = if let Some((x, y)) = self.position {
            let (left, top) = crate::layout::coord::to_screen_coords(x, y);
            let (width, height) = self.size.unwrap_or((400, 300));
            format!(
                r#"<div style="position:absolute;left:{}px;top:{}px;width:{}px;height:{}px;transform:translate(-50%,-50%);"><div id="{}" style="width:100%;height:100%"></div></div>"#,
                left, top, width, height, id
            )
        } else {
            let (class, height) = match self.size {
                Some((w, h)) if w <= 160 || h <= 160 => ("chart-container chart-small", h),
                Some((w, h)) if w <= 220 || h <= 220 => ("chart-container chart-medium", h),
                Some((_, h)) => ("chart-container", h),
                None => ("chart-container", 400),
            };
            format!(
                r#"<div class="{}"><div id="{}" style="height:{}px"></div></div>"#,
                class, id, height
            )
        };
        crate::io::gui::add_output(format!("{}{}", div, self.generate_script(&id)));
    }
}

pub trait ChartData {
    fn to_chart(self, kind: &str) -> Chart;
}

fn basic(kind: &str, data: Vec<f64>) -> Chart {
    Chart {
        kind: kind.into(),
        xlabel: String::new(),
        ylabel: String::new(),
        xlabels: Vec::new(),
        color: "#3498db".into(),
        data,
        name: None,
        pie_labels: Vec::new(),
        multi_series: None,
        indicators: None,
        heatdata: None,
        candledata: None,
        bubbledata: None,
        hole: None,
        position: None,
        size: None,
    }
}

impl<T: Into<f64> + Copy> ChartData for &[T] {
    fn to_chart(self, kind: &str) -> Chart {
        basic(kind, self.iter().map(|&x| x.into()).collect())
    }
}

impl<T: Into<f64> + Copy> ChartData for &Vec<T> {
    fn to_chart(self, kind: &str) -> Chart {
        basic(kind, self.iter().map(|&x| x.into()).collect())
    }
}

impl<T: Into<f64>> ChartData for Vec<T> {
    fn to_chart(self, kind: &str) -> Chart {
        basic(kind, self.into_iter().map(Into::into).collect())
    }
}

impl<K: ToString + Eq + Hash, V: Into<f64> + Copy> ChartData for &HashMap<K, V> {
    fn to_chart(self, kind: &str) -> Chart {
        let mut pairs: Vec<(String, f64)> = self
            .iter()
            .map(|(k, v)| (k.to_string(), (*v).into()))
            .collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        let mut ch = basic(kind, pairs.iter().map(|(_, v)| *v).collect());
        ch.xlabels = pairs.into_iter().map(|(k, _)| k).collect();
        ch
    }
}

pub struct PieData(pub Vec<String>, pub Vec<f64>);

impl ChartData for PieData {
    fn to_chart(self, _: &str) -> Chart {
        let mut c = basic("pie", self.1);
        c.pie_labels = self.0;
        c
    }
}

pub fn chart<D: ChartData>(data: D, kind: &str) -> Chart {
    data.to_chart(kind)
}
pub fn area_chart<D: ChartData>(data: D) -> Chart {
    data.to_chart("area")
}
pub fn radar_chart(data: Vec<f64>, indicators: Vec<(String, f64)>) -> Chart {
    basic("radar", data).indicators(indicators)
}
pub fn heatmap_chart(data: Vec<Vec<f64>>, xlabels: Vec<String>) -> Chart {
    basic("heatmap", vec![]).heatdata(data).xlabels(xlabels)
}
pub fn gauge_chart(value: f64) -> Chart {
    basic("gauge", vec![value])
}
pub fn funnel_chart(labels: Vec<String>, values: Vec<f64>) -> Chart {
    let mut c = basic("funnel", values);
    c.pie_labels = labels;
    c
}
pub fn candlestick_chart(data: Vec<CandlestickPoint>, dates: Vec<String>) -> Chart {
    let mut c = basic("candlestick", vec![]);
    c.candledata = Some(data);
    c.xlabels = dates;
    c
}
pub fn doughnut_chart(labels: Vec<String>, values: Vec<f64>) -> Chart {
    let mut c = basic("doughnut", values);
    c.pie_labels = labels;
    c.hole = Some(50);
    c
}
pub fn polar_area_chart(labels: Vec<String>, values: Vec<f64>) -> Chart {
    let mut c = basic("polar_area", values);
    c.pie_labels = labels;
    c
}
pub fn bubble_chart(data: Vec<BubblePoint>) -> Chart {
    basic("bubble", vec![]).bubbledata(data)
}
