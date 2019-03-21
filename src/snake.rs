use std::collections::LinkedList;
use piston_window::{Context, G2d};
use crate::draw::{draw_block};

#[derive(Debug, Copy, Clone)]
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
    body: LinkedList<Block>,
    direction: Direction,
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
            direction: Direction::Right,
        }
    }

    pub fn draw(&self, context: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block([0.0, 1.0, 0.0, 1.0], block.x, block.y, context, g);
        }
    }

    pub fn move_forward(&mut self, direction: Option<Direction>) {
        
        match direction {
            Some(d) => self.direction = d,
            None => (),
        }

        let head_block = self.body.front().unwrap();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: head_block.x,
                y: head_block.y - 1,
            },
            Direction::Down => Block {
                x: head_block.x,
                y: head_block.y + 1,
            },
            Direction::Left => Block {
                x: head_block.x - 1,
                y: head_block.y,
            },
            Direction::Right => Block {
                x: head_block.x + 1,
                y: head_block.y,
            },
        };

        self.body.push_front(new_block);
        self.body.pop_back().unwrap();
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn is_tail_collision(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}