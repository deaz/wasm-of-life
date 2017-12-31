extern crate rand;

use self::rand::{Rng, SeedableRng, StdRng};

pub struct Game {
    pub width: usize,
    pub height: usize,
    cells: Vec<Vec<bool>>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = vec![vec![false; height]; width];
        // wasm doesn't have OsRng so seed is hardcoded
        let seed = [42];
        let mut rng: StdRng = SeedableRng::from_seed(&seed[..]);
        for column in cells.iter_mut() {
            for cell in (*column).iter_mut() {
                *cell = rng.gen();
            }
        }
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get_cells(&self) -> &[Vec<bool>] {
        &self.cells[..]
    }
}
