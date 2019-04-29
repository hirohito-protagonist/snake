use std::collections::LinkedList;
use piston_window::{Context, G2d};
use crate::draw::{draw_block};
use crate::types::{Block, Direction};
use crate::theme;



pub struct Snake {
    body: LinkedList<Block>,
    direction: Direction,
    last_tail_block: Option<Block>
}

impl Snake {
    pub fn new(x: u32, y: u32) -> Snake {
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
            last_tail_block: None,
        }
    }

    pub fn draw(&self, context: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(theme::SNAKE_COLOR, block.x, block.y, context, g);
        }
    }

    pub fn move_forward(&mut self, direction: Option<Direction>) {
        
        match direction {
            Some(d) => self.direction = d,
            None => (),
        }

        self.body.push_front(self.create_block());
        let tail = self.body.pop_back().unwrap();
        self.last_tail_block = Some(tail);
    }

    pub fn head_position(&self) -> (u32, u32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (u32, u32) {
        let (head_x, head_y): (u32, u32) = self.head_position();

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

    pub fn is_tail_collision(&self, x: u32, y: u32) -> bool {
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

    pub fn add_tail(&mut self) {
        match &self.last_tail_block {
            Some(block) => self.body.push_back(block.clone()),
            None => (),
        }
    }

    fn create_block(&self) -> Block {


        let (last_x, last_y): (u32, u32) = self.head_position();

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