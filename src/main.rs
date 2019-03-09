extern crate piston_window;
extern crate find_folder;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    // println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
    
    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g| {
            let transform = _c.transform.trans((640.0 / 2.0) - 70.0, 480.0 / 2.0);

            clear([0.0, 0.0, 0.0, 1.0], g);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                "Snake v0.0.1",
                &mut glyphs,
                &_c.draw_state,
                transform,
                g
            ).unwrap();
        });
    }
}