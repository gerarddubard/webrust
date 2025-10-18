// webrust/src/io/gui.rs
//! # Web-based Graphical User Interface
//!
//! Automatic web server that creates browser-based interfaces for Rust applications
//! with integrated visualization and real-time communication.
//!
//! ## Server Lifecycle
//!
//! - Starts on `127.0.0.1:8080` automatically
//! - Opens default browser on launch
//! - Auto-shutdown after 3 seconds of inactivity
//! - Maximum 30-second safety timeout
//!
//! ## Examples
//!
//!
//! use webrust::prelude::*;
//!
//! // Basic usage with default theme
//! #[gui]
//! fn main() {
//!     println("Hello, WebRust!");
//! }
//!
//! // Custom theme
//! #[gui(bg = "navy", fg = "white", color = "cyan")]
//! fn main() {
//!     println("Styled interface!");
//! }
//!
//!
//! The `#[gui]` macro handles all server setup, HTML/CSS/JS injection,
//! and bidirectional communication between Rust and the browser.

use std::{collections::HashMap, str::FromStr, sync::{mpsc, Arc, Mutex}, thread, time::{Duration, Instant}};
use tiny_http::{Server, Response, Header, Method, Request};
use serde_json::{json, Value};

#[derive(Clone)]
pub struct StyleConfig { pub bg:String, pub fg:String, pub font:String, pub color:String, pub size:String }
impl Default for StyleConfig {
    fn default()->Self{ Self{ bg:"white".into(), fg:"lightgray".into(), font:"Arial, sans-serif".into(), color:"black".into(), size:"14px".into() } }
}

pub struct GuiState{
    pub pending_inputs:HashMap<String,(mpsc::Sender<String>,String)>,
    pub output_buffer:Vec<String>, pub input_counter:usize, pub program_finished:bool,
    pub last_request_time:Option<Instant>, pub has_received_requests:bool, pub style_config:StyleConfig,
}
impl Default for GuiState{
    fn default()->Self{ Self{
        pending_inputs:HashMap::new(), output_buffer:Vec::new(), input_counter:0, program_finished:false,
        last_request_time:None, has_received_requests:false, style_config:StyleConfig::default()
    }}
}

lazy_static::lazy_static!{ static ref GUI_STATE:Arc<Mutex<GuiState>>=Arc::new(Mutex::new(GuiState::default())); }

fn validate_input_rust(value:&str, expected:&str)->Result<(),String>{
    match expected {
        "i32"=>value.trim().parse::<i32>().map(|_|()).map_err(|e|e.to_string()),
        "f64"=>value.trim().parse::<f64>().map(|_|()).map_err(|e|e.to_string()),
        "bool"=>value.trim().parse::<bool>().map(|_|()).map_err(|e|e.to_string()),
        "char"=>value.trim().parse::<char>().map(|_|()).map_err(|e|e.to_string()),
        _=>Ok(())
    }
}

fn open_browser(){
    thread::spawn(||{
        thread::sleep(Duration::from_millis(500));
        #[cfg(target_os="windows")] { let _=std::process::Command::new("cmd").args(&["/c","start","http://127.0.0.1:8080"]).spawn(); }
        #[cfg(target_os="macos")]  { let _=std::process::Command::new("open").arg("http://127.0.0.1:8080").spawn(); }
        #[cfg(target_os="linux")]  { let _=std::process::Command::new("xdg-open").arg("http://127.0.0.1:8080").spawn(); }
    });
}

pub fn start_gui_server<F>(user:F) where F:FnOnce()+Send+'static { start_gui_server_with_style(StyleConfig::default(), user); }

pub fn start_gui_server_with_style<F>(style:StyleConfig, user:F) where F:FnOnce()+Send+'static {
    { GUI_STATE.lock().unwrap().style_config=style; }
    let server=Server::http("127.0.0.1:8080").expect("Failed to start server");
    open_browser();
    thread::spawn(move ||{
        user();
        { GUI_STATE.lock().unwrap().program_finished=true; }
        let start=Instant::now(); let max=Duration::from_secs(30); let min=Duration::from_secs(3);
        loop{
            thread::sleep(Duration::from_millis(500));
            let elapsed=start.elapsed();
            let exit={
                let st=GUI_STATE.lock().unwrap();
                if !st.has_received_requests && elapsed>Duration::from_secs(5){ true }
                else if let Some(last)=st.last_request_time{ elapsed>min && last.elapsed()>Duration::from_secs(1) }
                else { elapsed>max }
            };
            if exit{ break; }
        }
        std::process::exit(0);
    });
    for req in server.incoming_requests(){
        { let mut st=GUI_STATE.lock().unwrap(); st.last_request_time=Some(Instant::now()); st.has_received_requests=true; }
        handle_request(req);
    }
}

fn respond(req:Request, body:impl Into<String>, ct:&str){
    let h=Header::from_bytes(&b"Content-Type"[..], ct.as_bytes()).unwrap();
    let _=req.respond(Response::from_string(body.into()).with_header(h));
}

fn handle_request(request:Request){
    let url=request.url().to_string();
    match request.method(){
        &Method::Get=>{
            match url.as_str(){
                "/"=>serve_file(request,"index.html","text/html"),
                "/style.css"=>serve_dynamic_css(request),
                "/script.js"=>serve_file(request,"script.js","application/javascript"),
                _ if url.starts_with("/api/")=>handle_api_get(request,&url),
                _=>serve_404(request)
            }
        }
        &Method::Post=>{
            if url.starts_with("/api/"){ handle_api_post(request,&url); } else { serve_404(request); }
        }
        _=>serve_404(request)
    }
}

fn serve_file(request:Request, filename:&str, ct:&str){
    let content=match filename{
        "index.html"=>include_str!("../../static/index.html"),
        "script.js"=>include_str!("../../static/script.js"),
        _=>"Not Found"
    };
    respond(request, content, ct);
}

fn serve_dynamic_css(request:Request){
    let sc={ GUI_STATE.lock().unwrap().style_config.clone() };
    let border=if sc.bg==sc.fg { sc.bg.as_str() } else { "#ccc" };
    let css=format!(r#"body{{font-family:{};font-size:{};background:{};color:{};margin:0;padding:20px;min-height:100vh}}
#terminal{{position:relative;background:{};border:2px solid {};border-radius:12px;padding:20px;height:90vh;overflow-y:auto;line-height:1.6}}
.table-container{{margin:20px 0;overflow-x:auto}}
table.webrust-table{{border-collapse:separate;border-spacing:0;background:#fff;font-size:{};margin:12px 0}}
table.webrust-table th,table.webrust-table td{{padding:10px 14px;border:1px solid purple;white-space:nowrap;vertical-align:middle;border-radius:4px}}
.webrust-th-header{{background:#cce5ff;color:#003366;font-weight:bold;text-align:center}}
.webrust-td-value{{color:#1e3a8a;font-style:italic}}
.webrust-td-number{{color:#1e3a8a;font-style:italic;text-align:right}}
.no-border{{border:none!important;background:transparent!important;padding:0!important}}
.input-container{{margin:10px 0}}
.input-line{{display:flex;align-items:center;gap:10px}}
.input-prompt{{font-weight:bold;color:#333}}
.user-input{{padding:8px 12px;border:2px solid #ddd;border-radius:6px;font-size:{};min-width:200px}}
.user-input:focus{{outline:none;border-color:#007bff;box-shadow:0 0 0 3px rgba(0,123,255,0.25)}}
.completed-input{{color:#28a745;font-weight:bold}}
.error-message{{color:#dc3545;font-size:12px;margin-top:5px;padding:4px 8px;background:#f8d7da;border:1px solid #f5c6cb;border-radius:4px}}
.terminal-line{{margin:2px 0}}
.chart{{background:white;border-radius:8px;padding:20px;margin:20px 0;box-shadow:0 2px 8px rgba(0,0,0,0.2)}}
.header{{color:#A9A9A9;font-weight:bold;font-size:18px;margin-bottom:20px}}"#,
                    sc.font, sc.size, sc.bg, sc.color, sc.fg, border, sc.size, sc.size
    );
    respond(request, css, "text/css");
}

fn handle_api_get(request:Request, url:&str){
    if url=="/api/state"{
        let st=GUI_STATE.lock().unwrap();
        respond(request, json!({
            "output": st.output_buffer,
            "pending_inputs": st.pending_inputs.keys().collect::<Vec<_>>(),
            "program_finished": st.program_finished
        }).to_string(), "application/json");
    } else { serve_404(request); }
}

fn handle_api_post(mut request:Request, url:&str){
    if url=="/api/input"{
        let mut body=String::new(); let _=request.as_reader().read_to_string(&mut body);
        if let Ok(data)=serde_json::from_str::<Value>(&body){
            if let (Some(id),Some(value))=(data["id"].as_str(),data["value"].as_str()){
                let mut st=GUI_STATE.lock().unwrap();
                if let Some((tx,_))=st.pending_inputs.remove(id){
                    st.output_buffer.push(value.to_string()); let _=tx.send(value.to_string());
                }
            }
        }
        respond(request,"OK","text/plain");
    } else if url=="/api/validate"{
        let mut body=String::new(); let _=request.as_reader().read_to_string(&mut body);
        if let Ok(data)=serde_json::from_str::<Value>(&body){
            if let (Some(id),Some(value))=(data["id"].as_str(),data["value"].as_str()){
                let st=GUI_STATE.lock().unwrap();
                if let Some((_,expected))=st.pending_inputs.get(id){
                    match validate_input_rust(value,expected){
                        Ok(_)=>{ respond(request, json!({"valid":true}).to_string(), "application/json"); return; }
                        Err(e)=>{ respond(request, json!({"valid":false,"error":e}).to_string(), "application/json"); return; }
                    }
                }
            }
        }
        respond(request, json!({"valid":false,"error":"Invalid request"}).to_string(), "application/json");
    } else { serve_404(request); }
}

fn serve_404(request:Request){ let _=request.respond(Response::from_string("404 Not Found").with_status_code(404)); }

pub fn add_output_same_line(text:String){
    let mut st=GUI_STATE.lock().unwrap(); let mut gap:Option<u32>=None;
    if let Some(start)=text.find("viz-line-gap=\""){
        let s=&text[start+15..]; if let Some(end)=s.find('"'){ if let Ok(v)=s[..end].parse::<u32>(){ gap=Some(v); } }
    }
    if let Some(last)=st.output_buffer.last_mut(){
        if last.contains("class=\"webrust-line\""){
            if let Some(pos)=last.rfind("</div>"){
                last.insert_str(pos,&text);
                if let Some(g)=gap{
                    if let Some(mpos)=last.rfind("margin:"){
                        if let Some(end)=last[mpos..].find(';'){ let a=mpos; let b=mpos+end+1; last.replace_range(a..b,&format!("margin:{}px 0;",g)); }
                    } else if let Some(sty)=last.rfind("style=\""){ let ins=sty+7; last.insert_str(ins,&format!("margin:{}px 0;",g)); }
                }
            } else { last.push_str(&text); }
        } else {
            let g=gap.unwrap_or(6);
            *last=format!(r#"<div class="webrust-line" style="display:block;margin:{g}px 0;">{}</div>"#,text);
        }
    } else {
        let g=gap.unwrap_or(6);
        st.output_buffer.push(format!(r#"<div class="webrust-line" style="display:block;margin:{g}px 0;">{}</div>"#,text));
    }
}

pub fn add_output_new_line(text:String){ GUI_STATE.lock().unwrap().output_buffer.push(text); }
pub fn add_output(text:String){ add_output_new_line(text); }

pub fn create_input_request_typed<T:FromStr>(prompt:&str)->String{
    let (tx,rx)=mpsc::channel(); let ty=std::any::type_name::<T>().rsplit("::").next().unwrap_or("String");
    let _id={ let mut st=GUI_STATE.lock().unwrap(); st.input_counter+=1; let id=format!("input_{}",st.input_counter);
        st.pending_inputs.insert(id.clone(),(tx,ty.to_string())); st.output_buffer.push(format!("INPUT_REQUEST:{}:{}",id,prompt)); id };
    rx.recv().unwrap_or_default()
}
pub fn create_input_request(prompt:&str)->String{ create_input_request_typed::<String>(prompt) }
