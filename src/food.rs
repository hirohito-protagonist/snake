use piston_window::{Context, G2d};
use crate::draw::{draw_block};
use rand::{thread_rng, Rng};
use rand::rngs::{ThreadRng};
use crate::theme;

pub struct Food {
    x: u32,
    y: u32,
    rng:  ThreadRng,
}

impl Food {
    pub fn new(x: u32, y: u32) -> Food {
        Food{
            x,
            y,
            rng: thread_rng(),
        }
    }

    pub fn render(&self, context: &Context, g: &mut G2d) {
        draw_block(theme::FOOD_COLOR, self.x, self.y, context, g);
    }

    pub fn reposition(&mut self, width: u32, height: u32) {
        let new_x = self.rng.gen_range(1, width - 1);
        let new_y = self.rng.gen_range(1, height - 1);
        self.x = new_x;
        self.y = new_y;
    }

    pub fn position(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

#[cfg(test)]
mod food_tests {
    use crate::food::{Food};
    #[test]
    fn it_should_reposition_in_the_field_boundaries() {

        // Given
        let field_width = 10;
        let field_height = 10;
        let mut food = Food::new(200, 300);

        // When
        food.reposition(field_width, field_height);

        // Then
        let (food_x, food_y) = food.position();
        assert!(food_x <= field_width);
        assert!(food_y <= field_height);
    }
}