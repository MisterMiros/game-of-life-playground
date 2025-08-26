use game_of_life_pixel::{Game, GameConfig, window_conf};

#[macroquad::main(window_conf)]
async fn main() {
    let config = GameConfig::new(5000, 5000);
    let mut game = Game::new(config);
    game.start().await;
}
