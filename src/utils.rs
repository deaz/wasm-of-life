use std::mem;

// Modulo, not remainder. modulo(-1, 10) -> 9
pub fn modulo(a: isize, b: isize) -> usize {
    (((a % b) + b) % b) as usize
}

extern "C" {
    fn log(ptr: *const u8);
}

pub fn safe_log(text: &str) {
    let s = String::from(text);
    let p = s.as_ptr();
    // Do not deallocate on rust side. `dealloc_str` should be called from JS
    mem::forget(s);
    unsafe {
        log(p as *const u8);
    }
}
