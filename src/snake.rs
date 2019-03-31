use std::collections::LinkedList;
use piston_window::{Context, G2d};
use crate::draw::{draw_block};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
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

        self.body.push_front(self.create_block(direction));
        self.body.pop_back().unwrap();
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
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

    pub fn add_tail(&mut self, direction: Option<Direction>) {
        match direction {
            Some(d) => self.direction = d,
            None => (),
        }
        self.body.push_back(self.create_block(direction));
    }

    fn create_block(&self, direction: Option<Direction>) -> Block {


        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };

        new_block
    }
}