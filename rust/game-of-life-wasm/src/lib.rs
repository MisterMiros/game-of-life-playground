use game_of_life_engine::Cell;
#[cfg(not(feature = "gpu"))]
use game_of_life_engine::LifeEngine;
#[cfg(feature = "gpu")]
use game_of_life_gpu::LifeEngine;
use js_sys::{Array, Function, Number, Uint32Array};
use wasm_bindgen::prelude::*;

/* ===== WASM surface for JS/TS ===== */

#[wasm_bindgen]
pub struct LifeEngineWrapper {
    engine: LifeEngine,
}

#[wasm_bindgen]
impl LifeEngineWrapper {
    // Create a new Engine and return an opaque pointer to it.
    #[wasm_bindgen(constructor)]
    pub fn new(cols: u32, rows: u32) -> LifeEngineWrapper {
        LifeEngineWrapper {
            engine: LifeEngine::new(cols, rows),
        }
    }

    // Advance the engine by one tick.
    #[wasm_bindgen]
    pub fn next(&mut self) {
        self.engine.next();
    }

    // Activate a cell at (x, y).
    #[wasm_bindgen]
    pub fn activate_cell(&mut self, x: u32, y: u32) -> Result<(), String> {
        self.engine.activate_cell(x, y)
    }

    #[wasm_bindgen]
    pub fn activate_cells(&mut self, cells_x: Uint32Array, cells_y: Uint32Array) -> Result<(), String> {
        if cells_x.length() != cells_y.length() {
            return Err(String::from("cell arrays must have the same length"));
        }
        let mut cells = Vec::with_capacity(cells_x.length() as usize);
        for i in 0..cells_x.length() {
            let x = cells_x.get_index(i);
            let y = cells_y.get_index(i);
            cells.push(Cell::new(x, y));
        }
        self.engine.activate_cells(&cells)
    }

    // Generate a random square of cells.
    #[wasm_bindgen]
    pub fn generate_random_square(&mut self, top_left_x: u32, top_left_y: u32, size: u32) {
        self.engine
            .generate_random_square(Cell::new(top_left_x, top_left_y), size);
    }

    // Gets the amount of currently alive cells
    #[wasm_bindgen]
    pub fn get_alive_cells_count(&self) -> usize {
        self.engine.get_alive_cells_count()
    }

    // Iterates over cells and applies JS function to them
    #[wasm_bindgen]
    pub fn for_each_cell_do(&mut self, callback: &Function) {
        self.engine.get_alive_cells().for_each(|c| {
            let _ = callback.call2(&JsValue::NULL, &Number::from(c.x), &Number::from(c.y));
        });
    }
}
