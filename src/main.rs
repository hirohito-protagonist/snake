extern crate piston_window;
extern crate find_folder;


mod types;
mod draw;
mod theme;
mod food;
mod snake;
mod ui;
mod game;
mod window;


fn main() {
    
    let viewport: (u32, u32) = (80, 58);
    let mut game = game::Game::new(viewport);
    let mut game_window = window::GameWindow::new(viewport);
    game_window.game_loop(&mut game);
}
