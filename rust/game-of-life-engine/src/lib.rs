use mpsc::Receiver;
use rand::Rng;
use rustc_hash::{FxBuildHasher, FxHashSet};
use std::cmp::min;
use std::collections::HashSet;
use std::collections::hash_set::Iter;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
#[repr(C)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
}

enum Message {
    Start(usize),
    Add(Cell),
    Finish,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Cell {
        Cell { x, y }
    }
}

#[derive(Debug)]
pub struct LifeEngine {
    cols: u32,
    rows: u32,
    alive_cells: FxHashSet<Cell>,
    potential_cells: FxHashSet<Cell>,
    alive_cell_sender: Sender<Message>,
    potential_cell_sender: Sender<Message>,
    alive_cells_receiver: Receiver<FxHashSet<Cell>>,
    potential_cells_receiver: Receiver<FxHashSet<Cell>>,
}

fn start_alive_cells_thread() -> (Sender<Message>, Receiver<FxHashSet<Cell>>) {
    let (cell_sender, cell_receiver) = mpsc::channel::<Message>();
    let (set_sender, set_receiver) = mpsc::channel::<FxHashSet<Cell>>();
    thread::spawn(move || {
        let mut alive_cells: Option<FxHashSet<Cell>> = None;
        loop {
            let message = cell_receiver.recv();
            if let Err(_) = message {
                break;
            }
            match message.unwrap() {
                Message::Start(capacity) => {
                    alive_cells = Some(FxHashSet::with_capacity_and_hasher(
                        capacity,
                        FxBuildHasher::default(),
                    ));
                }
                Message::Add(cell) => {
                    if let Some(ref mut set) = alive_cells {
                        set.insert(cell);
                    }
                }
                Message::Finish => {
                    if let Some(set) = alive_cells.take() {
                        set_sender.send(set).unwrap();
                    }
                }
            }
        }
    });
    (cell_sender, set_receiver)
}

fn start_potential_cells_thread(
    cols: u32,
    rows: u32,
) -> (Sender<Message>, Receiver<FxHashSet<Cell>>) {
    let (cell_sender, cell_receiver) = mpsc::channel::<Message>();
    let (set_sender, set_receiver) = mpsc::channel::<FxHashSet<Cell>>();
    thread::spawn(move || {
        let mut potential_cells: Option<FxHashSet<Cell>> = None;
        let mut neighbours: Vec<Cell> = Vec::with_capacity(8);
        loop {
            let message = cell_receiver.recv();
            if let Err(_) = message {
                break;
            }
            match message.unwrap() {
                Message::Start(capacity) => {
                    potential_cells = Some(FxHashSet::with_capacity_and_hasher(
                        capacity,
                        FxBuildHasher::default(),
                    ));
                }
                Message::Add(cell) => {
                    if let Some(ref mut set) = potential_cells {
                        set.insert(cell.clone());
                        get_neighbours(cols, rows, &cell, &mut neighbours);
                        for neighbour in neighbours.iter() {
                            set.insert(neighbour.clone());
                        }
                    }
                }
                Message::Finish => {
                    if let Some(set) = potential_cells.take() {
                        set_sender.send(set).unwrap();
                        potential_cells = None;
                    }
                }
            }
        }
    });
    (cell_sender, set_receiver)
}

impl LifeEngine {
    pub fn new(cols: u32, rows: u32) -> LifeEngine {
        let (alive_cell_sender, alive_cells_receiver) = start_alive_cells_thread();
        let (potential_cell_sender, potential_cells_receiver) =
            start_potential_cells_thread(cols, rows);

        LifeEngine {
            cols,
            rows,
            alive_cells: FxHashSet::default(),
            potential_cells: FxHashSet::default(),
            alive_cell_sender,
            potential_cell_sender,
            alive_cells_receiver,
            potential_cells_receiver,
        }
    }

    pub fn with_initial_cells(cols: u32, rows: u32, initial_cells: HashSet<Cell>) -> LifeEngine {
        let mut alive_cells =
            FxHashSet::with_capacity_and_hasher(initial_cells.len(), FxBuildHasher::default());
        let mut potential_cells =
            FxHashSet::with_capacity_and_hasher(initial_cells.len() * 8, FxBuildHasher::default());
        alive_cells.extend(initial_cells);

        let mut neighbours: Vec<Cell> = Vec::with_capacity(8);
        for cell in alive_cells.iter() {
            potential_cells.insert(cell.clone());
            get_neighbours(cols, rows, cell, &mut neighbours);
            for neighbour in neighbours.iter() {
                potential_cells.insert(neighbour.clone());
            }
        }

        let (alive_cell_sender, alive_cells_receiver) = start_alive_cells_thread();
        let (potential_cell_sender, potential_cells_receiver) =
            start_potential_cells_thread(cols, rows);

        LifeEngine {
            cols,
            rows,
            alive_cells,
            potential_cells,
            alive_cell_sender,
            potential_cell_sender,
            alive_cells_receiver,
            potential_cells_receiver,
        }
    }

    pub fn activate_cells(&mut self, cells: HashSet<Cell>) {
        self.alive_cells.reserve(cells.len());
        self.potential_cells.reserve(cells.len() * 8);
        for cell in cells {
            self.do_activate_cell(cell);
        }
    }

    pub fn activate_cell(&mut self, x: u32, y: u32) {
        let cell = Cell::new(x, y);
        self.do_activate_cell(cell);
    }

    fn do_activate_cell(&mut self, cell: Cell) {
        if self.is_cell_within_bounds(&cell) {
            self.alive_cells.insert(cell.clone());
            self.potential_cells.insert(cell.clone());
            let mut neighbours = Vec::with_capacity(8);
            self.get_neighbours(&cell, &mut neighbours);
            for neighbour in neighbours {
                self.potential_cells.insert(neighbour);
            }
        }
    }

    pub fn next(&mut self) {
        self.alive_cell_sender.send(Message::Start(self.alive_cells.capacity())).unwrap();
        self.potential_cell_sender.send(Message::Start(self.potential_cells.capacity())).unwrap();

        let mut neighbours = Vec::with_capacity(8);
        for cell in &self.potential_cells {
            let is_alive = self.alive_cells.contains(cell);
            self.get_neighbours(cell, &mut neighbours);
            let alive_neighbours_count = neighbours
                .iter()
                .filter(|c| self.alive_cells.contains(c))
                .count();
            if is_alive {
                if alive_neighbours_count == 2 || alive_neighbours_count == 3 {
                    self.alive_cell_sender.send(Message::Add(cell.clone())).unwrap();
                } else {
                    self.potential_cell_sender.send(Message::Add(cell.clone())).unwrap();
                }
            } else {
                if alive_neighbours_count == 3 {
                    self.alive_cell_sender.send(Message::Add(cell.clone())).unwrap();
                    self.potential_cell_sender.send(Message::Add(cell.clone())).unwrap();
                }
            }
        }

        self.alive_cell_sender.send(Message::Finish).unwrap();
        self.potential_cell_sender.send(Message::Finish).unwrap();

        self.alive_cells = self.alive_cells_receiver.recv().unwrap();
        self.potential_cells = self.potential_cells_receiver.recv().unwrap();
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

    pub fn get_alive_cells(&'_ self) -> Iter<'_, Cell> {
        self.alive_cells.iter()
    }

    pub fn get_alive_cells_count(&self) -> usize {
        self.alive_cells.len()
    }

    fn get_neighbours(&self, cell: &Cell, container: &mut Vec<Cell>) {
        get_neighbours(self.cols, self.rows, cell, container);
    }

    fn is_cell_within_bounds(&self, cell: &Cell) -> bool {
        cell.x < self.cols && cell.y < self.rows
    }
}

fn get_neighbours(cols: u32, rows: u32, cell: &Cell, container: &mut Vec<Cell>) {
    container.clear();
    for dx in -1i32..=1i32 {
        if dx == -1 && cell.x == 0 {
            continue;
        }
        if dx == 1 && cell.x == cols - 1 {
            continue;
        }
        for dy in -1i32..=1i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if dy == -1 && cell.y == 0 {
                continue;
            }
            if dy == 1 && cell.y == rows - 1 {
                continue;
            }
            container.push(Cell::new(
                u32::saturating_add_signed(cell.x, dx),
                u32::saturating_add_signed(cell.y, dy),
            ));
        }
    }
}
