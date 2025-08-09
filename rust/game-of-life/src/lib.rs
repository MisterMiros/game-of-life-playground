use std::collections::HashSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct Cell {
    x: u32,
    y: u32,
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
            self.alive_cells.insert(cell);
        }
    }

    pub fn next(&mut self) {
        let mut alive_cells_next: HashSet<Cell> =
            HashSet::with_capacity(self.alive_cells.capacity());
        let mut potential_cells_next: HashSet<Cell> =
            HashSet::with_capacity(self.alive_cells.capacity() * 8);

        for cell in &self.potential_cells {
            let is_alive = self.alive_cells.contains(cell);
            let neighbours = self.get_cell_neighbours(cell);
            let alive_neighbouts_count = neighbours
                .iter()
                .filter(|c| self.alive_cells.contains(c))
                .count();
            if is_alive {
                if alive_neighbouts_count == 2 || alive_neighbouts_count == 3 {
                    alive_cells_next.insert(cell.clone());
                } else {
                    potential_cells_next.insert(cell.clone());
                    neighbours.into_iter().for_each(|c| {
                        potential_cells_next.insert(c);
                    });
                }
            } else {
                if alive_neighbouts_count == 3 {
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
