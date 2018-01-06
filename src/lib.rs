#[macro_use]
extern crate lazy_static;

use std::slice;
use std::os::raw::{c_char, c_void};
use std::ffi::CString;
use std::sync::Mutex;

mod game;
mod utils;

use game::Game;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new(10, 10));
    static ref BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

// In order to work with the memory we expose (de)allocation methods
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let buf: &mut Vec<_> = &mut BUFFER.lock().unwrap();
    *buf = Vec::with_capacity(size);

    utils::safe_log(&format!("Allocated {} bytes\n", size));

    let ptr = buf.as_mut_ptr();
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

// the Javascript side passes a pointer to a buffer, the size of the corresponding canvas
// and the current timestamp
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

fn draw_cell(
    buffer: &mut [u8],
    canvas_width: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    cell: bool,
) {
    for i in x..(x + width) {
        for j in y..(y + height) {
            let offset = (j * canvas_width + i) * 4;
            buffer[offset] = if cell { 0 } else { 255 };
            buffer[offset + 1] = if cell { 0 } else { 255 };
            buffer[offset + 2] = if cell { 0 } else { 255 };
            buffer[offset + 3] = 255; // alpha
        }
    }
}
