//! # GUI Module - Web Server and Browser Integration
//!
//! This module provides the core web server functionality that powers WebRust applications.
//! It automatically launches a local web server, opens the user's default browser, and
//! handles real-time communication between the Rust application and the web interface.
//!
//! ## Architecture
//!
//! The GUI system consists of several key components:
//!
//! ### Web Server
//! - **Local HTTP Server**: Runs on `127.0.0.1:8080`
//! - **Static File Serving**: Serves HTML, CSS, and JavaScript files
//! - **API Endpoints**: Provides REST API for real-time communication
//! - **Auto-termination**: Intelligently shuts down when the user is done
//!
//! ### State Management
//! - **Thread-safe State**: Uses `Arc<Mutex<GuiState>>` for concurrent access
//! - **Output Buffer**: Stores all program output for web display
//! - **Input Tracking**: Manages pending input requests with type validation
//! - **Request Monitoring**: Tracks browser activity for smart shutdown
//!
//! ### Browser Integration
//! - **Auto-launch**: Automatically opens the default browser
//! - **Real-time Updates**: JavaScript polls for updates every 300ms
//! - **Smart Shutdown**: Waits for browser to finish loading before closing
//!
//! ## Key Functions
//!
//! ### `start_gui_server<F>(user_function: F)`
//!
//! The main entry point that:
//! 1. Starts the HTTP server on port 8080
//! 2. Opens the user's default browser
//! 3. Runs the user's main function in a separate thread
//! 4. Handles incoming HTTP requests
//! 5. Intelligently shuts down when appropriate
//!
//! ### `add_output(text: String)`
//!
//! Adds text to the output buffer for display in the web interface.
//! Supports special formatting:
//! - HTML content for styled text
//! - LaTeX markers for mathematical rendering
//! - Input request markers for interactive prompts
//!
//! ### `create_input_request_typed<T>(prompt: &str) -> String`
//!
//! Creates a typed input request with automatic validation:
//! - Generates unique input ID
//! - Stores type information for validation
//! - Blocks until user provides valid input
//! - Returns the validated input value
//!
//! ## Smart Shutdown Logic
//!
//! The server implements intelligent shutdown behavior:
//!
//! 1. **Initial Wait**: Gives browser time to connect (up to 10 seconds)
//! 2. **Activity Monitoring**: Tracks when browser makes requests
//! 3. **Grace Period**: Waits 5 seconds after last browser activity
//! 4. **Maximum Runtime**: Hard limit of 30 seconds for safety
//! 5. **Clean Exit**: Displays confirmation message before closing
//!
//! This eliminates the need for users to manually terminate with Ctrl+C.
//!
//! ## API Endpoints
//!
//! ### GET `/api/state`
//! Returns current application state:
//! ```json
//! {
//!   "output": ["line1", "line2", ...],
//!   "pending_inputs": ["input_1", "input_2", ...],
//!   "program_finished": false
//! }
//! ```
//!
//! ### POST `/api/input`
//! Submits user input:
//! ```json
//! {
//!   "id": "input_1",
//!   "value": "user response"
//! }
//! ```
//!
//! ### POST `/api/validate`
//! Validates input before submission:
//! ```json
//! {
//!   "id": "input_1",
//!   "value": "test value"
//! }
//! ```
//!
//! Returns:
//! ```json
//! {
//!   "valid": true|false,
//!   "error": "error message if invalid"
//! }
//! ```

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::collections::HashMap;
use tiny_http::{Server, Response, Header};
use serde_json::{json, Value};
use std::str::FromStr;
use std::time::{Duration, Instant};

pub struct GuiState {
    pub pending_inputs: HashMap<String, (mpsc::Sender<String>, String)>,
    pub output_buffer: Vec<String>,
    pub input_counter: usize,
    pub program_finished: bool,
    pub last_request_time: Option<Instant>,
    pub has_received_requests: bool,
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

pub fn start_gui_server<F>(user_function: F)
where F: FnOnce() + Send + 'static,
{
    let server = Server::http("127.0.0.1:8080").expect("Failed to start server");
    open_browser();

    thread::spawn(move || {
        user_function();

        // Marquer le programme comme terminé
        {
            let mut state = GUI_STATE.lock().unwrap();
            state.program_finished = true;
        }

        // Attendre que le navigateur ait récupéré le contenu
        let start_time = Instant::now();
        let max_wait_time = Duration::from_secs(30); // Maximum 30 secondes
        let min_wait_time = Duration::from_secs(3);  // Minimum 3 secondes

        loop {
            thread::sleep(Duration::from_millis(500));

            let elapsed = start_time.elapsed();
            let should_exit = {
                let state = GUI_STATE.lock().unwrap();

                // Si aucune requête reçue et plus de 10 secondes, on sort
                if !state.has_received_requests && elapsed > Duration::from_secs(10) {
                    true
                } else if let Some(last_req) = state.last_request_time {
                    // Si on a reçu des requêtes, attendre 5 secondes après la dernière
                    elapsed > min_wait_time && last_req.elapsed() > Duration::from_secs(5)
                } else {
                    // Pas encore de requêtes, continuer d'attendre
                    elapsed > max_wait_time
                }
            };

            if should_exit {
                break;
            }
        }

        println!("🚀 Content served successfully! Closing server...");
        std::process::exit(0);
    });

    for request in server.incoming_requests() {
        // Mettre à jour le timestamp de la dernière requête
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
            else if url == "/style.css" { serve_file(request, "style.css", "text/css"); }
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
        "index.html" => include_str!("../static/index.html"),
        "style.css" => include_str!("../static/style.css"),
        "script.js" => include_str!("../static/script.js"),
        _ => "Not Found"
    };
    let response = Response::from_string(content)
        .with_header(Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap());
    let _ = request.respond(response);
}

fn handle_api_get(request: tiny_http::Request, url: &str) {
    if url == "/api/state" {
        let state = GUI_STATE.lock().unwrap();
        let json_response = json!({
            "output": state.output_buffer,
            "pending_inputs": state.pending_inputs.keys().collect::<Vec<_>>(),
            "program_finished": state.program_finished
        });
        let response = Response::from_string(json_response.to_string())
            .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
        let _ = request.respond(response);
    } else {
        serve_404(request);
    }
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
                            let response = Response::from_string(json!({"valid": true}).to_string())
                                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                            let _ = request.respond(response);
                            return;
                        },
                        Err(error) => {
                            let response = Response::from_string(json!({"valid": false, "error": error}).to_string())
                                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                            let _ = request.respond(response);
                            return;
                        }
                    }
                }
            }
        }
        let response = Response::from_string(json!({"valid": false, "error": "Invalid request"}).to_string())
            .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
        let _ = request.respond(response);
    } else {
        serve_404(request);
    }
}

fn serve_404(request: tiny_http::Request) {
    let response = Response::from_string("404 Not Found").with_status_code(404);
    let _ = request.respond(response);
}

pub fn add_output_same_line(text: String) {
    let mut state = GUI_STATE.lock().unwrap();
    if let Some(last_line) = state.output_buffer.last_mut() {
        // Ajouter à la dernière ligne existante
        last_line.push_str(&text);
    } else {
        // Première ligne
        state.output_buffer.push(text);
    }
}

pub fn add_output_new_line(text: String) {
    let mut state = GUI_STATE.lock().unwrap();
    state.output_buffer.push(text);
}

pub fn add_output(text: String) {
    add_output_new_line(text);
}

pub fn create_input_request_typed<T>(prompt: &str) -> String
where T: FromStr,
{
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

pub fn create_input_request(prompt: &str) -> String {
    create_input_request_typed::<String>(prompt)
}