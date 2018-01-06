//! Implementation of game of life in rust and wasm.

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::ffi::CString;
use std::os::raw::c_char;

mod game;
mod utils;
mod browser;

use game::Game;
use utils::{draw_black_rect, draw_white_rect};

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new(10, 10));
}

#[no_mangle]
/// Initializes game with given field size.
pub extern "C" fn init(width: usize, height: usize) {
    *GAME.lock().unwrap() = Game::new(width, height);
}

/// Method for string deallocation on JS side.
#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
/// Draws game field and updates game field state.
pub fn draw(canvas_width: usize, canvas_height: usize) {
    let game: &mut Game = &mut GAME.lock().unwrap();

    let updated = game.update();

    let cell_width = canvas_width / game.width;
    let cell_height = canvas_height / game.height;
    let cells = game.get_cells();
    for (col_num, row_num) in updated {
        let x = col_num * cell_width;
        let y = row_num * cell_height;
        let cell = cells[col_num][row_num];
        if cell {
            draw_black_rect(x, y, cell_width, cell_height);
        } else {
            draw_white_rect(x, y, cell_width, cell_height);
        }
    }
}
