use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::collections::HashMap;

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Debug)]
struct Response {
    status_code: i32,
    headers: HashMap<String, String>,
    body: String,
}

#[no_mangle]
pub extern "C" fn handle_http_request(request_ptr: *const c_char) -> *mut c_char {
    let request_str = unsafe { CStr::from_ptr(request_ptr).to_str().unwrap_or("") };
    let request = parse_request(request_str);
    let response = handle_request(request);
    let response_json = serialize_response(&response);
    CString::new(response_json).unwrap_or_default().into_raw()
}

fn parse_request(request_str: &str) -> Request {
    let parsed: serde_json::Value = serde_json::from_str(request_str).unwrap_or_default();
    
    Request {
        method: parsed["method"].as_str().unwrap_or("").to_string(),
        path: parsed["path"].as_str().unwrap_or("").to_string(),
        headers: parsed["headers"].as_object()
            .map(|h| h.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string())).collect())
            .unwrap_or_default(),
        body: parsed["body"].as_str().unwrap_or("").to_string(),
    }
}

fn handle_request(req: Request) -> Response {
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/api/data") => handle_get_data(),
        ("POST", "/api/data") => handle_post_data(&req.body),
        ("PUT", "/api/data") => handle_put_data(&req.body),
        ("DELETE", "/api/data") => handle_delete_data(),
        _ => not_found_response(),
    }
}

fn handle_get_data() -> Response {
    Response {
        status_code: 200,
        headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
        body: r#"{"message": "Hello from WebAssembly API!"}"#.to_string(),
    }
}

fn handle_post_data(body: &str) -> Response {
    Response {
        status_code: 201,
        headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
        body: format!(r#"{{"message": "Data created successfully", "received": {}}}"#, body),
    }
}

fn handle_put_data(body: &str) -> Response {
    Response {
        status_code: 200,
        headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
        body: format!(r#"{{"message": "Data updated successfully", "received": {}}}"#, body),
    }
}

fn handle_delete_data() -> Response {
    Response {
        status_code: 200,
        headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
        body: r#"{"message": "Data deleted successfully"}"#.to_string(),
    }
}

fn not_found_response() -> Response {
    Response {
        status_code: 404,
        headers: HashMap::from([("Content-Type".to_string(), "text/plain".to_string())]),
        body: "Not Found".to_string(),
    }
}

fn serialize_response(response: &Response) -> String {
    serde_json::json!({
        "statusCode": response.status_code,
        "headers": response.headers,
        "body": response.body
    }).to_string()
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    }
}