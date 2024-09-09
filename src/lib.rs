pub mod ffi;
pub mod transactions;
pub mod query;
mod utils;

// Re-export main functions for easier use
pub use transactions::{execute_credit_leg, process_credit_result, execute_debit_leg};
pub use query::generate_balance_query;