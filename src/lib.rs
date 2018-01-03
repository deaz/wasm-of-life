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

extern "C" {
    fn log(ptr: *const u8);
}

// In order to work with the memory we expose (de)allocation methods
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);

    safe_log("Allocated!");

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

// the Javascript side passes a pointer to a buffer, the size of the corresponding canvas
// and the current timestamp
#[no_mangle]
pub fn draw(pointer: *mut u8, max_width: usize, max_height: usize) {
    // pixels are stored in RGBA, so each pixel is 4 bytes
    let byte_size = max_width * max_height * 4;
    let buffer = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

    let game: &mut Game = &mut GAME.lock().unwrap();
    {
        // Separate scope for borrow of `game` because of mutable borrow for `next_step()` below
        let cell_width = max_width / game.width;
        let cell_height = max_height / game.height;
        let cells = game.get_cells();
        for (col_num, column) in cells.iter().enumerate() {
            for (row_num, &cell) in column.iter().enumerate() {
                let x = col_num * cell_width;
                let y = row_num * cell_height;
                draw_rect(buffer, max_width, x, y, cell_width, cell_height, cell);
            }
        }
    }

    game.next_step();
}

fn draw_rect(
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

fn safe_log(text: &str) {
    let s = String::from(text);
    let p = s.as_ptr();
    // Do not deallocate on rust side. `dealloc_str` should be called from JS
    mem::forget(s);
    unsafe {
        log(p as *const u8);
    }
}
