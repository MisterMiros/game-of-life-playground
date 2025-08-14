use rand::Rng;
use std::cmp::min;
use std::collections::HashSet;
use std::collections::hash_set::Iter;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
#[repr(C)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Cell {
        Cell { x, y }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Engine {
    cols: u32,
    rows: u32,
    alive_cells: HashSet<Cell>,
    potential_cells: HashSet<Cell>,
}

impl Engine {
    pub fn new(cols: u32, rows: u32) -> Engine {
        Engine {
            cols,
            rows,
            alive_cells: HashSet::new(),
            potential_cells: HashSet::new(),
        }
    }

    pub fn activate_cell(&mut self, x: u32, y: u32) {
        let cell = Cell::new(x, y);
        if self.is_cell_within_bounds(&cell) {
            self.activate_cell_internal(&cell);
        }
    }

    pub fn activate_cells(&mut self, cells: &[Cell]) {
        self.alive_cells.reserve(cells.len());
        // worst case scenario: each cell has 8 neighbors, but probably there would be a lot
        // of intersections when adding many cells at once
        // so less space is reserved than the worst case scenario
        self.potential_cells.reserve(cells.len() * 6);
        for cell in cells {
            self.activate_cell_internal(cell);
        }
    }

    pub fn next(&mut self) {
        let mut alive_cells_next: HashSet<Cell> =
            HashSet::with_capacity(self.alive_cells.capacity());
        let mut potential_cells_next: HashSet<Cell> =
            HashSet::with_capacity(self.potential_cells.capacity());

        for cell in &self.potential_cells {
            let is_alive = self.alive_cells.contains(cell);
            let neighbours = self.get_cell_neighbours(cell);
            let alive_neighbours_count = neighbours
                .iter()
                .filter(|c| self.alive_cells.contains(c))
                .count();
            if is_alive {
                if alive_neighbours_count == 2 || alive_neighbours_count == 3 {
                    alive_cells_next.insert(cell.clone());
                } else {
                    potential_cells_next.insert(cell.clone());
                    neighbours.into_iter().for_each(|c| {
                        potential_cells_next.insert(c);
                    });
                }
            } else {
                if alive_neighbours_count == 3 {
                    alive_cells_next.insert(cell.clone());
                    potential_cells_next.insert(cell.clone());
                    neighbours.into_iter().for_each(|c| {
                        potential_cells_next.insert(c);
                    });
                }
            }
        }

        self.alive_cells = alive_cells_next;
        self.potential_cells = potential_cells_next;
    }

    pub fn generate_random_square(&mut self, top_left: Cell, size: u32) {
        if !self.is_cell_within_bounds(&top_left) {
            return;
        }
        let bottom_right = Cell::new(
            min(top_left.x + size, self.cols) - 1,
            min(top_left.y + size, self.rows) - 1,
        );
        if top_left.x >= bottom_right.x || top_left.y >= bottom_right.y {
            return;
        }
        let area = (bottom_right.x - top_left.x) as usize * (bottom_right.y - top_left.y) as usize;
        let mut rng = rand::rng();
        let amount_to_generate = rng.random_range(0..area);

        self.alive_cells.reserve(amount_to_generate);
        self.potential_cells.reserve(amount_to_generate * 6);
        for _ in 0..amount_to_generate {
            let x = rng.random_range(top_left.x..bottom_right.x + 1);
            let y = rng.random_range(top_left.y..bottom_right.y + 1);
            self.activate_cell(x, y);
        }
    }

    pub fn get_alive_cells(&self) -> Iter<Cell> {
        self.alive_cells.iter()
    }

    fn activate_cell_internal(&mut self, cell: &Cell) {
        if self.is_cell_within_bounds(cell) {
            self.alive_cells.insert(cell.clone());
            self.potential_cells.insert(cell.clone());
            for neighbour in self.get_cell_neighbours(&cell) {
                self.potential_cells.insert(neighbour);
            }
        }
    }

    fn get_cell_neighbours(&self, cell: &Cell) -> Vec<Cell> {
        [
            Cell::new(cell.x - 1, cell.y - 1), // top left
            Cell::new(cell.x - 1, cell.y),     // left
            Cell::new(cell.x - 1, cell.y + 1), // bottom left
            Cell::new(cell.x, cell.y + 1),     // bottom
            Cell::new(cell.x + 1, cell.y + 1), // bottom right
            Cell::new(cell.x + 1, cell.y),     // right
            Cell::new(cell.x + 1, cell.y - 1), // top right
            Cell::new(cell.x, cell.y - 1),     // top
        ]
        .into_iter()
        .filter(|c| self.is_cell_within_bounds(c))
        .collect()
    }

    fn is_cell_within_bounds(&self, cell: &Cell) -> bool {
        cell.x < self.cols && cell.y < self.rows
    }
}
