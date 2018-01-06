//! Module with exports of JS functions.

extern "C" {
    pub fn drawBlackRect(x: usize, y: usize, width: usize, height: usize);
    pub fn drawWhiteRect(x: usize, y: usize, width: usize, height: usize);
    pub fn log(ptr: *const u8);
}
