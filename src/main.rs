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
    
    const UPDATES_PER_SECOND: u64 = 60;

    
    let viewport: (u32, u32) = (80, 58);
    let window_resolution: (u32, u32) = (viewport.0 * 10, (viewport.1 * 10) + 20);
    
    let mut window: PistonWindow = WindowSettings::new("Snake", window_resolution)
        .exit_on_esc(true)
        .vsync(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    // println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
    
    let mut game = game::Game::new(viewport);

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
            
            let transform = _c.transform.trans(10.0, 595.0);

            text::Text::new_color(theme::TEXT_COLOR, 18).draw(
                "Snake v0.0.1",
                &mut glyphs,
                &_c.draw_state,
                transform,
                g
            ).unwrap();

            
        });

        e.update(|arg| {
            game.update(arg.dt);
        });
    }
}