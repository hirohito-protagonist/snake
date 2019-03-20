use piston_window::*;

use crate::draw::{draw_rectangle};
use crate::snake::{Snake, Direction};


const MOVING_PERIOD: f64 = 0.1;

pub struct Game {
    snake: Snake,
    width: i32,
    height: i32,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game{
            snake: Snake::new(1, 1),
            width,
            height,
            waiting_time: 0.0,
        }
    }

    pub fn draw(&self, context: &Context, g: &mut G2d) {
        self.snake.draw(context, g);
        draw_rectangle([0.0, 1.0, 0.0, 1.0], 0, 0, self.width, 1, context, g);
        draw_rectangle([0.0, 1.0, 0.0, 1.0], 0, self.height - 1, self.width, 1, context, g);
        draw_rectangle([0.0, 1.0, 0.0, 1.0], 0, 0, 1, self.height, context, g);
        draw_rectangle([0.0, 1.0, 0.0, 1.0], self.width - 1, 0, 1, self.height, context, g);
    }

    pub fn key_pressed(&mut self, key: Key) {

        let direction = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction())
        };
        self.update_snake(direction);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn update_snake(&mut self, direction: Option<Direction>) {
        self.snake.move_forward(direction);
        self.waiting_time = 0.0;
    }
}