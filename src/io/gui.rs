// webrust/src/io/gui.rs
//! # WebRust GUI — Embedded HTTP server + browser renderer
//!
//! `io::gui` starts a tiny HTTP server (on `127.0.0.1:8080`) that renders your
//! program’s output in the browser. It exposes a minimal REST API for polling
//! output and handling typed input requests, plus a simple theming system.
//!
//! ## What it does
//! - Serves the core HTML/CSS/JS assets (`index.html`, `style.css`, `main.js`, etc.).
//! - Opens your default browser automatically.
//! - Streams your program output (HTML fragments) to the page.
//! - Issues input prompts and validates responses (with Rust-like types).
//! - Shuts down automatically once the program is finished and the client has been idle.
//!
//! ## Public types
//! - **`StyleConfig`**: UI theme (background, text color, font family, base font size).
//! - **`GuiState`**: Shared state (outputs, pending inputs, style, lifecycle flags).
//!
//! ## Key functions
//! - **`start_gui_server(f)`**: Launch server with default style, run `f()` on a worker thread.
//! - **`start_gui_server_with_style(style, f)`**: Same, but with a custom `StyleConfig`.
//! - **`add_output_new_line(html)`**: Push an HTML block on a new line.
//! - **`add_output_same_line(html)`**: Append an HTML fragment to the current line (preserves vertical rhythm).
//! - **`add_output(html)`**: Alias of `add_output_new_line`.
//! - **`create_input_request_typed::<T>(prompt)`**: Ask the user for a value of type `T: FromStr` and block until provided.
//! - **`create_input_request(prompt)`**: Ask for a `String`.
//!
//! ## HTTP routes
//! - `GET /` → `index.html`
//! - `GET /style.css` → base styles
//! - `GET /table.js`, `GET /turtle.js`, `GET /main.js` → client scripts
//! - `GET /api/state` → JSON: `{ output, pending_inputs, program_finished }`
//! - `POST /api/input` → `{ id, value }` to submit an answer
//! - `POST /api/validate` → `{ id, value }` to validate without submitting
//!
//! ## Input validation
//! `POST /api/validate` checks the expected Rust-ish type (`"i32"`, `"f64"`, `"bool"`, `"char"`, or free-form).
//! Type expectations are carried with the pending input entry and enforced before accept.
//!
//! ## Lifecycle and shutdown
//! - The user function runs on a background thread.
//! - The server remains alive while the page is active or outputs are flowing.
//! - It exits gracefully once the program is done and the client has been idle long enough.
//!
//! ## Theming
//! - Use `#[gui(Font, Size, color, !bg)]` (from the prelude) to configure in one line,
//!
//! ## Example
//! ```rust,no_run
//! use webrust::prelude::*;
//!
//! #[gui(Arial, 14px, black, !white)]
//! fn main() {
//!     println("<b>Hello WebRust</b>");
//!     let name = input("Your name?");
//!     println(format!("Welcome, <green>{name}</green>!"));
//! }
//! ```
//!
//! ## Notes
//! - All output is HTML you control; escape user data as appropriate on the producer side.
//! - Networking is local-only (`127.0.0.1:8080`). No TLS; intended for development and demos.
//!

use serde_json::{json, Value};
use std::{
    collections::HashMap,
    str::FromStr,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use tiny_http::{Header, Method, Request, Response, Server};

#[derive(Clone)]
pub struct StyleConfig {
    pub bg: String,
    pub color: String,
    pub font: String,
    pub size: String,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            bg: "white".into(),
            color: "black".into(),
            font: "Arial".into(),
            size: "14px".into(),
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

impl Default for GuiState {
    fn default() -> Self {
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

lazy_static::lazy_static! {pub static ref GUI_STATE: Arc<Mutex<GuiState>> = Arc::new(Mutex::new(GuiState::default()));}

fn validate_input_rust(value: &str, expected: &str) -> Result<(), String> {
    match expected {
        "i32" => value
            .trim()
            .parse::<i32>()
            .map(|_| ())
            .map_err(|e| e.to_string()),
        "f64" => value
            .trim()
            .parse::<f64>()
            .map(|_| ())
            .map_err(|e| e.to_string()),
        "bool" => value
            .trim()
            .parse::<bool>()
            .map(|_| ())
            .map_err(|e| e.to_string()),
        "char" => value
            .trim()
            .parse::<char>()
            .map(|_| ())
            .map_err(|e| e.to_string()),
        _ => Ok(()),
    }
}

fn open_browser() {
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(500));
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("cmd")
                .args(&["/c", "start", "http://127.0.0.1:8080"])
                .spawn();
        }
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("open")
                .arg("http://127.0.0.1:8080")
                .spawn();
        }
        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("xdg-open")
                .arg("http://127.0.0.1:8080")
                .spawn();
        }
    });
}

pub fn start_gui_server<F>(user: F)
where
    F: FnOnce() + Send + 'static,
{
    start_gui_server_with_style(StyleConfig::default(), user);
}

pub fn start_gui_server_with_style<F>(style: StyleConfig, user: F)
where
    F: FnOnce() + Send + 'static,
{
    {
        GUI_STATE.lock().unwrap().style_config = style;
    }
    let server = Server::http("127.0.0.1:8080").expect("Failed to start server");
    open_browser();
    thread::spawn(move || {
        user();
        {
            GUI_STATE.lock().unwrap().program_finished = true;
        }
        let start = Instant::now();
        let max = Duration::from_secs(30);
        let min = Duration::from_secs(3);
        loop {
            thread::sleep(Duration::from_millis(500));
            let elapsed = start.elapsed();
            let exit = {
                let st = GUI_STATE.lock().unwrap();
                if !st.has_received_requests && elapsed > Duration::from_secs(5) {
                    true
                } else if let Some(last) = st.last_request_time {
                    elapsed > min && last.elapsed() > Duration::from_secs(1)
                } else {
                    elapsed > max
                }
            };
            if exit { break; }
        }
        std::process::exit(0);
    });
    for req in server.incoming_requests() {
        {
            let mut st = GUI_STATE.lock().unwrap();
            st.last_request_time = Some(Instant::now());
            st.has_received_requests = true;
        }
        handle_request(req);
    }
}

fn respond(req: Request, body: impl Into<String>, ct: &str) {
    let h = Header::from_bytes(&b"Content-Type"[..], ct.as_bytes()).unwrap();
    let _ = req.respond(Response::from_string(body.into()).with_header(h));
}

fn handle_request(request: Request) {
    let url = request.url().to_string();
    match request.method() {
        &Method::Get => match url.as_str() {
            "/" => serve_file(request, "index.html", "text/html"),
            "/style.css" => serve_file(request, "style.css", "text/css"),
            "/table.js" => serve_file(request, "table.js", "application/javascript"),
            "/turtle.js" => serve_file(request, "turtle.js", "application/javascript"),
            "/main.js" => serve_file(request, "main.js", "application/javascript"),
            _ if url.starts_with("/api/") => handle_api_get(request, &url),
            _ => serve_404(request),
        },
        &Method::Post => {
            if url.starts_with("/api/") { handle_api_post(request, &url); }
            else { serve_404(request); }
        }
        _ => serve_404(request),
    }
}

fn serve_file(request: Request, filename: &str, ct: &str) {
    let content = match filename {
        "index.html" => include_str!("../../static/index.html"),
        "style.css"  => include_str!("../../static/style.css"),
        "table.js"  => include_str!("../../static/table.js"),
        "turtle.js" => include_str!("../../static/turtle.js"),
        "main.js"   => include_str!("../../static/main.js"),
        _ => "Not Found",
    };
    respond(request, content, ct);
}

fn handle_api_get(request: Request, url: &str) {
    if url == "/api/state" {
        let st = GUI_STATE.lock().unwrap();
        respond(
            request,
            json!({
                "output": st.output_buffer,
                "pending_inputs": st.pending_inputs.keys().collect::<Vec<_>>(),
                "program_finished": st.program_finished
            })
                .to_string(),
            "application/json",
        );
    } else {
        serve_404(request);
    }
}

fn handle_api_post(mut request: Request, url: &str) {
    if url == "/api/input" {
        let mut body = String::new();
        let _ = request.as_reader().read_to_string(&mut body);
        if let Ok(data) = serde_json::from_str::<Value>(&body) {
            if let (Some(id), Some(value)) = (data["id"].as_str(), data["value"].as_str()) {
                let mut st = GUI_STATE.lock().unwrap();
                if let Some((tx, _)) = st.pending_inputs.remove(id) { let _ = tx.send(value.to_string()); }
            }
        }
        respond(request, "OK", "text/plain");
    } else if url == "/api/validate" {
        let mut body = String::new();
        let _ = request.as_reader().read_to_string(&mut body);
        if let Ok(data) = serde_json::from_str::<Value>(&body) {
            if let (Some(id), Some(value)) = (data["id"].as_str(), data["value"].as_str()) {
                let st = GUI_STATE.lock().unwrap();
                if let Some((_, expected)) = st.pending_inputs.get(id) {
                    match validate_input_rust(value, expected) {
                        Ok(_) => {
                            respond(request, json!({"valid":true}).to_string(), "application/json", );
                            return;
                        }
                        Err(e) => {
                            respond(request, json!({"valid":false,"error":e}).to_string(), "application/json", );
                            return;
                        }
                    }
                }
            }
        }
        respond(request, json!({"valid":false,"error":"Invalid request"}).to_string(), "application/json", );
    } else {
        serve_404(request);
    }
}

fn serve_404(request: Request) { let _ = request.respond(Response::from_string("404 Not Found").with_status_code(404)); }

pub fn add_output_same_line(text: String) {
    let mut st = GUI_STATE.lock().unwrap();
    let mut gap: Option<u32> = None;
    if let Some(start) = text.find("viz-line-gap=\"") {
        let s = &text[start + 15..];
        if let Some(end) = s.find('"') {
            if let Ok(v) = s[..end].parse::<u32>() { gap = Some(v); }
        }
    }
    if let Some(last) = st.output_buffer.last_mut() {
        if last.contains("class=\"webrust-line\"") {
            if let Some(pos) = last.rfind("</div>") {
                last.insert_str(pos, &text);
                if let Some(g) = gap {
                    if let Some(mpos) = last.rfind("margin:") {
                        if let Some(end) = last[mpos..].find(';') {
                            let a = mpos;
                            let b = mpos + end + 1;
                            last.replace_range(a..b, &format!("margin:{}px 0;", g));
                        }
                    } else if let Some(sty) = last.rfind("style=\"") {
                        let ins = sty + 7;
                        last.insert_str(ins, &format!("margin:{}px 0;", g));
                    }
                }
            } else {
                last.push_str(&text);
            }
        } else {
            let g = gap.unwrap_or(6);
            *last = format!(r#"<div class="webrust-line" style="display:block;margin:{g}px 0;">{}</div>"#, text);
        }
    } else {
        let g = gap.unwrap_or(6);
        st.output_buffer.push(format!(r#"<div class="webrust-line" style="display:block;margin:{g}px 0;">{}</div>"#, text));
    }
}

pub fn add_output_new_line(text: String) { GUI_STATE.lock().unwrap().output_buffer.push(text); }

pub fn add_output(text: String) { add_output_new_line(text); }

pub fn create_input_request_typed<T: FromStr>(prompt: &str) -> String {
    let (tx, rx) = mpsc::channel();
    let ty = std::any::type_name::<T>()
        .rsplit("::")
        .next()
        .unwrap_or("String");
    let _id = {
        let mut st = GUI_STATE.lock().unwrap();
        st.input_counter += 1;
        let id = format!("input_{}", st.input_counter);
        st.pending_inputs.insert(id.clone(), (tx, ty.to_string()));
        st.output_buffer.push(format!("INPUT_REQUEST:{}:{}", id, prompt));
        id
    };
    rx.recv().unwrap_or_default()
}

pub fn create_input_request(prompt: &str) -> String { create_input_request_typed::<String>(prompt) }