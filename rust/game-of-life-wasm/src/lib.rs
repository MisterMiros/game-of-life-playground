use game_of_life_engine::{Cell, LifeEngine};
use js_sys::{Function, Number};
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
    pub fn activate_cell(&mut self, x: u32, y: u32) {
        self.engine.activate_cell(x, y);
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
        self.engine.get_alive_cells().for_each(|c: &Cell| {
            let _ = callback.call2(&JsValue::NULL, &Number::from(c.x), &Number::from(c.y));
        });
    }
}
