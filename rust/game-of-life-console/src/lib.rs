use game_of_life_engine::{Cell, LifeEngine};
use itertools::Itertools;
use std::collections::HashSet;

pub struct ConsoleRunner {}

impl ConsoleRunner {
    pub fn run() -> Result<(), String> {
        println!("Running Game of Life in console...");
        println!(
            "Enter the size of the grid (columns and rows) using the following format: cols,rows"
        );
        let (cols, rows) = Self::read_grid_size()?;

        println!("Enter the initial cell configuration using the following format:");
        println!("- Each line should contain one cell position as x,y coordinates");
        println!("- Type 'END' on a new line when you have finished entering all cells");
        let initial_cells = Self::read_initial_cells(cols, rows)?;
        let mut engine = LifeEngine::new(cols, rows);
        initial_cells
            .into_iter()
            .for_each(|c| engine.activate_cell(c.x, c.y));

        println!("Initial alive cells: {}", engine.get_alive_cells_count());
        let mut input = String::new();
        let stdin = std::io::stdin();
        loop {
            stdin
                .read_line(&mut input)
                .map_err(|_| "Unable to read line")?;
            if (input.trim().eq_ignore_ascii_case("N")) {
                let mut instant = std::time::Instant::now();
                engine.next();
                println!("Next generation is ready. Active cells: {}. Elapsed time: {} ms", engine.get_alive_cells().count(), instant.elapsed().as_millis());
            } else if (input.trim().eq_ignore_ascii_case("Q")) {
                break;
            }
            input.clear();
        }

        Ok(())
    }

    const READ_GRID_SIZE_ERROR: &'static str = "Invalid grid format, aborting";
    fn read_grid_size() -> Result<(u32, u32), String> {
        let mut input = String::new();
        let stdin = std::io::stdin();
        stdin
            .read_line(&mut input)
            .map_err(|_| "Unable to read line for grid size")?;
        let split = input.trim().split(',').collect::<Vec<&str>>();
        if split.len() != 2 {
            return Err(Self::READ_GRID_SIZE_ERROR.to_string());
        }

        let cols = split[0]
            .parse::<u32>()
            .map_err(|_| Self::READ_GRID_SIZE_ERROR)?;

        let rows = split[1]
            .parse::<u32>()
            .map_err(|_| Self::READ_GRID_SIZE_ERROR)?;
        Ok((cols, rows))
    }

    const READ_CELL_ERROR: &'static str = "Invalid cell format, aborting";
    fn read_initial_cells(cols: u32, rows: u32) -> Result<HashSet<Cell>, String> {
        let mut cells = HashSet::new();

        let mut input = String::new();
        loop {
            let stdin = std::io::stdin();
            stdin
                .read_line(&mut input)
                .map_err(|_| "Unable to read line for initial cells")?;

            if input.trim().eq_ignore_ascii_case("END") {
                break;
            }

            let split = input.trim().split(',').collect::<Vec<&str>>();
            if split.len() != 2 {
                return Err(Self::READ_CELL_ERROR.to_string());
            }
            let x = split[0].parse::<u32>().map_err(|_| Self::READ_CELL_ERROR)?;
            let y = split[1].parse::<u32>().map_err(|_| Self::READ_CELL_ERROR)?;

            if x >= cols || y >= rows {
                return Err(format!("Invalid cell position: ({}, {}), aborting", x, y));
            }

            cells.insert(Cell::new(x, y));
            input.clear();
        }
        Ok(cells)
    }

    fn format_active_cells(engine: &LifeEngine) -> String {
        engine
            .get_alive_cells()
            .map(|c| format!("{},{}", c.x, c.y))
            .join("\n")
    }
}
