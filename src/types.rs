#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Block {
    pub x: u32,
    pub y: u32,
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
    waiting_time: f64,
    is_game_over: bool,
    food_exist: bool,
    pub score: u32,
    snake_speed: f64,
    is_pause: bool,
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

    pub fn get_snake_speed(&self) -> f64 {
        self.snake_speed
    }

    pub fn increase_score(&mut self) {
        self.score = self.score + 1;
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn add_waiting_time(&mut self, t: f64) {
        self.waiting_time = self.waiting_time + t;
    }

    pub fn set_waiting_time(&mut self, t: f64) {
        self.waiting_time = t;
    }

    pub fn get_waiting_time(&self) -> f64 {
        self.waiting_time
    }

    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }

    pub fn set_game_over(&mut self, game_over: bool) {
        self.is_game_over = game_over;
    }

    pub fn is_pause(&self) -> bool {
        self.is_pause
    }

    pub fn set_pause(&mut self, pause: bool) {
        self.is_pause = pause;
    }

    pub fn is_food_exists(&self) -> bool {
        self.food_exist
    }

    pub fn set_food_exists(&mut self, food_exists: bool) {
        self.food_exist = food_exists;
    }
}

#[cfg(test)]
mod state_tests {
    use crate::types::{State};

    #[test]
    fn it_should_increase_score_by_one_point() {

        // Given
        let mut state = State::new();

        // When
        state.increase_score();
        state.increase_score();

        // Then
        assert_eq!(state.score, 2);
    }

    #[test]
    fn it_should_speed_up_snake_until_reach_max_speed() {
        // Given
        let mut state = State::new();

        // When
        state.speed_up_snake();
        state.speed_up_snake();
        state.speed_up_snake();
        state.speed_up_snake();
        state.speed_up_snake();
        state.speed_up_snake();
        state.speed_up_snake();
        state.speed_up_snake();
        state.speed_up_snake();
        state.speed_up_snake();
        state.speed_up_snake();
        state.speed_up_snake();

        // Then
        assert_eq!(state.snake_speed, 0.030000000000000013);
    }
}