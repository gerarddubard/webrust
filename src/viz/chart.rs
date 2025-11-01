// webrust/src/io/chart.rs
//! # Module `io::chart` — Minimal, fast ECharts wrapper for WebRust
//!
//! This module exposes a tiny, allocation-lean API to render common charts in
//! the browser using ECharts. It is designed to be:
//! - **Simple:** one constructor per chart type (`line`, `bar`, `pie`, `radar`, …).
//! - **Zero-setup:** charts auto-mount and render when dropped.
//! - **Compact:** minimal JS per chart, no global state beyond an internal id counter.
//!
//! ## Quick start
//! ```rust,no_run
//! use webrust::prelude::*;
//!
//! #[gui(Arial 14px black !white)]
//! fn main() {
//!     let months = ["Jan","Feb","Mar","Apr","May"];
//!     let sales  = [120.0, 200.0, 150.0, 300.0, 250.0];
//!
//!     line(&months, &sales);      // category line
//!     bar(&months, &sales);       // category bar
//!     pie(&["A","B","C"], &[60.0,30.0,10.0]);
//!
//!     let skills = [85.0,90.0,75.0,95.0,80.0];
//!     let inds   = [("Technical",100.0),("Communication",100.0),("Leadership",100.0),
//!                   ("Innovation",100.0),("Quality",100.0)];
//!     radar(&skills, &inds);      // interactive radar
//! }
//! ```
//!
//! ## Chart constructors
//! Each constructor returns a `Chart` that renders automatically when it goes out
//! of scope (on `Drop`). You can optionally position/size a chart with `.at(x,y)`
//! (Cartesian coordinates managed by WebRust) and `.size(w,h)` (pixels).
//!
//! **Category (x = labels, y = values)**
//! - `line(labels, values)` — Polyline (tooltip on axis).
//! - `curve(labels, values)` — Smoothed line.
//! - `area(labels, values)` — Filled line area.
//! - `bar(labels, values)` — Vertical bars.
//!
//! **XY pairs (numeric x,y)**
//! - `line_xy(x, y)` / `curve_xy(x, y)` / `area_xy(x, y)` — Use numeric x axis.
//! - `scatter(x, y)` — Dots with item tooltip.
//!
//! **Compositions**
//! - `pie(labels, values)` — Standard pie (`values` are % or any totals).
//! - `doughnut(labels, values)` — Pie with a hole (default inner radius 50%).
//! - `funnel(stages)` — Descending funnel (`&[(&str, f64)]`).
//! - `gauge(value)` — 0–100 radial gauge with % tooltip.
//!
//! **Radar**
//! - `radar(values, indicators)` — One-series radar with custom indicators
//!   (`indicators: &[(&str, f64)]` where `max` is the axis maximum).
//!   The chart shows clean axis names by default and reveals `name: value %` near
//!   the corresponding key when hovering the vertex (no clutter, one value at a time).
//!
//! **Matrix / Heatmap**
//! - `matrix(data, xlabels, ylabels)` — Heatmap with auto visual scale.
//!
//! **Tree / Treemap**
//! - `treemap(items)` — Accepts flat items or path-like `"A/B/C"` entries.
//! - `tree(input)` — Accepts:
//!   - `&serde_json::Value` (object becomes a tree),
//!   - `&str` (JSON string or newline-separated paths),
//!   - `&[&str]` (path slices).
//!
//! **Function sampling**
//! - `function(f, x0, x1, step)` — Samples `f64 -> f64` and renders a smooth curve.
//!
//! ## Layout helpers
//! - `.at(x, y)` — Places the chart at world coordinates (converted by WebRust).
//! - `.size(w, h)` — Sets container size (px). Without `.size`, a sensible
//!   default height is applied depending on the responsive container class.
//!
//! ## Rendering model
//! - Charts render via ECharts once the GUI is ready (async init in an inline `<script>`).
//! - A unique DOM id is generated using an atomic counter; no global mutable Rust state
//!   is exposed.
//! - `Drop` injects the container `<div>` + the minimal JS to configure the instance.
//!
//! ## Styling
//! - Charts inherit the global theme from `#[gui(...)]` (font, base color, background)
//!   through the surrounding page styles.
//! - Axis/label font sizes are tuned for compact layouts (e.g., `fontSize: 7` on axes).
//! - Radar axes show names in black; the hovered key/value is highlighted for clarity.
//!
//! ## Data contracts & panics
//! - All constructors expect **matching lengths** between label/value vectors where
//!   applicable. Mismatches or missing internals will cause a panic via `unwrap()`.
//! - Radar requires `values.len() == indicators.len()`.
//! - `matrix` requires consistent row lengths; `xlabels.len()` must match columns,
//!   `ylabels.len()` must match rows.
//!
//! ## Performance & footprint
//! - JS emitted per chart is small and self-contained; no shared registries.
//! - The Rust side avoids unnecessary cloning beyond converting inputs to owned
//!   vectors (to survive `Drop` scheduling).
//!
//! ## Accessibility & behavior
//! - Tooltips are concise; radar shows a single value near the hovered vertex
//!   to reduce overload.
//! - Charts resize responsively; a resize listener updates layout.
//!
//! ## Troubleshooting
//! - **Blank chart:** ensure ECharts is available in the page (WebRust GUI loads it).
//! - **Panic at runtime:** verify inputs (vector lengths, percentages, matrix dimensions).
//! - **Small texts:** increase container size with `.size(w,h)` or change GUI font size.
//!
//! ---


use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};

static CHART_COUNTER: AtomicUsize = AtomicUsize::new(0);
fn next_chart_id() -> String { format!("chart_{}", CHART_COUNTER.fetch_add(1, Ordering::Relaxed) + 1) }

#[derive(Serialize, Clone)]
pub struct RadarIndicator { name: String, max: f64 }

#[derive(Serialize, Clone)]
pub struct Chart {
    kind: String,
    x: Option<Vec<f64>>,
    y: Option<Vec<f64>>,
    x_labels: Option<Vec<String>>,
    smooth: bool,
    pie_labels: Option<Vec<String>>,
    pie_values: Option<Vec<f64>>,
    indicators: Option<Vec<RadarIndicator>>,
    radar_values: Option<Vec<f64>>,
    gauge_val: Option<f64>,
    funnel_labels: Option<Vec<String>>,
    funnel_values: Option<Vec<f64>>,
    hole: Option<u32>,
    matrix: Option<Vec<Vec<f64>>>,
    matrix_xlabels: Option<Vec<String>>,
    matrix_ylabels: Option<Vec<String>>,
    treemap_data: Option<serde_json::Value>,
    tree_data: Option<serde_json::Value>,
    position: Option<(f64, f64)>,
    size: Option<(u32, u32)>,
}

impl Chart {
    fn script(&self, div_id: &str) -> String {
        let start = format!(r#"<script>
requestAnimationFrame(function init(){{if(!window.echarts||!document.getElementById('{}')){{requestAnimationFrame(init);return;}}var c=echarts.init(document.getElementById('{}'));"#, div_id, div_id);
        let opts = match self.kind.as_str() {
            "line" => self.opt_line(false),
            "curve" => self.opt_line(true),
            "area" => self.opt_area(),
            "line_xy" => self.opt_line_xy(false),
            "curve_xy" => self.opt_line_xy(true),
            "area_xy" => self.opt_area_xy(),
            "bar" => self.opt_bar(),
            "scatter" => self.opt_scatter(),
            "pie" => self.opt_pie(false),
            "doughnut" => self.opt_pie(true),
            "radar" => self.opt_radar(div_id),
            "gauge" => self.opt_gauge(),
            "funnel" => self.opt_funnel(),
            "matrix" => self.opt_matrix(),
            "treemap" => self.opt_treemap(),
            "tree" => self.opt_tree(),
            _ => String::new(),
        };
        format!("{}{}\nwindow.addEventListener('resize',function(){{c.resize();}});}});</script>", start, opts)
    }

    fn ds_pairs(&self) -> String {
        let xs = self.x.as_ref().unwrap();
        let ys = self.y.as_ref().unwrap();
        serde_json::to_string(&xs.iter().zip(ys.iter()).map(|(a,b)| serde_json::json!([a,b])).collect::<Vec<_>>()).unwrap()
    }

    fn tooltip_xy_axis() -> &'static str {
        "formatter:(p)=>{if(!p||!p.length)return'';const d=p[0].data;return Array.isArray(d)?`(${d[0].toFixed(2)}, ${d[1].toFixed(2)})`:`(${p[0].axisValue}, ${p[0].data.toFixed(2)})`;}"
    }

    fn opt_line_xy(&self, smooth: bool) -> String {
        let ds = self.ds_pairs();
        let sm = if smooth || self.smooth { "true" } else { "false" };
        format!(r#"c.setOption({{
 tooltip:{{trigger:'axis',{}}},
 grid:{{left:30,right:10,top:10,bottom:20}},
 xAxis:{{type:'value',axisLabel:{{fontSize:7}}}},
 yAxis:{{type:'value',axisLabel:{{fontSize:7}},axisLine:{{show:true}}}},
 dataset:{{source:{}}},
 series:[{{type:'line',encode:{{x:0,y:1}},smooth:{},lineStyle:{{width:2}}}}]
}});"#, Self::tooltip_xy_axis(), ds, sm)
    }

    fn opt_area_xy(&self) -> String {
        let ds = self.ds_pairs();
        format!(r#"c.setOption({{
 tooltip:{{trigger:'axis',{}}},
 grid:{{left:30,right:10,top:10,bottom:20}},
 xAxis:{{type:'value',axisLabel:{{fontSize:7}}}},
 yAxis:{{type:'value',axisLabel:{{fontSize:7}}}},
 dataset:{{source:{}}},
 series:[{{type:'line',encode:{{x:0,y:1}},smooth:true,areaStyle:{{}},lineStyle:{{width:2}}}}]
}});"#, Self::tooltip_xy_axis(), ds)
    }

    fn opt_line(&self, smooth: bool) -> String {
        let labels = serde_json::to_string(self.x_labels.as_ref().unwrap()).unwrap();
        let values = serde_json::to_string(self.y.as_ref().unwrap()).unwrap();
        let sm = if smooth || self.smooth { "true" } else { "false" };
        format!(r#"c.setOption({{
 tooltip:{{trigger:'axis'}},
 grid:{{left:30,right:10,top:10,bottom:20}},
 xAxis:{{type:'category',data:{},axisLabel:{{fontSize:7}}}},
 yAxis:{{type:'value',axisLabel:{{fontSize:7}}}},
 series:[{{type:'line',data:{},smooth:{},lineStyle:{{width:2}}}}]
}});"#, labels, values, sm)
    }

    fn opt_area(&self) -> String {
        let labels = serde_json::to_string(self.x_labels.as_ref().unwrap()).unwrap();
        let values = serde_json::to_string(self.y.as_ref().unwrap()).unwrap();
        format!(r#"c.setOption({{
 tooltip:{{trigger:'axis'}},
 grid:{{left:30,right:10,top:10,bottom:20}},
 xAxis:{{type:'category',data:{},axisLabel:{{fontSize:7}}}},
 yAxis:{{type:'value',axisLabel:{{fontSize:7}}}},
 series:[{{type:'line',data:{},smooth:true,areaStyle:{{}},lineStyle:{{width:2}}}}]
}});"#, labels, values)
    }

    fn opt_bar(&self) -> String {
        let labels = serde_json::to_string(self.x_labels.as_ref().unwrap()).unwrap();
        let values = serde_json::to_string(self.y.as_ref().unwrap()).unwrap();
        format!(r#"c.setOption({{
 tooltip:{{trigger:'axis',axisPointer:{{type:'shadow'}}}},
 grid:{{left:30,right:10,top:10,bottom:20}},
 xAxis:{{type:'category',data:{},axisLabel:{{fontSize:7}}}},
 yAxis:{{type:'value',axisLabel:{{fontSize:7}}}},
 series:[{{type:'bar',data:{}}}]
}});"#, labels, values)
    }

    fn opt_scatter(&self) -> String {
        let ds = self.ds_pairs();
        format!(r#"c.setOption({{
 tooltip:{{trigger:'item',formatter:(p)=>`(${{p.data[0].toFixed(2)}}, ${{p.data[1].toFixed(2)}})`}},
 grid:{{left:45,right:10,top:15,bottom:35}},
 xAxis:{{type:'value',axisLabel:{{fontSize:7}}}},
 yAxis:{{type:'value',axisLabel:{{fontSize:7}}}},
 dataset:{{source:{}}},
 series:[{{type:'scatter',encode:{{x:0,y:1}},symbolSize:6}}]
}});"#, ds)
    }

    fn opt_pie(&self, doughnut: bool) -> String {
        let labels = self.pie_labels.as_ref().unwrap();
        let values = self.pie_values.as_ref().unwrap();
        let data = serde_json::to_string(&labels.iter().zip(values.iter()).map(|(n,v)| serde_json::json!({"name":n,"value":v})).collect::<Vec<_>>()).unwrap();
        let radius = if doughnut { format!(r#"["{}%","70%"]"#, self.hole.unwrap_or(50)) } else { r#""50%""#.into() };
        format!(r#"c.setOption({{
 tooltip:{{trigger:'item',formatter:'{{b}}: {{d}}%'}},
 legend:{{show:false}},
 series:[{{type:'pie',radius:{},center:['50%','50%'],avoidLabelOverlap:true,label:{{fontSize:7,formatter:'{{b}}'}},labelLine:{{length:5,length2:3}},data:{}}}]
}});"#, radius, data)
    }

    fn opt_radar(&self, div_id: &str) -> String {
        let inds = serde_json::to_string(self.indicators.as_ref().unwrap()).unwrap();
        let vals = serde_json::to_string(self.radar_values.as_ref().unwrap()).unwrap();
        let indicators = self.indicators.as_ref().unwrap();
        let values = self.radar_values.as_ref().unwrap();
        let mut names = Vec::new();
        let mut vals_list = Vec::new();
        for (i, ind) in indicators.iter().enumerate() {
            names.push(format!("'{}'", ind.name));
            vals_list.push(values[i].to_string());
        }
        let names_js = format!("[{}]", names.join(","));
        let vals_js = format!("[{}]", vals_list.join(","));

        format!(r#"c.setOption({{
 tooltip:{{show:false}},
 legend:{{show:false}},
 radar:{{indicator:{}}},
 series:[{{
  type:'radar',
  data:[{{value:{},name:'Data'}}],
  symbol:'circle',
  symbolSize:6,
  lineStyle:{{width:2}},
  areaStyle:{{opacity:0.25}}
 }}]
}});
var showValues=false;
var names={};
var values={};
document.getElementById('{}').addEventListener('mouseenter',function(){{
 showValues=true;
 c.setOption({{
  radar:{{
   indicator:names.map(function(n,i){{return{{name:n+': '+values[i]+' %',max:100}};}})
  }}
 }});
}});
document.getElementById('{}').addEventListener('mouseleave',function(){{
 showValues=false;
 c.setOption({{
  radar:{{
   indicator:names.map(function(n){{return{{name:n,max:100}};}})
  }}
 }});
}});"#, inds, vals, names_js, vals_js, div_id, div_id)
    }

    fn opt_gauge(&self) -> String {
        let v = self.gauge_val.unwrap_or(0.0);
        format!(r#"c.setOption({{
 tooltip:{{show:true,formatter:'{{c}}%'}},
 series:[{{type:'gauge',startAngle:200,endAngle:-20,min:0,max:100,progress:{{show:true,width:12}},pointer:{{show:true,length:'70%'}},axisLine:{{lineStyle:{{width:12}}}},detail:{{valueAnimation:true,formatter:'{{value}}%',fontSize:18}},data:[{{value:{}}}]}}]
}});"#, v)
    }

    fn opt_funnel(&self) -> String {
        let labels = self.funnel_labels.as_ref().unwrap();
        let values = self.funnel_values.as_ref().unwrap();
        let data = serde_json::to_string(&labels.iter().zip(values.iter()).map(|(n,v)| serde_json::json!({"name":n,"value":v})).collect::<Vec<_>>()).unwrap();
        format!(r#"c.setOption({{
 tooltip:{{trigger:'item',formatter:'{{b}}: {{d}}%'}},
 legend:{{show:false}},
 series:[{{type:'funnel',left:'10%',top:10,bottom:10,width:'80%',sort:'descending',gap:2,label:{{show:true,position:'inside',fontSize:7}},labelLine:{{length:5}},itemStyle:{{borderColor:'#fff',borderWidth:1}},data:{}}}]
}});"#, data)
    }

    fn opt_matrix(&self) -> String {
        let m = self.matrix.as_ref().unwrap();
        let mut values = Vec::new();
        for (i,row) in m.iter().enumerate() { for (j,&v) in row.iter().enumerate() { values.push(serde_json::json!([j,i,v])); } }
        let data = serde_json::to_string(&values).unwrap();
        let xl = serde_json::to_string(self.matrix_xlabels.as_ref().unwrap()).unwrap();
        let yl = serde_json::to_string(self.matrix_ylabels.as_ref().unwrap()).unwrap();
        let vmax = m.iter().flat_map(|r| r.iter().copied()).fold(0.0, f64::max);
        format!(r#"c.setOption({{
 tooltip:{{position:'top'}},
 grid:{{height:'60%',top:'10%'}},
 xAxis:{{type:'category',data:{},splitArea:{{show:true}}}},
 yAxis:{{type:'category',data:{},splitArea:{{show:true}}}},
 visualMap:{{min:0,max:{},calculable:true,orient:'horizontal',left:'center',bottom:'5%'}},
 series:[{{type:'heatmap',data:{},label:{{show:false}}}}]
}});"#, xl, yl, vmax, data)
    }

    fn opt_treemap(&self) -> String {
        let data = serde_json::to_string(self.treemap_data.as_ref().unwrap()).unwrap();
        format!(r#"c.setOption({{
 toolbox:{{show:false,feature:{{}}}},
 tooltip:{{formatter:(p)=>p.name?`${{p.name}}: ${{p.value}}`:''}},
 series:[{{
    type:'treemap',
    data:{},
    leafDepth:3,
    label:{{show:true,fontSize:10}},
    upperLabel:{{show:true}},
    levels:[{{itemStyle:{{borderColor:'#fff',borderWidth:1}}}}],
    roam:false,
    breadcrumb:{{show:false}}
}}]
}});"#, data)
    }

    fn opt_tree(&self) -> String {
        let data = serde_json::to_string(self.tree_data.as_ref().unwrap()).unwrap();
        format!(r#"c.setOption({{
 tooltip:{{trigger:'item',triggerOn:'mousemove',formatter:(p)=>p.name||''}},
 series:[{{type:'tree',data:{},symbol:'circle',symbolSize:8,edgeShape:'polyline',left:'15%',right:'15%',top:'5%',bottom:'5%',label:{{position:'left',verticalAlign:'middle',align:'right'}},leaves:{{label:{{position:'right',verticalAlign:'middle',align:'left'}}}},expandAndCollapse:true,animationDuration:300,animationDurationUpdate:300}}]
}});"#, data)
    }

    pub fn at(mut self, x: f64, y: f64) -> Self { self.position = Some((x, y)); self }
    pub fn size(mut self, w: u32, h: u32) -> Self { self.size = Some((w, h)); self }
}

impl crate::layout::grid::Sizable for Chart { fn set_size(&mut self, size: (u32, u32)) { self.size = Some(size); } }

impl Drop for Chart {
    fn drop(&mut self) {
        let id = next_chart_id();
        let div = if let Some((x, y)) = self.position {
            let (left, top) = crate::layout::coord::to_screen_coords(x, y);
            let (w, h) = self.size.unwrap_or((400, 300));
            format!(r#"<div style="position:absolute;left:{}px;top:{}px;width:{}px;height:{}px;transform:translate(-50%,-50%);"><div id="{}" style="width:100%;height:100%"></div></div>"#, left, top, w, h, id)
        } else {
            let (class, h) = match self.size {
                Some((w, h)) if w <= 160 || h <= 160 => ("chart-container chart-small", h),
                Some((w, h)) if w <= 220 || h <= 220 => ("chart-container chart-medium", h),
                Some((_, h)) => ("chart-container", h),
                None => ("chart-container", 400),
            };
            format!(r#"<div class="{}"><div id="{}" style="height:{}px"></div></div>"#, class, id, h)
        };
        crate::io::gui::add_output(format!("{}{}", div, self.script(&id)));
    }
}

fn base(kind: &str) -> Chart {
    Chart {
        kind: kind.into(),
        x: None,
        y: None,
        x_labels: None,
        smooth: false,
        pie_labels: None,
        pie_values: None,
        indicators: None,
        radar_values: None,
        gauge_val: None,
        funnel_labels: None,
        funnel_values: None,
        hole: None,
        matrix: None,
        matrix_xlabels: None,
        matrix_ylabels: None,
        treemap_data: None,
        tree_data: None,
        position: None,
        size: None,
    }
}

pub fn line_xy(x: &[f64], y: &[f64]) -> Chart { let mut c = base("line_xy"); c.x = Some(x.to_vec()); c.y = Some(y.to_vec()); c }
pub fn curve_xy(x: &[f64], y: &[f64]) -> Chart { let mut c = base("curve_xy"); c.x = Some(x.to_vec()); c.y = Some(y.to_vec()); c.smooth = true; c }
pub fn area_xy(x: &[f64], y: &[f64]) -> Chart { let mut c = base("area_xy"); c.x = Some(x.to_vec()); c.y = Some(y.to_vec()); c }

pub fn line(labels: &[&str], values: &[f64]) -> Chart { let mut c = base("line"); c.x_labels = Some(labels.iter().map(|s| (*s).to_string()).collect()); c.y = Some(values.to_vec()); c }
pub fn curve(labels: &[&str], values: &[f64]) -> Chart { let mut c = base("curve"); c.x_labels = Some(labels.iter().map(|s| (*s).to_string()).collect()); c.y = Some(values.to_vec()); c.smooth = true; c }
pub fn area(labels: &[&str], values: &[f64]) -> Chart { let mut c = base("area"); c.x_labels = Some(labels.iter().map(|s| (*s).to_string()).collect()); c.y = Some(values.to_vec()); c }
pub fn bar(labels: &[&str], values: &[f64]) -> Chart { let mut c = base("bar"); c.x_labels = Some(labels.iter().map(|s| (*s).to_string()).collect()); c.y = Some(values.to_vec()); c }

pub fn scatter(x: &[f64], y: &[f64]) -> Chart { let mut c = base("scatter"); c.x = Some(x.to_vec()); c.y = Some(y.to_vec()); c }
pub fn pie(labels: &[&str], values: &[f64]) -> Chart { let mut c = base("pie"); c.pie_labels = Some(labels.iter().map(|s| (*s).to_string()).collect()); c.pie_values = Some(values.to_vec()); c }
pub fn doughnut(labels: &[&str], values: &[f64]) -> Chart { let mut c = base("doughnut"); c.pie_labels = Some(labels.iter().map(|s| (*s).to_string()).collect()); c.pie_values = Some(values.to_vec()); c.hole = Some(50); c }

pub fn radar(values: &[f64], indicators: &[(&str, f64)]) -> Chart {
    let mut c = base("radar");
    c.radar_values = Some(values.to_vec());
    c.indicators = Some(indicators.iter().map(|(n,m)| RadarIndicator { name:(*n).to_string(), max:*m }).collect());
    c
}

pub fn gauge(value: f64) -> Chart { let mut c = base("gauge"); c.gauge_val = Some(value); c }

pub fn funnel(stages: &[(&str, f64)]) -> Chart {
    let mut c = base("funnel");
    c.funnel_labels = Some(stages.iter().map(|(n,_)| (*n).to_string()).collect());
    c.funnel_values = Some(stages.iter().map(|(_,v)| *v).collect());
    c
}

fn path_push(root: &mut serde_json::Value, path: &str, val: Option<f64>) {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let mut cur = root;
    for (i, part) in parts.iter().enumerate() {
        let is_last = i == parts.len() - 1;
        cur = {
            let children = cur.get_mut("children").and_then(|c| c.as_array_mut()).unwrap();
            if is_last {
                children.push(serde_json::json!({"name":part.to_string(),"value":val.unwrap_or(1.0)}));
                return;
            } else {
                let pos = children.iter().position(|n| n.get("name").and_then(|s| s.as_str()) == Some(part));
                let idx = match pos {
                    Some(i) => i,
                    None => { children.push(serde_json::json!({"name":part.to_string(),"children":[] })); children.len()-1 }
                };
                children.get_mut(idx).unwrap()
            }
        };
    }
}

pub fn treemap(items: &[(&str, f64)]) -> Chart {
    let mut root = serde_json::json!([]);
    let has_path = items.iter().any(|(n,_)| n.contains('/'));
    if has_path {
        let mut r = serde_json::json!({"name":"root","children":[]});
        for (name,val) in items { path_push(&mut r, name, Some(*val)); }
        root.as_array_mut().unwrap().push(r);
    } else {
        for (n,v) in items { root.as_array_mut().unwrap().push(serde_json::json!({"name":n.to_string(),"value":v})); }
    }
    let mut c = base("treemap");
    c.treemap_data = Some(root);
    c
}

fn json_to_tree(val: &serde_json::Value) -> Option<serde_json::Value> {
    match val {
        serde_json::Value::Object(map) => {
            let mut result = Vec::new();
            for (key, value) in map {
                match value {
                    serde_json::Value::Null => {
                        result.push(serde_json::json!({"name": key}));
                    }
                    serde_json::Value::Object(_) => {
                        if let Some(children) = json_to_tree(value) {
                            result.push(serde_json::json!({
                                "name": key,
                                "children": children
                            }));
                        }
                    }
                    _ => {
                        result.push(serde_json::json!({"name": key}));
                    }
                }
            }
            Some(serde_json::Value::Array(result))
        }
        _ => None
    }
}

pub fn tree<T: TreeInput>(input: T) -> Chart { input.to_tree_chart() }

pub trait TreeInput { fn to_tree_chart(self) -> Chart; }

impl TreeInput for &serde_json::Value {
    fn to_tree_chart(self) -> Chart {
        let tree_data = if let Some(children) = json_to_tree(self) {
            children
        } else {
            serde_json::json!([])
        };
        let mut c = base("tree");
        c.tree_data = Some(tree_data);
        c
    }
}

impl TreeInput for &str {
    fn to_tree_chart(self) -> Chart {
        match serde_json::from_str::<serde_json::Value>(self) {
            Ok(val) => (&val).to_tree_chart(),
            Err(_) => {
                let paths: Vec<&str> = self.lines().map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
                paths.as_slice().to_tree_chart()
            }
        }
    }
}

impl TreeInput for &[&str] {
    fn to_tree_chart(self) -> Chart {
        let mut temp_root = serde_json::json!({"name":"root","children":[]});
        for p in self { path_push(&mut temp_root, p, None); }
        let children = temp_root.get("children").and_then(|c| c.as_array()).unwrap_or(&vec![]).clone();
        let mut c = base("tree");
        c.tree_data = Some(serde_json::Value::Array(children));
        c
    }
}

pub fn matrix(data: &[Vec<f64>], xlabels: &[&str], ylabels: &[&str]) -> Chart {
    let mut c = base("matrix");
    c.matrix = Some(data.to_vec());
    c.matrix_xlabels = Some(xlabels.iter().map(|s| (*s).to_string()).collect());
    c.matrix_ylabels = Some(ylabels.iter().map(|s| (*s).to_string()).collect());
    c
}

pub fn function<F: Fn(f64)->f64>(f: F, x0: f64, x1: f64, step: f64) -> Chart {
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut t = x0;
    while t <= x1 {
        x.push(t);
        y.push(f(t));
        t += step;
    }
    curve_xy(&x, &y)
}