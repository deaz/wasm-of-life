#[macro_use]
extern crate lazy_static;

use std::mem;
use std::os::raw::{c_char, c_void};
use std::ffi::CString;
use std::sync::Mutex;

mod game;
mod utils;

use game::Game;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new(10, 10));
}

extern "C" {
    fn drawBlackRect(x: f64, y: f64, width: f64, height: f64);
    fn clearAll();
}

// In order to work with the memory we expose (de)allocation methods
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);

    utils::safe_log(&format!("Allocated {} bytes\n", size));

    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr as *mut c_void
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn init(width: usize, height: usize) {
    *GAME.lock().unwrap() = Game::new(width, height);
}

#[no_mangle]
pub fn draw(pointer: *mut u8, canvas_width: usize, canvas_height: usize) {
    // pixels are stored in RGBA, so each pixel is 4 bytes
    let byte_size = canvas_width * canvas_height * 4;
    let buffer = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

    let game: &mut Game = &mut GAME.lock().unwrap();

    let updated = game.update();

    let cell_width = canvas_width / game.width;
    let cell_height = canvas_height / game.height;
    let cells = game.get_cells();
    for (col_num, row_num) in updated {
        let x = col_num * cell_width;
        let y = row_num * cell_height;
        let cell = cells[col_num][row_num];
        draw_cell(buffer, canvas_width, x, y, cell_width, cell_height, cell);
    }
}

fn draw_cell(x: f64, y: f64, width: f64, height: f64, cell: bool) {
    unsafe {
        if cell {
            drawBlackRect(x, y, width, height);
        }
    }
}
