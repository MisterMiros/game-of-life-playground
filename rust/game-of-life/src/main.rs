use std::{thread::sleep, time::Duration};

use game_of_life::Engine;

fn main() {
    let mut engine = Engine::new(256, 256);
    for i in (0..256).step_by(9) {
        for j in (0..256).step_by(9) {
            engine.activate_cell(0 + i, 1 + j);
            engine.activate_cell(1 + i, 2 + j);
            engine.activate_cell(2 + i, 0 + j);
            engine.activate_cell(2 + i, 1 + j);
            engine.activate_cell(2 + i, 2 + j);
        }
    }
    for i in 0..1000 {
        println!("Step: {i}, State: {engine:?}");
        engine.next();
        sleep(Duration::from_millis(10));
    }
}
