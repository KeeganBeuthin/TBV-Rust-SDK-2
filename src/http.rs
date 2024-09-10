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
    let request_str = unsafe { CStr::from_ptr(request_ptr).to_str().unwrap() };
    let request = parse_request(request_str);
    let response = handle_request(request);
    let response_json = serialize_response(response);
    CString::new(response_json).unwrap().into_raw()
}

fn parse_request(request_str: &str) -> Request {
    let mut lines = request_str.lines();
    let first_line = lines.next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("").to_string();

    let mut headers = HashMap::new();
    let mut body = String::new();
    let mut reading_body = false;

    for line in lines {
        if line.is_empty() {
            reading_body = true;
            continue;
        }
        if reading_body {
            body.push_str(line);
            body.push('\n');
        } else {
            let mut header_parts = line.splitn(2, ':');
            if let (Some(key), Some(value)) = (header_parts.next(), header_parts.next()) {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }

    Request { method, path, headers, body }
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
        body: r#"{"message": "Hello from WebAssembly API!"}"#.to_string(),
    }
}

fn handle_post_data(body: String) -> Response {
    Response {
        status_code: 201,
        headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
        body: format!(r#"{{"message": "Data created successfully", "received": {}}}"#, body),
    }
}

fn handle_put_data(body: String) -> Response {
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

fn serialize_response(response: Response) -> String {
    let headers_json = response.headers.iter()
        .map(|(k, v)| format!(r#""{}":"{}""#, k, v))
        .collect::<Vec<String>>()
        .join(",");

    format!(
        r#"{{"statusCode":{},"headers":{{{}}},"body":"{}"}}"#,
        response.status_code,
        headers_json,
        response.body.replace("\"", "\\\"")
    )
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    }
}