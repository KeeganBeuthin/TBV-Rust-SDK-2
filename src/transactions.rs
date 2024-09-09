use crate::ffi::string_to_ptr;
use crate::utils::log;

#[no_mangle]
pub extern "C" fn execute_credit_leg(amount_ptr: *const u8, amount_len: usize, account_ptr: *const u8, account_len: usize) -> *const u8 {
    let amount = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(amount_ptr, amount_len)) };
    let account = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(account_ptr, account_len)) };
    
    let query = crate::query::generate_balance_query(account);
    
    string_to_ptr(&query)
}

#[no_mangle]
pub extern "C" fn process_credit_result(result_ptr: *const u8, result_len: usize, amount_ptr: *const u8, amount_len: usize) -> *const u8 {
    let result = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(result_ptr, result_len)) };
    let amount = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(amount_ptr, amount_len)) };
    
    log(&format!("Processing result: {}, amount: {}", result, amount));

    let parsed_result: serde_json::Value = match serde_json::from_str(result) {
        Ok(v) => v,
        Err(e) => {
            let error_msg = format!("Failed to parse result JSON: {}", e);
            log(&error_msg);
            return string_to_ptr(&error_msg);
        }
    };

    log(&format!("Parsed result: {:?}", parsed_result));

    let balance = match parsed_result["results"].as_array()
        .and_then(|results| results.get(0))
        .and_then(|first_result| first_result["balance"].as_str())
        .and_then(|balance| balance.parse::<f64>().ok()) {
        Some(b) => b,
        None => {
            let error_msg = "Failed to extract balance from result";
            log(error_msg);
            return string_to_ptr(error_msg);
        }
    };

    log(&format!("Extracted balance: {}", balance));

    let amount_float: f64 = match amount.parse() {
        Ok(a) => a,
        Err(e) => {
            let error_msg = format!("Failed to parse amount as float: {}", e);
            log(&error_msg);
            return string_to_ptr(&error_msg);
        }
    };

    let new_balance = balance + amount_float;
    
    let response = format!("Current balance: {}. After credit of {}, new balance: {}", balance, amount_float, new_balance);
    log(&response);
    
    string_to_ptr(&response)
}

#[no_mangle]
pub extern "C" fn execute_debit_leg(amount_ptr: *const u8, amount_len: usize, account_ptr: *const u8, account_len: usize) -> *const u8 {
    let amount = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(amount_ptr, amount_len)) };
    let account = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(account_ptr, account_len)) };
    
    log(&format!("Executing debit leg: amount = {}, account = {}", amount, account));

    let result = format!("Debiting {} from account {}", amount, account);
    log(&result);

    string_to_ptr(&result)
}