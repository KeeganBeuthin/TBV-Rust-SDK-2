extern crate serde;
extern crate serde_json;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::collections::HashMap;
use self::serde::{Deserialize, Serialize};
use self::serde_json::{Value, json};

#[derive(Debug, Deserialize)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Value,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub status_code: i32,
    pub headers: HashMap<String, String>,
    pub body: Value,
}

#[no_mangle]
pub extern "C" fn handle_http_request(request_ptr: *const c_char) -> *mut c_char {
    let request_str = unsafe { CStr::from_ptr(request_ptr).to_str().unwrap() };
    let request: Request = serde_json::from_str(request_str).unwrap_or_else(|_| Request {
        method: "GET".to_string(),
        path: "/".to_string(),
        headers: HashMap::new(),
        body: Value::Null,
    });
    let response = handle_request(request);
    let response_json = serde_json::to_string(&response).unwrap();
    
    // Debug logging
    println!("Debug - Response JSON: {}", response_json);
    
    let c_string = CString::new(response_json).unwrap();
    let ptr = c_string.into_raw();
    
    // Debug logging
    unsafe {
        let debug_str = CStr::from_ptr(ptr).to_str().unwrap();
        println!("Debug - Final C string: {}", debug_str);
    }
    
    ptr
}

fn handle_request(req: Request) -> Response {
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/api/data") => handle_get_data(),
        ("POST", "/api/data") => handle_post_data(req.body),
        ("PUT", "/api/data") => handle_put_data(req.body),
        ("DELETE", "/api/data") => handle_delete_data(),
        _ => not_found_response(),
    }
}

fn handle_get_data() -> Response {
    Response {
        status_code: 200,
        headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
        body: json!({"message": "Hello from WebAssembly API!"}),
    }
}

fn handle_post_data(body: Value) -> Response {
    Response {
        status_code: 201,
        headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
        body: json!({
            "message": "Data created successfully",
            "received": body
        }),
    }
}

fn handle_put_data(body: Value) -> Response {
    Response {
        status_code: 200,
        headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
        body: json!({
            "message": "Data updated successfully",
            "received": body
        }),
    }
}

fn handle_delete_data() -> Response {
    Response {
        status_code: 200,
        headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
        body: json!({"message": "Data deleted successfully"}),
    }
}

fn not_found_response() -> Response {
    Response {
        status_code: 404,
        headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
        body: json!({"error": "Not Found"}),
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    }
}