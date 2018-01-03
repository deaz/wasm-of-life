#[macro_use]
extern crate lazy_static;

use std::mem;
use std::slice;
use std::os::raw::{c_char, c_void};
use std::ffi::CString;
use std::sync::Mutex;

mod game;
mod utils;

use game::Game;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new(10, 10));
}

// In order to work with the memory we expose (de)allocation methods
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);

    utils::safe_log("Allocated!");

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
    *GAME.lock().unwrap() = Game::new(width as u16, height as u16);
}

// the Javascript side passes a pointer to a buffer, the size of the corresponding canvas
// and the current timestamp
#[no_mangle]
pub fn draw(pointer: *mut u8, max_width: u16, max_height: u16) {
    // pixels are stored in RGBA, so each pixel is 4 bytes
    let byte_size = max_width as usize * max_height as usize * 4;
    let buffer = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

    let game: &mut Game = &mut GAME.lock().unwrap();
    {
        // Separate scope for borrow of `game` because of mutable borrow for `next_step()` below
        let cell_width = max_width / game.width;
        let cell_height = max_height / game.height;
        let cells = game.get_cells();
        for (col_num, column) in cells.iter().enumerate() {
            for (row_num, &cell) in column.iter().enumerate() {
                let x = col_num as u16 * cell_width;
                let y = row_num as u16 * cell_height;
                draw_cell(buffer, max_width, x, y, cell_width, cell_height, cell);
            }
        }
    }

    game.next_step();
}

fn draw_cell(
    buffer: &mut [u8],
    canvas_width: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    cell: bool,
) {
    for i in x..(x + width) {
        for j in y..(y + height) {
            let offset = (j as usize * canvas_width as usize + i as usize) * 4;
            buffer[offset] = if cell { 0 } else { 255 };
            buffer[offset + 1] = if cell { 0 } else { 255 };
            buffer[offset + 2] = if cell { 0 } else { 255 };
            buffer[offset + 3] = 255; // alpha
        }
    }
}
