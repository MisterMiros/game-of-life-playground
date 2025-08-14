use game_of_life_engine::{Cell, Engine};
use macroquad::prelude::*;

#[macroquad::main("Life")]
async fn main() {
    let mut engine = Engine::new(1000, 1000);
    let mut running = false;

    let mut cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));

    loop {
        // input
        if is_key_pressed(KeyCode::Space) {
            running = !running;
        }
        let (wx, wy) = mouse_position();

        // zoom with wheel
        let (_, wheel_y) = mouse_wheel(); // positive = zoom in
        if wheel_y != 0.0 {
            cam.zoom *= 1.0 + wheel_y * 0.1;
        }

        // pan with drag
        if is_mouse_button_down(MouseButton::Left) {
            let Vec2 { x: dx, y: dy } = mouse_delta_position();
            cam.target.x -= dx / cam.zoom.x;
            cam.target.y -= dy / cam.zoom.y;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let world = cam.screen_to_world(Vec2::new(wx, wy));
            let (cx, cy) = (world.x as u32, world.y as u32);
            engine.generate_random_square(Cell { x: cx, y: cy }, 32);
        }

        if running {
            engine.next();
        }

        // draw
        clear_background(BLACK);
        set_camera(&cam);
        for cell in engine.get_alive_cells() {
            draw_rectangle(cell.x as f32, cell.y as f32, 1.0, 1.0, WHITE);
        }
        set_default_camera();

        next_frame().await
    }
}
