// This code is adapted from https://github.com/rustwasm/wasm_game_of_life
// by six different contributors, licensed under the MIT and Apache 2.0 licenses.

#![no_std]

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

const UNIVERSE_WIDTH: u32 = 10;
const UNIVERSE_HEIGHT: u32 = 10;

pub struct Universe {
    cells: [Cell; 100],
}

impl Universe {
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * UNIVERSE_WIDTH + column) as usize
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 {
            UNIVERSE_HEIGHT - 1
        } else {
            row - 1
        };

        let south = if row == UNIVERSE_HEIGHT - 1 {
            0
        } else {
            row + 1
        };

        let west = if column == 0 {
            UNIVERSE_WIDTH - 1
        } else {
            column - 1
        };

        let east = if column == UNIVERSE_WIDTH - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }
}

/// Public methods, exported to JavaScript.
impl Universe {
    pub fn tick(&mut self) {
        // let _timer = Timer::new("Universe::tick");

        let mut next = self.cells.clone();

        for row in 0..UNIVERSE_HEIGHT {
            for col in 0..UNIVERSE_WIDTH {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = UNIVERSE_WIDTH;
        let height = UNIVERSE_HEIGHT;

        let mut cells = [Cell::Dead; (UNIVERSE_WIDTH * UNIVERSE_HEIGHT) as usize];

        for i in 0 .. (width * height) {
            if i % 2 == 0 || i % 7 == 0 {
                cells[i as usize] = Cell::Alive;
            }
        }

        Universe {
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        UNIVERSE_WIDTH
    }

    pub fn height(&self) -> u32 {
        UNIVERSE_HEIGHT
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
}
