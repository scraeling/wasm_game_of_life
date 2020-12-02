mod utils;

use rand::{Rng, rngs::OsRng};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    infinite: bool,
    rng: OsRng,
}

#[wasm_bindgen]
impl Universe {
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn set_infinite(&mut self, state: bool) {
        self.infinite = state
    }

    pub fn activate_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx] = Cell::Alive;
    }

    pub fn new(height: u32, width: u32) -> Universe {
        let cells = (0..width * height).map(|_| Cell::Dead).collect();
        let rng = OsRng;

        Universe {
            width,
            height,
            cells,
            infinite: false,
            rng,
        }
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, mut row: u32, mut col: u32, delta: u32) -> u8 {
        let mut count = 0;
        if row == 0 { row = self.height}
        if col == 0 { col = self.width}
        for mut scan_row in (row - delta)..=(row + delta) {
            scan_row %= self.height;
            for mut scan_col in (col - delta)..=(col + delta) {
                scan_col %= self.width;
                if scan_row == row && scan_col == col {
                    continue;
                }
                let idx = self.get_index(scan_row, scan_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let live_neighbors = self.live_neighbor_count(row, col, 1);
                let cell = self.cells[idx];
                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (default, _) => default,
                };
                next[idx] = next_cell;
            }
        }
        if self.infinite {
            next[self.rng.gen_range(0, self.height * self.width) as usize] = Cell::Alive;
        }
        self.cells = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wierd_mirroring_thing_doesnt_happen() {
        let mut u = Universe::new(60, 120);
        u.activate_cell(0, 0);
        u.activate_cell(25, 45);
        let x = u
            .cells
            .into_iter()
            .filter_map(|cell| {
                if cell == Cell::Alive {
                    Some(true)
                } else {
                    None
                }
            })
            .collect::<Vec<bool>>()
            .len();
        assert_eq!(2, x);
    }

    #[test]
    fn runs_for_a_few_ticks() {
        let mut universe = Universe::new(10,20);
        universe.tick();
        universe.tick();
        universe.set_infinite(true);
        universe.tick();
        universe.tick();
        universe.tick();
        universe.set_infinite(false);
        universe.tick();
        universe.tick();
        universe.tick();
    }
}
