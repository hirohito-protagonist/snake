extern crate piston_window;
extern crate find_folder;

use piston_window::*;

use crate::theme;


pub struct UI {
    width: i32,
    height: i32,
}

impl UI {
    
    pub fn new(viewport: (u32, u32)) -> UI {
        let (width, height) = viewport;
        UI{
            width: width as i32,
            height: height as i32,
        }
    }

    pub fn render_game_over(&self, context: &Context, g: &mut G2d, glyphs: &mut piston_window::glyph_cache::rusttype::GlyphCache<GfxFactory, G2dTexture>) {
        let pos_x = (((self.width * 10) as f32 / 2.0) - 60.0).into();
        let pos_y = (((self.height * 10) as f32 / 2.0) + 16.0).into();
        let game_over_pos = context.transform.trans(pos_x, pos_y);
        let reset_information_pos = context.transform.trans(pos_x - 25.0, pos_y + 18.0);

        text::Text::new_color(theme::TEXT_COLOR, 32).draw(
            "Game Over",
            glyphs,
            &context.draw_state,
            game_over_pos,
            g
        ).unwrap();

        text::Text::new_color(theme::TEXT_COLOR, 18).draw(
            "Press SPACEBAR to reset",
            glyphs,
            &context.draw_state,
            reset_information_pos,
            g
        ).unwrap();
    }

    pub fn render_pause(&self, context: &Context, g: &mut G2d, glyphs: &mut piston_window::glyph_cache::rusttype::GlyphCache<GfxFactory, G2dTexture>) {
        let pos_x = (((self.width * 10) as f32 / 2.0) - 60.0).into();
        let pos_y = (((self.height * 10) as f32 / 2.0) + 16.0).into();
        let pause_pos = context.transform.trans(pos_x, pos_y);

        text::Text::new_color(theme::TEXT_COLOR, 32).draw(
            "Pause",
            glyphs,
            &context.draw_state,
            pause_pos,
            g
        ).unwrap();
    }

    pub fn render_score(&self, score: u32, context: &Context, g: &mut G2d, glyphs: &mut piston_window::glyph_cache::rusttype::GlyphCache<GfxFactory, G2dTexture>) {
        let transform = context.transform.trans((self.width * 10 - 200).into(), (self.height * 10 + 15).into());
        
        text::Text::new_color(theme::TEXT_COLOR, 18).draw(
            &format!("Score: {}", score),
            glyphs,
            &context.draw_state,
            transform,
            g
        ).unwrap();
    }

    pub fn render_title(&self, context: &Context, g: &mut G2d, glyphs: &mut piston_window::glyph_cache::rusttype::GlyphCache<GfxFactory, G2dTexture>) {
        let transform = context.transform.trans(10.0,  (self.height * 10 + 15).into());

        text::Text::new_color(theme::TEXT_COLOR, 18).draw(
            "Snake v0.0.1",
            glyphs,
            &context.draw_state,
            transform,
            g
        ).unwrap();
    }
}