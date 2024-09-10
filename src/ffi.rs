#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn custom_dealloc_str(ptr: *mut u8, len: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, len);
    }
}

pub(crate) fn string_to_ptr(s: &str) -> *const u8 {
    let mut bytes = s.as_bytes().to_vec();
    bytes.push(0); // Null terminator
    let ptr = alloc(bytes.len());
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
    }
    ptr as *const u8
}

extern "C" {
    pub fn log_message(ptr: *const u8, len: i32);
}