//! Module with "game of life" logic.

extern crate rand;

use std::collections::HashSet;
use self::rand::{Rng, SeedableRng, StdRng};
use super::utils::modulo;

/// Struct with game state.
pub struct Game {
    /// Field width in cells.
    pub width: usize,
    /// Field height in cells.
    pub height: usize,
    /// Field cells. `true` - cell is alive, `false` - cell is dead.
    cells: Vec<Vec<bool>>,
    /// Cells that need to be checked on next update.
    cells_to_check: HashSet<(usize, usize)>,
}

impl Game {
    /// Creates new game field with given size.
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = vec![vec![false; height]; width];
        let mut cells_to_check = HashSet::new();
        // wasm doesn't have OsRng so seed is hardcoded
        let seed = [42];
        let mut rng: StdRng = SeedableRng::from_seed(&seed[..]);
        for (col_num, column) in cells.iter_mut().enumerate() {
            for (row_num, cell) in (*column).iter_mut().enumerate() {
                *cell = rng.gen();
                cells_to_check.insert((col_num, row_num));
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

    /// Returns slice with field cells.
    pub fn get_cells(&self) -> &[Vec<bool>] {
        &self.cells[..]
    }

    /// Updates field state and returns updated cells.
    pub fn update(&mut self) -> Vec<(usize, usize)> {
        let mut new_cells = self.cells.clone();

        let cells_to_check = self.cells_to_check.clone();
        self.cells_to_check.clear();

        let mut updated = Vec::new();
        for &(col_num, row_num) in cells_to_check.iter() {
            let count = self.count_neighbour_cells(col_num, row_num);
            let cell = self.cells[col_num][row_num];
            if cell && (count < 2 || count > 3) {
                new_cells[col_num][row_num] = false;
            } else if !cell && count == 3 {
                new_cells[col_num][row_num] = true;
            } else {
                continue;
            }

            self.cells_to_check.insert((col_num, row_num));
            for &point in self.get_neighbours(col_num, row_num).iter() {
                self.cells_to_check.insert(point);
            }

            updated.push((col_num, row_num));
        }
        self.cells = new_cells;

        updated
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
        for i in col_num as isize - 1..col_num as isize + 2 {
            for j in row_num as isize - 1..row_num as isize + 2 {
                if col_num as isize != i || row_num as isize != j {
                    neighbours[index] = (
                        modulo(i, self.width as isize),
                        modulo(j, self.height as isize),
                    );
                    index += 1;
                }
            }
        }
        neighbours
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glider() {
        // *____
        // _**__
        // **___
        // _____
        // _____
        let glider_1 = vec![
            vec![true, false, true, false, false],
            vec![false, true, true, false, false],
            vec![false, true, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ];

        // _*___
        // __*__
        // ***__
        // _____
        // _____
        let glider_2 = vec![
            vec![false, false, true, false, false],
            vec![true, false, true, false, false],
            vec![false, true, true, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ];

        let mut game = Game::new(5, 5);
        game.cells = glider_1;
        game.update();
        assert_eq!(game.cells, glider_2);
    }
}
