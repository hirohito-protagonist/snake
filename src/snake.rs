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

    pub fn is_alive(&self, dir: Option<Direction>, viewport: (u32, u32)) -> bool {
        let (next_x, next_y) = self.next_head(dir);

        if self.is_tail_collision(next_x, next_y) {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < viewport.0 - 1 && next_y < viewport.1 - 1
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

#[cfg(test)]
mod snake_tests {
    use crate::snake::{Snake};
    use crate::types::{Direction};

    #[test]
    fn it_should_be_created_from_3_segments_on_initial() {

        // Given
        let mut snake = Snake::new(0, 0);

        // When
        let (head_x, head_y) = snake.head_position();

        // Then
        assert_eq!(snake.body.len(), 3);
        assert_eq!(head_x, 2);
        assert_eq!(head_y, 0);
    }

    #[test]
    fn it_move_snake_and_store_lat_tail_position() {

        // Given
        let mut snake = Snake::new(0, 0);

        // When
        snake.move_forward(Some(Direction::Right));
        let (head_x, head_y) = snake.head_position();
        // Then
        assert_eq!(snake.direction, Direction::Right);
        assert_eq!(head_x, 3);
        assert_eq!(head_y, 0);
    }

    #[test]
    fn it_should_calculate_next_head_snake_position() {

        // Given
        let snake = Snake::new(0, 0);

        // When
        let actual_head_position = snake.head_position();
        let future_head_position = snake.next_head(Some(Direction::Right));

        // Then
        assert_eq!(actual_head_position.0, 2);
        assert_eq!(actual_head_position.1, 0);

        assert_eq!(future_head_position.0, 3);
        assert_eq!(future_head_position.1, 0);
    }

    #[test]
    fn it_should_add_tail_only_when_snake_was_moved_at_least_in_one_position() {

        // Given
        let mut snake = Snake::new(0, 0);

        // When
        snake.add_tail();
        snake.add_tail();
        snake.add_tail();
        snake.add_tail();
        snake.move_forward(Some(Direction::Right));
        snake.add_tail();
        snake.move_forward(Some(Direction::Right));
        snake.add_tail();

        // Then
        assert_eq!(snake.body.len(), 5);
    }

    #[test]
    fn it_should_return_false_when_body_is_not_in_collision_with_head() {

        // Given
        let mut snake = Snake::new(0, 0);
        let (head_x, head_y) = snake.next_head(Some(Direction::Right));

        // When
        let is_tail_collision = snake.is_tail_collision(head_x, head_y);

        // Then
        assert_eq!(is_tail_collision, false);
    }

    #[test]
    fn it_should_return_true_when_body_is_in_collision_with_head() {
        
        // Given
        let mut snake = Snake::new(0, 0);
        
        // When
        snake.move_forward(Some(Direction::Right));
        snake.add_tail();
        snake.add_tail();
        snake.add_tail();
        snake.add_tail();
        snake.move_forward(Some(Direction::Down));
        snake.move_forward(Some(Direction::Down));
        snake.move_forward(Some(Direction::Left));
        snake.move_forward(Some(Direction::Up));
        let (head_x, head_y) = snake.next_head(Some(Direction::Right));
        let is_tail_collision = snake.is_tail_collision(head_x, head_y);

        // Then
        assert_eq!(is_tail_collision, true);
    }
}