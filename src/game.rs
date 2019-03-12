use piston_window::*;

use crate::draw::{draw_block, draw_rectangle};

pub struct Game {
    width: i32,
    height: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game{
            width,
            height,
        }
    }

    pub fn draw(&self, context: &Context, g: &mut G2d) {
        draw_block([0.0, 1.0, 0.0, 1.0], 1, 2, context, g);
        draw_rectangle([0.0, 1.0, 0.0, 1.0], 0, 0, self.width, 1, context, g);
        draw_rectangle([0.0, 1.0, 0.0, 1.0], 0, self.height - 1, self.width, 1, context, g);
        draw_rectangle([0.0, 1.0, 0.0, 1.0], 0, 0, 1, self.height, context, g);
        draw_rectangle([0.0, 1.0, 0.0, 1.0], self.width - 1, 0, 1, self.height, context, g);
    }
}