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

    pub fn next_step(&mut self) {
        let mut new_cells = self.cells.clone();
        for (col_num, column) in self.cells.iter().enumerate() {
            for (row_num, &cell) in (*column).iter().enumerate() {
                let count = self.count_neighbour_cells(col_num, row_num);
                if cell && (count < 2 || count > 3) {
                    new_cells[col_num][row_num] = false
                } else if !cell && count == 3 {
                    new_cells[col_num][row_num] = true
                }
            }
        }
        self.cells = new_cells;
    }

    fn count_neighbour_cells(&self, col_num: usize, row_num: usize) -> u8 {
        let mut count = 0u8;
        let neighbours = self.get_neighbours(col_num, row_num);
        for &(x, y) in neighbours.iter() {
            if self.cells[x % self.width][y % self.height] {
                count += 1;
            }
        }
        count
    }

    fn get_neighbours(&self, col_num: usize, row_num: usize) -> [(usize, usize); 8] {
        let mut neighbours = [(0usize, 0usize); 8];
        let mut index = 0;
        for i in col_num - 1..col_num + 2 {
            for j in row_num - 1..row_num + 2 {
                if col_num != i || row_num != j {
                    neighbours[index] = (i, j);
                    index += 1;
                }
            }
        }
        neighbours
    }
}
