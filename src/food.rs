use piston_window::{Context, G2d};
use crate::draw::{draw_block};
use rand::{thread_rng, Rng};
use crate::theme;

pub struct Food {
    x: i32,
    y: i32,
}

impl Food {
    pub fn new(x: i32, y: i32) -> Food {
        Food{
            x,
            y
        }
    }

    pub fn draw(&self, context: &Context, g: &mut G2d) {
        draw_block(theme::FOOD_COLOR, self.x, self.y, context, g);
    }

    pub fn reposition(&mut self, width: i32, height: i32) {
        let mut rng = thread_rng();
        let new_x = rng.gen_range(1, width - 1);
        let new_y = rng.gen_range(1, height - 1);
        self.x = new_x;
        self.y = new_y;
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}