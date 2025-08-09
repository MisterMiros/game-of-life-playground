use std::collections::hash_set::Iter;
use std::collections::HashSet;

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
            self.alive_cells.insert(cell.clone());
            self.potential_cells.insert(cell.clone());
            for neighbour in self.get_cell_neighbours(&cell) {
                self.potential_cells.insert(neighbour);           
            }
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

    pub fn alive_cells_iterator(&self) -> Iter<Cell> {
        self.alive_cells.iter()
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

/* ===== C-compatible FFI surface for C#/PInvoke ===== */

#[repr(C)]
pub struct IteratorContainer<'a> {
    pub iterator: Iter<'a, Cell>
}

// Create a new Engine and return an opaque pointer to it.
#[unsafe(no_mangle)]
pub extern "C" fn engine_new(cols: u32, rows: u32) -> *mut Engine {
    Box::into_raw(Box::new(Engine::new(cols, rows)))
}

// Destroy an Engine previously created by engine_new.
#[unsafe(no_mangle)]
pub extern "C" fn engine_free(ptr: *mut Engine) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}

// Advance the engine by one tick.
#[unsafe(no_mangle)]
pub extern "C" fn engine_next(ptr: *mut Engine) {
    if let Some(engine) = unsafe { ptr.as_mut() } {
        engine.next();
    }
}

// Activate a cell at (x, y).
#[unsafe(no_mangle)]
pub extern "C" fn engine_activate_cell(ptr: *mut Engine, x: u32, y: u32) {
    if let Some(engine) = unsafe { ptr.as_mut() } {
        engine.activate_cell(x, y);
    }
}

#[unsafe(no_mangle)]
fn engine_alive_cells_iterator_get<'a>(ptr: *const Engine) -> *mut IteratorContainer<'a> {
    if let Some(engine) = unsafe { ptr.as_ref() } {
        Box::into_raw(Box::new(IteratorContainer {
            iterator: engine.alive_cells.iter(),
        }))
    } else {
        std::ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
fn engine_cells_iterator_free(ptr: *mut IteratorContainer) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}

#[unsafe(no_mangle)]
fn engine_cells_iterator_next(
    ptr: *mut IteratorContainer,
) -> *const Cell {
    if let Some(container) = unsafe { ptr.as_mut() } {
        if let Some(cell) = container.iterator.next() {
            &raw const (*cell)
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
}

