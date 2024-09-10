mod ffi;
mod transactions;
mod query;
mod utils;
mod http;

// Re-export main functions for easier use
pub use transactions::{execute_credit_leg, process_credit_result, execute_debit_leg};
pub use query::generate_balance_query;
pub use http::handle_http_request as custom_handle_http_request;

#[no_mangle]
pub extern "C" fn init() {
    // Any initialization code can go here
}