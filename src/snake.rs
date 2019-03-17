use std::collections::LinkedList;
use piston_window::{Context, G2d};
use crate::draw::{draw_block};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    body: LinkedList<Block>
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body = LinkedList::new();
        body.push_back(Block {
            x: x + 2,
            y,
        });
        body.push_back(Block {
            x: x + 1,
            y,
        });
        body.push_back(Block {
            x,
            y,
        });
        Snake {
            body,
        }
    }

    pub fn draw(&self, context: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block([0.0, 1.0, 0.0, 1.0], block.x, block.y, context, g);
        }
    }

    pub fn move_forward(&self, direction: Option<Direction>) {
        println!("Move snake");
    }
}