use std::mem;
use std::ffi::CString;

use super::browser;

/// Modulo function.
///
/// # Example
///
/// ```
/// assert!(modulo(-1, 10), 9);
/// assert!(modulo(7, 6), 1);
/// ```
pub fn modulo(a: isize, b: isize) -> usize {
    (((a % b) + b) % b) as usize
}

/// Safe wrapper for `log` function.
/// Logs `text` to browser console.
pub fn log(text: &str) {
    let s = CString::new(text).unwrap();
    let p = s.as_ptr();
    // Do not deallocate on rust side. `dealloc_str` should be called from JS
    mem::forget(s);
    unsafe {
        browser::log(p as *const u8);
    }
}

/// Safe wrapper for `drawBlackRect` from browser side.
pub fn draw_black_rect(x: usize, y: usize, width: usize, height: usize) {
    unsafe {
        browser::drawBlackRect(x, y, width, height);
    }
}

/// Safe wrapper for `drawWhiteRect` from browser side.
pub fn draw_white_rect(x: usize, y: usize, width: usize, height: usize) {
    unsafe {
        browser::drawWhiteRect(x, y, width, height);
    }
}
