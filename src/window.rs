use piston_window::*;

use crate::game::{Game};
use crate::theme;

pub struct GameWindow {
    window: PistonWindow,
    events: Events,
    font: Glyphs
}

impl GameWindow {
    pub fn new(viewport: (u32, u32)) -> GameWindow {
        const UPDATES_PER_SECOND: u64 = 60;

        let window_resolution: (u32, u32) = (viewport.0 * 10, (viewport.1 * 10) + 20);
        
        let window: PistonWindow = WindowSettings::new("Snake", window_resolution)
            .exit_on_esc(true)
            .vsync(true)
            .resizable(false)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

        let ref font = assets.join("FiraSans-Regular.ttf");
        let factory = window.factory.clone();
        let font = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
        

        let events = Events::new(EventSettings::new())
            .max_fps(UPDATES_PER_SECOND)
            .ups(UPDATES_PER_SECOND);
        
        GameWindow{
            window,
            events,
            font
        }
    }

    pub fn game_loop(&mut self, game: &mut Game) {
        let events = &mut self.events;
        let window = &mut self.window;
        let font = &mut self.font;

        while let Some(e) = events.next(window) {
            if let Some(Button::Keyboard(key)) = e.press_args() {
                game.key_pressed(key);
            }
            window.draw_2d(&e, |_c, g| {
                clear(theme::BACKGROUND_COLOR, g);
                game.render(&_c, g, font); 
            });

            e.update(|arg| {
                game.update(arg.dt);
            });
        }
    }
}