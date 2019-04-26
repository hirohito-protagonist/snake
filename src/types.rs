#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

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

pub struct State {
    pub waiting_time: f64,
    pub is_game_over: bool,
    pub food_exist: bool,
    pub score: i32,
    pub snake_speed: f64,
    pub is_pause: bool,
}

impl State {
    pub fn new() -> State {
        State{
            waiting_time: 0.0,
            is_game_over: false,
            food_exist: false,
            score: 0,
            snake_speed: 0.1,
            is_pause: false,
        }
    }

    pub fn speed_up_snake(&mut self) {
        if self.snake_speed > 0.04 {
            self.snake_speed = self.snake_speed - 0.01;
        }
    }

    pub fn increase_score(&mut self) {
        self.score = self.score + 1;
    }

    pub fn is_food_exist(&mut self, is_exists: bool) {
        self.food_exist = is_exists;
    }
}