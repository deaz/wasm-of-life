extern crate rand;

use std::collections::HashSet;
use self::rand::{Rng, SeedableRng, StdRng};
use super::utils::modulo;

pub struct Game {
    pub width: u16,
    pub height: u16,
    cells: Vec<Vec<bool>>,
    cells_to_check: HashSet<(u16, u16)>,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Self {
        let mut cells = vec![vec![false; height as usize]; width as usize];
        let mut cells_to_check = HashSet::new();
        // wasm doesn't have OsRng so seed is hardcoded
        let seed = [42];
        let mut rng: StdRng = SeedableRng::from_seed(&seed[..]);
        for (col_num, column) in cells.iter_mut().enumerate() {
            for (row_num, cell) in (*column).iter_mut().enumerate() {
                *cell = rng.gen();
                cells_to_check.insert((col_num as u16, row_num as u16));
            }
        }

        // Glider
        // cells[1][0] = true;
        // cells[2][1] = true;
        // cells[0][2] = true;
        // cells[1][2] = true;
        // cells[2][2] = true;

        Self {
            width,
            height,
            cells,
            cells_to_check,
        }
    }

    pub fn get_cells(&self) -> &[Vec<bool>] {
        &self.cells[..]
    }

    pub fn next_step(&mut self) {
        let mut new_cells = self.cells.clone();
        let mut new_cells_to_check = HashSet::new();
        for &(col_num, row_num) in self.cells_to_check.iter() {
            let count = self.count_neighbour_cells(col_num, row_num);
            let (x, y) = (col_num as usize, row_num as usize);
            let cell = self.cells[x][y];
            if cell && (count < 2 || count > 3) {
                new_cells[x][y] = false;
            } else if !cell && count == 3 {
                new_cells[x][y] = true;
            } else {
                continue;
            }
            new_cells_to_check.insert((col_num, row_num));
            for &point in self.get_neighbours(col_num, row_num).iter() {
                new_cells_to_check.insert(point);
            }
        }
        self.cells = new_cells;
        self.cells_to_check = new_cells_to_check;
    }

    fn count_neighbour_cells(&self, col_num: u16, row_num: u16) -> u8 {
        let mut count = 0u8;
        let neighbours = self.get_neighbours(col_num, row_num);
        for &(x, y) in neighbours.iter() {
            if self.cells[x as usize][y as usize] {
                count += 1;
            }
        }
        count
    }

    fn get_neighbours(&self, col_num: u16, row_num: u16) -> [(u16, u16); 8] {
        let mut neighbours = [(0u16, 0u16); 8];
        let mut index = 0;
        for i in col_num as i16 - 1..col_num as i16 + 2 {
            for j in row_num as i16 - 1..row_num as i16 + 2 {
                if col_num as i16 != i || row_num as i16 != j {
                    neighbours[index] =
                        (modulo(i, self.width as i16), modulo(j, self.height as i16));
                    index += 1;
                }
            }
        }
        neighbours
    }
}
