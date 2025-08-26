use game_of_life_engine::{Cell, LifeEngine};
use macroquad::camera::{set_camera, Camera2D};
use macroquad::color::{BLACK, LIME, WHITE};
use macroquad::input::{
    is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, mouse_delta_position, mouse_position,
    mouse_wheel, KeyCode, MouseButton,
};
use macroquad::math::{clamp, vec2, Rect, Vec2};
use macroquad::prelude::{get_frame_time, get_screen_data};
use macroquad::shapes::{draw_rectangle, draw_rectangle_lines};
use macroquad::ui::{root_ui, Skin};
use macroquad::window::{clear_background, next_frame, Conf};
use std::cmp::min;

pub struct GameConfig {
    cols: u32,
    rows: u32,
    cell_size: f32,
    game_iterations_per_second: f32,
    min_zoom: f32,
    max_zoom: f32,
}

impl GameConfig {
    pub fn new(cols: u32, rows: u32) -> Self {
        Self {
            cols,
            rows,
            cell_size: 5.0,
            game_iterations_per_second: 10.0,
            min_zoom: 0.00001,
            max_zoom: 1.0,
        }
    }
}
pub struct Game {
    engine: LifeEngine,
    camera: Camera2D,
    is_running: bool,
    step: f32,
    accumulator: f32,
    random_cells_square_size: u32,
    min_zoom_vec: Vec2,
    max_zoom_vec: Vec2,
    skin: Skin,
    config: GameConfig,
}

impl Game {
    pub fn new(config: GameConfig) -> Self {
        let smaller_side = min(config.cols, config.rows);
        let random_cells_square_size = min(smaller_side / 10, 1000);

        let mut camera = Camera2D::from_display_rect(Rect::new(
            0.0,
            0.0,
            random_cells_square_size as f32 * config.cell_size,
            random_cells_square_size as f32 * config.cell_size,
        ));
        camera.zoom = camera.zoom.abs();

        let label_style = root_ui()
            .style_builder()
            .text_color(LIME)
            .font_size(30)
            .build();
        let skin = Skin {
            label_style,
            ..root_ui().default_skin()
        };

        Self {
            engine: LifeEngine::new(config.cols, config.rows),
            camera,
            is_running: false,
            step: 1.0 / config.game_iterations_per_second,
            accumulator: 0.0,
            random_cells_square_size,
            min_zoom_vec: Vec2::new(config.min_zoom, config.min_zoom),
            max_zoom_vec: Vec2::new(config.max_zoom, config.max_zoom),
            skin,
            config,
        }
    }

    pub async fn start(&mut self) {
        loop {
            self.on_update().await;
        }
    }

    async fn on_update(&mut self) {
        self.handle_run_toggle();
        self.handle_move();
        self.handle_zoom();
        self.handle_generate_square();
        self.handle_next_generation();
        self.handle_generate_square();

        self.run_engine();
        self.draw_cells();

        next_frame().await;
    }

    fn handle_run_toggle(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.is_running = !self.is_running;
            if self.is_running {
                self.accumulator = 0.0;
            }
        }
    }

    fn handle_move(&mut self) {
        if is_mouse_button_down(MouseButton::Right) {
            self.camera.target += mouse_delta_position() / self.camera.zoom;
        }
    }

    fn handle_zoom(&mut self) {
        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            let mp: Vec2 = mouse_position().into();
            let mp_world_before = self.camera.screen_to_world(mp);

            let new_zoom = Vec2::clamp(
                self.camera.zoom * (1.0 - wheel_y / 500.0),
                self.min_zoom_vec,
                self.max_zoom_vec,
            );

            self.camera.zoom = new_zoom;

            let mp_world_after = self.camera.screen_to_world(mp);
            self.camera.target += mp_world_before - mp_world_after;
        }
    }

    fn handle_generate_square(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let wp = self.camera.screen_to_world(mouse_position().into());
            let center_cell = self.to_cell(wp);
            let top_left_cell = Cell::new(
                clamp(
                    u32::saturating_sub(center_cell.x, self.random_cells_square_size / 2),
                    0,
                    self.config.cols - 1,
                ),
                clamp(
                    u32::saturating_sub(center_cell.y, self.random_cells_square_size / 2),
                    0,
                    self.config.rows - 1,
                ),
            );
            self.engine
                .generate_random_square(top_left_cell, self.random_cells_square_size);
        }
    }

    fn handle_next_generation(&mut self) {
        if is_key_pressed(KeyCode::Enter) {
            self.engine.next()
        }
    }

    fn run_engine(&mut self) {
        if self.is_running {
            self.accumulator += get_frame_time();
            while self.accumulator >= self.step {
                self.engine.next();
                self.accumulator -= self.step;
            }
        }
    }

    fn draw_cells(&mut self) {
        clear_background(WHITE);
        set_camera(&self.camera);
        let visible_top_left = self.camera.screen_to_world(Vec2::new(0.0, 0.0));
        let visible_top_right = self.camera.screen_to_world(Vec2::new(
            f32::from(get_screen_data().width),
            f32::from(get_screen_data().height),
        ));

        let visible_top_left_cell = self.to_cell(visible_top_left);
        let visible_top_right_cell = self.to_cell(visible_top_right);

        let cell_size = self.config.cell_size;
        self.engine
            .get_alive_cells()
            .filter(|c| {
                c.x >= visible_top_left_cell.x
                    && c.x <= visible_top_right_cell.x
                    && c.y >= visible_top_left_cell.y
                    && c.y <= visible_top_right_cell.y
            })
            .for_each(|c| {
                draw_rectangle(
                    c.x as f32 * cell_size,
                    c.y as f32 * cell_size,
                    cell_size,
                    cell_size,
                    BLACK,
                )
            });

        draw_rectangle_lines(
            0.0,
            0.0,
            self.config.cols as f32 * self.config.cell_size,
            self.config.rows as f32 * self.config.cell_size,
            3.0 * cell_size,
            BLACK,
        );

        root_ui().push_skin(&self.skin);
        root_ui().label(
            vec2(10.0, 10.0),
            if self.is_running { "Running" } else { "Paused" },
        );
        root_ui().pop_skin();
    }

    fn to_cell(&self, pos: Vec2) -> Cell {
        let x = clamp(
            (pos.x / self.config.cell_size).floor() as u32,
            0,
            self.config.cols - 1,
        );
        let y = clamp(
            (pos.y / self.config.cell_size).floor() as u32,
            0,
            self.config.rows - 1,
        );
        Cell::new(x, y)
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        window_width: 1024,
        window_height: 1024,
        window_resizable: true,
        ..Default::default()
    }
}
