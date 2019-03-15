use piston_window::*;

use crate::draw::{draw_rectangle};
use crate::snake::{Snake};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    snake: Snake,
    width: i32,
    height: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game{
            snake: Snake::new(1, 1),
            width,
            height,
        }
    }

    pub fn draw(&self, context: &Context, g: &mut G2d) {
        self.snake.draw(context, g);
        draw_rectangle([0.0, 1.0, 0.0, 1.0], 0, 0, self.width, 1, context, g);
        draw_rectangle([0.0, 1.0, 0.0, 1.0], 0, self.height - 1, self.width, 1, context, g);
        draw_rectangle([0.0, 1.0, 0.0, 1.0], 0, 0, 1, self.height, context, g);
        draw_rectangle([0.0, 1.0, 0.0, 1.0], self.width - 1, 0, 1, self.height, context, g);
    }

    pub fn key_pressed(&self, key: Key) {

        let direction = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(Direction::Up)
        };
        println!("{:?}", direction);
    }
}