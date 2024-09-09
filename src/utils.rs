use crate::ffi::{alloc, log_message};

pub fn log(message: &str) {
    let bytes = message.as_bytes();
    let ptr = alloc(bytes.len());
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        log_message(ptr as *const u8, bytes.len() as i32);
    }
}