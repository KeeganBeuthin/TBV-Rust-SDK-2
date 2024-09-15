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
    let request_str = unsafe { std::ffi::CStr::from_ptr(request_ptr).to_str().unwrap() };
    let request: Value = serde_json::from_str(request_str).unwrap();
    
    let response = handle_request(request);
    let response_json = serde_json::to_string(&response).unwrap();
    
    // Use CString to ensure proper null-termination
    let c_str = CString::new(response_json).unwrap();
    c_str.into_raw()
}

fn handle_request(request: Value) -> Value {
    let method = request["method"].as_str().unwrap();
    let path = request["path"].as_str().unwrap();
    
    match (method, path) {
        ("GET", "/api/data") => json!({
            "status_code": 200,
            "headers": {"Content-Type": "application/json"},
            "body": {"message": "Hello from WebAssembly API!"}
        }),
        ("POST", "/api/data") => json!({
            "status_code": 201,
            "headers": {"Content-Type": "application/json"},
            "body": {
                "message": "Data created successfully",
                "received": request["body"]
            }
        }),
        ("PUT", "/api/data") => json!({
            "status_code": 200,
            "headers": {"Content-Type": "application/json"},
            "body": {
                "message": "Data updated successfully",
                "received": request["body"]
            }
        }),
        ("DELETE", "/api/data") => json!({
            "status_code": 200,
            "headers": {"Content-Type": "application/json"},
            "body": {"message": "Data deleted successfully"}
        }),
        _ => json!({
            "status_code": 404,
            "headers": {"Content-Type": "application/json"},
            "body": {"error": "Not Found"}
        }),
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
