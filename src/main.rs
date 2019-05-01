extern crate piston_window;
extern crate find_folder;

use piston_window::*;

mod types;
mod draw;
mod theme;
mod food;
mod snake;
mod ui;
mod game;


fn main() {
    
    let viewport: (u32, u32) = (80, 58);
    let mut game = game::Game::new(viewport);
    create_window(viewport, &mut game);
}

fn create_window(viewport: (u32, u32), game: &mut game::Game) {
    const UPDATES_PER_SECOND: u64 = 60;

    let window_resolution: (u32, u32) = (viewport.0 * 10, (viewport.1 * 10) + 20);
    
    let mut window: PistonWindow = WindowSettings::new("Snake", window_resolution)
        .exit_on_esc(true)
        .vsync(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
    

    let mut events = Events::new(EventSettings::new())
        .max_fps(UPDATES_PER_SECOND)
        .ups(UPDATES_PER_SECOND);

    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&e, |_c, g| {
            clear(theme::BACKGROUND_COLOR, g);
            game.draw(&_c, g, &mut glyphs); 
        });

        e.update(|arg| {
            game.update(arg.dt);
        });
    }
}
