use game_of_life_engine::{Cell, LifeEngine};
use std::collections::HashSet;
use std::fmt::Write;
use std::io::{BufRead, Error};

trait Reader {
    fn read_line(&mut self, buf: &mut String) -> Result<usize, std::io::Error>;
}

struct StdinReader {
    stdin: std::io::Stdin,
}

impl StdinReader {
    fn new() -> Self {
        Self {
            stdin: std::io::stdin(),
        }
    }
}

impl Reader for StdinReader {
    fn read_line(&mut self, buf: &mut String) -> Result<usize, Error> {
        self.stdin.read_line(buf)
    }
}
struct FileReader {
    file: std::io::BufReader<std::fs::File>,
}

impl FileReader {
    fn new(path: String) -> Self {
        Self {
            file: std::io::BufReader::new(std::fs::File::open(path).unwrap()),
        }
    }
}

impl Reader for FileReader {
    fn read_line(&mut self, buf: &mut String) -> Result<usize, Error> {
        let result = self.file.read_line(buf);
        if let Ok(0usize) = result {
            buf.clear();
            buf.write_str("END").expect("Unable to write to buffer");
        }
        result
    }
}

pub fn run(file: Option<String>, size: u32) -> Result<(), String> {
    println!("Running Game of Life in console...");
    println!("Grid size: {}x{}", size, size);

    let initial_cells = match file {
        Some(path) => {
            println!("Reading initial cell configuration from file: {}", path);
            read_initial_cells(size, &mut FileReader::new(path))
        }
        None => {
            println!("Enter the initial cell configuration using the following format:");
            println!("- Each line should contain one cell position as x,y coordinates");
            println!("- Type 'END' on a new line when you have finished entering all cells");
            read_initial_cells(size, &mut StdinReader::new())
        }
    }?;

    let mut engine = LifeEngine::with_initial_cells(size, size, initial_cells.clone());

    println!("Initial alive cells: {}", engine.get_alive_cells_count());
    println!("Press 'N' to run the next generation, 'Q' to quit");
    let mut input = String::new();
    let stdin = std::io::stdin();
    loop {
        stdin
            .read_line(&mut input)
            .map_err(|_| "Unable to read line")?;
        if input.trim().eq_ignore_ascii_case("N") {
            let instant = std::time::Instant::now();
            engine.next();
            println!(
                "Next generation is ready. Active cells: {}. Elapsed time: {} ms",
                engine.get_alive_cells().count(),
                instant.elapsed().as_millis()
            );
        } else if input.trim().eq_ignore_ascii_case("Q") {
            break;
        }
        input.clear();
    }

    Ok(())
}

const READ_CELL_ERROR: &'static str = "Invalid cell format, aborting";
fn read_initial_cells(size: u32, reader: &mut impl Reader) -> Result<HashSet<Cell>, String> {
    let mut cells = HashSet::new();

    let mut input = String::new();
    loop {
        reader
            .read_line(&mut input)
            .map_err(|_| "Unable to read line for initial cells")?;

        if input.trim().eq_ignore_ascii_case("END") {
            break;
        }

        let split = input.trim().split(',').collect::<Vec<&str>>();
        if split.len() != 2 {
            return Err(READ_CELL_ERROR.to_string());
        }
        let x = split[0].parse::<u32>().map_err(|_| READ_CELL_ERROR)?;
        let y = split[1].parse::<u32>().map_err(|_| READ_CELL_ERROR)?;

        if x >= size || y >= size {
            return Err(format!("Invalid cell position: ({}, {}), aborting", x, y));
        }

        cells.insert(Cell::new(x, y));
        input.clear();
    }
    Ok(cells)
}
