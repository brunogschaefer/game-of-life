use std::fmt;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::cells::Cells;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cells>,
}

impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
    
        let cells = (0..width * height)
        .map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cells::ALIVE
            } else {
                Cells::DEAD
            }
        }).collect();
    
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count: u8 = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
    
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let index = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[index] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cells::ALIVE, x) if x < 2 => Cells::DEAD,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cells::ALIVE, 2) | (Cells::ALIVE, 3) => Cells::ALIVE,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cells::ALIVE, x) if x > 3 => Cells::DEAD,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cells::DEAD, 3) => Cells::ALIVE,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
                next[index] = next_cell;
            }
        }
        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cells::DEAD { '◻' } else { '◼' };
                write!(f, "`{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}