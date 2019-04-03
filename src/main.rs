extern crate piston_window;
extern crate find_folder;

use piston_window::*;

mod draw;
mod theme;
mod snake;
mod game;

// use draw::draw_block;

fn main() {
    let (width, height) = (30, 30);
    let mut window: PistonWindow = WindowSettings::new("Snake", (draw::to_coord(width),  draw::to_coord(height)))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    // println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
    
    let mut game = game::Game::new(width as i32, height as i32);

    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&e, |_c, g| {
            clear(theme::BACKGROUND_COLOR, g);
            game.draw(&_c, g, &mut glyphs);
            
            let transform = _c.transform.trans((640.0 / 2.0) - 70.0, 480.0 / 2.0);

            text::Text::new_color(theme::TEXT_COLOR, 32).draw(
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