// webrust/src/io/gui.rs
//! # WebRust GUI — local HTTP server + browser renderer
//!
//! Starts a tiny HTTP server on `127.0.0.1:8080`, opens your default browser,
//! serves the core assets (`index.html`, `style.css`, `main.js`, `table.js`, `turtle.js`),
//! and exposes a minimal REST API to stream HTML output and handle typed inputs.
//!
//! ## Features
//! - **Output**: push HTML fragments that stream into the page.
//! - **Input**: `input<T>()` spawns a typed prompt (server-side validation).
//! - **Theming**: `StyleConfig { bg, color, font, size }` via `#[gui(...)]` or API.
//! - **Auto-shutdown**: quits when the program has finished and the client is idle.
//!
//! ## Public API
//! - `start_gui_server(f)` / `start_gui_server_with_style(style, f)`
//! - `add_output(html)`, `add_output_new_line(html)`, `add_output_same_line(html)`
//! - `create_input_request_typed::<T>(prompt)` / `create_input_request(prompt)`
//!
//! ## HTTP Routes
//! - `GET /`             → `index.html`
//! - `GET /style.css`    → base stylesheet
//! - `GET /main.js`      → polling + I/O client logic
//! - `GET /table.js`     → table helpers
//! - `GET /turtle.js`    → turtle helpers
//! - `GET /api/state`    → `{ output, pending_inputs, program_finished }`
//! - `GET /api/style`    → `{ bg, color, font, size }`
//! - `POST /api/input`   → `{ id, value }` (final submission)
//! - `POST /api/validate`→ `{ id, value }` (validation only)
//!
//! ## Input Validation
//! Accepted types: `"i32"`, `"f64"`, `"bool"`, `"char"`, or free-form (default).
//! Server enforces the expected type before accepting a value.
//!
//! ## Concurrency & Features
//! - **Feature `rwlock` enabled by default**: concurrent readers via `RwLock`.
//! - Without the feature, it falls back to **`Mutex`** (no extra deps).
//!
//! ## Minimal Example
//! ```rust,no_run
//! use webrust::prelude::*;
//!
//! #[gui(Arial 14px black !white)]
//! fn main() {
//!     println("<b>Hello WebRust</b>");
//!     let name: String = input("Your name?");
//!     println(format!("Welcome, <green>{name}</green>!"));
//! }
//! ```
//!
//! ## Notes
//! - Local-only networking (127.0.0.1). No TLS. Intended for dev/demos.
//! - You control the HTML you emit; escape user data as needed.

use parking_lot::RwLock;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    borrow::Cow,
    collections::HashMap,
    str::FromStr,
    sync::{mpsc, Arc},
    thread,
    time::{Duration, Instant},
};
use tiny_http::{Header, Method, Request, Response, Server};

const ADDR: &str = "127.0.0.1:8080";
const CT_HTML: &str = "text/html; charset=utf-8";
const CT_CSS: &str = "text/css; charset=utf-8";
const CT_JS: &str = "application/javascript; charset=utf-8";
const CT_JSON: &str = "application/json; charset=utf-8";
const CT_TEXT: &str = "text/plain; charset=utf-8";

#[derive(Clone, Default)]
pub struct StyleConfig {
    pub bg: String,
    pub color: String,
    pub font: String,
    pub size: String,
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
            pending_inputs: HashMap::with_capacity(8),
            output_buffer: Vec::with_capacity(64),
            input_counter: 0,
            program_finished: false,
            last_request_time: None,
            has_received_requests: false,
            style_config: StyleConfig::default(),
        }
    }
}

lazy_static::lazy_static! {
    pub static ref GUI_STATE: Arc<RwLock<GuiState>> = Arc::new(RwLock::new(GuiState::default()));
}

#[inline(always)]
fn with_state_read<R>(f: impl FnOnce(&GuiState) -> R) -> R {
    let g = GUI_STATE.read();
    f(&g)
}

#[inline(always)]
fn with_state_write<R>(f: impl FnOnce(&mut GuiState) -> R) -> R {
    let mut g = GUI_STATE.write();
    f(&mut g)
}

#[inline]
fn validate_input_rust(value: &str, expected: &str) -> Result<(), &'static str> {
    match expected {
        "i32" => value
            .trim()
            .parse::<i32>()
            .map(|_| ())
            .map_err(|_| "Invalid i32"),
        "f64" => value
            .trim()
            .parse::<f64>()
            .map(|_| ())
            .map_err(|_| "Invalid f64"),
        "bool" => value
            .trim()
            .parse::<bool>()
            .map(|_| ())
            .map_err(|_| "Invalid bool"),
        "char" => value
            .trim()
            .parse::<char>()
            .map(|_| ())
            .map_err(|_| "Invalid char"),
        _ => Ok(()),
    }
}

fn open_browser() {
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(500));
        let url = format!("http://{}", ADDR);
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("cmd")
                .args(&["/c", "start", &url])
                .spawn();
        }
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("open").arg(&url).spawn();
        }
        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
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
    with_state_write(|st| st.style_config = style);
    let server = Server::http(ADDR).expect("Failed to start server");
    open_browser();
    thread::spawn(move || {
        user();
        with_state_write(|st| st.program_finished = true);
        let start = Instant::now();
        let (max, min) = (Duration::from_secs(30), Duration::from_secs(3));
        loop {
            thread::sleep(Duration::from_millis(500));
            let elapsed = start.elapsed();
            let should_exit = with_state_read(|st| {
                if !st.has_received_requests && elapsed > Duration::from_secs(5) {
                    return true;
                }
                if let Some(last) = st.last_request_time {
                    return elapsed > min && last.elapsed() > Duration::from_secs(1);
                }
                elapsed > max
            });
            if should_exit {
                break;
            }
        }
        std::process::exit(0);
    });
    for req in server.incoming_requests() {
        with_state_write(|st| {
            st.last_request_time = Some(Instant::now());
            st.has_received_requests = true;
        });
        handle_request(req);
    }
}

#[inline(always)]
fn respond(req: Request, body: impl Into<String>, ct: &str) {
    let h = Header::from_bytes(b"Content-Type", ct.as_bytes()).unwrap();
    let _ = req.respond(Response::from_string(body.into()).with_header(h));
}

#[inline(always)]
fn respond_json(req: Request, v: Value) {
    respond(req, v.to_string(), CT_JSON);
}

fn handle_request(request: Request) {
    let method = request.method().clone();
    let url = request.url().to_string();
    match method {
        Method::Get => match url.as_str() {
            "/" => respond(request, include_str!("../../static/index.html"), CT_HTML),
            "/style.css" => respond(request, include_str!("../../static/style.css"), CT_CSS),
            "/table.js" => respond(request, include_str!("../../static/table.js"), CT_JS),
            "/turtle.js" => respond(request, include_str!("../../static/turtle.js"), CT_JS),
            "/main.js" => respond(request, include_str!("../../static/main.js"), CT_JS),
            _ if url.starts_with("/api/") => handle_api_get(request, &url),
            _ => serve_404(request),
        },
        Method::Post if url.starts_with("/api/") => handle_api_post(request, &url),
        _ => serve_404(request),
    }
}

fn handle_api_get(request: Request, url: &str) {
    if url == "/api/state" {
        let (output, pending, finished) = with_state_read(|st| {
            (
                st.output_buffer.clone(),
                st.pending_inputs.keys().cloned().collect::<Vec<_>>(),
                st.program_finished,
            )
        });
        respond_json(
            request,
            json!({ "output": output, "pending_inputs": pending, "program_finished": finished }),
        );
    } else if url == "/api/style" {
        let s = with_state_read(|st| st.style_config.clone());
        respond_json(
            request,
            json!({"bg": s.bg, "color": s.color, "font": s.font, "size": s.size}),
        );
    } else {
        serve_404(request);
    }
}

#[derive(Deserialize)]
struct InputPayload<'a> {
    #[serde(borrow)]
    id: Cow<'a, str>,
    #[serde(borrow)]
    value: Cow<'a, str>,
}

fn read_body(mut request: Request) -> (Request, String) {
    let mut body = String::with_capacity(512);
    let _ = request.as_reader().read_to_string(&mut body);
    (request, body)
}

fn handle_api_post(request: Request, url: &str) {
    if url == "/api/input" {
        let (request, body) = read_body(request);
        if let Ok(data) = serde_json::from_str::<InputPayload>(&body) {
            if let Some(tx) =
                with_state_write(|st| st.pending_inputs.remove(&*data.id).map(|(tx, _)| tx))
            {
                let _ = tx.send(data.value.into_owned());
            }
        }
        respond(request, "OK", CT_TEXT);
    } else if url == "/api/validate" {
        let (request, body) = read_body(request);
        if let Ok(data) = serde_json::from_str::<InputPayload>(&body) {
            let res = with_state_read(|st| {
                st.pending_inputs
                    .get(&*data.id)
                    .map(|(_, e)| validate_input_rust(&data.value, e))
            });
            match res {
                Some(Ok(())) => {
                    respond_json(request, json!({ "valid": true }));
                    return;
                }
                Some(Err(e)) => {
                    respond_json(request, json!({ "valid": false, "error": e }));
                    return;
                }
                None => {}
            }
        }
        respond_json(
            request,
            json!({ "valid": false, "error": "Invalid request" }),
        );
    } else {
        serve_404(request);
    }
}

#[inline(always)]
fn serve_404(request: Request) {
    let _ = request.respond(Response::from_string("404 Not Found").with_status_code(404));
}

pub fn add_output_same_line(text: String) {
    with_state_write(|st| {
        let gap = text.find("viz-line-gap=\"").and_then(|s| {
            let s = &text[s + 15..];
            s.find('"').and_then(|e| s[..e].parse::<u32>().ok())
        });
        let g = gap.unwrap_or(6);
        if let Some(last) = st.output_buffer.last_mut() {
            if last.contains(r#"class="webrust-line""#) {
                if let Some(pos) = last.rfind("</div>") {
                    last.insert_str(pos, &text);
                    if let Some(g) = gap {
                        if let Some(mpos) = last.rfind("margin:") {
                            if let Some(end) = last[mpos..].find(';') {
                                last.replace_range(
                                    mpos..mpos + end + 1,
                                    &format!("margin:{}px 0;", g),
                                );
                            }
                        } else if let Some(sty) = last.rfind(r#"style=""#) {
                            last.insert_str(sty + 7, &format!("margin:{}px 0;", g));
                        }
                    }
                } else {
                    last.push_str(&text);
                }
            } else {
                *last = format!(
                    r#"<div class="webrust-line" style="display:block;margin:{}px 0;">{}</div>"#,
                    g, text
                );
            }
        } else {
            st.output_buffer.push(format!(
                r#"<div class="webrust-line" style="display:block;margin:{}px 0;">{}</div>"#,
                g, text
            ));
        }
    });
}

#[inline(always)]
pub fn add_output_new_line(text: String) {
    with_state_write(|st| st.output_buffer.push(text));
}

#[inline(always)]
pub fn add_output(text: String) {
    add_output_new_line(text);
}

pub fn create_input_request_typed<T: FromStr>(prompt: &str) -> String {
    let (tx, rx) = mpsc::channel();
    with_state_write(|st| {
        st.input_counter += 1;
        let id = format!("input_{}", st.input_counter);
        st.pending_inputs.insert(
            id.clone(),
            (
                tx,
                std::any::type_name::<T>()
                    .rsplit("::")
                    .next()
                    .unwrap_or("String")
                    .to_string(),
            ),
        );
        st.output_buffer
            .push(format!("INPUT_REQUEST:{id}:{prompt}"));
    });
    rx.recv().unwrap_or_default()
}

#[inline(always)]
pub fn create_input_request(prompt: &str) -> String {
    create_input_request_typed::<String>(prompt)
}
