// webrust/src/data/chart.rs
//! # Interactive Data Visualization
//!
//! Professional chart generation with ECharts integration supporting 9+ chart types,
//! animations, and responsive design.
//!
//! ## Chart Types
//!
//! - `line` - Trends and time series
//! - `bar` - Comparisons and categories
//! - `pie` - Proportions and percentages
//! - `doughnut` - Ring charts with inner radius
//! - `radar` - Multi-dimensional analysis
//! - `area` - Filled line charts
//! - `scatter` - Distribution and correlation
//! - `gauge` - KPI indicators
//! - `funnel` - Conversion processes
//! - `heatmap`, `candlestick`, `bubble`, `polar_area` - Specialized visualizations
//!
//! ## Examples
//!
//! ```rust,no_run
//! use webrust::prelude::*;
//! use std::collections::HashMap;
//! # #[gui] fn example() {
//! // From vector
//! let sales = vec![120.0, 200.0, 150.0, 300.0];
//! chart(&sales, "line")
//!     .title("Monthly Sales")
//!     .xlabels(vec!["Q1", "Q2", "Q3", "Q4"])
//!     .color("#2ecc71");
//!
//! // From HashMap (automatic labels)
//! let data = HashMap::from([("A", 25.0), ("B", 45.0), ("C", 35.0)]);
//! chart(&data, "bar").title("Comparison");
//!
//! // Pie chart with custom data
//! let pie_data = PieData(
//!     vec!["Product A".into(), "Product B".into()],
//!     vec![65.0, 35.0]
//! );
//! chart(pie_data, "pie").title("Market Share");
//! # }
//! ```
//!
//! Charts support absolute positioning with `.at(x, y)` and sizing with `.size(w, h)`.

use serde::Serialize;
use std::{collections::HashMap, hash::Hash, sync::atomic::{AtomicUsize, Ordering}};

static CHART_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_chart_id() -> String {
    format!("chart_{}", CHART_COUNTER.fetch_add(1, Ordering::Relaxed) + 1)
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
    title: String,
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
        let items: Vec<_> = self.pie_labels.iter().zip(self.data.iter())
            .map(|(name, value)| serde_json::json!({"name": name, "value": value})).collect();
        serde_json::to_string(&items).unwrap()
    }

    pub fn title<T: Into<String>>(mut self, t: T) -> Self { self.title = t.into(); self }
    pub fn xlabel<T: Into<String>>(mut self, l: T) -> Self { self.xlabel = l.into(); self }
    pub fn ylabel<T: Into<String>>(mut self, l: T) -> Self { self.ylabel = l.into(); self }
    pub fn xlabels<T: Into<String>>(mut self, labels: Vec<T>) -> Self {
        self.xlabels = labels.into_iter().map(Into::into).collect(); self
    }
    pub fn color<T: Into<String>>(mut self, c: T) -> Self { self.color = c.into(); self }
    pub fn name<T: Into<String>>(mut self, n: T) -> Self { self.name = Some(n.into()); self }
    pub fn at(mut self, x: f64, y: f64) -> Self { self.position = Some((x, y)); self }

    pub fn stacked(mut self, stacked: bool) -> Self {
        if stacked && !self.kind.contains("stacked") { self.kind = format!("{}_stacked", self.kind); }
        self
    }

    pub fn add<T: Into<String>>(mut self, name: T, data: Vec<f64>, color: Option<String>) -> Self {
        let series = SeriesData { name: name.into(), data, color };
        if let Some(ref mut multi) = self.multi_series { multi.push(series); }
        else { self.multi_series = Some(vec![series]); }
        self
    }

    pub fn indicators(mut self, indicators: Vec<(String, f64)>) -> Self {
        self.indicators = Some(indicators.into_iter().map(|(name, max)| RadarIndicator { name, max }).collect());
        self
    }

    pub fn heatdata(mut self, data: Vec<Vec<f64>>) -> Self { self.heatdata = Some(data); self }
    pub fn candledata(mut self, data: Vec<CandlestickPoint>) -> Self { self.candledata = Some(data); self }
    pub fn bubbledata(mut self, data: Vec<BubblePoint>) -> Self { self.bubbledata = Some(data); self }
    pub fn hole(mut self, radius: u32) -> Self { self.hole = Some(radius); self }

    fn generate_script(&self, div_id: &str) -> String {
        let start = format!(r#"<script>
setTimeout(function(){{if(!window.echarts){{setTimeout(arguments.callee,100);return;}}var el=document.getElementById('{}');if(!el){{setTimeout(arguments.callee,100);return;}}var c=echarts.init(el);"#, div_id);
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
            _ => if self.multi_series.is_some() { self.generate_multi_series_options() } else { self.generate_standard_options() }
        };
        format!("{}{}\nwindow.addEventListener('resize',function(){{c.resize();}});\n}},200);</script>", start, opts)
    }

    fn generate_pie_options(&self) -> String {
        let data = self.create_labeled_items();
        let title = serde_json::to_string(&self.title).unwrap();
        format!(r#"c.setOption({{title:{{text:{},left:'center',top:5,textStyle:{{fontSize:9}}}},tooltip:{{trigger:'item',formatter:'{{b}}: {{c}} ({{d}}%)'}},legend:{{show:false}},series:[{{name:{},type:'pie',radius:'50%',center:['50%','58%'],avoidLabelOverlap:true,label:{{fontSize:6,formatter:'{{b}}'}},labelLine:{{show:true,length:4,length2:2}},data:{}}}]}});"#, title, title, data)
    }

    fn generate_doughnut_options(&self) -> String {
        let data = self.create_labeled_items();
        let title = serde_json::to_string(&self.title).unwrap();
        let inner_radius = self.hole.unwrap_or(50);
        format!(r#"c.setOption({{title:{{text:{},left:'center',top:5,textStyle:{{fontSize:9}}}},tooltip:{{trigger:'item',formatter:'{{b}}: {{c}} ({{d}}%)'}},legend:{{show:false}},series:[{{name:{},type:'pie',radius:['{}%','70%'],center:['50%','65%'],avoidLabelOverlap:true,label:{{fontSize:6,formatter:'{{b}}'}},labelLine:{{show:true,length:4,length2:2}},emphasis:{{label:{{show:true,fontSize:8,fontWeight:'bold'}}}},data:{}}}]}});"#, title, title, inner_radius, data)
    }

    fn generate_polar_area_options(&self) -> String {
        let title = serde_json::to_string(&self.title).unwrap();
        format!(r#"c.setOption({{title:{{text:{},left:'center',top:5,textStyle:{{fontSize:9}}}},tooltip:{{trigger:'item'}},legend:{{show:false}},angleAxis:{{type:'category',data:{},startAngle:90,axisLabel:{{fontSize:7}}}},radiusAxis:{{min:0,axisLabel:{{fontSize:7}}}},polar:{{}},series:[{{type:'bar',data:{},coordinateSystem:'polar',name:{},itemStyle:{{color:function(params){{var colorList=['#c23531','#2f4554','#61a0a8','#d48265','#91c7ae','#749f83','#ca8622','#bda29a','#6e7074','#546570','#c4ccd3'];return colorList[params.dataIndex];}}}},label:{{show:true,position:'middle',formatter:'{{b}}: {{c}}',fontSize:6}}}}]}});"#, title, serde_json::to_string(&self.pie_labels).unwrap(), serde_json::to_string(&self.data).unwrap(), title)
    }

    fn generate_standard_options(&self) -> String {
        let chart_json = serde_json::to_string(self).unwrap();
        let symbol = match self.kind.as_str() { "line" => ",symbol:'none'", "scatter" => ",symbolSize:8", _ => "" };
        format!(r#"var cd={};c.setOption({{title:{{text:cd.title,left:'center',textStyle:{{fontSize:9}}}},tooltip:{{trigger:'axis'}},legend:{{show:false}},grid:{{left:30,right:10,top:30,bottom:20,containLabel:false}},xAxis:{{type:'category',name:cd.xlabel,data:cd.xlabels,axisLabel:{{fontSize:7,rotate:0}},nameTextStyle:{{fontSize:7}}}},yAxis:{{type:'value',name:cd.ylabel,axisLabel:{{fontSize:7}},nameTextStyle:{{fontSize:7}}}},series:[{{name:cd.name||'',type:cd.kind,data:cd.data,itemStyle:{{color:cd.color}}{}}}]}});"#, chart_json, symbol)
    }

    fn generate_area_options(&self) -> String {
        let stacked = self.kind.contains("stacked");
        let chart_json = serde_json::to_string(self).unwrap();
        format!(r#"var cd={};c.setOption({{title:{{text:cd.title,left:'center',textStyle:{{fontSize:9}}}},tooltip:{{trigger:'axis'}},legend:{{show:false}},grid:{{left:30,right:10,top:30,bottom:20}},xAxis:{{type:'category',data:cd.xlabels,axisLabel:{{fontSize:7}}}},yAxis:{{type:'value',axisLabel:{{fontSize:7}}}},series:[{{name:cd.name||'',type:'line',data:cd.data,areaStyle:{{}},stack:{}?'total':undefined,itemStyle:{{color:cd.color}},smooth:true}}]}});"#, chart_json, stacked)
    }

    fn generate_radar_options(&self) -> String {
        let indicators = if let Some(ref ind) = self.indicators { serde_json::to_string(ind).unwrap() } else { "[]".to_string() };
        let data = serde_json::to_string(&self.data).unwrap();
        let title = serde_json::to_string(&self.title).unwrap();
        format!(r#"c.setOption({{title:{{text:{},left:'center',top:5,textStyle:{{fontSize:9}}}},tooltip:{{}},radar:{{indicator:{},radius:'55%',center:['50%','60%'],name:{{textStyle:{{fontSize:6}}}}}},series:[{{type:'radar',data:[{{value:{},name:'{}'}}],areaStyle:{{opacity:0.3}},itemStyle:{{color:'{}'}}}}]}});"#, title, indicators, data, self.name.as_ref().unwrap_or(&String::new()), self.color)
    }

    fn generate_heatmap_options(&self) -> String {
        let heatmap_data = if let Some(ref data) = self.heatdata {
            let flat_data: Vec<_> = data.iter().enumerate().flat_map(|(y, row)| {
                row.iter().enumerate().map(move |(x, &val)| serde_json::json!([x, y, val]))
            }).collect();
            serde_json::to_string(&flat_data).unwrap()
        } else { "[]".to_string() };
        let title = serde_json::to_string(&self.title).unwrap();
        let x_labels = serde_json::to_string(&self.xlabels).unwrap();
        format!(r#"c.setOption({{title:{{text:{},left:'center',textStyle:{{fontSize:9}}}},tooltip:{{position:'top'}},grid:{{left:50,right:10,top:35,bottom:35}},xAxis:{{type:'category',data:{},splitArea:{{show:true}},axisLabel:{{fontSize:7}}}},yAxis:{{type:'category',splitArea:{{show:true}},axisLabel:{{fontSize:7}}}},visualMap:{{min:0,max:100,calculable:true,orient:'horizontal',left:'center',bottom:5,textStyle:{{fontSize:7}}}},series:[{{type:'heatmap',data:{},label:{{show:true,fontSize:6}},emphasis:{{itemStyle:{{shadowBlur:10}}}}}}]}});"#, title, x_labels, heatmap_data)
    }

    fn generate_gauge_options(&self) -> String {
        let value = self.data.first().copied().unwrap_or(0.0);
        let title = serde_json::to_string(&self.title).unwrap();
        format!(r#"c.setOption({{title:{{text:{},left:'center',top:5,textStyle:{{fontSize:9}}}},series:[{{type:'gauge',center:['50%','80%'],radius:'62%',startAngle:200,endAngle:-20,axisLine:{{lineStyle:{{width:8,color:[[0.3,'#67e0e3'],[0.7,'#37a2da'],[1,'#fd666d']]}}}},pointer:{{itemStyle:{{color:'auto'}}}},axisTick:{{show:false}},splitLine:{{distance:-10,length:7}},axisLabel:{{distance:-18,fontSize:6}},detail:{{valueAnimation:true,formatter:'{{value}}%',fontSize:9,offsetCenter:[0,'54%']}},data:[{{value:{}}}]}}]}});"#, title, value)
    }

    fn generate_funnel_options(&self) -> String {
        let data = self.create_labeled_items();
        let title = serde_json::to_string(&self.title).unwrap();
        format!(r#"c.setOption({{title:{{text:{},left:'center',textStyle:{{fontSize:9}}}},tooltip:{{trigger:'item',formatter:'{{b}}: {{c}} ({{d}}%)'}},legend:{{show:false}},series:[{{type:'funnel',left:'10%',top:28,bottom:30,width:'80%',sort:'descending',gap:2,label:{{show:true,position:'inside',fontSize:7}},labelLine:{{length:5,lineStyle:{{width:1}}}},itemStyle:{{borderColor:'#fff',borderWidth:1}},emphasis:{{label:{{fontSize:8}}}},data:{}}}]}});"#, title, data)
    }

    fn generate_candlestick_options(&self) -> String {
        let candlestick_data = if let Some(ref data) = self.candledata {
            let values: Vec<_> = data.iter().map(|c| serde_json::json!([c.open, c.close, c.low, c.high])).collect();
            serde_json::to_string(&values).unwrap()
        } else { "[]".to_string() };
        let title = serde_json::to_string(&self.title).unwrap();
        let x_labels = serde_json::to_string(&self.xlabels).unwrap();
        format!(r#"c.setOption({{title:{{text:{},left:'center',textStyle:{{fontSize:9}}}},tooltip:{{trigger:'axis',axisPointer:{{type:'cross'}}}},grid:{{left:45,right:10,top:35,bottom:35}},xAxis:{{type:'category',data:{},scale:true,axisLabel:{{fontSize:7}}}},yAxis:{{scale:true,axisLabel:{{fontSize:7}}}},series:[{{type:'candlestick',data:{},itemStyle:{{color:'#ec0000',color0:'#00da3c',borderColor:'#8a0000',borderColor0:'#008f28'}}}}]}});"#, title, x_labels, candlestick_data)
    }

    fn generate_bubble_options(&self) -> String {
        let bubble_data = if let Some(ref data) = self.bubbledata {
            let values: Vec<_> = data.iter().map(|b| serde_json::json!([b.x, b.y, b.size, b.name.as_ref().unwrap_or(&String::new())])).collect();
            serde_json::to_string(&values).unwrap()
        } else { "[]".to_string() };
        let title = serde_json::to_string(&self.title).unwrap();
        format!(r#"c.setOption({{title:{{text:{},left:'center',textStyle:{{fontSize:9}}}},tooltip:{{trigger:'item',formatter:function(params){{return params.data[3]+'<br/>X: '+params.data[0]+'<br/>Y: '+params.data[1]+'<br/>Taille: '+params.data[2];}}}},grid:{{left:45,right:10,top:35,bottom:35}},xAxis:{{name:'{}',nameTextStyle:{{fontSize:7}},splitLine:{{show:true}},axisLabel:{{fontSize:7}}}},yAxis:{{name:'{}',nameTextStyle:{{fontSize:7}},splitLine:{{show:true}},axisLabel:{{fontSize:7}}}},series:[{{name:'{}',type:'scatter',symbolSize:function(data){{return Math.sqrt(data[2])*2;}},data:{},itemStyle:{{color:'{}',opacity:0.8}},emphasis:{{itemStyle:{{shadowBlur:10,shadowOffsetX:0,shadowColor:'rgba(0, 0, 0, 0.5)'}}}}}}]}});"#, title, self.xlabel, self.ylabel, self.name.as_ref().unwrap_or(&String::new()), bubble_data, self.color)
    }

    fn generate_multi_series_options(&self) -> String {
        let multi_series = self.multi_series.as_ref().unwrap();
        let series_json: Vec<_> = multi_series.iter().map(|s| {
            let base_type = if self.kind.contains("area") { "line" } else { self.kind.split('_').next().unwrap_or(&self.kind) };
            format!(r#"{{name:'{}',type:'{}',data:{},itemStyle:{{color:'{}'}},{}}}"#, s.name, base_type, serde_json::to_string(&s.data).unwrap(), s.color.as_ref().unwrap_or(&self.color), if self.kind.contains("area") { "areaStyle:{}" } else { "" })
        }).collect();
        let title = serde_json::to_string(&self.title).unwrap();
        let x_labels = serde_json::to_string(&self.xlabels).unwrap();
        let legend_data: Vec<_> = multi_series.iter().map(|s| &s.name).collect();
        let legend_json = serde_json::to_string(&legend_data).unwrap();
        format!(r#"c.setOption({{title:{{text:{},left:'center',textStyle:{{fontSize:9}}}},tooltip:{{trigger:'axis'}},legend:{{data:{},top:20,textStyle:{{fontSize:7}}}},grid:{{left:30,right:10,top:40,bottom:20}},xAxis:{{type:'category',data:{},axisLabel:{{fontSize:7}}}},yAxis:{{type:'value',axisLabel:{{fontSize:7}}}},series:[{}]}});"#, title, legend_json, x_labels, series_json.join(","))
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
            format!(r#"<div style="position:absolute;left:{}px;top:{}px;width:{}px;height:{}px;transform:translate(-50%,-50%);"><div id="{}" style="width:100%;height:100%"></div></div>"#, left, top, width, height, id)
        } else {
            let height = self.size.map(|(_, h)| h).unwrap_or(400);
            format!(r#"<div class="chart"><div id="{}" style="height:{}px"></div></div>"#, id, height)
        };
        crate::io::gui::add_output(format!("{}{}", div, self.generate_script(&id)));
    }
}

pub trait ChartData { fn to_chart(self, kind: &str) -> Chart; }

fn basic(kind: &str, data: Vec<f64>) -> Chart {
    Chart { kind: kind.into(), title: String::new(), xlabel: String::new(), ylabel: String::new(), xlabels: Vec::new(), color: "#3498db".into(), data, name: None, pie_labels: Vec::new(), multi_series: None, indicators: None, heatdata: None, candledata: None, bubbledata: None, hole: None, position: None, size: None }
}

impl<T: Into<f64> + Copy> ChartData for &[T] {
    fn to_chart(self, kind: &str) -> Chart { basic(kind, self.iter().map(|&x| x.into()).collect()) }
}

impl<T: Into<f64> + Copy> ChartData for &Vec<T> {
    fn to_chart(self, kind: &str) -> Chart { basic(kind, self.iter().map(|&x| x.into()).collect()) }
}

impl<T: Into<f64>> ChartData for Vec<T> {
    fn to_chart(self, kind: &str) -> Chart { basic(kind, self.into_iter().map(Into::into).collect()) }
}

impl<K: ToString + Eq + Hash, V: Into<f64> + Copy> ChartData for &HashMap<K, V> {
    fn to_chart(self, kind: &str) -> Chart {
        let mut pairs: Vec<(String, f64)> = self.iter().map(|(k, v)| (k.to_string(), (*v).into())).collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        let mut ch = basic(kind, pairs.iter().map(|(_, v)| *v).collect());
        ch.xlabels = pairs.into_iter().map(|(k, _)| k).collect();
        ch
    }
}

pub struct PieData(pub Vec<String>, pub Vec<f64>);

impl ChartData for PieData {
    fn to_chart(self, _: &str) -> Chart { let mut c = basic("pie", self.1); c.pie_labels = self.0; c }
}

pub fn chart<D: ChartData>(data: D, kind: &str) -> Chart { data.to_chart(kind) }
pub fn area_chart<D: ChartData>(data: D) -> Chart { data.to_chart("area") }
pub fn radar_chart(data: Vec<f64>, indicators: Vec<(String, f64)>) -> Chart { basic("radar", data).indicators(indicators) }
pub fn heatmap_chart(data: Vec<Vec<f64>>, xlabels: Vec<String>) -> Chart { basic("heatmap", vec![]).heatdata(data).xlabels(xlabels) }
pub fn gauge_chart(value: f64) -> Chart { basic("gauge", vec![value]) }
pub fn funnel_chart(labels: Vec<String>, values: Vec<f64>) -> Chart { let mut c = basic("funnel", values); c.pie_labels = labels; c }
pub fn candlestick_chart(data: Vec<CandlestickPoint>, dates: Vec<String>) -> Chart { let mut c = basic("candlestick", vec![]); c.candledata = Some(data); c.xlabels = dates; c }
pub fn doughnut_chart(labels: Vec<String>, values: Vec<f64>) -> Chart { let mut c = basic("doughnut", values); c.pie_labels = labels; c.hole = Some(50); c }
pub fn polar_area_chart(labels: Vec<String>, values: Vec<f64>) -> Chart { let mut c = basic("polar_area", values); c.pie_labels = labels; c }
pub fn bubble_chart(data: Vec<BubblePoint>) -> Chart { basic("bubble", vec![]).bubbledata(data) }