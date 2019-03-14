use piston_window::{Context, G2d};
use crate::draw::{draw_block};

pub struct Snake {

}

impl Snake {
    pub fn new() -> Snake {
        Snake {

        }
    }

    pub fn draw(&self, context: &Context, g: &mut G2d) {
        draw_block([0.0, 1.0, 0.0, 1.0], 1, 2, context, g);
    }
}