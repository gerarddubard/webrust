// webrust/src/io/gui.rs
//! # Web-based Graphical User Interface
//!
//! Provides an automatic web server that creates a modern browser-based
//! interface for Rust applications. No manual server management required.
//!
//! ## Features
//!
//! - **Automatic browser opening** - Launches default browser on startup
//! - **Smart server lifecycle** - Shuts down when user closes browser
//! - **Real-time communication** - Bidirectional data flow between Rust and web
//! - **Theme customization** - Control colors, fonts, and styling
//! - **Cross-platform** - Works on Windows, macOS, and Linux
//!
//! ## Server Management
//!
//! The server intelligently manages its lifecycle:
//! - Starts on `127.0.0.1:8080`
//! - Shuts down 3 seconds after browser closes
//! - Maximum 30-second runtime for safety
//! - No manual Ctrl+C required
//!
//! ## Theme Configuration
//!
//!
//! #[gui(bg = "navy", fg = "white", font = "Arial", color = "cyan", size = "14px")]
//! fn main() {
//!     println("Styled application with custom theme!");
//! }
//!
//!
//! ## Architecture
//!
//! Uses a lightweight HTTP server with WebSocket-like communication
//! for real-time updates. All styling is handled via dynamic CSS
//! generation based on the theme configuration.
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::collections::HashMap;
use tiny_http::{Server, Response, Header};
use serde_json::{json, Value};
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct StyleConfig {
    pub bg: String,
    pub fg: String,
    pub font: String,
    pub color: String,
    pub size: String,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            bg: "white".to_string(),
            fg: "lightgray".to_string(),
            font: "Arial, sans-serif".to_string(),
            color: "black".to_string(),
            size: "14px".to_string(),
        }
    }
}

pub struct GuiState {
    pub pending_inputs: HashMap<String, (mpsc::Sender<String>, String)>,
    pub output_buffer: Vec<String>,
    pub input_counter: usize,
    pub program_finished: bool,
    pub last_request_time: Option<Instant>,
    pub has_received_requests: bool,
    pub style_config: StyleConfig,
}

impl GuiState {
    fn new() -> Self {
        Self {
            pending_inputs: HashMap::new(),
            output_buffer: Vec::new(),
            input_counter: 0,
            program_finished: false,
            last_request_time: None,
            has_received_requests: false,
            style_config: StyleConfig::default(),
        }
    }
}

lazy_static::lazy_static! { 
    static ref GUI_STATE: Arc<Mutex<GuiState>> = Arc::new(Mutex::new(GuiState::new())); 
}

fn validate_input_rust(value: &str, expected_type: &str) -> Result<(), String> {
    match expected_type {
        "i32" => value.trim().parse::<i32>().map(|_| ()).map_err(|e| e.to_string()),
        "f64" => value.trim().parse::<f64>().map(|_| ()).map_err(|e| e.to_string()),
        "bool" => value.trim().parse::<bool>().map(|_| ()).map_err(|e| e.to_string()),
        "char" => value.trim().parse::<char>().map(|_| ()).map_err(|e| e.to_string()),
        _ => Ok(())
    }
}

fn open_browser() {
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(500));
        #[cfg(target_os = "windows")]
        { let _ = std::process::Command::new("cmd").args(&["/c", "start", "http://127.0.0.1:8080"]).spawn(); }
        #[cfg(target_os = "macos")]
        { let _ = std::process::Command::new("open").arg("http://127.0.0.1:8080").spawn(); }
        #[cfg(target_os = "linux")]
        { let _ = std::process::Command::new("xdg-open").arg("http://127.0.0.1:8080").spawn(); }
    });
}

pub fn start_gui_server<F>(user_function: F) where F: FnOnce() + Send + 'static, {
    start_gui_server_with_style(StyleConfig::default(), user_function);
}

pub fn start_gui_server_with_style<F>(style_config: StyleConfig, user_function: F) where F: FnOnce() + Send + 'static, {
    {
        let mut state = GUI_STATE.lock().unwrap();
        state.style_config = style_config;
    }
    let server = Server::http("127.0.0.1:8080").expect("Failed to start server");
    open_browser();
    thread::spawn(move || {
        user_function();
        {
            let mut state = GUI_STATE.lock().unwrap();
            state.program_finished = true;
        }
        let start_time = Instant::now();
        let max_wait_time = Duration::from_secs(30);
        let min_wait_time = Duration::from_secs(3);
        loop {
            thread::sleep(Duration::from_millis(500));
            let elapsed = start_time.elapsed();
            let should_exit = {
                let state = GUI_STATE.lock().unwrap();
                if !state.has_received_requests && elapsed > Duration::from_secs(5) {
                    true
                } else if let Some(last_req) = state.last_request_time {
                    elapsed > min_wait_time && last_req.elapsed() > Duration::from_secs(1)
                } else {
                    elapsed > max_wait_time
                }
            };
            if should_exit { break; }
        }
        std::process::exit(0);
    });
    for request in server.incoming_requests() {
        {
            let mut state = GUI_STATE.lock().unwrap();
            state.last_request_time = Some(Instant::now());
            state.has_received_requests = true;
        }
        handle_request(request);
    }
}

fn handle_request(request: tiny_http::Request) {
    let url = request.url().to_string();
    let method = request.method().clone();
    match method {
        tiny_http::Method::Get => {
            if url == "/" { serve_file(request, "index.html", "text/html"); }
            else if url == "/style.css" { serve_dynamic_css(request); }
            else if url == "/script.js" { serve_file(request, "script.js", "application/javascript"); }
            else if url.starts_with("/api/") { handle_api_get(request, &url); }
            else { serve_404(request); }
        }
        tiny_http::Method::Post => {
            if url.starts_with("/api/") { handle_api_post(request, &url); }
            else { serve_404(request); }
        }
        _ => serve_404(request),
    }
}

fn serve_file(request: tiny_http::Request, filename: &str, content_type: &str) {
    let content = match filename {
        "index.html" => include_str!("../../static/index.html"),
        "script.js" => include_str!("../../static/script.js"),
        _ => "Not Found"
    };
    let response = Response::from_string(content).with_header(Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap());
    let _ = request.respond(response);
}

fn serve_dynamic_css(request: tiny_http::Request) {
    let style_config = { let state = GUI_STATE.lock().unwrap(); state.style_config.clone() };
    let border_color = if style_config.bg == style_config.fg { &style_config.bg } else { "#ccc" };
    let css_content = format!(r#"body{{font-family:{};font-size:{};background:{};color:{};margin:0;padding:20px;min-height:100vh}}#terminal{{background:{};border:2px solid {};border-radius:12px;padding:20px;height:90vh;overflow-y:auto;line-height:1.6}}.table-container{{margin:20px 0;overflow-x:auto}}table.webrust-table{{border-collapse:separate;border-spacing:0;background:#fff;font-size:{};margin:12px 0}}table.webrust-table th,table.webrust-table td{{padding:10px 14px;border:1px solid purple;white-space:nowrap;vertical-align:middle;border-radius:4px}}.webrust-th-header{{background:#cce5ff;color:#003366;font-weight:bold;text-align:center}}.webrust-td-value{{color:#1e3a8a;font-style:italic}}.webrust-td-number{{color:#1e3a8a;font-style:italic;text-align:right}}.no-border{{border:none!important;background:transparent!important;padding:0!important}}.input-container{{margin:10px 0}}.input-line{{display:flex;align-items:center;gap:10px}}.input-prompt{{font-weight:bold;color:#333}}.user-input{{padding:8px 12px;border:2px solid #ddd;border-radius:6px;font-size:{};min-width:200px}}.user-input:focus{{outline:none;border-color:#007bff;box-shadow:0 0 0 3px rgba(0,123,255,0.25)}}.completed-input{{color:#28a745;font-weight:bold}}.error-message{{color:#dc3545;font-size:12px;margin-top:5px;padding:4px 8px;background:#f8d7da;border:1px solid #f5c6cb;border-radius:4px}}.terminal-line{{margin:2px 0}}"#, style_config.font, style_config.size, style_config.bg, style_config.color, style_config.fg, border_color, style_config.size, style_config.size);
    let response = Response::from_string(css_content).with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/css"[..]).unwrap());
    let _ = request.respond(response);
}

fn handle_api_get(request: tiny_http::Request, url: &str) {
    if url == "/api/state" {
        let state = GUI_STATE.lock().unwrap();
        let json_response = json!({"output": state.output_buffer, "pending_inputs": state.pending_inputs.keys().collect::<Vec<_>>(), "program_finished": state.program_finished});
        let response = Response::from_string(json_response.to_string()).with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
        let _ = request.respond(response);
    } else { serve_404(request); }
}

fn handle_api_post(mut request: tiny_http::Request, url: &str) {
    if url == "/api/input" {
        let mut body = String::new();
        let _ = request.as_reader().read_to_string(&mut body);
        if let Ok(data) = serde_json::from_str::<Value>(&body) {
            if let (Some(id), Some(value)) = (data["id"].as_str(), data["value"].as_str()) {
                let mut state = GUI_STATE.lock().unwrap();
                if let Some((sender, _)) = state.pending_inputs.remove(id) {
                    state.output_buffer.push(value.to_string());
                    let _ = sender.send(value.to_string());
                }
            }
        }
        let response = Response::from_string("OK");
        let _ = request.respond(response);
    } else if url == "/api/validate" {
        let mut body = String::new();
        let _ = request.as_reader().read_to_string(&mut body);
        if let Ok(data) = serde_json::from_str::<Value>(&body) {
            if let (Some(id), Some(value)) = (data["id"].as_str(), data["value"].as_str()) {
                let state = GUI_STATE.lock().unwrap();
                if let Some((_, expected_type)) = state.pending_inputs.get(id) {
                    match validate_input_rust(value, expected_type) {
                        Ok(_) => {
                            let response = Response::from_string(json!({"valid": true}).to_string()).with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                            let _ = request.respond(response);
                            return;
                        },
                        Err(error) => {
                            let response = Response::from_string(json!({"valid": false, "error": error}).to_string()).with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                            let _ = request.respond(response);
                            return;
                        }
                    }
                }
            }
        }
        let response = Response::from_string(json!({"valid": false, "error": "Invalid request"}).to_string()).with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
        let _ = request.respond(response);
    } else { serve_404(request); }
}

fn serve_404(request: tiny_http::Request) {
    let response = Response::from_string("404 Not Found").with_status_code(404);
    let _ = request.respond(response);
}

pub fn add_output_same_line(text: String) {
    let mut state = GUI_STATE.lock().unwrap();
    let mut desired_gap: Option<u32> = None;
    if let Some(start) = text.find("data-line-gap=\"") {
        let s = &text[start + 15..];
        if let Some(end) = s.find('"') {
            if let Ok(v) = s[..end].parse::<u32>() { desired_gap = Some(v); }
        }
    }
    if let Some(last) = state.output_buffer.last_mut() {
        if last.contains("class=\"webrust-line\"") {
            if let Some(pos) = last.rfind("</div>") {
                last.insert_str(pos, &text);
                if let Some(gap) = desired_gap {
                    if let Some(mpos) = last.rfind("margin:") {
                        if let Some(end) = last[mpos..].find(';') {
                            let a = mpos;
                            let b = mpos + end + 1;
                            last.replace_range(a..b, &format!("margin:{}px 0;", gap));
                        }
                    } else if let Some(st) = last.rfind("style=\"") {
                        let ins = st + 7;
                        last.insert_str(ins, &format!("margin:{}px 0;", gap));
                    }
                }
            } else { last.push_str(&text); }
        } else {
            let gap = desired_gap.unwrap_or(6);
            let html = format!(r#"<div class="webrust-line" style="display:block;margin:{gap}px 0;">{}</div>"#, text);
            *last = html;
        }
    } else {
        let gap = desired_gap.unwrap_or(6);
        let html = format!(r#"<div class="webrust-line" style="display:block;margin:{gap}px 0;">{}</div>"#, text);
        state.output_buffer.push(html);
    }
}

pub fn add_output_new_line(text: String) {
    let mut state = GUI_STATE.lock().unwrap();
    state.output_buffer.push(text);
}

pub fn add_output(text: String) { add_output_new_line(text); }

pub fn create_input_request_typed<T>(prompt: &str) -> String where T: FromStr, {
    let (tx, rx) = mpsc::channel();
    let type_name = std::any::type_name::<T>().split("::").last().unwrap_or("String");
    let _input_id = {
        let mut state = GUI_STATE.lock().unwrap();
        state.input_counter += 1;
        let id = format!("input_{}", state.input_counter);
        state.pending_inputs.insert(id.clone(), (tx, type_name.to_string()));
        state.output_buffer.push(format!("INPUT_REQUEST:{}:{}", id, prompt));
        id
    };
    rx.recv().unwrap_or_default()
}

pub fn create_input_request(prompt: &str) -> String { create_input_request_typed::<String>(prompt) }