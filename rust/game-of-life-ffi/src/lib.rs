use std::collections::hash_set::Iter;
use game_of_life_engine::{Cell, Engine};

/* ===== C-compatible FFI surface for C#/PInvoke ===== */

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

// Generate a random square of cells.
#[unsafe(no_mangle)]
pub extern "C" fn engine_generate_random_square(
    ptr: *mut Engine,
    top_left_x: u32,
    top_left_y: u32,
    size: u32,
) {
    if let Some(engine) = unsafe { ptr.as_mut() } {
        engine.generate_random_square(Cell::new(top_left_x, top_left_y), size);
    }
}

// Produce an iterator over the alive cells.
#[unsafe(no_mangle)]
fn engine_alive_cells_iterator_get<'a>(ptr: *const Engine) -> *mut Iter<'a, Cell> {
    if let Some(engine) = unsafe { ptr.as_ref() } {
        Box::into_raw(Box::new(engine.get_alive_cells()))
    } else {
        std::ptr::null_mut()
    }
}

// Destroy an iterator previously created by engine_alive_cells_iterator_get.
#[unsafe(no_mangle)]
fn engine_alive_cells_iterator_free(ptr: *mut Iter<Cell>) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}

// Get the next alive cell from the iterator.
#[unsafe(no_mangle)]
fn engine_alive_cells_iterator_next(ptr: *mut Iter<Cell>) -> *const Cell {
    if let Some(iterator) = unsafe { ptr.as_mut() } {
        if let Some(cell) = iterator.next() {
            &raw const (*cell)
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
}
